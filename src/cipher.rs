#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use il2cpp_runtime::prelude::*;

#[il2cpp_ref_type("RPG.Client.RelicItemData")]
pub struct RPG_Client_RelicItemData;

impl RPG_Client_RelicItemData {
    #[il2cpp_getter_property(property = "BelongAvatarID")]
    pub fn get_BelongAvatarID(&self) -> u32 {}
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