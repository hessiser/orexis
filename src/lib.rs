#![feature(int_roundings)]
mod cipher;
mod logging;
mod models;
mod relic_utils;
mod server;
mod updater;
mod hooks;

use std::{
    ffi::c_void, io::Cursor, os::windows::process::CommandExt, process::Command, sync::LazyLock, thread, time::Duration
};

use anyhow::{Context, Result, anyhow};
use ctor::ctor;
use il2cpp_runtime::api::ApiIndexTable;
use tokio::runtime::Runtime;
use win32_notif::{
    NotificationActivatedEventHandler, NotificationBuilder, ToastsNotifier,
    notification::{
        actions::{ActionButton, action::ActivationType},
        visual::{Text, text::HintStyle},
    },
};
use windows::{
    Win32::System::{
        Diagnostics::Debug::ReadProcessMemory,
        LibraryLoader::GetModuleHandleW,
        ProcessStatus::{GetModuleInformation, MODULEINFO},
        Threading::{CREATE_NO_WINDOW, GetCurrentProcess},
    },
    core::{PCWSTR, w},
};

use crate::hooks::install_hooks;

pub static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Runtime::new().unwrap_or_else(|e| {
        log::error!("{e}");
        panic!("{e}");
    })
});

#[ctor]
pub fn main() {
    std::thread::spawn(|| {
        match run() {
            Ok(()) => {
                if let Ok(aumid) = resolve_aumid() {
                    if let Err(notif_err) =
                        show_notification(&aumid, "Core initialized successfully.")
                    {
                        log::error!("Failed to show success notification: {notif_err:#}");
                    }
                } else {
                    log::error!("Failed to resolve AUMID for success notification");
                }
            }
            Err(err) => {
                let msg = format!("Core init failed: {err}");
                if let Ok(aumid) = resolve_aumid() {
                    if let Err(notif_err) = show_notification(&aumid, &msg) {
                        log::error!("Failed to show error notification: {notif_err:#}");
                    }
                } else {
                    log::error!("Failed to resolve AUMID for error notification: {err:#}");
                }
            }
        }
        server::start_server();
    });
}

fn run() -> Result<()> {
    #[cfg(debug_assertions)]
    unsafe {
        windows::Win32::System::Console::AllocConsole().unwrap()
    };
    // Check for updates asynchronously
    RUNTIME.spawn(async {
        let updater = updater::Updater::new(env!("CARGO_PKG_VERSION"));
        match updater.check_update().await {
            Ok(Some(new_version)) => {
                log::info!("Update available: {}", new_version);
                if let Ok(aumid) = resolve_aumid() {
                    if let Err(err) = show_update_notification(&aumid, &new_version) {
                        log::error!("Failed to show update notification: {err:#}");
                    }
                }
            }
            Ok(None) => {
                log::debug!("No update available");
            }
            Err(err) => {
                log::error!("Failed to check for updates: {err:#}");
            }
        }
    });

    unsafe {
        while GetModuleHandleW(windows::core::w!("GameAssembly")).is_err()
            || GetModuleHandleW(windows::core::w!("UnityPlayer")).is_err()
        {
            thread::sleep(Duration::from_secs(3));
        }
    }
    logging::MultiLogger::init().context("Failed to initialize logging")?;

    init_runtime().context("Failed to initialize il2cpp runtime")?;
    Ok(())
}

fn resolve_aumid() -> Result<String> {
    // PowerShell script to get the AUMID of Honkai Star Rail or fallback to Windows PowerShell
    let script = indoc::indoc! {r#"
        # Try to find Honkai Star Rail first
        $app = Get-StartApps | Where-Object { $_.Name -eq 'Honkai Star Rail' }

        if ($app) {
            $app.AppID
        } else {
            # Fallback: find Windows PowerShell
            $fallback = Get-StartApps | Where-Object { $_.Name -like 'Windows PowerShell*' }
            if ($fallback) {
                $fallback.AppID
            } else {
                ''
            }
        }
    "#};

    // Run PowerShell silently
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .creation_flags(CREATE_NO_WINDOW.0)
        .output()
        .context("Failed to execute PowerShell to resolve AUMID")?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn show_notification(aumid: &str, message: &str) -> Result<()> {
    let notifier = ToastsNotifier::new(aumid).context("Failed to create Windows toast notifier")?;
    let notif = NotificationBuilder::new()
        .visual(
            Text::create(0, env!("CARGO_PKG_NAME"))
                .with_align_center(true)
                .with_wrap(true)
                .with_style(HintStyle::Title),
        )
        .visual(
            Text::create_binded(1, "desc")
                .with_align_center(true)
                .with_wrap(true)
                .with_style(HintStyle::Body),
        )
        .value("desc", message)
        .build(0, &notifier, "01", "readme")
        .context("Failed to build toast notification")?;

    notif.show().context("Failed to show toast notification")?;
    Ok(())
}

fn show_update_notification(aumid: &str, new_version: &str) -> Result<()> {
    let notifier = ToastsNotifier::new(aumid).context("Failed to create Windows toast notifier")?;
    let message = format!("New version {} available", new_version);

    let notif = NotificationBuilder::new()
        .visual(
            Text::create(0, "Update Available")
                .with_align_center(true)
                .with_wrap(true)
                .with_style(HintStyle::Title),
        )
        .visual(
            Text::create_binded(1, "desc")
                .with_align_center(true)
                .with_wrap(true)
                .with_style(HintStyle::Body),
        )
        .value("desc", &message)
        .action(ActionButton::create("Download").with_activation_type(ActivationType::Foreground).with_id("download_btn"))
        .on_activated(NotificationActivatedEventHandler::new(|_, args: Option<win32_notif::handler::ToastActivatedArgs>| {
            if let Some(id) = args.and_then(|a| a.button_id) {
                if id == "download_btn" {
                    log::info!("User clicked Download button");
                    RUNTIME.spawn(async {
                        let updater = updater::Updater::new(env!("CARGO_PKG_VERSION"));
                        match updater.download_update().await {
                            Ok(()) => log::info!("Update downloaded successfully"),
                            Err(e) => log::error!("Failed to download update: {e:#}"),
                        }
                    });
                }
            }
            Ok(())
        }))
        .build(0, &notifier, "01", "update")
        .context("Failed to build update notification")?;

    notif.show().context("Failed to show update notification")?;
    Ok(())
}

fn get_module_handle(name: PCWSTR) -> Result<usize> {
    unsafe {
        GetModuleHandleW(name)
            .map(|v| v.0 as usize)
            .context("Failed to get module handle")
    }
}

pub fn get_il2cpp_table_offset() -> Result<usize> {
    unsafe {
        let unityplayer_offset = get_module_handle(w!("UnityPlayer"))
            .map_err(|e| anyhow!(e.to_string()))
            .context("Failed to resolve UnityPlayer module")?;
        let module = windows::Win32::Foundation::HMODULE(unityplayer_offset as *mut c_void);

        let process_handle = GetCurrentProcess();
        let mut lp_mod_info = MODULEINFO::default();

        GetModuleInformation(
            process_handle,
            module,
            &mut lp_mod_info,
            size_of::<MODULEINFO>() as u32,
        )
        .context("Failed to read module information")?;

        let buffer = vec![0u8; lp_mod_info.SizeOfImage as usize];
        let mut bytes_read = 0usize;

        ReadProcessMemory(
            process_handle,
            module.0,
            buffer.as_ptr() as _,
            lp_mod_info.SizeOfImage as usize,
            Some(&mut bytes_read),
        )
        .context("Failed to read module memory")?;

        static PATTERN: &str = "48 8B 05 ? ? ? ? 48 8D 0D ? ? ? ? FF D0";
        let locs = patternscan::scan(Cursor::new(buffer), &PATTERN)
            .context("Failed to scan for il2cpp pattern")?;
        let addr = locs
            .get(0)
            .context("Pattern not found in UnityPlayer module")?
            + module.0 as usize;

        let qword_addr = addr + 7 + *((addr + 3) as *const i32) as usize;
        Ok(qword_addr)
    }
}

fn init_runtime() -> Result<()> {
    let table = ApiIndexTable {
        il2cpp_assembly_get_image: 22,
        il2cpp_class_get_methods: 35,
        il2cpp_class_get_name: 37,
        il2cpp_class_from_type: 49,
        il2cpp_domain_get: 63,
        il2cpp_domain_get_assemblies: 65,
        il2cpp_field_get_name: 73,
        il2cpp_field_get_value_object: 77,
        il2cpp_method_get_return_type: 116,
        il2cpp_method_get_name: 117,
        il2cpp_method_get_param_count: 123,
        il2cpp_method_get_param: 124,
        il2cpp_thread_attach: 154,
        il2cpp_type_get_name: 161,
        il2cpp_image_get_class_count: 169,
        il2cpp_image_get_class: 170,
    };
    il2cpp_runtime::init(get_il2cpp_table_offset()?, table)?;
    (unsafe { install_hooks().context("Failed to install hooks") })?;
    Ok(())
}
