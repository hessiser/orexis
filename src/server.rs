use anyhow::{Context, Result};
use futures_util::{SinkExt, StreamExt};
use il2cpp_runtime::{Il2CppObject, get_cached_class};
use il2cpp_runtime::api::{il2cpp_class_get_fields, il2cpp_domain_get, il2cpp_field_get_type, il2cpp_thread_attach};
use il2cpp_runtime::types::{Il2CppArray, Il2CppString, List, System_RuntimeType, System_Type, System_UInt32};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ffi::c_void;
use std::ptr::null;
use std::sync::{OnceLock, RwLock};
use std::thread;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use serde::{Deserializer};
use std::collections::BTreeSet;

use crate::RUNTIME;
use crate::cipher::{
    Dictionary, RPG_Client_GlobalVars, RPG_Client_Promises_PromiseState, RPG_Client_RelicItemData, RPG_Client_RelicPresetAvatarPlans, RPG_Client_RelicPresetPlanData, RPG_Client_RelicSmartSuit_RankType, RPG_Client_RelicSmartSuit_RelicSmartSuitCalculator, RelicPresetPlanNetworkSource, System_Activator
};
use crate::models::{ReliquaryLightCone, ReliquaryRelic};
use crate::relic_utils::{get_light_cones_snapshot, get_relics_snapshot};

const WS_SERVER_ADDR: &str = "127.0.0.1:945";
const LIVE_IMPORT_SOURCE: &str = "reliquary_archiver";
const LIVE_IMPORT_BUILD: &str = "v0.8.0";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CharacterLoadout {
    #[serde(deserialize_with = "deserialize_u32_from_any")]
    pub avatar_id: u32,
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_relic_uids")]
    pub relic_uids: Vec<u32>,
}

#[derive(Serialize, Clone, Debug)]
pub struct RelicPresetPlan {
    pub plan_uid: u32,
    pub relic_uids: Vec<u32>,
    pub plan_name: String,
    pub plan_score: String
}

static LOADOUTS: OnceLock<RwLock<Vec<CharacterLoadout>>> = OnceLock::new();
static LIVE_IMPORT_SENDER: OnceLock<broadcast::Sender<LiveImportEvent>> = OnceLock::new();

#[derive(Deserialize)]
#[serde(untagged)]
#[allow(non_snake_case)]
enum IncomingMessage {
    SetLoadout {
        SetLoadout: CharacterLoadout,
    },
    SetLoadouts {
        SetLoadouts: Vec<CharacterLoadout>,
    },
    Tagged {
        #[serde(rename = "type")]
        msg_type: String,
        loadouts: Option<Vec<CharacterLoadout>>,
        loadout: Option<CharacterLoadout>,
        data: Option<Value>,
    },
}

fn parse_u32_from_value(value: &Value) -> Option<u32> {
    match value {
        Value::Number(num) => num.as_u64().and_then(|v| u32::try_from(v).ok()),
        Value::String(raw) => raw.trim().parse::<u32>().ok(),
        _ => None,
    }
}

fn deserialize_u32_from_any<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    parse_u32_from_value(&value).ok_or_else(|| {
        serde::de::Error::custom("expected a positive integer (number or numeric string)")
    })
}

fn deserialize_relic_uids<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let values = Vec::<Value>::deserialize(deserializer)?;
    Ok(values
        .into_iter()
        .filter_map(|value| parse_u32_from_value(&value))
        .collect())
}

fn parse_loadout_value(value: &Value) -> Option<CharacterLoadout> {
    serde_json::from_value::<CharacterLoadout>(value.clone()).ok()
}

fn parse_loadouts_value(value: &Value) -> Option<Vec<CharacterLoadout>> {
    serde_json::from_value::<Vec<CharacterLoadout>>(value.clone()).ok()
}

fn resolve_single_loadout(loadout: Option<CharacterLoadout>, data: Option<&Value>) -> Option<CharacterLoadout> {
    if loadout.is_some() {
        return loadout;
    }

    let Some(data) = data else {
        return None;
    };

    parse_loadout_value(data)
        .or_else(|| data.get("loadout").and_then(parse_loadout_value))
        .or_else(|| data.get("SetLoadout").and_then(parse_loadout_value))
}

fn resolve_many_loadouts(loadouts: Option<Vec<CharacterLoadout>>, data: Option<&Value>) -> Option<Vec<CharacterLoadout>> {
    if loadouts.is_some() {
        return loadouts;
    }

    let Some(data) = data else {
        return None;
    };

    parse_loadouts_value(data)
        .or_else(|| data.get("loadouts").and_then(parse_loadouts_value))
        .or_else(|| data.get("SetLoadouts").and_then(parse_loadouts_value))
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum OutgoingMessage {
    #[serde(rename = "loadouts_updated")]
    LoadoutsUpdated { count: usize },
    #[serde(rename = "error")]
    Error { message: String },
}

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "event", content = "data")]
enum LiveImportEvent {
    InitialScan(LiveExport),
    UpdateRelics(Vec<ReliquaryRelic>),
    UpdateLightCones(Vec<ReliquaryLightCone>),
}

#[derive(Serialize, Clone, Debug)]
struct LiveExport {
    source: &'static str,
    build: &'static str,
    version: u32,
    metadata: LiveMetadata,
    gacha: LiveGachaFunds,
    materials: Vec<Value>,
    light_cones: Vec<ReliquaryLightCone>,
    relics: Vec<ReliquaryRelic>,
    characters: Vec<Value>,
}

#[derive(Serialize, Clone, Debug)]
struct LiveMetadata {
    uid: Option<u32>,
    trailblazer: Option<&'static str>,
}

#[derive(Serialize, Clone, Debug, Default)]
struct LiveGachaFunds {
    stellar_jade: u32,
    oneric_shards: u32,
}

pub fn start_server() {
    RUNTIME.block_on(async {
        tokio::spawn(async {
            if let Err(e) = start_ws_server().await {
                log::error!("WebSocket server error: {e}");
            }
        });

        futures_util::future::pending::<()>().await;
    });
}

async fn start_ws_server() -> Result<()> {
    let listener = TcpListener::bind(WS_SERVER_ADDR).await.unwrap_or_else(|e| {
        log::error!("{e}");
        panic!("{e}");
    });
    log::info!("WebSocket server listening on {WS_SERVER_ADDR}");

    while let Ok((stream, addr)) = listener.accept().await {
        log::info!("New connection from: {addr}");
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                log::error!("Connection error: {e}");
            }
        });
    }

    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    let (mut write, mut read) = ws_stream.split();
    let mut live_rx = get_live_import_sender().subscribe();

    let initial_scan = build_initial_scan_event();
    let initial_json = serde_json::to_string(&initial_scan)?;
    write.send(Message::Text(initial_json)).await?;

    loop {
        tokio::select! {
            msg = read.next() => {
                let Some(msg) = msg else {
                    break;
                };
                let msg = msg?;

                if msg.is_text() || msg.is_binary() {
                    let text = msg.to_text()?;
                    log::debug!("Received: {text}");

                    let response = match serde_json::from_str::<IncomingMessage>(text) {
                        Ok(IncomingMessage::SetLoadout { SetLoadout: loadout }) => {
                            handle_apply_loadout(loadout)
                        }
                        Ok(IncomingMessage::SetLoadouts { SetLoadouts: loadouts }) => {
                            handle_apply_loadouts(loadouts)
                        }
                        Ok(IncomingMessage::Tagged { msg_type, loadouts, loadout, data }) => {
                            match msg_type.as_str() {
                                "set_loadouts" => {
                                    if let Some(items) = resolve_many_loadouts(loadouts, data.as_ref()) {
                                        handle_apply_loadouts(items)
                                    } else {
                                        OutgoingMessage::Error { message: "Missing loadouts".to_string() }
                                    }
                                }
                                "set_loadout" => {
                                    if let Some(item) = resolve_single_loadout(loadout, data.as_ref()) {
                                        handle_apply_loadout(item)
                                    } else {
                                        OutgoingMessage::Error { message: "Missing loadout".to_string() }
                                    }
                                }
                                _ => OutgoingMessage::Error { message: format!("Unsupported message type: {msg_type}") },
                            }
                        }
                        Err(e) => OutgoingMessage::Error { message: format!("Invalid message: {e}") },
                    };

                    let response_json = serde_json::to_string(&response)?;
                    write.send(Message::Text(response_json)).await?;
                }
            }
            event = live_rx.recv() => {
                match event {
                    Ok(event) => {
                        let json = serde_json::to_string(&event)?;
                        write.send(Message::Text(json)).await?;
                    }
                    Err(RecvError::Lagged(_)) => {
                        continue;
                    }
                    Err(RecvError::Closed) => {
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

fn handle_apply_loadout(loadout: CharacterLoadout) -> OutgoingMessage {
    set_loadouts(vec![loadout.clone()]);
    match apply_loadout(loadout.avatar_id, &loadout.relic_uids) {
        Ok(equipped_count) => OutgoingMessage::LoadoutsUpdated {
            count: equipped_count,
        },
        Err(e) => {
            log::error!("Failed to apply loadout: {e}");
            OutgoingMessage::Error {
                message: format!("{e}"),
            }
        }
    }
}

fn handle_apply_loadouts(loadouts: Vec<CharacterLoadout>) -> OutgoingMessage {
    set_loadouts(loadouts.clone());
    let mut total = 0;
    let mut errors = Vec::new();
    for loadout in loadouts {
        match apply_loadout(loadout.avatar_id, &loadout.relic_uids) {
            Ok(count) => total += count,
            Err(e) => {
                log::error!("Failed to apply loadout '{}': {e}", loadout.name);
                errors.push(format!("{}: {e}", loadout.name));
            }
        }
    }
    if errors.is_empty() {
        OutgoingMessage::LoadoutsUpdated { count: total }
    } else {
        OutgoingMessage::Error {
            message: errors.join("; "),
        }
    }
}

fn set_loadouts(loadouts: Vec<CharacterLoadout>) {
    log::info!("Received {} loadout configurations", loadouts.len());
    if let Some(lock) = LOADOUTS.get() {
        let mut existing = lock.write().unwrap();
        for loadout in loadouts {
            existing.retain(|l| !(l.avatar_id == loadout.avatar_id && l.name == loadout.name));
            existing.push(loadout);
        }
    } else {
        let _ = LOADOUTS.set(RwLock::new(loadouts));
    }
}

fn apply_loadout(avatar_id: u32, relic_uids: &Vec<u32>) -> Result<usize> {
    unsafe {
        log::info!(
            "[Orexis] Applying loadout for avatar {avatar_id} with {} relic UIDs",
            relic_uids.len()
        );
        for (i, uid) in relic_uids.iter().enumerate() {
            log::info!("requested [{i}] uid={uid}");
        }

        let type_name = RPG_Client_RelicItemData::ffi_name();
        let runtime_type = System_RuntimeType::from_name(type_name)?;
        let ty = runtime_type.get_il2cpp_type();

        let module_manager = RPG_Client_GlobalVars::s_ModuleManager()?;
        let inventory_module = module_manager.InventoryModule()?;

        let mut relics_to_equip: Vec<RPG_Client_RelicItemData> = Vec::new();
        for uid in relic_uids {
            let relic_data = inventory_module
                .get_relic_data_by_uid(*uid)
                .with_context(|| format!("Failed to get relic data for uid {uid}"))?;
            if relic_data.0.is_null() {
                log::warn!("uid={uid} returned null RelicItemData, skipping");
                continue;
            }

            match relic_data.get_BelongAvatarID() {
                Ok(current_avatar) => {
                    if current_avatar == avatar_id {
                        log::info!("uid={uid} already equipped on avatar {avatar_id}, skipping");
                        continue;
                    }
                    if current_avatar != 0 {
                        log::info!(
                            "uid={uid} currently on avatar {current_avatar}, will move to {avatar_id}"
                        );
                    } else {
                        log::info!("uid={uid} not equipped on anyone, will equip on {avatar_id}");
                    }
                }
                Err(e) => {
                    log::warn!("uid={uid} couldn't read equipped avatar: {e}, including anyway");
                }
            }

            relics_to_equip.push(relic_data);
        }

        if relics_to_equip.is_empty() {
            log::info!("All relics already equipped on avatar {avatar_id}, nothing to do");
            return Ok(0);
        }

        log::info!(
            "Equipping {} relics (filtered from {} requested)",
            relics_to_equip.len(),
            relic_uids.len()
        );
        let equipped_count = relics_to_equip.len();

        let type_handle = System_Type::get_type_from_handle(ty)
            .context("Failed to resolve System.Type handle")?;
        let mut array = Il2CppArray::create_instance(type_handle, equipped_count as i32)
            .context("Failed to create Il2CppArray")?;

        for (i, relic_data) in relics_to_equip.into_iter().enumerate() {
            *(array.get_mut(i)) = relic_data;
        }

        log::info!("Il2CppArray created: len={}", array.len());

        for i in 0..array.len() {
            let item: &RPG_Client_RelicItemData = array.get(i);
            let null_str = if item.0.is_null() { "NULL" } else { "valid" };
            log::info!("array[{i}] = {null_str} (ptr=0x{:x})", item.0 as usize);
        }

        let network_manager = RPG_Client_GlobalVars::s_NetworkManager()
            .context("Failed to resolve NetworkManager")?;
        network_manager
            .apply_avatar_relics(avatar_id, array)
            .with_context(|| format!("Failed to change avatar relics for id {avatar_id}"))?;

        log::info!("[Orexis] Loadout applied successfully for avatar {avatar_id}");
        Ok(equipped_count)
    }
}

// #[allow(dead_code)]
// fn apply_lightcone(id: u32, lightcone: u32) -> Result<()> {
//     unsafe {
//         log::info!("Applying lightcone for avatar id {id}");

//         let network_manager = RPG_Client_GlobalVars::s_NetworkManager()
//             .context("Failed to resolve NetworkManager")?;
//         network_manager
//             .change_avatar_lightcone(id, lightcone)
//             .with_context(|| format!("Failed to change avatar lightcone for id {id}"))?;

//         log::info!("Lightcone applied successfully for avatar id {id}");
//         Ok(())
//     }
// }

pub fn get_type_handle<S: AsRef<str>>(type_name: S) -> Result<System_Type> {
    let type_name = type_name.as_ref();
    let runtime_type = System_RuntimeType::from_name(type_name)?;
    let ty = runtime_type.get_il2cpp_type();
    Ok(unsafe { System_Type::get_type_from_handle(ty)? })
}

pub fn create_list<S, A, T>(
    element_type_name: S,
    elements: A,
) -> Result<List>
where
    S: AsRef<str>,
    A: AsRef<[T]>,
    T: Copy,
{
    let type_name = element_type_name.as_ref();
    let generic_ty = get_type_handle(type_name)?;
    let length = elements.as_ref().len();
    let mut values = unsafe { Il2CppArray::create_instance(generic_ty, length as i32) }?;
    for (i, element) in elements.as_ref().iter().enumerate() {
        unsafe {
            let dest = values.get_mut(i);
            std::ptr::write(dest as *mut T, element.clone());
        }
    }
    let list_type = {
        let mut array = unsafe { Il2CppArray::create_instance(generic_ty, 1)? };
        unsafe { std::ptr::write(array.get_mut(0) as *mut System_Type, generic_ty) };
        (unsafe { System_RuntimeType::from_name("System.Collections.Generic.List<T>")?.make_generic_type(array) })?
    };
    let list_object = {
        let mut array  = unsafe { Il2CppArray::create_instance(get_type_handle("System.Object")?, 1)? };
        unsafe { std::ptr::write(array.get_mut(0) as *mut Il2CppArray, values) };
        unsafe { System_Activator::CreateInstance(System_Type::get_type_from_handle(list_type.get_il2cpp_type())?, array)? }
    };
    
    Ok(List(list_object))

}

fn get_relic_plan_network_source() -> Result<RelicPresetPlanNetworkSource> {
    let di_container = RPG_Client_GlobalVars::s_DiContainer()?;
    
    let field_iter: *const c_void = null();
    let mut network_source_type = None;
    loop {
        let field = il2cpp_class_get_fields(get_cached_class("RPG.Client.RelicPresetPlanViewModel")?, &field_iter);
        if field.0.is_null() {
            break;
        }
        if field.name() == "_NetworkService" {
            network_source_type = Some(il2cpp_field_get_type(field).name());

        }
    }

    let network_source = if let Some(type_name) = network_source_type {
        RelicPresetPlanNetworkSource(unsafe {
            di_container.resolve(get_type_handle(type_name)?, Il2CppString(null()))
        }?)
    } else {
        return Err(anyhow::anyhow!("Failed to find _NetworkService field in RelicPresetPlanViewModel"));
    };
    Ok(network_source)
}

fn calculate_relic_set_score(avatar_id: u32, relic_uids: &Vec<u32>) -> Result<RPG_Client_RelicSmartSuit_RankType> {
    unsafe {
        let module_manager = RPG_Client_GlobalVars::s_ModuleManager()?;
        let avatar_module = module_manager.AvatarModule()?;
        let inventory_module = module_manager.InventoryModule()?;

        let avatar_data = avatar_module.get_avatar(avatar_id)?;

        let mut relics_to_equip: Vec<RPG_Client_RelicItemData> = Vec::new();
        for uid in relic_uids {
            relics_to_equip.push(
                inventory_module.get_relic_data_by_uid(*uid)?
            );
        }

        let array = create_list(
            RPG_Client_RelicItemData::ffi_name(),
            relics_to_equip,
        )?;
        let score = RPG_Client_RelicSmartSuit_RelicSmartSuitCalculator::CalculateRank(array, avatar_data)?;
        Ok(score)
    }
}

fn add_relic_plan(avatar_id: u32, relic_uids: Vec<u32>) -> Result<()> {
    thread::spawn(move || -> Result<()> {
        let domain = il2cpp_domain_get();
        il2cpp_thread_attach(domain);

        let network_source = get_relic_plan_network_source()?;

        let relic_set_score = calculate_relic_set_score(avatar_id, &relic_uids)?;
        let relic_uids = create_list(System_UInt32::ffi_name(), relic_uids)?;

        unsafe {
            network_source
                .add_plan(avatar_id, relic_uids, Il2CppString::new("").unwrap(), relic_set_score as i32, 5)?;
        };

        Ok(())
    }).join().unwrap_or_else(|e| {
        log::error!("Thread panicked: {:?}", e);
        Err(anyhow::anyhow!("Thread panicked"))
    })?;
    Ok(())
}

// If plan_name is null, map to None
// If plan_name is Some but empty, handle as empty string
fn update_relic_plan(avatar_id: u32, relic_uids: Vec<u32>, plan_uid: u32, plan_name: Option<String>) -> Result<()> {
    thread::spawn(move || -> Result<()> {
        let domain = il2cpp_domain_get();
        il2cpp_thread_attach(domain);

        let network_source = get_relic_plan_network_source()?;

        let relic_set_score = calculate_relic_set_score(avatar_id, &relic_uids)?;
        let relic_uids = create_list(System_UInt32::ffi_name(), relic_uids)?;

        unsafe {
            network_source
                .update_plan(avatar_id, plan_uid, relic_uids, relic_set_score as i32, 0)?;

            if let Some(name) = plan_name {
                network_source.update_plan_name(avatar_id, plan_uid, Il2CppString::new(name)?)?;
            }
        };

        Ok(())
    }).join().unwrap_or_else(|e| {
        log::error!("Thread panicked: {:?}", e);
        Err(anyhow::anyhow!("Thread panicked"))
    })?;
    Ok(())
}

fn delete_relic_plan(avatar_id: u32, plan_uid: u32) -> Result<()> {
    thread::spawn(move || -> Result<()> {
        let domain = il2cpp_domain_get();
        il2cpp_thread_attach(domain);

        let network_source = get_relic_plan_network_source()?;
        unsafe { network_source.delete_plan(avatar_id, plan_uid)? };
        Ok(())
    }).join().unwrap_or_else(|e| {
        log::error!("Thread panicked: {:?}", e);
        Err(anyhow::anyhow!("Thread panicked"))
    })?;
    Ok(())
}

fn get_relic_plans(avatar_id: u32) -> Result<Vec<RelicPresetPlan>> {
    thread::spawn(move || -> Result<Vec<RelicPresetPlan>> {
        unsafe {
            let domain = il2cpp_domain_get();
            il2cpp_thread_attach(domain);

            let relic_preset_model = RPG_Client_GlobalVars::s_ModuleManager()?
                .RelicModule()?
                .RelicPresetModel()?;
            let promise = relic_preset_model.GetAvatarRelicPresetPlan(avatar_id)?;

            const MAX_RETRIES: u32 = 20; // ~2 seconds at 100ms intervals
            const POLL_INTERVAL_MS: u64 = 100;

            for attempt in 0..MAX_RETRIES {
                let state = *promise.get_CurState()?;

                match state {
                    RPG_Client_Promises_PromiseState::Resolved => {
                        log::info!(
                            "Promise for avatar ID {} resolved after {} attempts",
                            avatar_id,
                            attempt
                        );
                        break;
                    }
                    RPG_Client_Promises_PromiseState::Rejected => {
                        return Err(anyhow::anyhow!("Promise rejected for avatar ID {}", avatar_id));
                    }
                    RPG_Client_Promises_PromiseState::Pending => {
                        // Still pending, will check again
                    }
                }

                if attempt == MAX_RETRIES - 1 {
                    return Err(anyhow::anyhow!(
                        "Promise timeout for avatar ID {} after {}ms",
                        avatar_id,
                        MAX_RETRIES as u64 * POLL_INTERVAL_MS
                    ));
                }

                thread::sleep(std::time::Duration::from_millis(POLL_INTERVAL_MS));
            }

            let relic_preset_plans = RPG_Client_RelicPresetAvatarPlans(null());
            let mut plans = Vec::new();

            if (
                relic_preset_model.TryGetPlans(
                    avatar_id,
                    &relic_preset_plans,
                )
            )? {
                let dict_ptr = relic_preset_plans.PlanDict()?.as_ptr()
                    as *const Dictionary<u32, RPG_Client_RelicPresetPlanData>;
                if dict_ptr.is_null() {
                    return Err(anyhow::anyhow!(
                        "Relic preset dictionary was null for avatar ID {}",
                        avatar_id
                    ));
                }

                for value in (*dict_ptr).get_values().iter() {
                    let name = match value.get_Name() {
                        Ok(n) => n.to_string(),
                        Err(il2cpp_runtime::errors::Il2CppError::NullPointerDereference) => {
                            String::new()
                        }
                        Err(e) => {
                            return Err(anyhow::anyhow!(
                                "Failed to get Name from RelicPresetPlanData: {:?}",
                                e
                            ));
                        }
                    };
                    let relic_uids = value
                        .get_RelicUniqueIDs()
                        .context("Failed to get relic UID array from RelicPresetPlanData")?
                        .to_vec::<u32>();
                    let plan_uid = value
                        .get_UniqueID()
                        .context("Failed to get plan UID from RelicPresetPlanData")?;

                    let plan_score = calculate_relic_set_score(avatar_id, &relic_uids)?;
                    plans.push(RelicPresetPlan {
                        plan_uid,
                        relic_uids,
                        plan_name: name,
                        plan_score: plan_score.to_string(),
                    });
                }
            } else {
                return Err(anyhow::anyhow!(
                    "Failed to retrieve relic preset plans for avatar ID {}",
                    avatar_id
                ));
            }

            Ok(plans)
        }
    })
    .join()
    .unwrap_or_else(|e| {
        log::error!("Thread panicked: {:?}", e);
        Err(anyhow::anyhow!("Thread panicked"))
    })
}

pub fn send_live_relic_update(relics: Vec<ReliquaryRelic>) {
    if relics.is_empty() {
        return;
    }

    let _ = get_live_import_sender().send(LiveImportEvent::UpdateRelics(relics));
}

pub fn send_live_light_cone_update(light_cones: Vec<ReliquaryLightCone>) {
    if light_cones.is_empty() {
        return;
    }

    let _ = get_live_import_sender().send(LiveImportEvent::UpdateLightCones(light_cones));
}

fn get_live_import_sender() -> &'static broadcast::Sender<LiveImportEvent> {
    LIVE_IMPORT_SENDER.get_or_init(|| {
        let (sender, _) = broadcast::channel(128);
        sender
    })
}

fn build_initial_scan_event() -> LiveImportEvent {
    let relics: Vec<ReliquaryRelic> = get_relics_snapshot()
        .into_iter()
        .map(|relic| ReliquaryRelic::from(&relic))
        .collect();

    let light_cones: Vec<ReliquaryLightCone> = get_light_cones_snapshot()
        .into_iter()
        .map(|lc| ReliquaryLightCone::from(&lc))
        .collect();

    let characters = build_characters_from_equipment(&relics, &light_cones);

    LiveImportEvent::InitialScan(LiveExport {
        source: LIVE_IMPORT_SOURCE,
        build: LIVE_IMPORT_BUILD,
        version: 4,
        metadata: LiveMetadata {
            uid: None,
            trailblazer: None,
        },
        gacha: LiveGachaFunds::default(),
        materials: Vec::new(),
        light_cones,
        relics,
        characters,
    })
}

fn build_characters_from_equipment(
    relics: &[ReliquaryRelic],
    light_cones: &[ReliquaryLightCone],
) -> Vec<Value> {
    let mut ids = BTreeSet::<String>::new();

    for relic in relics {
        if !relic.location.is_empty() {
            ids.insert(relic.location.clone());
        }
    }

    for light_cone in light_cones {
        if !light_cone.location.is_empty() {
            ids.insert(light_cone.location.clone());
        }
    }

    if let Some(loadouts) = LOADOUTS.get() {
        for loadout in loadouts.read().unwrap().iter() {
            ids.insert(loadout.avatar_id.to_string());
        }
    }

    ids.into_iter()
        .map(|id| {
            serde_json::json!({
                "id": id,
                "name": "Unknown",
                "path": "Unknown",
                "level": 80,
                "ascension": 6,
                "eidolon": 0,
                "ability_version": 0
            })
        })
        .collect()
}
