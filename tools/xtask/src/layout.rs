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
//! The zone markers of every `.rs` under `sys/` and `tools/` (milestone M15).
//!
//! A file is split into zones, each opened and closed by a comment line of its own, in this
//! order:
//!
//! ```text
//! /* $OpenBSD: ... */           ported files: the id lines, then
//! /* <LICENSES> */              the author's block, then the original notice, verbatim
//! /* </LICENSES> */
//! /* <CODE> */                  the `//!` docs, inner attributes, `use` lines, code
//! /* </CODE> */
//! /* <TESTS> */                 the tests, inline, at the end (only in files that have tests)
//! /* </TESTS> */
//! ```
//!
//! [`check`] validates one file; `cargo xtask ports check` runs it over the tree with the
//! licence policy `ports.toml` gives each path. Reading rule:
//! `sed -n '/<CODE>/,/<\/CODE>/p' file.rs`.
//!
//! Outside the zones there are only blank lines and, before the first zone, block comments (the
//! `$OpenBSD$` id lines, a "generated, do not edit" banner). Anything else is an error.
//! "Has test code" means the file declares a `mod tests` (inline); a `#[test]` or a `mod tests`
//! in the CODE zone is an error, as is a TESTS zone without a `mod tests`.

/// The author's notice, the first block of every `<LICENSES>` zone (the user's rule of
/// 2026-10-09, `.claude/rules/scope-and-stubs.md`): before the original notice of a port, or
/// alone in a file that has none.
pub(crate) const AUTHOR_BLOCK: &str = "\
/*
 * Copyright (c) 2026 Emilio Navarrete Lineros <enavarre@outlook.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED \"AS IS\" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
";

/// What a path's LICENSES zone holds after the author's block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Licenses {
    /// No original notice: not the port of a C file, or the port of one that carries no
    /// licence text upstream (`license = \"none\"`). The author's block is alone.
    Own,
    /// Only `todo` or `skipped` entries point here: either form.
    Either,
    /// The port of a C file with licence text: the author's block, then its notice, whole.
    Original,
}

const NAMES: [&str; 3] = ["LICENSES", "CODE", "TESTS"];
const LICENSES: usize = 0;
const CODE: usize = 1;
const TESTS: usize = 2;

/// The zone a marker line opens (`Some((zone, true))`) or closes (`Some((zone, false))`).
fn marker(line: &str) -> Option<(usize, bool)> {
    let t = line.trim_end();
    NAMES
        .iter()
        .position(|n| t == format!("/* <{n}> */"))
        .map_or_else(
            || {
                NAMES
                    .iter()
                    .position(|n| t == format!("/* </{n}> */"))
                    .map(|z| (z, false))
            },
            |z| Some((z, true)),
        )
}

/// Does `line` declare the module `tests` (`mod tests {`, `pub(crate) mod tests;`)?
fn declares_tests_mod(line: &str) -> bool {
    let t = line.trim_start();
    let t = t.strip_prefix("pub(crate) ").unwrap_or(t);
    let t = t.strip_prefix("pub ").unwrap_or(t);
    t.strip_prefix("mod tests")
        .is_some_and(|r| r.trim_start().starts_with(['{', ';']))
}

/// The lines of a block, without the blank lines around it.
fn trimmed<'a>(lines: &'a [&'a str]) -> &'a [&'a str] {
    let blank = |l: &&str| l.trim().is_empty();
    let start = lines.iter().position(|l| !blank(l)).unwrap_or(lines.len());
    let end = lines
        .iter()
        .rposition(|l| !blank(l))
        .map_or(start, |e| e + 1);
    &lines[start..end]
}

/// Validate what a `<LICENSES>` zone holds: the author's block, whole, once and first, then,
/// after one blank line, the original notice (by `lic`).
fn check_licenses(rel: &str, zone: &[&str], lic: Licenses) -> Vec<String> {
    let author: Vec<&str> = AUTHOR_BLOCK.lines().collect();
    let body = trimmed(zone);
    let copyright = author[1];
    let times = body.iter().filter(|l| l.trim_end() == copyright).count();
    if times == 0 {
        return vec![format!(
            "{rel}: <LICENSES> lacks the author's block (`AUTHOR_BLOCK`, whole, first)"
        )];
    }
    if times > 1 {
        return vec![format!(
            "{rel}: the author's block appears {times} times in <LICENSES>"
        )];
    }
    let starts_with_author = body.len() >= author.len()
        && body[..author.len()]
            .iter()
            .zip(&author)
            .all(|(l, a)| l.trim_end() == *a);
    if !starts_with_author {
        return vec![format!(
            "{rel}: the author's block is altered or not the first block of <LICENSES>"
        )];
    }
    let original = &body[author.len()..];
    let mut errs = Vec::new();
    // An `Original` zone may still lack a notice: a few ports of C files without one predate
    // `license = "none"` and hold the author's block alone.
    if lic == Licenses::Own && !original.is_empty() {
        errs.push(format!(
            "{rel}: <LICENSES> holds a notice in a file with no original one (not a port, or \
             `license = \"none\"`): only the author's block goes there"
        ));
    }
    if !original.is_empty()
        && (original.len() < 2 || !original[0].trim().is_empty() || original[1].trim().is_empty())
    {
        errs.push(format!(
            "{rel}: one blank line goes between the author's block and the original notice"
        ));
    }
    errs
}

/// Validate the zones of one file; `rel` only labels the messages. Returns the errors.
pub(crate) fn check(rel: &str, src: &str, lic: Licenses) -> Vec<String> {
    let mut errs: Vec<String> = Vec::new();
    // (opening line, closing line) of each zone, 1-based
    let mut seen: [(Option<usize>, Option<usize>); 3] = [(None, None); 3];
    let mut cur: Option<usize> = None;
    let mut last_opened: Option<usize> = None;
    let mut in_block = false;
    let mut first_zone_seen = false;
    let mut content: [Vec<&str>; 3] = [Vec::new(), Vec::new(), Vec::new()];

    for (i, line) in src.lines().enumerate() {
        let n = i + 1;
        match marker(line) {
            Some((z, true)) => {
                let name = NAMES[z];
                if let Some(c) = cur {
                    let cname = NAMES[c];
                    errs.push(format!("{rel}:{n}: <{name}> opened inside <{cname}>"));
                } else if seen[z].0.is_some() {
                    errs.push(format!("{rel}:{n}: <{name}> opened twice"));
                } else if last_opened.is_some_and(|l| l > z) {
                    errs.push(format!("{rel}:{n}: <{name}> is out of order"));
                }
                if cur.is_none() {
                    cur = Some(z);
                }
                seen[z].0.get_or_insert(n);
                last_opened = Some(last_opened.map_or(z, |l| l.max(z)));
                first_zone_seen = true;
            }
            Some((z, false)) => {
                let name = NAMES[z];
                if cur == Some(z) {
                    if seen[z].1.is_some() {
                        errs.push(format!("{rel}:{n}: </{name}> closed twice"));
                    }
                    seen[z].1.get_or_insert(n);
                    cur = None;
                } else {
                    errs.push(format!(
                        "{rel}:{n}: </{name}> closes a zone that is not open"
                    ));
                }
            }
            None => match cur {
                Some(z) => content[z].push(line),
                None => {
                    let t = line.trim();
                    if t.is_empty() {
                        continue;
                    }
                    if !first_zone_seen && (in_block || t.starts_with("/*")) {
                        if t.starts_with("/*") && !in_block {
                            in_block = true;
                        }
                        if in_block && t.contains("*/") {
                            in_block = false;
                        }
                        continue;
                    }
                    errs.push(format!("{rel}:{n}: text outside the zones: {t}"));
                }
            },
        }
    }
    if let Some(c) = cur {
        let name = NAMES[c];
        errs.push(format!("{rel}: <{name}> is never closed"));
    }
    for (z, name) in NAMES.iter().enumerate() {
        if seen[z].0.is_some() && seen[z].1.is_none() && cur != Some(z) {
            errs.push(format!("{rel}: <{name}> has no closing marker"));
        }
    }
    if seen[CODE].0.is_none() {
        errs.push(format!("{rel}: no <CODE> zone"));
    }
    if seen[LICENSES].0.is_some() {
        errs.extend(check_licenses(rel, &content[LICENSES], lic));
    } else {
        errs.push(format!(
            "{rel}: no <LICENSES> zone (every file carries the author's block; a port keeps its \
             notice before it)"
        ));
    }
    for l in &content[CODE] {
        if l.trim() == "#[test]" {
            errs.push(format!(
                "{rel}: a #[test] in the <CODE> zone (tests go in <TESTS>)"
            ));
            break;
        }
    }
    if content[CODE].iter().any(|l| declares_tests_mod(l)) {
        errs.push(format!(
            "{rel}: `mod tests` in the <CODE> zone (it goes in <TESTS>)"
        ));
    }
    if seen[TESTS].0.is_some() && !content[TESTS].iter().any(|l| declares_tests_mod(l)) {
        errs.push(format!("{rel}: a <TESTS> zone without a `mod tests`"));
    }
    errs
}
/* </CODE> */

/* <TESTS> */
#[cfg(test)]
mod tests {
    use super::*;

    /// A file from its lines (so no marker ever sits on a line of this file by itself).
    fn bare(lines: &[&str]) -> String {
        let mut s = lines.join("\n");
        s.push('\n');
        s
    }

    /// The `<LICENSES>` zone with the author's block alone, then a blank line.
    fn own_zone() -> Vec<&'static str> {
        let mut v = vec!["/* <LICENSES> */"];
        v.extend(AUTHOR_BLOCK.lines());
        v.extend(["/* </LICENSES> */", ""]);
        v
    }

    /// [`bare`], after [`own_zone`] unless the lines have a `<LICENSES>` zone of their own.
    fn file(lines: &[&str]) -> String {
        if lines.iter().any(|l| l.contains("<LICENSES>")) {
            return bare(lines);
        }
        let mut v = own_zone();
        v.extend_from_slice(lines);
        bare(&v)
    }

    const OK: [&str; 7] = [
        "/* <CODE> */",
        "//! docs",
        "fn f() {}",
        "/* </CODE> */",
        "",
        "/* <TESTS> */",
        "#[cfg(test)]",
    ];

    fn with_tests(extra: &[&str]) -> String {
        let mut v: Vec<&str> = OK.to_vec();
        v.extend_from_slice(extra);
        file(&v)
    }

    #[test]
    fn a_code_only_file_is_fine() {
        let s = file(&["/* <CODE> */", "fn f() {}", "/* </CODE> */"]);
        assert!(check("a.rs", &s, Licenses::Own).is_empty());
    }

    #[test]
    fn code_and_tests_are_fine() {
        let s = with_tests(&[
            "mod tests {",
            "    #[test]",
            "    fn t() {}",
            "}",
            "/* </TESTS> */",
        ]);
        assert_eq!(check("a.rs", &s, Licenses::Own), Vec::<String>::new());
    }

    #[test]
    fn a_ported_file_has_id_lines_licences_and_code() {
        let mut v = vec![
            "/*\t$OpenBSD: x.c,v 1.1 2020/01/01 00:00:00 a Exp $\t*/",
            "/* <LICENSES> */",
        ];
        v.extend(AUTHOR_BLOCK.lines());
        v.extend([
            "",
            "/* Copyright. */",
            "/* </LICENSES> */",
            "",
            "/* <CODE> */",
            "fn f() {}",
            "/* </CODE> */",
        ]);
        let s = file(&v);
        assert!(check("a.rs", &s, Licenses::Original).is_empty());
        assert!(check("a.rs", &s, Licenses::Either).is_empty());
        let e = check("a.rs", &s, Licenses::Own);
        assert_eq!(e.len(), 1, "{e:?}");
        assert!(e[0].contains("only the author's block goes there"));
    }

    #[test]
    fn a_banner_block_comment_before_the_first_zone_is_fine() {
        let mut v = vec!["/*", " * THIS FILE AUTOMATICALLY GENERATED.", " */"];
        v.extend(own_zone());
        v.extend(["/* <CODE> */", "/* </CODE> */"]);
        let s = bare(&v);
        assert!(check("a.rs", &s, Licenses::Own).is_empty());
    }

    #[test]
    fn every_file_needs_the_licences_zone_and_the_author_block() {
        let s = bare(&["/* <CODE> */", "/* </CODE> */"]);
        for lic in [Licenses::Own, Licenses::Either, Licenses::Original] {
            let e = check("a.rs", &s, lic);
            assert_eq!(e.len(), 1, "{e:?}");
            assert!(e[0].contains("no <LICENSES> zone"));
        }
        let s = bare(&[
            "/* <LICENSES> */",
            "/* Copyright. */",
            "/* </LICENSES> */",
            "/* <CODE> */",
            "/* </CODE> */",
        ]);
        let e = check("a.rs", &s, Licenses::Original);
        assert!(
            e.iter().any(|m| m.contains("lacks the author's block")),
            "{e:?}"
        );
    }

    #[test]
    fn code_is_required() {
        let e = check("a.rs", "fn f() {}\n", Licenses::Own);
        assert!(e.iter().any(|m| m.contains("no <LICENSES> zone")), "{e:?}");
        assert!(
            e.iter().any(|m| m.contains("text outside the zones")),
            "{e:?}"
        );
        assert!(e.iter().any(|m| m.contains("no <CODE> zone")), "{e:?}");
    }

    #[test]
    fn text_between_or_after_zones_is_an_error() {
        let s = file(&["/* <CODE> */", "/* </CODE> */", "fn g() {}"]);
        let e = check("a.rs", &s, Licenses::Own);
        assert_eq!(e.len(), 1, "{e:?}");
        let line = own_zone().len() + 3;
        assert!(e[0].starts_with(&format!("a.rs:{line}:")), "{e:?}");
        // a comment after the first zone is text too
        let s = file(&["/* <CODE> */", "/* </CODE> */", "/* late */"]);
        assert_eq!(check("a.rs", &s, Licenses::Own).len(), 1);
    }

    #[test]
    fn zones_open_and_close_once_in_order() {
        // CODE twice
        let s = file(&[
            "/* <CODE> */",
            "/* </CODE> */",
            "/* <CODE> */",
            "/* </CODE> */",
        ]);
        assert!(
            check("a.rs", &s, Licenses::Own)
                .iter()
                .any(|m| m.contains("twice"))
        );
        // TESTS before CODE
        let s = file(&[
            "/* <TESTS> */",
            "mod tests {}",
            "/* </TESTS> */",
            "/* <CODE> */",
            "/* </CODE> */",
        ]);
        assert!(
            check("a.rs", &s, Licenses::Own)
                .iter()
                .any(|m| m.contains("out of order"))
        );
        // LICENSES after CODE
        let s = file(&[
            "/* <CODE> */",
            "/* </CODE> */",
            "/* <LICENSES> */",
            "/* </LICENSES> */",
        ]);
        assert!(
            check("a.rs", &s, Licenses::Either)
                .iter()
                .any(|m| m.contains("out of order"))
        );
    }

    #[test]
    fn unclosed_nested_and_stray_markers() {
        let s = file(&["/* <CODE> */", "fn f() {}"]);
        assert!(
            check("a.rs", &s, Licenses::Own)
                .iter()
                .any(|m| m.contains("never closed"))
        );
        let s = file(&[
            "/* <CODE> */",
            "/* <TESTS> */",
            "/* </TESTS> */",
            "/* </CODE> */",
        ]);
        assert!(
            check("a.rs", &s, Licenses::Own)
                .iter()
                .any(|m| m.contains("inside"))
        );
        let s = file(&["/* <CODE> */", "/* </CODE> */", "/* </CODE> */"]);
        assert!(
            check("a.rs", &s, Licenses::Own)
                .iter()
                .any(|m| m.contains("not open"))
        );
        let s = file(&["/* </CODE> */"]);
        assert!(
            check("a.rs", &s, Licenses::Own)
                .iter()
                .any(|m| m.contains("not open"))
        );
    }

    #[test]
    fn tests_belong_in_the_tests_zone() {
        let s = file(&[
            "/* <CODE> */",
            "#[cfg(test)]",
            "mod tests {",
            "}",
            "/* </CODE> */",
        ]);
        let e = check("a.rs", &s, Licenses::Own);
        assert!(
            e.iter()
                .any(|m| m.contains("`mod tests` in the <CODE> zone")),
            "{e:?}"
        );
        let s = file(&[
            "/* <CODE> */",
            "#[cfg(test)]",
            "mod tests;",
            "/* </CODE> */",
        ]);
        assert!(!check("a.rs", &s, Licenses::Own).is_empty());
        let s = file(&["/* <CODE> */", "#[test]", "fn t() {}", "/* </CODE> */"]);
        let e = check("a.rs", &s, Licenses::Own);
        assert!(e.iter().any(|m| m.contains("#[test]")), "{e:?}");
        // an indented `mod tests` (a nested module) or a helper named like it is not flagged
        let s = file(&[
            "/* <CODE> */",
            "    mod tests_helper {}",
            "mod testsuite {}",
            "/* </CODE> */",
        ]);
        assert!(
            check("a.rs", &s, Licenses::Own)
                .iter()
                .all(|m| !m.contains("mod tests"))
        );
    }

    #[test]
    fn a_tests_zone_needs_a_tests_module() {
        let s = file(&[
            "/* <CODE> */",
            "/* </CODE> */",
            "/* <TESTS> */",
            "fn helper() {}",
            "/* </TESTS> */",
        ]);
        let e = check("a.rs", &s, Licenses::Own);
        assert!(
            e.iter().any(|m| m.contains("without a `mod tests`")),
            "{e:?}"
        );
    }

    #[test]
    fn markers_inside_a_line_are_not_markers() {
        let s = file(&[
            "/* <CODE> */",
            "let a = \"/* <TESTS> */\";",
            "/* </CODE> */",
        ]);
        assert!(check("a.rs", &s, Licenses::Own).is_empty());
    }

    /// A file whose `<LICENSES>` zone holds the author's block, then `notice` (lines).
    fn with_author(notice: &[&str]) -> String {
        let mut v: Vec<&str> = vec!["/* <LICENSES> */"];
        v.extend(AUTHOR_BLOCK.lines());
        if !notice.is_empty() {
            v.push("");
        }
        v.extend_from_slice(notice);
        v.extend(["/* </LICENSES> */", "", "/* <CODE> */", "/* </CODE> */"]);
        file(&v)
    }

    #[test]
    fn the_author_block_alone_or_before_the_notice() {
        let own = with_author(&[]);
        assert_eq!(check("a.rs", &own, Licenses::Own), Vec::<String>::new());
        assert!(check("a.rs", &own, Licenses::Either).is_empty());
        assert!(check("a.rs", &own, Licenses::Original).is_empty());

        let port = with_author(&["/*", " * Copyright. */"]);
        assert_eq!(
            check("a.rs", &port, Licenses::Original),
            Vec::<String>::new()
        );
        assert!(check("a.rs", &port, Licenses::Either).is_empty());
        let e = check("a.rs", &port, Licenses::Own);
        assert!(
            e.iter().any(|m| m.contains("only the author's block")),
            "{e:?}"
        );
    }

    #[test]
    fn the_author_block_is_first_whole_once_and_before_a_blank_line() {
        // after the original notice
        let mut v: Vec<&str> = vec!["/* <LICENSES> */", "/* Copyright. */", ""];
        v.extend(AUTHOR_BLOCK.lines());
        v.extend(["/* </LICENSES> */", "/* <CODE> */", "/* </CODE> */"]);
        let e = check("a.rs", &file(&v), Licenses::Original);
        assert!(e.iter().any(|m| m.contains("not the first block")), "{e:?}");
        // altered
        let altered = with_author(&[]).replace("ANY SPECIAL", "SOME SPECIAL");
        let e = check("a.rs", &altered, Licenses::Own);
        assert!(e.iter().any(|m| m.contains("altered")), "{e:?}");
        // twice
        let mut v: Vec<&str> = vec!["/* <LICENSES> */"];
        v.extend(AUTHOR_BLOCK.lines());
        v.push("");
        v.extend(AUTHOR_BLOCK.lines());
        v.extend(["/* </LICENSES> */", "/* <CODE> */", "/* </CODE> */"]);
        let e = check("a.rs", &file(&v), Licenses::Own);
        assert!(e.iter().any(|m| m.contains("2 times")), "{e:?}");
        // glued to the notice
        let mut v: Vec<&str> = vec!["/* <LICENSES> */"];
        v.extend(AUTHOR_BLOCK.lines());
        v.extend([
            "/* Copyright. */",
            "/* </LICENSES> */",
            "/* <CODE> */",
            "/* </CODE> */",
        ]);
        let e = check("a.rs", &file(&v), Licenses::Original);
        assert!(e.iter().any(|m| m.contains("one blank line")), "{e:?}");
    }

    #[test]
    fn the_tests_mod_forms() {
        assert!(declares_tests_mod("mod tests {"));
        assert!(declares_tests_mod("mod tests;"));
        assert!(declares_tests_mod("pub(crate) mod tests;"));
        assert!(declares_tests_mod("pub mod tests {"));
        assert!(!declares_tests_mod("mod testsuite {"));
        assert!(!declares_tests_mod("// mod tests {"));
    }
}
/* </TESTS> */
