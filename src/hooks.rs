use anyhow::{Context, Result};
use il2cpp_runtime::{Il2CppObject, types::List};
use retour::static_detour;
use std::ffi::c_void;
use std::sync::OnceLock;

use crate::cipher::{
    HANGJNJOFEC, RPG_Client_EquipmentItemData, RPG_Client_InventoryModule, RPG_Client_RelicItemData, RPG_Client_TextmapStatic, RPG_GameCore_AvatarPropertyExcelTable, RPG_GameCore_FixPoint, RPG_GameCore_GamePlayStatic, RPG_GameCore_RelicBaseTypeExcelTable, RPG_GameCore_RelicSetConfigExcelTable, RPG_GameCore_RelicSubAffixConfigExcelTable
};
use crate::models::{LightCone, Relic, RelicMainStat, RelicRolls, RelicSubstat, ReliquaryLightCone, ReliquaryRelic};
use crate::relic_utils::{calc_initial_rolls, get_light_cones, get_relics, pick_low_mid_high, write_light_cones_to_json, write_relics_to_json};

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
    static _UpdateRelics_Detour: unsafe extern "C" fn(
        RPG_Client_InventoryModule,
        List,
        bool
    );
    static sync_relic_Detour: unsafe extern "C" fn(
        RPG_Client_RelicItemData,
        *const c_void
    );

    static sync_equipment_Detour: unsafe extern "C" fn(
        RPG_Client_EquipmentItemData,
        *const c_void
    );

    static _UpdateEquipments_Detour: unsafe extern "C" fn(
        RPG_Client_InventoryModule,
        List,
        bool
    );
}

impl Into<f64> for RPG_GameCore_FixPoint {
    fn into(self) -> f64 {
        const FLOAT_CONVERSION_CONSTANT: f64 = 1.0 / 4294967296.0;
        let raw_value = self.m_rawValue;
        let hi = ((raw_value as u64 & 0xFFFFFFFF00000000) >> 32) as u32;
        let lo = (raw_value as u64 & 0x00000000FFFFFFFF) as u32;
        hi as f64 + lo as f64 * FLOAT_CONVERSION_CONSTANT
    }
}

static ARE_RELICS_INITIALIZED: OnceLock<bool> = OnceLock::new();
static ARE_LIGHT_CONES_INITIALIZED: OnceLock<bool> = OnceLock::new();

fn update_relics(this: RPG_Client_InventoryModule, list: List, flag: bool) {
    unsafe { _UpdateRelics_Detour.call(this, list, flag) };
    write_relics_to_json("relics.json")
        .unwrap_or_else(|e| log::error!("Failed to write relics to JSON: {e:#}"));
    ARE_RELICS_INITIALIZED.get_or_init(|| true);
}

fn update_equipments(this: RPG_Client_InventoryModule, list: List, flag: bool) {
    unsafe { _UpdateEquipments_Detour.call(this, list, flag) };
    write_light_cones_to_json("light_cones.json")
        .unwrap_or_else(|e| log::error!("Failed to write light cones to JSON: {e:#}"));
    ARE_LIGHT_CONES_INITIALIZED.get_or_init(|| true);
}

fn sync_equipment(this: RPG_Client_EquipmentItemData, packet: *const c_void) {
    unsafe { sync_equipment_Detour.call(this, packet) };
    let Some(initialized) = ARE_LIGHT_CONES_INITIALIZED.get() else {
        return;
    };
    if !*initialized {
        return;
    }

    let func = || -> Result<()> {
        let uid = this.as_base().get_UID()?;
        let location = this.get_BelongAvatarID()?;
        let lock = this.get_IsProtected()?;
        let rank = (*this._Rank()?).0;
        let level = this.get_Level()?;
        let promotion = this.get_Promotion()?;

        let equipment_row = this.get_EquipmentRow()?;

        let name = RPG_Client_TextmapStatic::get_text(
            &*equipment_row.EquipmentName()?,
            std::ptr::null(),
        )?;
        let id = (*equipment_row.EquipmentID()?).0;

        let light_cone = LightCone {
            id: id.to_string(),
            name: name.to_string(),
            level: level as u32,
            promotion: promotion as u32,
            rank: rank as u32,
            equipped_by: if location > 0 {
                location.to_string()
            } else {
                String::new()
            },
            lock,
            uid: uid.to_string(),
        };

        let live_light_cone = ReliquaryLightCone::from(&light_cone);
        get_light_cones().write().insert(uid.to_string(), light_cone);
        crate::server::send_live_light_cone_update(vec![live_light_cone]);
        log::info!("Stored lightcone UID: {}", uid);

        Ok(())
    };
    match func() {
        Ok(()) => {},
        Err(e) => log::error!("Failed to sync lightcone data: {e:#}"),
    }
}

fn sync_relic(this: RPG_Client_RelicItemData, packet: *const c_void) {
    unsafe { sync_relic_Detour.call(this, packet) };
    let Some(initialized) = ARE_RELICS_INITIALIZED.get() else {
        return;
    };
    if !*initialized {
        return;
    }
    let func = || -> Result<()> {
        let relic_row = this.get_RelicRow()?;
        let set_id = (*relic_row.SetID()?).0;
        let location = this.get_BelongAvatarID()?;
        let lock = this.get_IsProtected()?;
        let discard = this.get_IsDiscard()?;
        let uid = this.as_base().get_UID()?;
        let rarity = (*relic_row.Rarity()?) as u32;
        let level = this.get_Level()?;
        let relic_set_config_data = RPG_GameCore_RelicSetConfigExcelTable::GetData(set_id)?;
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
        let mut total_count: i32 = 0;
        for substat in this.get_SubAffixList()?.to_vec::<HANGJNJOFEC>() {
            let sub_property =
                this._GetPropertyTypeBySubAffixID((*substat.property_id()?).0 as u32)?;
            let sub_row_data = RPG_GameCore_AvatarPropertyExcelTable::GetData(sub_property)?;
            let property_name = RPG_Client_TextmapStatic::get_text(
                &*sub_row_data.PropertyName()?,
                std::ptr::null(),
            )?
            .to_string();

            let count = (*substat.count()?).0;
            let step = (*substat.step()?).0;
            total_count = total_count.saturating_add(count);

            let relic_sub_affix_config = RPG_GameCore_RelicSubAffixConfigExcelTable::GetData(
                (*relic_row.SubAffixGroup()?).0,
                (*substat.property_id()?).0 as u32,
            )?;
            let mut value: f64 = RPG_GameCore_GamePlayStatic::CalcRelicSubAffixValue(
                *relic_sub_affix_config.BaseValue()?,
                *relic_sub_affix_config.StepValue()?,
                count as u32,
                step as u32,
            )?
            .into();
            let mut stat_name = property_name;
            if value < 1.0 {
                stat_name.push('%');
                value *= 100.0;
            }

            let (low, mid, high) = pick_low_mid_high(step, count);
            substats.push(RelicSubstat {
                stat: stat_name,
                value,
                rolls: RelicRolls { high, mid, low },
                added_rolls: (count - 1).max(0),
            });
        }

        let initial_rolls = if total_count > 0 {
            calc_initial_rolls(level as u32, total_count as u32)
        } else {
            0
        };

        let mut main_value: f64 = (this.GetMainAffixPropertyValue()?).into();
        let main_stat = main_stat_name.to_string();
        if main_value < 1.0 {
            main_value *= 100.0;
        }

        let relic = Relic {
            part: slot_name.to_string(),
            set_id: set_id.to_string(),
            set: relic_set_name.to_string(),
            enhance: level as u32,
            grade: rarity,
            main: RelicMainStat {
                stat: main_stat,
                value: main_value,
            },
            substats,
            equipped_by: if location > 0 {
                location.to_string()
            } else {
                String::new()
            },
            verified: true,
            id: uid.to_string(),
            age_index: uid,
            initial_rolls,
            lock,
            discard,
        };

        let live_relic = ReliquaryRelic::from(&relic);
        get_relics().write().insert(uid.to_string(), relic);
        crate::server::send_live_relic_update(vec![live_relic]);
        log::info!("Stored relic UID: {}", uid);

        Ok(())
    };
    match func() {
        Ok(()) => {}
        Err(e) => log::error!("Failed to sync relic data: {e:#}"),
    }
}

pub unsafe fn install_hooks() -> Result<()> {
    unsafe {
        hook_fn!(
            sync_relic_Detour,
            RPG_Client_RelicItemData::get_class_static()?
                .methods().iter().find(|method| {
                    method.name() == "Sync" && method.args_cnt() == 1
                }).context("Could not find Sync method")?
                .va(),
            sync_relic
        );

        hook_fn!(
            _UpdateRelics_Detour,
            RPG_Client_InventoryModule::get_class_static()?
                .methods().iter().find(|method| {
                    method.name() == "_UpdateRelics" && method.args_cnt() == 2
                }).context("Could not find _UpdateRelics method")?
                .va(),
            update_relics
        );

        hook_fn!(
            sync_equipment_Detour,
            RPG_Client_EquipmentItemData::get_class_static()?
                .methods().iter().find(|method| {
                    method.name() == "Sync" && method.args_cnt() == 1
                }).context("Could not find Sync method")?
                .va(),
            sync_equipment
        );


        hook_fn!(
            _UpdateEquipments_Detour,
            RPG_Client_InventoryModule::get_class_static()?
                .methods().iter().find(|method| {
                    method.name() == "_UpdateEquipments" && method.args_cnt() == 2
                }).context("Could not find _UpdateEquipments method")?
                .va(),
            update_equipments
        );

    }
    Ok(())
}
