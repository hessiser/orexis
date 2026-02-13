use anyhow::{Context, Result, anyhow};
use reqwest::Client;
use semver::Version;
use serde::{Deserialize, de};
use windows::Win32::{Foundation::{GetLastError, HMODULE, MAX_PATH}, System::{LibraryLoader::{GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, GetModuleFileNameW, GetModuleHandleExA}, Threading::CREATE_NO_WINDOW}};
use std::{env, ffi::OsString, fs, os::windows::{ffi::OsStringExt, process::CommandExt}, path::PathBuf, process::Command};

const GITHUB_RELEASES_ENDPOINT: &str = "https://api.github.com/repos/hessiser/orexis/releases";
const DLL_ASSET_NAME: &str = "orexis.dll";

#[derive(Clone, Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    assets: Vec<GithubAsset>,
    #[serde(default)]
    prerelease: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Clone)]
pub struct Updater {
    client: Client,
    current_version: String,
}

impl Updater {
    pub fn new(current_version: &str) -> Self {
        Self {
            client: Client::new(),
            current_version: current_version.to_string(),
        }
    }

    /// Check if an update is available
    pub async fn check_update(&self) -> Result<Option<String>> {
        let release = self
            .fetch_latest_release()
            .await?
            .ok_or_else(|| anyhow!("No release found"))?;

        let current_tag = self.current_version.trim_start_matches('v');
        let latest_tag = release.tag_name.trim_start_matches('v');

        let update_needed = match (Version::parse(latest_tag), Version::parse(current_tag)) {
            (Ok(latest), Ok(current)) => {
                log::debug!("semver compare: latest={:?}, current={:?}", latest, current);
                latest > current
            }
            _ => {
                // Fallback to string comparison
                latest_tag > current_tag
            }
        };

        if update_needed {
            Ok(Some(release.tag_name))
        } else {
            Ok(None)
        }
    }

    /// Download and apply the update
    pub async fn download_update(&self) -> Result<()> {
        let release = self
            .fetch_latest_release()
            .await?
            .ok_or_else(|| anyhow!("No eligible release found during download"))?;

        let dll_asset = release
            .assets
            .iter()
            .find(|a| a.name == DLL_ASSET_NAME)
            .ok_or_else(|| anyhow::anyhow!(
                "{DLL_ASSET_NAME} not found in release {}",
                release.tag_name
            ))?;

        let dll_path = module_path()?;
        let dll_path_str = dll_path.to_string_lossy().to_string();

        let tmp_dll_path = format!("{}.tmp", dll_path_str);

        let response = self
            .client
            .get(&dll_asset.browser_download_url)
            .send()
            .await?;

        let dll_bytes = response
            .bytes()
            .await?;

        fs::write(&tmp_dll_path, dll_bytes)?;

        let pid = std::process::id();

        // Build PowerShell script dynamically
        let mut script = String::new();
        let defender_exclusion = true; // Set to false if you don't want to add Defender exclusion
        if defender_exclusion {
            script.push_str(&indoc::formatdoc!(
                r#"
                Add-MpPreference -ExclusionPath {tmp_dll_path}
            "#
            ));
        }

        script.push_str(&indoc::formatdoc!(
            r#"
            Stop-Process -Id {pid}
            while (Get-Process -Id {pid} -ErrorAction SilentlyContinue) {{
                Start-Sleep -Milliseconds 200
            }}
            Move-Item -Force "{tmp_dll_path}" "{dll_path_str}"
            if (!$?) {{
                Write-Host "Move failed!"
                Pause
                Exit 1
            }}
        "#
        ));

        if defender_exclusion {
            script.push_str(&indoc::formatdoc!(
                r#"
                Remove-MpPreference -ExclusionPath "{tmp_dll_path}"
            "#
            ));
        }

        let env_args = env::args_os()
            .map(|x| x.to_string_lossy().to_string())
            .collect::<Vec<String>>()
            .join(" ");
        script.push_str(&format!("{}\n", &env_args));
        // script.push_str(
        //     "Read-Host -Prompt \"Press any key to continue or CTRL+C to quit\" | Out-Null",
        // );

        // Spawn PowerShell process
        Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                &script,
            ])
            .creation_flags(CREATE_NO_WINDOW.0)
            .spawn()?;

        Ok(())
    }

    async fn fetch_latest_release(&self) -> Result<Option<GithubRelease>> {
        let response = self
            .client
            .get(GITHUB_RELEASES_ENDPOINT)
            .header("User-Agent", "orexis-updater")
            .query(&[("per_page", "10")])
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let response = response.error_for_status()?;
        let releases = response.json::<Vec<GithubRelease>>().await?;

        let release = releases.into_iter().find(|release| {
            release
                .assets
                .iter()
                .any(|asset| asset.name == DLL_ASSET_NAME)
        });

        Ok(release)
    }
}

fn module_path() -> Result<PathBuf> {
    unsafe {
        let mut h_module = HMODULE::default();
        GetModuleHandleExA(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            windows::core::PCSTR("OreXis945".as_ptr()),
            &mut h_module,
        )
        .with_context(|| format!("GetModuleFileNameW failed with error {:#?}", GetLastError()))?;

        let mut lp_filename = [0u16; MAX_PATH as usize];
        let len = GetModuleFileNameW(Some(h_module), &mut lp_filename) as usize;
        if len == 0 {
            Err(anyhow!(
                "GetModuleFileNameW failed with error {:#?}",
                GetLastError()
            ))
        } else {
            Ok(PathBuf::from(OsString::from_wide(&lp_filename[..len])))
        }
    }
}
