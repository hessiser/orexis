#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use il2cpp_runtime::prelude::*;
use core::ffi::c_void;

#[il2cpp_value_type("RPG.GameCore.FixPoint")]
pub struct RPG_GameCore_FixPoint {
    pub m_rawValue: i64,
}

// IL2CPP Dictionary layout helper for FFI.
#[repr(C)]
pub struct Il2CppDictionaryEntry<TKey, TValue> {
    pub hashCode: i32,
    pub next: i32,
    pub key: TKey,
    pub value: TValue,
}

#[repr(C)]
pub struct Il2CppDictionary<TKey, TValue> {
    pub klass: *mut c_void,
    pub monitor: *mut c_void,
    pub buckets: *mut Il2CppArray,
    pub entries: *mut Il2CppArray,
    pub count: i32,
    pub version: i32,
    pub free_list: i32,
    pub free_count: i32,
    pub comparer: *mut c_void,
    pub keys: *mut Il2CppDictionaryKeysCollection<TKey, TValue>,
    pub values: *mut Il2CppDictionaryValueCollection<TKey, TValue>,
    pub sync_root: *mut c_void,
}

impl<TKey, TValue> Il2CppDictionary<TKey, TValue>
where
    TKey: Copy + Default + PartialEq,
    TValue: Copy + Default + PartialEq,
{
    pub unsafe fn get_comparer(&self) -> *mut c_void {
        self.comparer
    }

    pub unsafe fn get_count(&self) -> i32 {
        self.count
    }

    pub unsafe fn get_keys(&mut self) -> &Il2CppDictionaryKeysCollection<TKey, TValue> {
        if self.keys.is_null() {
            self.keys = Box::into_raw(Box::new(Il2CppDictionaryKeysCollection {
                dictionary: self as *mut _,
            }));
        }
        &*self.keys
    }

    pub unsafe fn get_values(&mut self) -> &Il2CppDictionaryValueCollection<TKey, TValue> {
        if self.values.is_null() {
            self.values = Box::into_raw(Box::new(Il2CppDictionaryValueCollection {
                dictionary: self as *mut _,
            }));
        }
        &*self.values
    }

    pub unsafe fn get(&self, key: TKey) -> TValue {
        let i = self.find_entry(key);
        if i >= 0 {
            return self.entry_at(i).value;
        }
        TValue::default()
    }

    pub unsafe fn find_entry(&self, key: TKey) -> i32 {
        if self.entries.is_null() {
            return -1;
        }
        let entries = &*self.entries;
        let count = self.count.max(0) as usize;
        for i in 0..count {
            if entries.get::<Il2CppDictionaryEntry<TKey, TValue>>(i).key == key {
                return i as i32;
            }
        }
        -1
    }

    pub unsafe fn contains_key(&self, key: TKey) -> bool {
        self.find_entry(key) >= 0
    }

    pub unsafe fn contains_value(&self, value: TValue) -> bool {
        if self.entries.is_null() {
            return false;
        }
        let entries = &*self.entries;
        let count = self.count.max(0) as usize;
        for i in 0..count {
            let entry = entries.get::<Il2CppDictionaryEntry<TKey, TValue>>(i);
            if entry.hashCode >= 0 && entry.value == value {
                return true;
            }
        }
        false
    }

    pub unsafe fn try_get_value(&self, key: TKey, out_value: &mut TValue) -> bool {
        let i = self.find_entry(key);
        if i >= 0 {
            *out_value = self.entry_at(i).value;
            return true;
        }
        *out_value = TValue::default();
        false
    }

    pub unsafe fn get_value_or_default(&self, key: TKey) -> TValue {
        let i = self.find_entry(key);
        if i >= 0 {
            return self.entry_at(i).value;
        }
        TValue::default()
    }

    unsafe fn entry_at(&self, index: i32) -> &Il2CppDictionaryEntry<TKey, TValue> {
        let entries = &*self.entries;
        &entries.get::<Il2CppDictionaryEntry<TKey, TValue>>(index.max(0) as usize)
    }
}

#[repr(C)]
pub struct Il2CppDictionaryKeysCollection<TKey, TValue> {
    pub dictionary: *mut Il2CppDictionary<TKey, TValue>,
}

impl<TKey, TValue> Il2CppDictionaryKeysCollection<TKey, TValue>
where
    TKey: Copy + Default + PartialEq,
    TValue: Copy + Default + PartialEq,
{
    pub unsafe fn get(&self, index: i32) -> TKey {
        let dictionary = &*self.dictionary;
        if dictionary.entries.is_null() {
            return TKey::default();
        }
        let entries = &*dictionary.entries;
        entries.get::<Il2CppDictionaryEntry<TKey, TValue>>(index.max(0) as usize).key
    }

    pub unsafe fn get_count(&self) -> i32 {
        let dictionary = &*self.dictionary;
        dictionary.get_count()
    }
}

#[repr(C)]
pub struct Il2CppDictionaryValueCollection<TKey, TValue> {
    pub dictionary: *mut Il2CppDictionary<TKey, TValue>,
}

impl<TKey, TValue> Il2CppDictionaryValueCollection<TKey, TValue>
where
    TKey: Copy + Default + PartialEq,
    TValue: Copy + Default + PartialEq,
{
    pub unsafe fn get(&self, index: i32) -> TValue {
        let dictionary = &*self.dictionary;
        if dictionary.entries.is_null() {
            return TValue::default();
        }
        let entries = &*dictionary.entries;
        entries.get::<Il2CppDictionaryEntry<TKey, TValue>>(index.max(0) as usize).value
    }

    pub unsafe fn get_count(&self) -> i32 {
        let dictionary = &*self.dictionary;
        dictionary.get_count()
    }
}

#[il2cpp_ref_type("HANGJNJOFEC")]
pub struct HANGJNJOFEC;
impl HANGJNJOFEC {
    #[il2cpp_field(name = "JLDLGGOAFPP")]
    pub fn count(&self) -> System_Int32__Boxed {}

    #[il2cpp_field(name = "IIONFBIAHLP")]
    pub fn step(&self) -> System_Int32__Boxed {}

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