#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use il2cpp_runtime::prelude::*;
use core::ffi::c_void;

#[il2cpp_value_type("RPG.GameCore.FixPoint")]
pub struct RPG_GameCore_FixPoint {
    pub m_rawValue: i64,
}

#[il2cpp_ref_type("HANGJNJOFEC")]
pub struct HANGJNJOFEC;
impl HANGJNJOFEC {
    #[il2cpp_field(name = "JLDLGGOAFPP")]
    pub fn step(&self) -> System_Int32__Boxed {}

    #[il2cpp_field(name = "IIONFBIAHLP")]
    pub fn count(&self) -> System_Int32__Boxed {}

    #[il2cpp_field(name = "LCEHFEFOPDM")]
    pub fn property_id(&self) -> System_Int32__Boxed {}
}

#[il2cpp_ref_type("RPG.Client.ItemData")]
pub struct RPG_Client_ItemData;
impl RPG_Client_ItemData {
    #[il2cpp_getter_property(property = "UID")]
    pub fn get_UID(&self) -> u32 {}
}

#[il2cpp_ref_type("RPG.GameCore.RelicSubAffixConfigRow")]
pub struct RPG_GameCore_RelicSubAffixConfigRow;
impl RPG_GameCore_RelicSubAffixConfigRow {
    #[il2cpp_field(name = "BaseValue")]
    pub fn BaseValue(&self) -> RPG_GameCore_FixPoint__Boxed {}

    #[il2cpp_field(name = "StepValue")]
    pub fn StepValue(&self) -> RPG_GameCore_FixPoint__Boxed {}
}

#[il2cpp_ref_type("RPG.GameCore.RelicSubAffixConfigExcelTable")]
pub struct RPG_GameCore_RelicSubAffixConfigExcelTable;
impl RPG_GameCore_RelicSubAffixConfigExcelTable {
    #[il2cpp_method(name = "GetData", args = ["uint", "uint"])]
    pub fn GetData(sub_affix_group: u32, avatar_property_type: u32) -> RPG_GameCore_RelicSubAffixConfigRow {}
}

#[il2cpp_ref_type("RPG.GameCore.GamePlayStatic")]
pub struct RPG_GameCore_GamePlayStatic;
impl RPG_GameCore_GamePlayStatic {
    #[il2cpp_method(name = "CalcRelicSubAffixValue", args = ["RPG.GameCore.FixPoint", "RPG.GameCore.FixPoint", "uint", "uint"])]
    pub fn CalcRelicSubAffixValue(base_config_value: RPG_GameCore_FixPoint, step_config_value: RPG_GameCore_FixPoint, count: u32, step: u32) -> RPG_GameCore_FixPoint {}
}
#[il2cpp_ref_type("RPG.Client.RelicItemData", base(RPG_Client_ItemData))]
pub struct RPG_Client_RelicItemData;

impl RPG_Client_RelicItemData {
    #[il2cpp_getter_property(property = "RelicRow")]
    pub fn get_RelicRow(&self) -> RPG_GameCore_RelicConfigRow {}

    #[il2cpp_getter_property(property = "BelongAvatarID")]
    pub fn get_BelongAvatarID(&self) -> u32 {}

    // #[il2cpp_getter_property(property = "MainAffixID")]
    // pub fn get_MainAffixID(&self) -> u32 {}

    #[il2cpp_getter_property(property = "MainAffixPropertyType")]
    pub fn get_MainAffixPropertyType(&self) -> RPG_GameCore_AvatarPropertyType {}

    #[il2cpp_method(name = "GetSubAffixPropertyValue", args = ["RPG.GameCore.AvatarPropertyType"])]
    pub fn GetSubAffixPropertyValue(&self, sub_affix_id: RPG_GameCore_AvatarPropertyType) -> RPG_GameCore_FixPoint__Boxed {}

    #[il2cpp_getter_property(property = "SubAffixList")]
    pub fn get_SubAffixList(&self) -> Il2CppArray {}
    
    #[il2cpp_getter_property(property = "IsDiscard")]
    pub fn get_IsDiscard(&self) -> bool {}

    #[il2cpp_getter_property(property = "IsProtected")]
    pub fn get_IsProtected(&self) -> bool {}

    #[il2cpp_getter_property(property = "Level")]
    pub fn get_Level(&self) -> u32 {}

    #[il2cpp_method(name = "_GetPropertyTypeByMainAffixID", args = ["uint"])]
    pub fn _GetPropertyTypeByMainAffixID(&self, main_affix_id: u32) -> RPG_GameCore_AvatarPropertyType {}

    #[il2cpp_method(name = "_GetPropertyTypeBySubAffixID", args = ["uint"])]
    pub fn _GetPropertyTypeBySubAffixID(&self, sub_affix_id: u32) -> RPG_GameCore_AvatarPropertyType {}

    #[il2cpp_method(name = "GetMainAffixPropertyValue", args = [])]
    pub fn GetMainAffixPropertyValue(&self) -> RPG_GameCore_FixPoint {}

}

#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_CombatPowerRelicRarityType {
    CombatPowerRelicRarity1,
    CombatPowerRelicRarity2,
    CombatPowerRelicRarity3,
    CombatPowerRelicRarity4,
    CombatPowerRelicRarity5,
}

#[il2cpp_ref_type("RPG.GameCore.RelicConfigRow")]
pub struct RPG_GameCore_RelicConfigRow;

impl RPG_GameCore_RelicConfigRow {
    #[il2cpp_field(name = "Rarity")]
    pub fn Rarity(&self) -> RPG_GameCore_CombatPowerRelicRarityType__Boxed {}

    #[il2cpp_field(name = "SetID")]
    pub fn SetID(&self) -> System_UInt32__Boxed {}

    #[il2cpp_field(name = "Type")]
    pub fn Type(&self) -> RPG_GameCore_RelicSetType__Boxed {}

    #[il2cpp_field(name = "SubAffixGroup")]
    pub fn SubAffixGroup(&self) -> System_UInt32__Boxed {}
}

#[il2cpp_value_type("RPG.Client.TextID")]
pub struct RPG_Client_TextID {
    pub hash: i32,
    pub hash64: u64,
}

#[il2cpp_ref_type("RPG.GameCore.RelicSetConfigRow")]
pub struct RPG_GameCore_RelicSetConfigRow;
impl RPG_GameCore_RelicSetConfigRow {
    #[il2cpp_field(name = "SetName")]
    pub fn SetName(&self) -> RPG_Client_TextID__Boxed {}
}

#[il2cpp_ref_type("RPG.Client.TextmapStatic")]
pub struct RPG_Client_TextmapStatic;
impl RPG_Client_TextmapStatic {
    #[il2cpp_method(name = "GetText", args = ["RPG.Client.TextID", "object[]"])]
    pub fn get_text(id: &RPG_Client_TextID, replace_params: *const c_void) -> Il2CppString {}
}

#[il2cpp_ref_type("RPG.GameCore.RelicSetConfigExcelTable")]
pub struct RPG_GameCore_RelicSetConfigExcelTable;
impl RPG_GameCore_RelicSetConfigExcelTable {
    #[il2cpp_method(name = "GetData", args = ["uint"])]
    pub fn GetData(set_id: u32) -> RPG_GameCore_RelicSetConfigRow {}
}

#[il2cpp_ref_type("RPG.GameCore.AvatarPropertyExcelTable")]
pub struct RPG_GameCore_AvatarPropertyExcelTable;
impl RPG_GameCore_AvatarPropertyExcelTable {
    #[il2cpp_method(name = "GetData", args = ["RPG.GameCore.AvatarPropertyType"])]
    pub fn GetData(property_type: RPG_GameCore_AvatarPropertyType) -> RPG_GameCore_AvatarPropertyRow {}
}

#[il2cpp_ref_type("RPG.GameCore.RelicBaseTypeExcelTable")]
pub struct RPG_GameCore_RelicBaseTypeExcelTable;
impl RPG_GameCore_RelicBaseTypeExcelTable {
    #[il2cpp_method(name = "GetData", args = ["RPG.GameCore.RelicType"])]
    pub fn GetData(relic_type: RPG_GameCore_RelicSetType) -> RPG_GameCore_RelicBaseTypeRow {}
}

#[il2cpp_ref_type("RPG.GameCore.RelicBaseTypeRow")]
pub struct RPG_GameCore_RelicBaseTypeRow;
impl RPG_GameCore_RelicBaseTypeRow {
    #[il2cpp_field(name = "BaseTypeText")]
    pub fn BaseTypeText(&self) -> RPG_Client_TextID__Boxed {}
}

#[il2cpp_ref_type("RPG.GameCore.AvatarPropertyRow")]
pub struct RPG_GameCore_AvatarPropertyRow;
impl RPG_GameCore_AvatarPropertyRow {
    #[il2cpp_field(name = "PropertyName")]
    pub fn PropertyName(&self) -> RPG_Client_TextID__Boxed {}
}


#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_AvatarPropertyType {
	Unknown,
	MaxHP,
	Attack,
	Defence,
	Speed,
	CriticalChance,
	CriticalDamage,
	HealRatio,
	StanceBreakAddedRatio,
	SPRatio,
	StatusProbability,
	StatusResistance,
	PhysicalAddedRatio,
	PhysicalResistance,
	FireAddedRatio,
	FireResistance,
	IceAddedRatio,
	IceResistance,
	ThunderAddedRatio,
	ThunderResistance,
	WindAddedRatio,
	WindResistance,
	QuantumAddedRatio,
	QuantumResistance,
	ImaginaryAddedRatio,
	ImaginaryResistance,
	BaseHP,
	HPDelta,
	BaseAttack,
	AttackDelta,
	BaseDefence,
	DefenceDelta,
	HPAddedRatio,
	AttackAddedRatio,
	DefenceAddedRatio,
	BaseSpeed,
}

#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_RelicSetType {
    Unknow,
    HEAD,
    HAND,
    BODY,
    FOOT,
    NECK,
    OBJECT
}
#[il2cpp_ref_type("RPG.Client.InventoryModule")]
pub struct RPG_Client_InventoryModule;

impl RPG_Client_InventoryModule {
    #[il2cpp_method(name = "GetRelicDataByUID", args = ["uint"])]
    pub fn get_relic_data_by_uid(&self, uid: u32) -> RPG_Client_RelicItemData {}
}

#[il2cpp_ref_type("RPG.Client.ModuleManager")]
pub struct RPG_Client_ModuleManager;

impl RPG_Client_ModuleManager {
    #[il2cpp_field(name = "InventoryModule")]
    pub fn InventoryModule(&self) -> RPG_Client_InventoryModule {}
}

#[il2cpp_ref_type("RPG.Client.NetworkManager")]
pub struct RPG_Client_NetworkManager;

impl RPG_Client_NetworkManager {
    #[il2cpp_method(name = "MJNBHOKDEJI", args = ["uint", "RPG.Client.RelicItemData[]"])]
    pub fn change_avatar_relics(&self, id: u32, relics: Il2CppArray) {}

    #[il2cpp_method(name = "PHJLIBAGMAE", args = ["uint", "uint"])]
    pub fn change_avatar_lightcone(&self, id: u32, lightcone: u32) {}
}

#[il2cpp_ref_type("RPG.Client.GlobalVars")]
pub struct RPG_Client_GlobalVars;
impl RPG_Client_GlobalVars {
    #[il2cpp_field(name = "s_ModuleManager")]
    pub fn s_ModuleManager() -> RPG_Client_ModuleManager {}

    #[il2cpp_field(name = "s_NetworkManager")]
    pub fn s_NetworkManager() -> RPG_Client_NetworkManager {}
}