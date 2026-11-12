//! Golden vectors: encoded bytes checked into the repository, so a change to any encoding fails a
//! test.
//!
//! A set is one text file in `tests/vectors/`, embedded at compile time, so the same tests can
//! later run on Android and iOS where the repository isn't on disk. Each vector is a name on its
//! own line followed by its bytes in hex, 16 bytes per indented line, so a diff of the file shows
//! exactly which bytes moved.
//!
//! A failing vector means the change is wrong, unless it is an agreed format change. Two opt-in
//! modes write the file:
//!
//! - `UPDATE_VECTORS=new cargo test` adds vectors the file doesn't have yet. It never changes or
//!   removes one.
//! - `UPDATE_VECTORS=all cargo test` rewrites the whole set. Use it only for an agreed format
//!   change.

use std::fmt::Write as _;
use std::path::PathBuf;

/// Opens the set stored in `tests/vectors/<name>.txt`. A new set starts as an empty file.
macro_rules! golden_set {
    ($name:literal) => {
        $crate::golden::GoldenSet::new(
            $name,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/vectors/",
                $name,
                ".txt"
            )),
        )
    };
}
pub(crate) use golden_set;

/// Bytes per indented line in a vector file, and per row in a failure diff.
const ROW: usize = 16;

/// The vectors a test records, compared with the stored set by `finish`.
pub struct GoldenSet {
    name: &'static str,
    /// (name, lowercase hex) pairs from the file, in file order.
    stored: Vec<(String, String)>,
    /// (name, lowercase hex) pairs from this test run, in the order they were recorded.
    recorded: Vec<(String, String)>,
}

impl GoldenSet {
    /// Parses the stored set. `golden_set!` supplies the embedded file.
    pub fn new(name: &'static str, stored: &str) -> Self {
        Self {
            name,
            stored: parse(name, stored),
            recorded: Vec::new(),
        }
    }

    /// Saves `bytes` as the vector `name` for `finish` to compare with the stored set. Nothing is
    /// compared here, so a wrong byte only fails at `finish`.
    ///
    /// Panics if `name` is empty, uses anything but letters, digits and `_-./` (the characters
    /// the vector file can hold as a name), or was already recorded in this run.
    pub fn record(&mut self, name: &str, bytes: &[u8]) {
        assert!(
            !name.is_empty()
                && name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || "_-./".contains(c)),
            "golden vector name {name:?} may only use letters, digits and _-./"
        );
        assert!(
            self.recorded.iter().all(|(n, _)| n != name),
            "golden vector {name} is recorded twice"
        );
        self.recorded.push((name.to_owned(), hex::encode(bytes)));
    }

    /// Compares every recorded vector with the stored set, or writes the file when
    /// `UPDATE_VECTORS` asks for it.
    pub fn finish(self) {
        let mut changed = Vec::new();
        let mut diffs = String::new();
        let mut added = Vec::new();
        for (name, hex) in &self.recorded {
            match self.stored.iter().find(|(n, _)| n == name) {
                Some((_, stored)) if stored != hex => {
                    changed.push(name.as_str());
                    diffs.push_str(&diff(name, stored, hex));
                }
                Some(_) => {}
                None => added.push(name.as_str()),
            }
        }
        let removed: Vec<&str> = self
            .stored
            .iter()
            .map(|(n, _)| n.as_str())
            .filter(|n| self.recorded.iter().all(|(r, _)| r != n))
            .collect();

        let in_sync = changed.is_empty() && added.is_empty() && removed.is_empty();
        let write = match update_mode() {
            Mode::Check => false,
            Mode::New => changed.is_empty() && removed.is_empty() && !added.is_empty(),
            Mode::All => !in_sync,
        };
        if write {
            let path = self.path();
            std::fs::write(&path, self.render())
                .unwrap_or_else(|e| panic!("cannot write {}: {e}", path.display()));
            eprintln!(
                "golden set {}: wrote {} ({} added, {} changed, {} removed)",
                self.name,
                path.display(),
                added.len(),
                changed.len(),
                removed.len()
            );
            return;
        }
        if in_sync {
            return;
        }

        let mut report = format!(
            "golden set \"{}\" does not match tests/vectors/{}.txt.\n\
             A failing golden vector means the change is wrong, \
             unless it is an agreed format change.\n",
            self.name, self.name
        );
        if !changed.is_empty() {
            let _ = write!(report, "\nchanged:\n{diffs}");
        }
        if !added.is_empty() {
            let _ = writeln!(
                report,
                "\nnot in the file yet (UPDATE_VECTORS=new cargo test adds them):"
            );
            for name in &added {
                let _ = writeln!(report, "  {name}");
            }
        }
        if !removed.is_empty() {
            let _ = writeln!(report, "\nin the file but no longer recorded:");
            for name in &removed {
                let _ = writeln!(report, "  {name}");
            }
        }
        if !changed.is_empty() || !removed.is_empty() {
            report.push_str(
                "\nOnly for an agreed format change: \
                 UPDATE_VECTORS=all cargo test rewrites the set.\n",
            );
        }
        panic!("{report}");
    }

    fn path(&self) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/vectors")
            .join(format!("{}.txt", self.name))
    }

    fn render(&self) -> String {
        let mut out = format!(
            "# Golden vectors: the \"{}\" set, checked by tests/golden/mod.rs. \
             Don't edit by hand.\n\
             # Each name is followed by its bytes in hex, {ROW} bytes per indented line.\n",
            self.name
        );
        for (name, hex) in &self.recorded {
            out.push_str(name);
            out.push('\n');
            for row in hex.as_bytes().chunks(ROW * 2) {
                out.push_str("  ");
                out.push_str(std::str::from_utf8(row).expect("hex is ASCII"));
                out.push('\n');
            }
        }
        out
    }
}

enum Mode {
    Check,
    New,
    All,
}

fn update_mode() -> Mode {
    match std::env::var("UPDATE_VECTORS") {
        Err(_) => Mode::Check,
        Ok(mode) if mode == "new" => Mode::New,
        Ok(mode) if mode == "all" => Mode::All,
        Ok(other) => panic!(
            "UPDATE_VECTORS={other:?} is not a mode: use `new` to add vectors, \
             or `all` for an agreed format change"
        ),
    }
}

/// Reads a vector file: `#` comments, a name per unindented line, and its hex on the indented
/// lines below it.
fn parse(set: &str, text: &str) -> Vec<(String, String)> {
    let mut vectors: Vec<(String, String)> = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(row) = line.strip_prefix("  ") {
            let Some((_, hex)) = vectors.last_mut() else {
                panic!("{set}.txt line {}: hex before any vector name", index + 1);
            };
            assert!(
                row.len() % 2 == 0 && row.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')),
                "{set}.txt line {}: {row:?} is not lowercase hex",
                index + 1
            );
            hex.push_str(row);
        } else {
            vectors.push((line.to_owned(), String::new()));
        }
    }
    vectors
}

/// Shows the rows of two hex strings that differ, with `^^` under each differing byte.
fn diff(name: &str, expected: &str, actual: &str) -> String {
    let mut out = format!(
        "  {name}: {} bytes expected, {} bytes now\n",
        expected.len() / 2,
        actual.len() / 2
    );
    let rows = expected.len().max(actual.len()).div_ceil(ROW * 2);
    for row in 0..rows {
        let (e, a) = (hex_row(expected, row), hex_row(actual, row));
        if e == a {
            continue;
        }
        let marks: String = (0..ROW)
            .map(|i| {
                if e.get(2 * i..2 * i + 2) == a.get(2 * i..2 * i + 2) {
                    "   "
                } else {
                    "^^ "
                }
            })
            .collect();
        let _ = writeln!(out, "    {:04x}  expected {}", row * ROW, spaced(e));
        let _ = writeln!(out, "          actual   {}", spaced(a));
        let _ = writeln!(out, "                   {}", marks.trim_end());
    }
    out
}

fn hex_row(hex: &str, row: usize) -> &str {
    let start = (row * ROW * 2).min(hex.len());
    let end = ((row + 1) * ROW * 2).min(hex.len());
    &hex[start..end]
}

fn spaced(hex: &str) -> String {
    let pairs: Vec<&str> = hex
        .as_bytes()
        .chunks(2)
        .map(|pair| std::str::from_utf8(pair).expect("hex is ASCII"))
        .collect();
    pairs.join(" ")
}
