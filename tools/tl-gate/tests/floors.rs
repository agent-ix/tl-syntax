//! FR-022-AC-4 / TC-116: the thresholds file may not lower the campaign floors.

use std::fs;
use std::path::Path;

/// Minimum kill percentages that `mutation-thresholds.toml` must not go below.
const FLOORS: [(&str, f64); 2] = [("core", 95.0), ("crate", 85.0)];

fn group_minimums(text: &str) -> Vec<(String, f64)> {
    let table: toml::Table = toml::from_str(text).unwrap();
    table["group"]
        .as_array()
        .expect("[[group]] tables")
        .iter()
        .map(|g| {
            let min = &g["min_kill_percent"];
            let min = min
                .as_float()
                .or_else(|| min.as_integer().map(|i| i as f64))
                .expect("numeric min_kill_percent");
            (g["name"].as_str().unwrap().to_owned(), min)
        })
        .collect()
}

#[test]
fn the_real_thresholds_keep_the_core_and_crate_floors() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../mutation-thresholds.toml");
    let text = fs::read_to_string(&path).unwrap();
    let groups = group_minimums(&text);
    for (name, floor) in FLOORS {
        let (_, min) = groups
            .iter()
            .find(|(n, _)| n == name)
            .unwrap_or_else(|| panic!("no {name} group in {}", path.display()));
        assert!(
            *min >= floor,
            "{name} min_kill_percent {min} is below {floor}"
        );
    }
}

#[test]
fn a_lowered_floor_is_detected() {
    let lowered = "[[group]]\nname = \"core\"\nmin_kill_percent = 94.0\n";
    let groups = group_minimums(lowered);
    assert!(groups[0].1 < FLOORS[0].1);
}
