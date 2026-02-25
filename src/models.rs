use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Substat {
    pub key: String,
    pub value: f64,
    pub count: i32,
    pub step: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelicMainStat {
    pub stat: String,
    pub value: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelicRolls {
    pub high: i32,
    pub mid: i32,
    pub low: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelicSubstat {
    pub stat: String,
    pub value: f64,
    pub rolls: RelicRolls,
    #[serde(rename = "addedRolls")]
    pub added_rolls: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Relic {
    pub part: String,
    #[serde(skip)]
    pub set_id: String,
    #[serde(rename = "set")]
    pub set: String,
    pub enhance: u32,
    pub grade: u32,
    pub main: RelicMainStat,
    pub substats: Vec<RelicSubstat>,
    #[serde(rename = "equippedBy")]
    pub equipped_by: String,
    pub verified: bool,
    pub id: String,
    #[serde(rename = "ageIndex")]
    pub age_index: u32,
    #[serde(rename = "initialRolls")]
    pub initial_rolls: u32,
    #[serde(skip)]
    pub lock: bool,
    #[serde(skip)]
    pub discard: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReliquaryRelic {
    pub set_id: String,
    pub name: String,
    pub slot: String,
    pub rarity: u32,
    pub level: u32,
    pub mainstat: String,
    pub substats: Vec<Substat>,
    pub location: String,
    pub lock: bool,
    pub discard: bool,
    pub _uid: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LightCone {
    pub id: String,
    pub name: String,
    pub level: u32,
    pub promotion: u32,
    pub rank: u32,
    pub equipped_by: String,
    pub lock: bool,
    pub uid: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReliquaryLightCone {
    pub id: String,
    pub name: String,
    pub level: u32,
    pub ascension: u32,
    pub superimposition: u32,
    pub location: String,
    pub lock: bool,
    pub _uid: String,
}

impl From<&LightCone> for ReliquaryLightCone {
    fn from(lc: &LightCone) -> Self {
        ReliquaryLightCone {
            id: lc.id.clone(),
            name: lc.name.clone(),
            level: lc.level,
            ascension: lc.promotion,
            superimposition: lc.rank,
            location: lc.equipped_by.clone(),
            lock: lc.lock,
            _uid: lc.uid.clone(),
        }
    }
}

impl From<&Relic> for ReliquaryRelic {
    fn from(relic: &Relic) -> Self {
        let substats = relic
            .substats
            .iter()
            .map(|substat| {
                let key = substat.stat.replace('%', "_");

                let count = substat.rolls.low + substat.rolls.mid + substat.rolls.high;
                let step = substat.rolls.mid + 2 * substat.rolls.high;

                Substat {
                    key,
                    value: substat.value,
                    count,
                    step,
                }
            })
            .collect();

        ReliquaryRelic {
            set_id: relic.set_id.clone(),
            name: relic.set.clone(),
            slot: relic.part.clone(),
            rarity: relic.grade,
            level: relic.enhance,
            mainstat: if let Some(base) = relic.main.stat.strip_suffix('%') {
                base.to_string()
            } else {
                relic.main.stat.clone()
            },
            substats,
            location: relic.equipped_by.clone(),
            lock: relic.lock,
            discard: relic.discard,
            _uid: relic.id.clone(),
        }
    }
}
