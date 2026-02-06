use axum::Router;
use il2cpp_runtime::{Il2CppObject, types::{Il2CppArray, System_RuntimeType, System_Type}};
use socketioxide::{SocketIo, extract::SocketRef};
use tokio::runtime::Runtime;
use std::{net::SocketAddr, str::FromStr, sync::OnceLock};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use anyhow::{anyhow, Context, Result};

use crate::cipher::{RPG_Client_GlobalVars, RPG_Client_RelicItemData};

const SERVER_ADDR: &str = "127.0.0.1:945";

static SOCKET_IO: OnceLock<SocketIo> = OnceLock::new();

pub fn start_server() {
    let runtime = Runtime::new().unwrap_or_else(|e| {
        log::error!("{e}");
        panic!("{e}");
    });
    runtime.block_on(async {
        let (layer, io) = SocketIo::new_layer();
        io.ns("/", on_connect);
        if SOCKET_IO.set(io).is_err() {
            let e = anyhow!("Failed to initialize SocketIO");
            log::error!("{e}");
            panic!("{e}");
        }

        let app = Router::new().layer(
            ServiceBuilder::new()
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                )
                .layer(layer),
        );

        // HTTP
        axum_server::bind(SocketAddr::from_str(SERVER_ADDR).unwrap_or_else(|e| {
            log::error!("{e}");
            panic!("{e}");
        }))
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|e| {
            log::error!("{e}");
            panic!("{e}");
        });
    });
}


fn on_connect(_socket: SocketRef) {
    log::info!("Client connected to orexis server");
}

fn apply_loadouts(id: u32, relics: Vec<u32>) -> Result<()> {
    log::info!("Applying loadout for avatar id {id}");

    let type_name = RPG_Client_RelicItemData::ffi_name();
    let runtime_type = System_RuntimeType::from_name(type_name)
        .context("Expected valid type name")?;
    let ty = runtime_type.get_il2cpp_type();

    let module_manager = RPG_Client_GlobalVars::s_ModuleManager()
        .context("Failed to resolve ModuleManager")?;
    let inventory_module = module_manager
        .InventoryModule()
        .context("Failed to resolve InventoryModule")?;

    let type_handle = System_Type::get_type_from_handle(ty)
        .context("Failed to resolve System.Type handle")?;
    let mut array = Il2CppArray::create_instance(
        type_handle,
        relics.len() as i32,
    )
    .context("Failed to create Il2CppArray")?;

    for (i, uid) in relics.iter().enumerate() {
        let relic_data = inventory_module
            .get_relic_data_by_uid(*uid)
            .with_context(|| format!("Failed to get relic data by uid {uid}"))?;
        if relic_data.0.is_null() {
            return Err(anyhow!("Relic data was null for uid {uid}"));
        }
        *(array.get_mut(i)) = relic_data;
    }

    let network_manager = RPG_Client_GlobalVars::s_NetworkManager()
        .context("Failed to resolve NetworkManager")?;
    network_manager
        .change_avatar_relics(id, array)
        .with_context(|| format!("Failed to change avatar relics for id {id}"))?;

    log::info!("Loadout applied successfully for avatar id {id}");
    Ok(())
}

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