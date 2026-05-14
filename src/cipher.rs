#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use il2cpp_runtime::prelude::*;
use core::ffi::c_void;
use std::marker::PhantomData;

#[il2cpp_value_type("RPG.GameCore.FixPoint")]
pub struct RPG_GameCore_FixPoint {
    pub m_rawValue: i64,
}

#[il2cpp_ref_type("DMFFNIJGNND")]
pub struct RelicSubAffix;
impl RelicSubAffix {
    #[il2cpp_field(name = "NOLNPNBKPOE")]
    pub fn count(&self) -> System_Int32__Boxed {}

    #[il2cpp_field(name = "BHFHNGEIPJG")]
    pub fn step(&self) -> System_Int32__Boxed {}

    #[il2cpp_field(name = "GOKGLGMNIGL")]
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


#[il2cpp_ref_type("RPG.Client.EquipmentItemData", base(RPG_Client_ItemData))]
pub struct RPG_Client_EquipmentItemData;
impl RPG_Client_EquipmentItemData {
    #[il2cpp_getter_property(property = "BelongAvatarID")]
    pub fn get_BelongAvatarID(&self) -> u32 {}

    #[il2cpp_getter_property(property = "IsDiscard")]
    pub fn get_IsDiscard(&self) -> bool {}

    #[il2cpp_getter_property(property = "IsProtected")]
    pub fn get_IsProtected(&self) -> bool {}

    #[il2cpp_getter_property(property = "Level")]
    pub fn get_Level(&self) -> u32 {}

    #[il2cpp_field(name = "_Rank")]
    pub fn _Rank(&self) -> System_UInt32__Boxed {}

    #[il2cpp_getter_property(property = "Version")]
    pub fn get_Version(&self) -> u32 {}

    #[il2cpp_getter_property(property = "Promotion")]
    pub fn get_Promotion(&self) -> u32 {}

    #[il2cpp_getter_property(property = "EquipmentRow")]
    pub fn get_EquipmentRow(&self) -> RPG_GameCore_EquipmentRow {}
}

#[il2cpp_ref_type("RPG.GameCore.EquipmentRow")]
pub struct RPG_GameCore_EquipmentRow;
impl RPG_GameCore_EquipmentRow {
    #[il2cpp_field(name = "EquipmentID")]
    pub fn EquipmentID(&self) -> System_UInt32__Boxed {}

    #[il2cpp_field(name = "EquipmentName")]
    pub fn EquipmentName(&self) -> RPG_Client_TextID__Boxed {}
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

#[il2cpp_ref_type("RPG.Client.RelicModule")]
pub struct RPG_Client_RelicModule;
impl RPG_Client_RelicModule {
	#[il2cpp_field(name = "RelicPresetModel")]
	pub fn RelicPresetModel(&self) -> RPG_Client_RelicPresetModel {}
}

#[il2cpp_ref_type("RPG.Client.RelicPresetModel")]
pub struct RPG_Client_RelicPresetModel;
impl RPG_Client_RelicPresetModel {
	#[il2cpp_method(name = "GetAvatarRelicPresetPlan", args = ["uint"])]
	pub fn GetAvatarRelicPresetPlan(&self, avatar_id: u32) -> RPG_Client_Promises_Promise {}

	#[il2cpp_method(name = "TryGetPlans", args = ["uint", "RPG.Client.RelicPresetAvatarPlans&"])]
	pub fn TryGetPlans(&self, avatar_id: u32, plans: &RPG_Client_RelicPresetAvatarPlans) -> bool {}
}

#[il2cpp_ref_type("RPG.Client.Promises.Promise")]
pub struct RPG_Client_Promises_Promise;
impl RPG_Client_Promises_Promise {
	#[il2cpp_method(name = "get_CurState", args = [])]
	pub fn get_CurState(&self) -> RPG_Client_Promises_PromiseState {}
}

#[il2cpp_enum_type(i32)]
pub enum RPG_Client_Promises_PromiseState {
    Pending,
    Rejected,
    Resolved
}

#[il2cpp_ref_type("RPG.Client.RelicPresetAvatarPlans")]
pub struct RPG_Client_RelicPresetAvatarPlans;
impl RPG_Client_RelicPresetAvatarPlans {
	#[il2cpp_field(name = "AvatarID")]
	pub fn AvatarID(&self) -> System_UInt32__Boxed {}

	// This is not right
	#[il2cpp_field(name = "PlanDict")]
	pub fn PlanDict(&self) -> Il2CppOpaquePtr {}
}

#[il2cpp_ref_type("RPG.Client.RelicPresetPlanData")]
pub struct RPG_Client_RelicPresetPlanData;
impl RPG_Client_RelicPresetPlanData {
	#[il2cpp_getter_property(property = "RelicUniqueIDs")]
	pub fn get_RelicUniqueIDs(&self) -> List {}

	#[il2cpp_getter_property(property = "Name")]
	pub fn get_Name(&self) -> Il2CppString {}

	#[il2cpp_getter_property(property = "UniqueID")]
	pub fn get_UniqueID(&self) -> u32 {}
}


#[il2cpp_ref_type("RPG.Client.ModuleManager")]
pub struct RPG_Client_ModuleManager;

impl RPG_Client_ModuleManager {
    #[il2cpp_field(name = "InventoryModule")]
    pub fn InventoryModule(&self) -> RPG_Client_InventoryModule {}

    #[il2cpp_field(name = "AvatarModule")]
    pub fn AvatarModule(&self) -> RPG_Client_AvatarModule {}

	#[il2cpp_field(name = "RelicModule")]
    pub fn RelicModule(&self) -> RPG_Client_RelicModule {}
}

#[il2cpp_ref_type("RPG.Client.AvatarModule")]
pub struct RPG_Client_AvatarModule;
impl RPG_Client_AvatarModule {
    #[il2cpp_method(name = "GetAvatar", args = ["uint"])]
    pub fn get_avatar(&self, avatar_id: u32) -> RPG_Client_AvatarData {}
}

#[il2cpp_ref_type("RPG.Client.AvatarData")]
pub struct RPG_Client_AvatarData;

#[il2cpp_ref_type("RPG.Client.NetworkManager")]
pub struct RPG_Client_NetworkManager;

impl RPG_Client_NetworkManager {
    // SendDressAvatar
    #[il2cpp_method(name = "*", args = ["uint", "RPG.Client.RelicItemData[]"])]
    pub fn apply_avatar_relics(&self, id: u32, relics: Il2CppArray) {}

    // #[il2cpp_method(name = "PHJLIBAGMAE", args = ["uint", "uint"])]
    // pub fn change_avatar_lightcone(&self, id: u32, lightcone: u32) {}
}

#[il2cpp_ref_type("RPG.Client.GlobalVars")]
pub struct RPG_Client_GlobalVars;
impl RPG_Client_GlobalVars {
    #[il2cpp_field(name = "s_ModuleManager")]
    pub fn s_ModuleManager() -> RPG_Client_ModuleManager {}

    #[il2cpp_field(name = "s_NetworkManager")]
    pub fn s_NetworkManager() -> RPG_Client_NetworkManager {}

	#[il2cpp_field(name = "s_DiContainer")]
	pub fn s_DiContainer() -> DIContainer {}
}

// Obfuscated name
// Will rely on runtime
#[il2cpp_ref_type("*")]
pub struct DIContainer;
impl DIContainer {
    #[il2cpp_method(name = "*", args = ["System.Type", "string"], ret_type = "object")]
    pub fn resolve(&self, ty: System_Type, name: Il2CppString) -> *const c_void {}
}

#[il2cpp_ref_type("*")]
pub struct RelicPresetPlanNetworkSource;
impl RelicPresetPlanNetworkSource {
	#[il2cpp_method(name = "*", args = ["uint", "System.Collections.Generic.List<uint>", "string", "*", "*"])]
	pub fn add_plan(&self, avatar_id: u32, relic_uids: List, name: Il2CppString, rank_type: i32, source_type: i32) -> *const c_void {}

	#[il2cpp_method(name = "*", args = ["uint", "uint", "System.Collections.Generic.List<uint>", "*", "*"])]
	pub fn update_plan(&self, avatar_id: u32, plan_uid: u32, relic_uids: List, rank_type: i32, source_type: i32) -> *const c_void {}

	#[il2cpp_method(name = "*", args = ["uint", "uint"])]
	pub fn delete_plan(&self, avatar_id: u32, plan_uid: u32) -> *const c_void {}

	#[il2cpp_method(name = "*", args = ["uint", "uint", "string"])]
	pub fn update_plan_name(&self, avatar_id: u32, plan_uid: u32, new_name: Il2CppString) -> *const c_void {}
}

#[il2cpp_ref_type("RPG.Client.RelicSmartSuit.RelicSmartSuitCalculator")]
pub struct RPG_Client_RelicSmartSuit_RelicSmartSuitCalculator;
impl RPG_Client_RelicSmartSuit_RelicSmartSuitCalculator {
	#[il2cpp_method(name = "CalculateRank", args = ["System.Collections.Generic.List<RPG.Client.RelicItemData>", "RPG.Client.IAvatarInfoProvider"])]
	pub fn CalculateRank(relics: List, avatar_info_provider: RPG_Client_AvatarData) -> RPG_Client_RelicSmartSuit_RankType {}
}

#[il2cpp_enum_type(i32)]
pub enum RPG_Client_RelicSmartSuit_RankType {
	None,
	SS,
	S,
	A,
	B,
	C,
	NeedUpgrade
}

#[il2cpp_ref_type("System.Activator")]
pub struct System_Activator;
impl System_Activator {
	#[il2cpp_method(name = "CreateInstance", args = ["System.Type","object[]"])]
	pub fn CreateInstance(ty: System_Type, args: Il2CppArray) -> *const c_void {}
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DictionaryEntry<TKey, TValue> {
	pub hash_code: i32,
	pub next: i32,
	pub key: TKey,
	pub value: TValue,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Dictionary<TKey, TValue> {
	pub klass: *mut c_void,
	pub monitor: *mut c_void,
	pub buckets: Il2CppArray,
	pub entries: Il2CppArray,
	pub count: i32,
	pub version: i32,
	pub free_list: i32,
	pub free_count: i32,
	pub comparer: *mut c_void,
	pub keys: *mut c_void,
	pub values: *mut c_void,
	pub sync_root: *mut c_void,
	pub _marker: PhantomData<(TKey, TValue)>,
}

impl<TKey, TValue> Dictionary<TKey, TValue>
where
	TKey: Copy + PartialEq,
	TValue: Copy,
{
	pub fn get_count(&self) -> i32 {
		self.count
	}

	pub fn get_entries(&self) -> Vec<DictionaryEntry<TKey, TValue>> {
		unsafe { self.entries.to_vec::<DictionaryEntry<TKey, TValue>>() }
	}

	pub fn get_entry(&self, index: usize) -> Option<DictionaryEntry<TKey, TValue>> {
		let entries = self.get_entries();
		entries.get(index).copied().filter(|entry| entry.hash_code >= 0)
	}

	pub fn get_key(&self, index: usize) -> Option<TKey> {
		let entry = self.get_entry(index)?;
		Some(entry.key)
	}

	pub fn get_value(&self, index: usize) -> Option<TValue> {
		let entry = self.get_entry(index)?;
		Some(entry.value)
	}

	pub fn find_entry(&self, key: TKey) -> Option<usize> {
		self.get_entries()
			.into_iter()
			.enumerate()
			.find(|(_, entry)| entry.hash_code >= 0 && entry.key == key)
			.map(|(index, _)| index)
	}

	pub fn contains_key(&self, key: TKey) -> bool {
		self.find_entry(key).is_some()
	}

	pub fn try_get_value(&self, key: TKey) -> Option<TValue> {
		self.find_entry(key)
			.and_then(|index| self.get_entry(index))
			.map(|entry| entry.value)
	}

	pub fn get_keys(&self) -> Vec<TKey> {
		self.get_entries()
			.into_iter()
			.filter(|entry| entry.hash_code >= 0)
			.map(|entry| entry.key)
			.collect()
	}

	pub fn get_values(&self) -> Vec<TValue> {
		self.get_entries()
			.into_iter()
			.filter(|entry| entry.hash_code >= 0)
			.map(|entry| entry.value)
			.collect()
	}
}


#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Il2CppOpaquePtr(*const c_void);
impl Il2CppObject for Il2CppOpaquePtr {
    fn as_ptr(&self) -> *const c_void {
        self.0 as *const c_void
    }
    
    fn ffi_name() -> &'static str {
        todo!()
    }
}

impl Il2CppRefType for Il2CppOpaquePtr {}