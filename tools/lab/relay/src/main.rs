/* <LICENSES> */
/*
 * Copyright (c) 2026 Emilio Navarrete Lineros <enavarre@outlook.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
/* </LICENSES> */

/* <CODE> */
//! `relay`: drives the relay box (`tools/lab/relayctl/relayctl.ino`) from the Mac.
//!
//! ```text
//! relay [-p PORT] STEP...
//!   reset pc | reset pi | on N | off N | pulse N MS | status | ping | sleep MS
//! ```
//!
//! Each step is one command of the sketch's protocol (`sleep` waits on the host).
//! The machines stay powered; the relays are their reset buttons: relay 1 is the PC's
//! front-panel RESET_SW, relay 2 the Raspberry Pi 4's RUN pad (J2). `reset pc` and
//! `reset pi` press them for `RESET_PULSE_MS`.
//! Every reply is printed; the exit status is 0 when every reply is `OK`, 1 on an `ERR`
//! or a timeout, 2 on a usage error.
//!
//! Opening the port resets the Nano (DTR auto-reset), which switches every relay off;
//! so `on` lasts until the next run opens the port. A sequence that needs a relay held
//! goes in one run: `relay on 2 sleep 3000 off 2`. A pulse outlives the run: the Nano
//! times it.
//!
//! The port is `-p`, else `$RELAYCTL_PORT`, else `/dev/cu.usbserial-10` (where the Nano's
//! CH340 shows up on the development Mac). The line settings are made by stty(1) on the
//! open descriptor: macOS resets them at the last close, so they must be set while the
//! port is held.

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::process::{Command, ExitCode, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};

const DEFAULT_PORT: &str = "/dev/cu.usbserial-10";
/// The Nano's bootloader waits about 1.5 s after a reset before the sketch starts.
const READY_TIMEOUT: Duration = Duration::from_secs(4);
const REPLY_TIMEOUT: Duration = Duration::from_secs(2);
/// How long `reset` holds a reset line, in milliseconds.
const RESET_PULSE_MS: u64 = 300;
/// The relay on each machine's reset line.
const RESET_CHANNELS: [(&str, u64); 2] = [("pc", 1), ("pi", 2)];

#[derive(Debug, PartialEq, Eq)]
enum Step {
    /// A line sent to the sketch, which answers one line.
    Send(String),
    /// A pause on the host, in milliseconds.
    Sleep(u64),
}

/// Turns the command line's steps into `Step`s. Numbers are checked for shape here;
/// the sketch checks their ranges.
fn parse_steps(args: &[String]) -> Result<Vec<Step>, String> {
    let mut steps = Vec::new();
    let mut it = args.iter();
    let num = |w: Option<&String>, what: &str| -> Result<u64, String> {
        w.ok_or_else(|| format!("missing {what}"))?
            .parse::<u64>()
            .map_err(|_| format!("bad {what}"))
    };
    while let Some(word) = it.next() {
        match word.to_ascii_lowercase().as_str() {
            "on" | "off" => {
                let ch = num(it.next(), "channel")?;
                steps.push(Step::Send(format!("{} {ch}", word.to_ascii_uppercase())));
            }
            "pulse" => {
                let ch = num(it.next(), "channel")?;
                let ms = num(it.next(), "milliseconds")?;
                steps.push(Step::Send(format!("PULSE {ch} {ms}")));
            }
            "reset" => {
                let who = it.next().ok_or("missing machine (pc or pi)")?;
                let ch = RESET_CHANNELS
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(who))
                    .map(|&(_, ch)| ch)
                    .ok_or_else(|| format!("unknown machine `{who}` (pc or pi)"))?;
                steps.push(Step::Send(format!("PULSE {ch} {RESET_PULSE_MS}")));
            }
            "status" | "ping" => steps.push(Step::Send(word.to_ascii_uppercase())),
            "sleep" => steps.push(Step::Sleep(num(it.next(), "milliseconds")?)),
            other => return Err(format!("unknown step `{other}`")),
        }
    }
    if steps.is_empty() {
        return Err("no steps".into());
    }
    Ok(steps)
}

/// The serial line, split into lines as they arrive.
struct Port {
    file: File,
    pending: Vec<u8>,
}

impl Port {
    fn open(path: &str) -> Result<Port, String> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .map_err(|e| format!("{path}: {e}"))?;
        // 115200 8N1, raw, no flow control; a read returns after 0.1 s without data
        // (VMIN 0, VTIME 1), so the timeouts below are kept by the read loop.
        let stdin = file.try_clone().map_err(|e| format!("{path}: {e}"))?;
        let status = Command::new("stty")
            .args([
                "115200", "raw", "-echo", "cs8", "-parenb", "-cstopb", "clocal",
            ])
            .args(["-crtscts", "min", "0", "time", "1"])
            .stdin(Stdio::from(stdin))
            .status()
            .map_err(|e| format!("stty: {e}"))?;
        if !status.success() {
            return Err(format!("stty on {path} failed: {status}"));
        }
        Ok(Port {
            file,
            pending: Vec::new(),
        })
    }

    /// The next non-empty line, or `None` once `timeout` has passed.
    fn read_line(&mut self, timeout: Duration) -> Result<Option<String>, String> {
        let deadline = Instant::now() + timeout;
        loop {
            if let Some(nl) = self.pending.iter().position(|&b| b == b'\n') {
                let raw: Vec<u8> = self.pending.drain(..=nl).collect();
                let line = String::from_utf8_lossy(&raw).trim().to_string();
                if line.is_empty() {
                    continue;
                }
                return Ok(Some(line));
            }
            if Instant::now() >= deadline {
                return Ok(None);
            }
            let mut buf = [0u8; 64];
            match self.file.read(&mut buf) {
                Ok(n) => self.pending.extend_from_slice(&buf[..n]),
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(format!("read: {e}")),
            }
        }
    }

    fn send(&mut self, line: &str) -> Result<(), String> {
        self.file
            .write_all(format!("{line}\n").as_bytes())
            .map_err(|e| format!("write: {e}"))
    }

    /// Sends one command and returns its reply line.
    fn command(&mut self, line: &str) -> Result<String, String> {
        self.send(line)?;
        self.read_line(REPLY_TIMEOUT)?
            .ok_or_else(|| format!("no reply to `{line}`"))
    }

    /// Waits for the sketch: its `READY` line after the reset the open caused, or,
    /// if the board did not reset, an answer to `PING`.
    fn sync(&mut self) -> Result<String, String> {
        let deadline = Instant::now() + READY_TIMEOUT;
        while let Some(left) = deadline.checked_duration_since(Instant::now()) {
            match self.read_line(left)? {
                Some(line) if line.starts_with("READY") => return Ok(line),
                Some(_) => {} // noise from before the reset
                None => break,
            }
        }
        match self.command("PING")?.as_str() {
            "OK PONG" => Ok("no READY, but PING answered".into()),
            other => Err(format!("unexpected answer to PING: `{other}`")),
        }
    }
}

fn usage() -> ExitCode {
    eprintln!("usage: relay [-p PORT] STEP...");
    eprintln!("  STEP: reset pc | reset pi | on N | off N | pulse N MS | status | ping");
    eprintln!("        | sleep MS");
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let mut port = std::env::var("RELAYCTL_PORT").unwrap_or_else(|_| DEFAULT_PORT.into());
    if args.first().map(String::as_str) == Some("-p") {
        if args.len() < 2 {
            return usage();
        }
        port = args[1].clone();
        args.drain(..2);
    }
    let steps = match parse_steps(&args) {
        Ok(steps) => steps,
        Err(e) => {
            eprintln!("relay: {e}");
            return usage();
        }
    };
    match run(&port, &steps) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("relay: {e}");
            ExitCode::from(1)
        }
    }
}

/// Runs the steps; `Ok(false)` if some reply was not `OK`. Stops at the first error.
fn run(port: &str, steps: &[Step]) -> Result<bool, String> {
    let mut p = Port::open(port)?;
    eprintln!("relay: {port}: {}", p.sync()?);
    for step in steps {
        match step {
            Step::Sleep(ms) => sleep(Duration::from_millis(*ms)),
            Step::Send(line) => {
                let reply = p.command(line)?;
                println!("{line}: {reply}");
                if !reply.starts_with("OK") {
                    return Ok(false);
                }
            }
        }
    }
    Ok(true)
}
/* </CODE> */

/* <TESTS> */
#[cfg(test)]
mod tests {
    use super::*;

    fn steps(s: &str) -> Result<Vec<Step>, String> {
        let args: Vec<String> = s.split_whitespace().map(String::from).collect();
        parse_steps(&args)
    }

    #[test]
    fn parses_a_sequence() {
        assert_eq!(
            steps("on 2 sleep 3000 OFF 2 pulse 1 500 status ping").unwrap(),
            vec![
                Step::Send("ON 2".into()),
                Step::Sleep(3000),
                Step::Send("OFF 2".into()),
                Step::Send("PULSE 1 500".into()),
                Step::Send("STATUS".into()),
                Step::Send("PING".into()),
            ]
        );
    }

    #[test]
    fn reset_presses_the_machine_s_relay() {
        assert_eq!(
            steps("reset pc reset PI").unwrap(),
            vec![
                Step::Send("PULSE 1 300".into()),
                Step::Send("PULSE 2 300".into()),
            ]
        );
        assert!(steps("reset").is_err());
        assert!(steps("reset mac").is_err());
    }

    #[test]
    fn rejects_bad_steps() {
        assert!(steps("").is_err());
        assert!(steps("on").is_err());
        assert!(steps("on x").is_err());
        assert!(steps("pulse 1").is_err());
        assert!(steps("sleep -1").is_err());
        assert!(steps("press 1").is_err());
    }
}
/* </TESTS> */
