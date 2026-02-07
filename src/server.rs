use il2cpp_runtime::Il2CppObject;
use il2cpp_runtime::types::{Il2CppArray, System_RuntimeType, System_Type};
use tokio::runtime::Runtime;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::{OnceLock, RwLock};

use crate::cipher::{RPG_Client_GlobalVars, RPG_Client_RelicItemData};

const WS_SERVER_ADDR: &str = "127.0.0.1:945";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CharacterLoadout {
    pub avatar_id: u32,
    pub name: String,
    pub relic_uids: Vec<u32>,
}

static LOADOUTS: OnceLock<RwLock<Vec<CharacterLoadout>>> = OnceLock::new();

#[derive(Deserialize)]
#[serde(untagged)]
#[allow(non_snake_case)]
enum IncomingMessage {
    SetLoadout { SetLoadout: CharacterLoadout },
    SetLoadouts { SetLoadouts: Vec<CharacterLoadout> },
    Tagged {
        #[serde(rename = "type")]
        msg_type: String,
        loadouts: Option<Vec<CharacterLoadout>>,
        loadout: Option<CharacterLoadout>,
    },
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum OutgoingMessage {
    #[serde(rename = "loadouts_updated")]
    LoadoutsUpdated { count: usize },
    #[serde(rename = "error")]
    Error { message: String },
}

pub fn start_server() {
    let runtime = Runtime::new().unwrap_or_else(|e| {
        log::error!("{e}");
        panic!("{e}");
    });
    runtime.block_on(async {
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

    while let Some(msg) = read.next().await {
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
                Ok(IncomingMessage::Tagged { msg_type, loadouts, loadout }) => {
                    match msg_type.as_str() {
                        "set_loadouts" => {
                            handle_apply_loadouts(loadouts.unwrap_or_default())
                        }
                        "set_loadout" => {
                            if let Some(item) = loadout {
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

    Ok(())
}

fn handle_apply_loadout(loadout: CharacterLoadout) -> OutgoingMessage {
    set_loadouts(vec![loadout.clone()]);
    match apply_loadout(loadout.avatar_id, loadout.relic_uids) {
        Ok(equipped_count) => OutgoingMessage::LoadoutsUpdated { count: equipped_count },
        Err(e) => {
            log::error!("Failed to apply loadout: {e}");
            OutgoingMessage::Error { message: format!("{e}") }
        }
    }
}

fn handle_apply_loadouts(loadouts: Vec<CharacterLoadout>) -> OutgoingMessage {
    set_loadouts(loadouts.clone());
    let mut total = 0;
    let mut errors = Vec::new();
    for loadout in loadouts {
        match apply_loadout(loadout.avatar_id, loadout.relic_uids) {
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
        OutgoingMessage::Error { message: errors.join("; ") }
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

fn apply_loadout(avatar_id: u32, relic_uids: Vec<u32>) -> Result<usize> {
    log::info!("[Orexis] Applying loadout for avatar {avatar_id} with {} relic UIDs", relic_uids.len());
    for (i, uid) in relic_uids.iter().enumerate() {
        log::info!("requested [{i}] uid={uid}");
    }

    let type_name = RPG_Client_RelicItemData::ffi_name();
    let runtime_type = System_RuntimeType::from_name(type_name)?;
    let ty = runtime_type.get_il2cpp_type();

    let module_manager = RPG_Client_GlobalVars::s_ModuleManager()?;
    let inventory_module = module_manager
        .InventoryModule()?;

    let mut relics_to_equip: Vec<RPG_Client_RelicItemData> = Vec::new();
    for uid in &relic_uids {
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
                    log::info!("uid={uid} currently on avatar {current_avatar}, will move to {avatar_id}");
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

    log::info!("Equipping {} relics (filtered from {} requested)", relics_to_equip.len(), relic_uids.len());
    let equipped_count = relics_to_equip.len();

    let type_handle = System_Type::get_type_from_handle(ty)
        .context("Failed to resolve System.Type handle")?;
    let mut array = Il2CppArray::create_instance(
        type_handle,
        equipped_count as i32,
    )
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
        .change_avatar_relics(avatar_id, array)
        .with_context(|| format!("Failed to change avatar relics for id {avatar_id}"))?;

    log::info!("[Orexis] Loadout applied successfully for avatar {avatar_id}");
    Ok(equipped_count)
}

#[allow(dead_code)]
fn apply_lightcone(id: u32, lightcone: u32) -> Result<()> {
    log::info!("Applying lightcone for avatar id {id}");

    let network_manager = RPG_Client_GlobalVars::s_NetworkManager()
        .context("Failed to resolve NetworkManager")?;
    network_manager
        .change_avatar_lightcone(id, lightcone)
        .with_context(|| format!("Failed to change avatar lightcone for id {id}"))?;

    log::info!("Lightcone applied successfully for avatar id {id}");
    Ok(())
}