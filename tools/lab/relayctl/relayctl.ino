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

/*
 * relayctl: press the test machines' buttons from the development Mac.
 *
 * Board: Arduino Nano (ATmega328P, CH340 USB serial), driving a two-channel
 * 5 V relay module. Each relay's COM/NO contacts sit in parallel with a
 * push button (the PC's front-panel PWR_SW or RESET_SW, the Raspberry Pi's
 * RUN reset), so closing a relay for a while is pressing that button.
 *
 * Wiring:
 *   Nano D2  -> module IN1   (relay 1)
 *   Nano D3  -> module IN2   (relay 2)
 *   Nano 5V  -> module VCC
 *   Nano GND -> module GND
 *
 * Protocol: 115200 8N1, one ASCII command per line (CR, LF or both),
 * case-insensitive, channels numbered 1 and 2. Every command gets exactly
 * one reply line, starting with "OK" or "ERR".
 *
 *   ON <ch>              switch relay <ch> on (contacts closed) until OFF
 *   OFF <ch>             switch relay <ch> off (contacts open)
 *   PULSE <ch> <ms>      on for <ms> milliseconds, then off: a button press.
 *                        Replies at once; the pulse runs in the background
 *   STATUS               OK 1=<on|off> 2=<on|off>
 *   PING                 OK PONG
 *
 * Opening the serial port resets the Nano (DTR auto-reset), which switches
 * every relay off; the host waits for the "READY" line (about 2 s after
 * the open) before sending commands.
 */

/* ---- configuration ---- */

/*
 * Most of these modules switch a relay on when IN is pulled LOW (the input
 * drives an optocoupler's LED from VCC). Set to false for a module that
 * switches on HIGH.
 */
const bool RELAY_ACTIVE_LOW = true;

const uint8_t RELAY_PINS[] = { 2, 3 };
const uint8_t NRELAYS = sizeof(RELAY_PINS) / sizeof(RELAY_PINS[0]);

const unsigned long MIN_PULSE_MS = 10;
const unsigned long MAX_PULSE_MS = 15000;

const uint8_t LED_PIN = LED_BUILTIN;	/* lit while any relay is on */
const long BAUD = 115200;
const char VERSION[] = "relayctl 1";

/* ---- state ---- */

struct Relay {
	bool on;
	bool timed;		/* on for a pulse, switched off by relay_poll */
	unsigned long start;	/* millis() when the pulse began */
	unsigned long length;	/* pulse length in ms */
};

Relay relays[NRELAYS];

char line[48];
uint8_t linelen;
bool overflow;		/* the current line is too long; drop it */

/* ---- relays ---- */

void
relay_write(uint8_t ch, bool on)
{
	bool level = on != RELAY_ACTIVE_LOW;	/* HIGH is on when active-high */

	digitalWrite(RELAY_PINS[ch], level ? HIGH : LOW);
	relays[ch].on = on;
}

void
relay_on(uint8_t ch)
{
	relays[ch].timed = false;
	relay_write(ch, true);
}

void
relay_off(uint8_t ch)
{
	relays[ch].timed = false;
	relay_write(ch, false);
}

void
relay_pulse(uint8_t ch, unsigned long ms)
{
	relays[ch].timed = true;
	relays[ch].start = millis();
	relays[ch].length = ms;
	relay_write(ch, true);
}

/* Switches off the pulses whose time is up. Unsigned subtraction survives the
 * wrap of millis() after about 49 days. */
void
relay_poll(void)
{
	bool any = false;
	unsigned long now = millis();

	for (uint8_t ch = 0; ch < NRELAYS; ch++) {
		Relay *r = &relays[ch];

		if (r->on && r->timed && now - r->start >= r->length)
			relay_off(ch);
		any |= r->on;
	}
	digitalWrite(LED_PIN, any ? HIGH : LOW);
}

/* ---- command parsing ---- */

/* Next blank-separated word of *p, upper-cased in place; NULL at the end. */
char *
next_word(char **p)
{
	char *s = *p, *w;

	while (*s == ' ' || *s == '\t')
		s++;
	if (*s == '\0')
		return NULL;
	w = s;
	for (; *s != '\0' && *s != ' ' && *s != '\t'; s++)
		*s = toupper((unsigned char)*s);
	if (*s != '\0')
		*s++ = '\0';
	*p = s;
	return w;
}

/* Decimal number in [lo, hi]; false if w is missing, not a number or out
 * of range. */
bool
parse_num(const char *w, unsigned long lo, unsigned long hi,
    unsigned long *out)
{
	unsigned long v = 0;

	if (w == NULL || *w == '\0')
		return false;
	for (; *w != '\0'; w++) {
		if (*w < '0' || *w > '9')
			return false;
		v = v * 10 + (*w - '0');
		if (v > hi)
			return false;
	}
	if (v < lo)
		return false;
	*out = v;
	return true;
}

/* Channel number as the user writes it (1-based) to an index. */
bool
parse_channel(const char *w, uint8_t *ch)
{
	unsigned long v;

	if (!parse_num(w, 1, NRELAYS, &v))
		return false;
	*ch = v - 1;
	return true;
}

void
reply_status(void)
{
	Serial.print(F("OK"));
	for (uint8_t ch = 0; ch < NRELAYS; ch++) {
		Serial.print(' ');
		Serial.print(ch + 1);
		Serial.print('=');
		Serial.print(relays[ch].on ? F("on") : F("off"));
	}
	Serial.println();
}

void
command(char *p)
{
	char *cmd = next_word(&p);
	uint8_t ch;
	unsigned long ms;

	if (cmd == NULL)
		return;		/* empty line: no reply */

	if (strcmp(cmd, "PING") == 0) {
		Serial.println(F("OK PONG"));
	} else if (strcmp(cmd, "ON") == 0 || strcmp(cmd, "OFF") == 0) {
		bool on = cmd[1] == 'N';

		if (!parse_channel(next_word(&p), &ch)) {
			Serial.println(F("ERR bad channel"));
			return;
		}
		if (on)
			relay_on(ch);
		else
			relay_off(ch);
		Serial.println(F("OK"));
	} else if (strcmp(cmd, "PULSE") == 0) {
		if (!parse_channel(next_word(&p), &ch)) {
			Serial.println(F("ERR bad channel"));
			return;
		}
		if (!parse_num(next_word(&p), MIN_PULSE_MS, MAX_PULSE_MS,
		    &ms)) {
			Serial.print(F("ERR bad length, "));
			Serial.print(MIN_PULSE_MS);
			Serial.print(F(".."));
			Serial.print(MAX_PULSE_MS);
			Serial.println(F(" ms"));
			return;
		}
		relay_pulse(ch, ms);
		Serial.println(F("OK"));
	} else if (strcmp(cmd, "STATUS") == 0) {
		reply_status();
	} else {
		Serial.println(F("ERR unknown command"));
	}
}

void
serial_poll(void)
{
	while (Serial.available() > 0) {
		char c = Serial.read();

		if (c == '\r' || c == '\n') {
			if (overflow)
				Serial.println(F("ERR line too long"));
			else {
				line[linelen] = '\0';
				command(line);
			}
			linelen = 0;
			overflow = false;
		} else if (linelen < sizeof(line) - 1) {
			line[linelen++] = c;
		} else
			overflow = true;
	}
}

/* ---- main ---- */

void
setup(void)
{
	/*
	 * Set the open level before the pin becomes an output, so the relay
	 * does not click while the board starts.
	 */
	for (uint8_t ch = 0; ch < NRELAYS; ch++) {
		relay_off(ch);
		pinMode(RELAY_PINS[ch], OUTPUT);
	}
	pinMode(LED_PIN, OUTPUT);
	digitalWrite(LED_PIN, LOW);

	Serial.begin(BAUD);
	Serial.print(F("READY "));
	Serial.println(VERSION);
}

void
loop(void)
{
	serial_poll();
	relay_poll();
}
