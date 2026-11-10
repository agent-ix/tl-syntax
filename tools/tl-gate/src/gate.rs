//! The gate: cargo-mutants outcomes against `mutation-thresholds.toml`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use serde::Deserialize;
use serde_json::Value;

/// Whole-crate run or a `--in-diff` pull-request run.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    /// Every group is judged against its own floor; an empty group fails.
    Full,
    /// Every mutant in the changed code is judged against
    /// `diff_min_kill_percent`; an empty population passes.
    Diff,
}

#[derive(Debug, Deserialize)]
struct Group {
    name: String,
    globs: Vec<String>,
    min_kill_percent: f64,
}

#[derive(Debug, Deserialize)]
struct Survivor {
    file: String,
    #[serde(default)]
    names: Vec<String>,
    #[serde(default)]
    reason: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Thresholds {
    #[serde(default)]
    diff_min_kill_percent: Option<f64>,
    #[serde(default)]
    exclude: Vec<String>,
    #[serde(default)]
    group: Vec<Group>,
    #[serde(default)]
    survivor: Vec<Survivor>,
    /// When true every missed or timed-out mutant must be listed under
    /// `[[survivor]]` with a non-empty reason (FR-022-AC-5).
    #[serde(default)]
    require_survivor_reasons: bool,
}

/// Result of judging one run.
#[derive(Debug, Eq, PartialEq)]
pub struct Judgement {
    /// Exit status: 0 pass, 1 failure.
    pub status: u8,
    /// Report lines, in print order.
    pub lines: Vec<String>,
    /// Diagnostics for standard error.
    pub errors: Vec<String>,
}

#[derive(Default)]
struct Counts {
    caught: u64,
    missed: u64,
    timeout: u64,
    unviable: u64,
}

/// Shell-style match (`*`, `?`, `[...]`); `*` also crosses `/`, as fnmatch(3) does
/// without FNM_PATHNAME, so the thresholds files keep their meaning.
fn fnmatch(pattern: &[char], text: &[char]) -> bool {
    match pattern.first() {
        None => text.is_empty(),
        Some('*') => (0..=text.len()).any(|i| fnmatch(&pattern[1..], &text[i..])),
        Some('?') => !text.is_empty() && fnmatch(&pattern[1..], &text[1..]),
        Some('[') => {
            let Some(close) = pattern[1..].iter().skip(1).position(|c| *c == ']') else {
                return text.first() == Some(&'[') && fnmatch(&pattern[1..], &text[1..]);
            };
            let class = &pattern[1..close + 2];
            let (negate, class) = match class.first() {
                Some('!') => (true, &class[1..]),
                _ => (false, class),
            };
            let Some(&c) = text.first() else { return false };
            let mut hit = false;
            let mut i = 0;
            while i < class.len() {
                if i + 2 < class.len() && class[i + 1] == '-' {
                    hit |= class[i] <= c && c <= class[i + 2];
                    i += 3;
                } else {
                    hit |= class[i] == c;
                    i += 1;
                }
            }
            hit != negate && fnmatch(&pattern[close + 3..], &text[1..])
        }
        Some(p) => text.first() == Some(p) && fnmatch(&pattern[1..], &text[1..]),
    }
}

fn matches(pattern: &str, path: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = path.chars().collect();
    fnmatch(&p, &t)
}

/// Strip the `file:line:col: ` prefix so a listing survives edits that move code.
fn strip_position(name: &str) -> &str {
    let mut parts = name.splitn(4, ':');
    let (Some(file), Some(line), Some(col), Some(rest)) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return name;
    };
    let numeric = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    if !file.is_empty()
        && !file.contains(char::is_whitespace)
        && numeric(line)
        && numeric(col)
        && rest.starts_with(' ')
    {
        &rest[1..]
    } else {
        name
    }
}

/// Judge `outcomes` (the concatenated `outcomes` arrays of the merged runs).
pub fn judge(outcomes: &[Value], thresholds_text: &str, mode: Mode) -> Result<Judgement, String> {
    let thresholds: Thresholds = toml::from_str(thresholds_text).map_err(|e| e.to_string())?;
    let groups: Vec<Group> = match mode {
        Mode::Diff => vec![Group {
            name: "diff".into(),
            globs: vec!["*".into()],
            min_kill_percent: thresholds
                .diff_min_kill_percent
                .ok_or("diff_min_kill_percent is missing from the thresholds file")?,
        }],
        Mode::Full => thresholds.group,
    };
    let mut counts: BTreeMap<&str, Counts> = groups
        .iter()
        .map(|g| (g.name.as_str(), Counts::default()))
        .collect();
    let listed: BTreeMap<(&str, &str), &str> = thresholds
        .survivor
        .iter()
        .flat_map(|s| {
            s.names
                .iter()
                .map(move |n| ((s.file.as_str(), n.as_str()), s.reason.trim()))
        })
        .collect();

    let mut baseline_ok = true;
    let mut merged: BTreeMap<(String, String), &Value> = BTreeMap::new();
    for outcome in outcomes {
        let scenario = &outcome["scenario"];
        if scenario == "Baseline" {
            baseline_ok &= outcome["summary"] == "Success";
            continue;
        }
        let mutant = &scenario["Mutant"];
        let field = |k: &str| {
            mutant[k]
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| format!("outcome without a mutant {k}"))
        };
        merged.insert((field("file")?, field("name")?), outcome);
    }

    let mut unreasoned = BTreeSet::new();
    let mut errors = Vec::new();
    for ((path, mutant_name), outcome) in &merged {
        if thresholds.exclude.iter().any(|p| matches(p, path)) {
            continue;
        }
        let summary = outcome["summary"].as_str().unwrap_or("");
        let key = match summary {
            "CaughtMutant" => 0,
            "MissedMutant" => 1,
            "Timeout" => 2,
            "Unviable" => 3,
            other => {
                errors.push(format!(
                    "mutation_gate: unexpected mutant summary {other:?} for {path}"
                ));
                return Ok(Judgement {
                    status: 1,
                    lines: Vec::new(),
                    errors,
                });
            }
        };
        if thresholds.require_survivor_reasons && (key == 1 || key == 2) {
            let name = strip_position(mutant_name);
            if listed
                .get(&(path.as_str(), name))
                .is_none_or(|r| r.is_empty())
            {
                unreasoned.insert(format!("{path}: {name}"));
            }
        }
        for group in &groups {
            if group.globs.iter().any(|g| matches(g, path)) {
                let c = counts.get_mut(group.name.as_str()).expect("group counted");
                match key {
                    0 => c.caught += 1,
                    1 => c.missed += 1,
                    2 => c.timeout += 1,
                    _ => c.unviable += 1,
                }
            }
        }
    }

    let mut failed = !baseline_ok || !unreasoned.is_empty();
    let mut lines: Vec<String> = unreasoned
        .iter()
        .map(|s| format!("FAIL survivor without a reason: {s}"))
        .collect();
    if !baseline_ok {
        lines.push("FAIL baseline: the unmutated tree did not pass its tests".into());
    }
    for group in &groups {
        let c = &counts[group.name.as_str()];
        let viable = c.caught + c.missed + c.timeout;
        if viable == 0 && mode == Mode::Diff {
            lines.push(format!(
                "ok   {}: no viable mutants in the diff",
                group.name
            ));
            continue;
        }
        if viable == 0 {
            lines.push(format!(
                "FAIL {}: no viable mutants (empty population)",
                group.name
            ));
            failed = true;
            continue;
        }
        let rate = 100.0 * c.caught as f64 / viable as f64;
        let ok = rate >= group.min_kill_percent;
        failed |= !ok;
        lines.push(format!(
            "{} {}: kill {rate:.2}% (min {:.2}%) caught={} missed={} timeout={} unviable={}",
            if ok { "ok  " } else { "FAIL" },
            group.name,
            group.min_kill_percent,
            c.caught,
            c.missed,
            c.timeout,
            c.unviable
        ));
    }
    Ok(Judgement {
        status: u8::from(failed),
        lines,
        errors,
    })
}

fn read_outcomes(dirs: &str, mode: Mode) -> Result<Vec<Value>, String> {
    let mut all = Vec::new();
    for dir in dirs.split(',') {
        let path = Path::new(dir).join("outcomes.json");
        let text = match fs::read_to_string(&path) {
            Ok(t) => t,
            // cargo-mutants writes no outcomes when the diff holds no mutable code.
            Err(e) if e.kind() == ErrorKind::NotFound && mode == Mode::Diff => continue,
            Err(e) => return Err(format!("{}: {e}", path.display())),
        };
        let value: Value =
            serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        let list = value["outcomes"]
            .as_array()
            .ok_or_else(|| format!("{}: no outcomes array", path.display()))?;
        all.extend(list.iter().cloned());
    }
    Ok(all)
}

pub(crate) fn command(args: Vec<String>) -> u8 {
    let (mode, args) = match args.first().map(String::as_str) {
        Some("--diff") => (Mode::Diff, &args[1..]),
        _ => (Mode::Full, &args[..]),
    };
    if !(1..=2).contains(&args.len()) {
        eprintln!("usage: tl-gate gate [--diff] DIR[,DIR...] [THRESHOLDS]");
        return 2;
    }
    let thresholds_path = args
        .get(1)
        .map_or("mutation-thresholds.toml", String::as_str);
    let result = read_outcomes(&args[0], mode).and_then(|outcomes| {
        let text =
            fs::read_to_string(thresholds_path).map_err(|e| format!("{thresholds_path}: {e}"))?;
        judge(&outcomes, &text, mode)
    });
    match result {
        Ok(j) => {
            for line in &j.lines {
                println!("{line}");
            }
            for line in &j.errors {
                eprintln!("{line}");
            }
            j.status
        }
        Err(e) => {
            eprintln!("mutation_gate: {e}");
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnmatch_crosses_slashes_and_supports_classes() {
        assert!(matches("src/*.rs", "src/a/b.rs"));
        assert!(matches("src/?.rs", "src/a.rs"));
        assert!(matches("src/[a-c].rs", "src/b.rs"));
        assert!(!matches("src/[!a-c].rs", "src/b.rs"));
        assert!(!matches("src/*.rs", "tests/a.rs"));
    }

    #[test]
    fn position_prefix_is_stripped_only_when_present() {
        assert_eq!(
            strip_position("src/lib.rs:12:5: replace a with b"),
            "replace a with b"
        );
        assert_eq!(strip_position("replace a with b"), "replace a with b");
    }
}
