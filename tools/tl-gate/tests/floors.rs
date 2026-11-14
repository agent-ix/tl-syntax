//! Floor guard of the shared mutation gate: the thresholds file may not lower the core or crate
//! floors below the campaign targets.

use std::fs;
use std::path::Path;

/// Minimum kill percentages that `mutation-thresholds.toml` must not go below.
const FLOORS: [(&str, f64); 2] = [("core", 95.0), ("crate", 85.0)];

/// Check the group floors of a thresholds file; the error names the first group below its floor.
fn check_floors(text: &str) -> Result<(), String> {
    let table: toml::Table = toml::from_str(text).map_err(|e| e.to_string())?;
    let groups = table
        .get("group")
        .and_then(toml::Value::as_array)
        .ok_or("no [[group]] tables")?;
    for (name, floor) in FLOORS {
        let group = groups
            .iter()
            .find(|g| g.get("name").and_then(toml::Value::as_str) == Some(name))
            .ok_or_else(|| format!("no {name} group"))?;
        let min = group
            .get("min_kill_percent")
            .and_then(|v| v.as_float().or_else(|| v.as_integer().map(|i| i as f64)))
            .ok_or_else(|| format!("{name} has no numeric min_kill_percent"))?;
        if min < floor {
            return Err(format!("{name} min_kill_percent {min} is below {floor}"));
        }
    }
    Ok(())
}

#[test]
fn the_real_thresholds_keep_the_core_and_crate_floors() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../mutation-thresholds.toml");
    let text = fs::read_to_string(&path).unwrap();
    if let Err(error) = check_floors(&text) {
        panic!("{}: {error}", path.display());
    }
}

#[test]
fn a_lowered_or_missing_floor_is_refused() {
    let group =
        |name: &str, min: &str| format!("[[group]]\nname = \"{name}\"\nmin_kill_percent = {min}\n");
    let ok = group("core", "95.0") + &group("crate", "85");
    assert_eq!(check_floors(&ok), Ok(()));
    let low_core = group("core", "94.0") + &group("crate", "100.0");
    assert!(check_floors(&low_core).unwrap_err().contains("core"));
    let low_crate = group("core", "100.0") + &group("crate", "84.9");
    assert!(check_floors(&low_crate).unwrap_err().contains("crate"));
    assert!(check_floors(&group("core", "100.0")).is_err());
}
