use anyhow::Result;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::models::{Relic, ReliquaryRelic};

pub fn get_relics() -> &'static RwLock<HashMap<String, Relic>> {
    static RELICS: OnceLock<RwLock<HashMap<String, Relic>>> = OnceLock::new();
    RELICS.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn calc_initial_rolls(level: u32, total_rolls: u32) -> u32 {
    total_rolls - level.div_floor(3)
}

pub fn solve_low_mid_high(step: i32, count: i32) -> Vec<(i32, i32, i32)> {
    if step < 0 || count < 0 {
        return Vec::new();
    }

    // 0*low + 1*mid + 2*high = step
    // low + mid + high = count
    // mid = step - 2*high
    // low = count - step + high
    let high_min = (step - count).max(0);
    let high_max = step / 2;

    if high_min > high_max {
        return Vec::new();
    }

    (high_min..=high_max)
        .map(|high| {
            let mid = step - 2 * high;
            let low = count - step + high;
            (low, mid, high)
        })
        .filter(|(low, mid, high)| *low >= 0 && *mid >= 0 && *high >= 0)
        .collect()
}

pub fn pick_low_mid_high(step: i32, count: i32) -> (i32, i32, i32) {
    solve_low_mid_high(step, count)
        .last()
        .copied()
        .unwrap_or((0, 0, 0))
}

pub fn write_relics_to_json(path: &str) -> Result<()> {
    let relics_map = get_relics().read();
    let relics: Vec<ReliquaryRelic> = relics_map
        .values()
        .map(|relic| ReliquaryRelic::from(relic))
        .collect();

    let json_obj = serde_json::json!({
        "relics": relics
    });

    let json_str = serde_json::to_string_pretty(&json_obj)?;
    std::fs::write(path, json_str)?;

    log::info!("Wrote {} relics to {}", relics.len(), path);
    Ok(())
}

pub fn get_relics_snapshot() -> Vec<Relic> {
    let relics_map = get_relics().read();
    relics_map.values().cloned().collect()
}
