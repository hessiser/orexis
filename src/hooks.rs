use anyhow::Result;
use il2cpp_runtime::{Il2CppObject, types::List};
use parking_lot::RwLock;
use retour::static_detour;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{LazyLock, OnceLock};

use crate::cipher::{
    HANGJNJOFEC, RPG_Client_InventoryModule, RPG_Client_RelicItemData, RPG_Client_TextmapStatic,
    RPG_GameCore_AvatarPropertyExcelTable, RPG_GameCore_AvatarPropertyType, RPG_GameCore_FixPoint,
    RPG_GameCore_GamePlayStatic, RPG_GameCore_RelicBaseTypeExcelTable,
    RPG_GameCore_RelicSetConfigExcelTable, RPG_GameCore_RelicSubAffixConfigExcelTable,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Substat {
    pub key: String,
    pub value: f64,
    pub count: i32,
    pub step: i32,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Relic {
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

fn get_relics() -> &'static RwLock<HashMap<String, Relic>> {
    static RELICS: OnceLock<RwLock<HashMap<String, Relic>>> = OnceLock::new();
    RELICS.get_or_init(|| RwLock::new(HashMap::new()))
}

macro_rules! hook_fn {
    (
        $detour:ident,
        $target:expr,
        $reroute:ident
    ) => {
        $detour.initialize(std::mem::transmute($target), $reroute)?;
        $detour.enable()?;
    };
}

static_detour! {
    static _UpdateRelics_Hook: unsafe extern "C" fn(
        RPG_Client_InventoryModule,
        List,
        bool
    );
    static sync_Hook: unsafe extern "C" fn(
        RPG_Client_RelicItemData,
        *const c_void
    );
}

fn fixpoint_to_raw(fixpoint: &RPG_GameCore_FixPoint) -> f64 {
    static FLOAT_CONVERSION_CONSTANT: LazyLock<f64> = LazyLock::new(|| 1f64 / 2f64.powf(32f64));
    let raw_value = fixpoint.m_rawValue;
    let hi = ((raw_value as u64 & 0xFFFFFFFF00000000) >> 32) as u32;
    let lo = (raw_value as u64 & 0x00000000FFFFFFFF) as u32;
    hi as f64 + lo as f64 * *FLOAT_CONVERSION_CONSTANT
}

pub fn update_relics_detour(this: RPG_Client_InventoryModule, list: List, flag: bool) {
    unsafe { _UpdateRelics_Hook.call(this, list, flag) };
    write_relics_to_json("dump.json")
        .unwrap_or_else(|e| log::error!("Failed to write relics to JSON: {e:#}"));
}

fn sync(this: RPG_Client_RelicItemData, packet: *const c_void) {
    unsafe { sync_Hook.call(this, packet) };
    let func = || -> Result<()> {
        let relic_row = this.get_RelicRow()?;
        let set_id = relic_row.SetID()?;
        let location = this.get_BelongAvatarID()?;
        let lock = this.get_IsProtected()?;
        let discard = this.get_IsDiscard()?;
        let uid = this.as_base().get_UID()?;
        let rarity = (*relic_row.Rarity()?) as u32 + 1;
        let level = this.get_Level()?;
        let relic_set_config_data = RPG_GameCore_RelicSetConfigExcelTable::GetData((*set_id).0)?;
        let relic_set_name = RPG_Client_TextmapStatic::get_text(
            &*relic_set_config_data.SetName()?,
            std::ptr::null(),
        )?;
        let main_affix_property = this.get_MainAffixPropertyType()?;

        let main_row_data = RPG_GameCore_AvatarPropertyExcelTable::GetData(main_affix_property)?;
        let main_stat_name =
            RPG_Client_TextmapStatic::get_text(&*main_row_data.PropertyName()?, std::ptr::null())?;

        let relic_type_row = RPG_GameCore_RelicBaseTypeExcelTable::GetData(*relic_row.Type()?)?;
        let slot_name =
            RPG_Client_TextmapStatic::get_text(&*relic_type_row.BaseTypeText()?, std::ptr::null())?;

        let mut substats = Vec::new();
        for substat in this.get_SubAffixList()?.to_vec::<HANGJNJOFEC>() {
            let sub_property =
                this._GetPropertyTypeBySubAffixID((*substat.property_id()?).0 as u32)?;
            let sub_row_data = RPG_GameCore_AvatarPropertyExcelTable::GetData(sub_property)?;
            let property_name = RPG_Client_TextmapStatic::get_text(
                &*sub_row_data.PropertyName()?,
                std::ptr::null(),
            )?
            .to_string()
            .replace("%", "_");

            let count = (*substat.count()?).0;
            let step = (*substat.step()?).0;

            let relic_sub_affix_config = RPG_GameCore_RelicSubAffixConfigExcelTable::GetData(
                (*relic_row.SubAffixGroup()?).0,
                (*substat.property_id()?).0 as u32,
            )?;
            let mut value = fixpoint_to_raw(&RPG_GameCore_GamePlayStatic::CalcRelicSubAffixValue(
                *relic_sub_affix_config.BaseValue()?,
                *relic_sub_affix_config.StepValue()?,
                step as u32,
                count as u32,
            )?);
            let mut key = property_name;
            if value < 1.0 {
                key.push('_');
                value *= 100.0;
            }
            substats.push(Substat {
                key,
                value,
                count,
                step,
            });
        }

        let relic = Relic {
            set_id: (*set_id).0.to_string(),
            name: relic_set_name.to_string(),
            slot: slot_name.to_string(),
            rarity,
            level: level as u32,
            mainstat: main_stat_name.to_string(),
            substats,
            location: if location > 0 { location.to_string() } else { String::new() },
            lock,
            discard,
            _uid: uid.to_string(),
        };

        get_relics().write().insert(uid.to_string(), relic);
        log::info!("Stored relic UID: {}", uid);

        Ok(())
    };
    match func() {
        Ok(()) => {}
        Err(e) => log::error!("Failed to sync relic data: {e:#}"),
    }
}

pub fn write_relics_to_json(path: &str) -> Result<()> {
    let relics_map = get_relics().read();
    let relics: Vec<Relic> = relics_map.values().cloned().collect();

    let json_obj = serde_json::json!({
        "relics": relics
    });

    let json_str = serde_json::to_string_pretty(&json_obj)?;
    std::fs::write(path, json_str)?;

    log::info!("Wrote {} relics to {}", relics.len(), path);
    Ok(())
}

pub unsafe fn install_hooks() -> Result<()> {
    unsafe {
        hook_fn!(
            sync_Hook,
            RPG_Client_RelicItemData::get_class_static()?
                .find_method("Sync", vec!["NNDLOJHOGAG"])?
                .va(),
            sync
        );

        hook_fn!(
            _UpdateRelics_Hook,
            RPG_Client_InventoryModule::get_class_static()?
                .find_method(
                    "_UpdateRelics",
                    vec!["System.Collections.Generic.IList<NNDLOJHOGAG>", "bool"]
                )?
                .va(),
            update_relics_detour
        );
    }
    Ok(())
}
