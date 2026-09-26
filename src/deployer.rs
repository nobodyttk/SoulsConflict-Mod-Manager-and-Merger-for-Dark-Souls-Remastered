use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeployedModInfo {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub variant: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeployedFileRecord {
    pub relative_path: String,
    pub had_vanilla: bool,
    pub backup_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeploymentManifest {
    pub is_deployed: bool,
    pub deployed_at: String,
    #[serde(default)]
    pub active_mods: Vec<DeployedModInfo>,
    pub files: Vec<DeployedFileRecord>,
}

impl Default for DeploymentManifest {
    fn default() -> Self {
        Self {
            is_deployed: false,
            deployed_at: String::new(),
            active_mods: Vec::new(),
            files: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployerConfig {
    pub game_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployStatus {
    pub game_path: String,
    pub game_found: bool,
    pub exe_path: String,
    pub is_modded: bool,
    pub deployed_files_count: usize,
    pub merged_files_count: usize,
    pub is_game_running: bool,
    pub deployed_at: String,
    pub active_mods: Vec<DeployedModInfo>,
    pub deployed_files: Vec<DeployedFileRecord>,
    pub is_steam_detected: bool,
    pub detected_launch_mode: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LaunchResult {
    pub success: bool,
    pub launch_mode: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeployResult {
    pub success: bool,
    pub placed_count: usize,
    pub backup_count: usize,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestoreResult {
    pub success: bool,
    pub removed_count: usize,
    pub restored_count: usize,
    pub already_vanilla: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeployError {
    pub error_code: String,
    pub message: String,
    #[serde(default)]
    pub detail: Option<String>,
}

pub struct Deployer;

impl Deployer {
    pub fn get_default_game_path() -> PathBuf {
        let candidates = [
            r"C:\Program Files (x86)\Steam\steamapps\common\DARK SOULS REMASTERED",
            r"D:\SteamLibrary\steamapps\common\DARK SOULS REMASTERED",
            r"E:\SteamLibrary\steamapps\common\DARK SOULS REMASTERED",
            r"F:\SteamLibrary\steamapps\common\DARK SOULS REMASTERED",
            r"C:\Program Files\Steam\steamapps\common\DARK SOULS REMASTERED",
            r"D:\Steam\steamapps\common\DARK SOULS REMASTERED",
            r"E:\Steam\steamapps\common\DARK SOULS REMASTERED",
        ];

        for cand in candidates {
            let p = PathBuf::from(cand);
            if p.exists() && p.join("DarkSoulsRemastered.exe").exists() {
                return p;
            }
        }

        PathBuf::from(candidates[0])
    }

    pub fn load_config() -> DeployerConfig {
        let base_dir = crate::get_app_dir();
        let cfg_path = base_dir.join("deployer_config.json");
        if cfg_path.exists() {
            if let Ok(content) = fs::read_to_string(&cfg_path) {
                if let Ok(cfg) = serde_json::from_str::<DeployerConfig>(&content) {
                    if !cfg.game_path.trim().is_empty() {
                        return cfg;
                    }
                }
            }
        }
        DeployerConfig {
            game_path: Self::get_default_game_path().to_string_lossy().to_string(),
        }
    }

    pub fn save_config(cfg: &DeployerConfig) -> Result<(), String> {
        let base_dir = crate::get_app_dir();
        let cfg_path = base_dir.join("deployer_config.json");
        let json = serde_json::to_string_pretty(cfg).map_err(|e| e.to_string())?;
        fs::write(&cfg_path, json).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn load_manifest() -> DeploymentManifest {
        let base_dir = crate::get_app_dir();
        let man_path = base_dir.join("deployer_manifest.json");
        if man_path.exists() {
            if let Ok(content) = fs::read_to_string(&man_path) {
                if let Ok(man) = serde_json::from_str::<DeploymentManifest>(&content) {
                    return man;
                }
            }
        }
        DeploymentManifest::default()
    }

    pub fn save_manifest(manifest: &DeploymentManifest) -> Result<(), String> {
        let base_dir = crate::get_app_dir();
        let man_path = base_dir.join("deployer_manifest.json");
        let json = serde_json::to_string_pretty(manifest).map_err(|e| e.to_string())?;
        fs::write(&man_path, json).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn is_process_running(process_name: &str) -> bool {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            let output = Command::new("tasklist")
                .args(["/FI", &format!("IMAGENAME eq {}", process_name)])
                .creation_flags(CREATE_NO_WINDOW)
                .output();
            if let Ok(out) = output {
                let text = String::from_utf8_lossy(&out.stdout).to_lowercase();
                return text.contains(&process_name.to_lowercase());
            }
        }
        false
    }

    pub fn count_merged_files() -> usize {
        let base_dir = crate::get_app_dir();
        let merged_dir = base_dir.join("merged");
        if !merged_dir.exists() {
            return 0;
        }
        let mut count = 0;
        let _ = Self::count_files_walk(&merged_dir, &mut count);
        count
    }

    fn count_files_walk(dir: &Path, count: &mut usize) -> std::io::Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_dir() {
                Self::count_files_walk(&p, count)?;
            } else if p.is_file() {
                *count += 1;
            }
        }
        Ok(())
    }

    pub fn get_status() -> DeployStatus {
        let cfg = Self::load_config();
        let game_path_buf = PathBuf::from(&cfg.game_path);
        let exe = game_path_buf.join("DarkSoulsRemastered.exe");
        let game_found = exe.exists();
        let manifest = Self::load_manifest();
        let is_running = Self::is_process_running("DarkSoulsRemastered.exe");
        let merged_count = Self::count_merged_files();

        let is_alt = Self::is_alternative_version(&game_path_buf);
        let is_steam = game_found && !is_alt;
        let detected_mode = if is_steam { "steam" } else { "direct" };

        DeployStatus {
            game_path: cfg.game_path,
            game_found,
            exe_path: exe.to_string_lossy().to_string(),
            is_modded: manifest.is_deployed,
            deployed_files_count: manifest.files.len(),
            merged_files_count: merged_count,
            is_game_running: is_running,
            deployed_at: manifest.deployed_at,
            active_mods: manifest.active_mods,
            deployed_files: manifest.files,
            is_steam_detected: is_steam,
            detected_launch_mode: detected_mode.to_string(),
        }
    }

    pub fn is_alternative_version(game_dir: &Path) -> bool {
        let emulator_indicators = [
            "steam_emu.ini",
            "hlm.ini",
            "valve.ini",
            "ColdClientLoader.ini",
            "steam_api64.cdx",
            "steam_api64.ini",
            "DS1R.ini",
            "CreamAPI.ini",
            "SmartSteamEmu.ini",
            "steam_interfaces.txt",
        ];

        for indicator in emulator_indicators {
            if game_dir.join(indicator).exists() {
                return true;
            }
        }

        if game_dir.join("steam_settings").is_dir() {
            return true;
        }

        let path_lower = game_dir.to_string_lossy().to_lowercase();
        if !path_lower.contains("steamapps") {
            return true;
        }

        false
    }

    pub fn deploy_mods(active_mods: Vec<DeployedModInfo>) -> Result<DeployResult, DeployError> {
        let cfg = Self::load_config();
        let game_dir = PathBuf::from(&cfg.game_path);
        let exe = game_dir.join("DarkSoulsRemastered.exe");
        if !exe.exists() {
            return Err(DeployError {
                error_code: "ERR_EXE_NOT_FOUND".into(),
                message: format!("DarkSoulsRemastered.exe not found at: {}", cfg.game_path),
                detail: Some(cfg.game_path),
            });
        }

        if Self::is_process_running("DarkSoulsRemastered.exe") {
            return Err(DeployError {
                error_code: "ERR_GAME_RUNNING".into(),
                message: "Dark Souls Remastered is currently running! Please close the game before deploying mods.".into(),
                detail: None,
            });
        }

        let base_dir = crate::get_app_dir();
        let merged_dir = base_dir.join("merged");
        if !merged_dir.exists() {
            return Err(DeployError {
                error_code: "ERR_MERGED_NOT_FOUND".into(),
                message: "The 'merged/' folder does not exist. Merge your mods in Diagnostic tab first!".into(),
                detail: None,
            });
        }

        let backup_dir = base_dir.join("vanilla_backup");
        if let Err(e) = fs::create_dir_all(&backup_dir) {
            return Err(DeployError {
                error_code: "ERR_IO".into(),
                message: format!("Failed to create backup dir: {}", e),
                detail: None,
            });
        }

        let mut merged_files = Vec::new();
        if let Err(e) = Self::collect_merged_files(&merged_dir, &merged_dir, &mut merged_files) {
            return Err(DeployError {
                error_code: "ERR_IO".into(),
                message: format!("Failed to collect merged files: {}", e),
                detail: None,
            });
        }

        if merged_files.is_empty() {
            return Err(DeployError {
                error_code: "ERR_MERGED_EMPTY".into(),
                message: "The 'merged/' folder is empty! Merge your mods in Diagnostic tab first.".into(),
                detail: None,
            });
        }

        let mut manifest = Self::load_manifest();
        if manifest.is_deployed {
            let _ = Self::restore_vanilla();
            manifest = DeploymentManifest::default();
        }

        let mut deployed_records = Vec::new();
        let mut backup_count = 0;
        let mut placed_count = 0;

        for (src_path, rel_str) in merged_files {
            let target_path = game_dir.join(&rel_str);
            let mut had_vanilla = false;
            let mut backup_saved_path = None;

            if target_path.exists() {
                had_vanilla = true;
                let bkp_path = backup_dir.join(&rel_str);
                if let Some(parent) = bkp_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                if fs::rename(&target_path, &bkp_path).is_err() {
                    if let Err(e) = fs::copy(&target_path, &bkp_path) {
                        return Err(DeployError {
                            error_code: "ERR_BACKUP_FAILED".into(),
                            message: format!("Failed to backup {}: {}", rel_str, e),
                            detail: Some(rel_str),
                        });
                    }
                    let _ = fs::remove_file(&target_path);
                }
                backup_saved_path = Some(bkp_path.to_string_lossy().to_string());
                backup_count += 1;
            }

            if let Some(parent) = target_path.parent() {
                let _ = fs::create_dir_all(parent);
            }

            let placed = {
                #[cfg(target_os = "windows")]
                {
                    if fs::hard_link(&src_path, &target_path).is_ok() {
                        true
                    } else if std::os::windows::fs::symlink_file(&src_path, &target_path).is_ok() {
                        true
                    } else {
                        fs::copy(&src_path, &target_path).is_ok()
                    }
                }
                #[cfg(not(target_os = "windows"))]
                {
                    fs::copy(&src_path, &target_path).is_ok()
                }
            };

            if !placed {
                return Err(DeployError {
                    error_code: "ERR_DEPLOY_FILE".into(),
                    message: format!("Failed to apply mod file: {}", rel_str),
                    detail: Some(rel_str),
                });
            }

            placed_count += 1;
            deployed_records.push(DeployedFileRecord {
                relative_path: rel_str,
                had_vanilla,
                backup_path: backup_saved_path,
            });
        }

        manifest.is_deployed = true;
        manifest.deployed_at = chrono_or_timestamp();
        manifest.active_mods = active_mods;
        manifest.files = deployed_records;
        if let Err(e) = Self::save_manifest(&manifest) {
            return Err(DeployError {
                error_code: "ERR_IO".into(),
                message: format!("Failed to save manifest: {}", e),
                detail: None,
            });
        }

        Ok(DeployResult {
            success: true,
            placed_count,
            backup_count,
            message: format!("{} files deployed, {} backed up.", placed_count, backup_count),
        })
    }

    pub fn restore_vanilla() -> Result<RestoreResult, DeployError> {
        let manifest = Self::load_manifest();
        if !manifest.is_deployed && manifest.files.is_empty() {
            return Ok(RestoreResult {
                success: true,
                removed_count: 0,
                restored_count: 0,
                already_vanilla: true,
                message: "Game is already in Vanilla state.".into(),
            });
        }

        if Self::is_process_running("DarkSoulsRemastered.exe") {
            return Err(DeployError {
                error_code: "ERR_GAME_RUNNING".into(),
                message: "Dark Souls Remastered is currently running! Please close the game before restoring Vanilla.".into(),
                detail: None,
            });
        }

        let cfg = Self::load_config();
        let game_dir = PathBuf::from(&cfg.game_path);
        let base_dir = crate::get_app_dir();
        let backup_dir = base_dir.join("vanilla_backup");

        let mut restored_count = 0;
        let mut removed_count = 0;

        for record in &manifest.files {
            let target_path = game_dir.join(&record.relative_path);
            if target_path.exists() {
                let _ = fs::remove_file(&target_path);
                removed_count += 1;
            }

            if record.had_vanilla {
                let bkp_path = backup_dir.join(&record.relative_path);
                if bkp_path.exists() {
                    if let Some(parent) = target_path.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    if fs::rename(&bkp_path, &target_path).is_err() {
                        let _ = fs::copy(&bkp_path, &target_path);
                        let _ = fs::remove_file(&bkp_path);
                    }
                    restored_count += 1;
                }
            }
        }

        let new_manifest = DeploymentManifest::default();
        if let Err(e) = Self::save_manifest(&new_manifest) {
            return Err(DeployError {
                error_code: "ERR_IO".into(),
                message: format!("Failed to save manifest: {}", e),
                detail: None,
            });
        }

        Ok(RestoreResult {
            success: true,
            removed_count,
            restored_count,
            already_vanilla: false,
            message: format!("{} mods removed, {} original files restored.", removed_count, restored_count),
        })
    }

    pub fn launch_game(preferred_mode: Option<&str>) -> Result<LaunchResult, String> {
        let cfg = Self::load_config();
        let game_dir = PathBuf::from(&cfg.game_path);
        let exe = game_dir.join("DarkSoulsRemastered.exe");
        if !exe.exists() {
            return Err(format!("DarkSoulsRemastered.exe not found at: {}", cfg.game_path));
        }

        let is_alt = Self::is_alternative_version(&game_dir);
        let mode = match preferred_mode.unwrap_or("auto") {
            "steam" => "steam",
            "direct" => "direct",
            "seamless" => "seamless",
            _ => {
                if is_alt {
                    "direct"
                } else {
                    "steam"
                }
            }
        };

        if mode == "seamless" {
            let sc_exe = game_dir.join("ds1sc_launcher.exe");
            if !sc_exe.exists() {
                return Err(format!("ds1sc_launcher.exe not found at: {}", cfg.game_path));
            }
            #[cfg(target_os = "windows")]
            {
                Command::new(&sc_exe)
                    .current_dir(&game_dir)
                    .spawn()
                    .map_err(|e| format!("Could not start Seamless Co-op launcher: {}", e))?;
            }
            return Ok(LaunchResult {
                success: true,
                launch_mode: "seamless".to_string(),
                message: "Dark Souls launched via Seamless Co-op launcher (ds1sc_launcher.exe).".to_string(),
            });
        }

        if mode == "steam" {
            #[cfg(target_os = "windows")]
            {
                use std::os::windows::process::CommandExt;
                const CREATE_NO_WINDOW: u32 = 0x08000000;
                let res = Command::new("cmd")
                    .args(["/C", "start", "", "steam://rungameid/570940"])
                    .creation_flags(CREATE_NO_WINDOW)
                    .spawn();

                if res.is_ok() {
                    return Ok(LaunchResult {
                        success: true,
                        launch_mode: "steam".to_string(),
                        message: "Dark Souls Remastered launched via Steam (AppID 570940).".to_string(),
                    });
                }
            }
        }

        // Direct executable launch (for alternative/cracked versions or fallback)
        #[cfg(target_os = "windows")]
        {
            Command::new(&exe)
                .current_dir(&game_dir)
                .spawn()
                .map_err(|e| format!("Could not start game executable: {}", e))?;
        }

        Ok(LaunchResult {
            success: true,
            launch_mode: "direct".to_string(),
            message: "Dark Souls Remastered launched directly via executable.".to_string(),
        })
    }

    fn collect_merged_files(root: &Path, current: &Path, list: &mut Vec<(PathBuf, String)>) -> Result<(), String> {
        if !current.is_dir() {
            return Ok(());
        }
        for entry in fs::read_dir(current).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_dir() {
                Self::collect_merged_files(root, &path, list)?;
            } else if path.is_file() {
                if let Ok(rel) = path.strip_prefix(root) {
                    let rel_str = rel.to_string_lossy().replace('\\', "/");
                    list.push((path, rel_str));
                }
            }
        }
        Ok(())
    }
}

fn chrono_or_timestamp() -> String {
    let now = std::time::SystemTime::now();
    let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let secs = duration.as_secs();
    format!("{}:{:02}:{:02} UTC", (secs / 3600) % 24, (secs / 60) % 60, secs % 60)
}
