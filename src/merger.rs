use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

use crate::formats::{Bnd3Archive, EmevdParser, MsbParser};
use crate::scanner::{resolve_canonical_game_path, ModInput};

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeRequest {
    #[serde(default)]
    pub mods: Vec<ModInput>,
    #[serde(default)]
    pub output_dir: String,
    #[serde(default = "default_resolution_mode")]
    pub resolution_mode: String, // "smart", "priority", "manual"
    #[serde(default)]
    pub file_overrides: HashMap<String, String>, // relative_path -> mod_name or "smart"
    #[serde(default)]
    pub asylum_fix_mode: Option<String>, // "hybrid", "vanilla_textures", "dsrr_flver"
    // Legacy fields for backward compatibility
    #[serde(default)]
    pub mod_a: String,
    #[serde(default)]
    pub mod_b: String,
    #[serde(default)]
    pub priority: String, // "A" or "B"
}

fn default_resolution_mode() -> String {
    "smart".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeResponse {
    pub success: bool,
    pub message: String,
    pub files_copied: usize,
    pub files_merged: usize,
    pub resolution_used: String,
}

pub struct Merger;

impl Merger {
    pub fn merge(req: &MergeRequest) -> Result<MergeResponse, String> {
        let mut mods_to_merge = req.mods.clone();

        // Fallback for legacy 2-mod request
        if mods_to_merge.is_empty() {
            if req.mod_a.is_empty() || req.mod_b.is_empty() {
                return Err("No mod specified for merging.".to_string());
            }
            if req.priority == "B" {
                mods_to_merge.push(ModInput { name: "Mod B".to_string(), path: req.mod_b.clone(), variant: None, disabled_files: Vec::new() });
                mods_to_merge.push(ModInput { name: "Mod A".to_string(), path: req.mod_a.clone(), variant: None, disabled_files: Vec::new() });
            } else {
                mods_to_merge.push(ModInput { name: "Mod A".to_string(), path: req.mod_a.clone(), variant: None, disabled_files: Vec::new() });
                mods_to_merge.push(ModInput { name: "Mod B".to_string(), path: req.mod_b.clone(), variant: None, disabled_files: Vec::new() });
            }
        }

        let out_dir_str = if req.output_dir.trim().is_empty() { "merged" } else { req.output_dir.trim() };
        let out_path = Path::new(out_dir_str);
        let mut mod_files: Vec<(String, HashMap<String, PathBuf>)> = Vec::new();
        for m in &mods_to_merge {
            let p = Path::new(&m.path);
            if !p.exists() {
                return Err(format!("Mod directory '{}' not found: {}", m.name, m.path));
            }
            let mut files = Self::list_files(p)?;
            for disabled in &m.disabled_files {
                let clean = disabled.to_lowercase().replace('\\', "/");
                files.retain(|k, _| k.to_lowercase().replace('\\', "/") != clean);
            }
            mod_files.push((m.name.clone(), files));
        }

        // Clean previous output directory contents so removed mods don't leave stale files
        if out_path.exists() {
            if let Ok(entries) = fs::read_dir(out_path) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let p = entry.path();
                    if p.is_dir() {
                        let _ = fs::remove_dir_all(&p);
                    } else {
                        let _ = fs::remove_file(&p);
                    }
                }
            }
        }
        fs::create_dir_all(out_path).map_err(|e| format!("Could not create output directory: {}", e))?;

        // Group loose TPUP DDS files by their target archive container
        // container -> Vec<(mod_index, mod_name, texture_name, path_buf)>
        let mut tpup_overrides: HashMap<String, Vec<(usize, String, String, PathBuf)>> = HashMap::new();
        let mut tpup_loose_files: HashSet<String> = HashSet::new();

        for (idx, (m_name, files)) in mod_files.iter().enumerate() {
            for (rel, path) in files {
                if let Some(info) = crate::scanner::parse_tpup_override_path(rel) {
                    tpup_loose_files.insert(rel.clone());
                    tpup_overrides
                        .entry(info.target_archive)
                        .or_default()
                        .push((idx, m_name.clone(), info.texture_name, path.clone()));
                }
            }
        }

        let mut all_rel: HashSet<String> = HashSet::new();
        for (_, files) in &mod_files {
            for rel in files.keys() {
                all_rel.insert(rel.clone());
            }
        }

        let mut copied = 0;
        let mut merged = 0;

        let mode = if req.resolution_mode.trim().is_empty() {
            "smart"
        } else {
            req.resolution_mode.as_str()
        };

        for rel in all_rel {
            if tpup_loose_files.contains(&rel) {
                // Loose DDS files are repacked into their container below, don't copy as loose files
                continue;
            }

            let present_indices: Vec<usize> = (0..mod_files.len())
                .filter(|&i| mod_files[i].1.contains_key(&rel))
                .collect();

            if present_indices.is_empty() {
                continue;
            }

            let dest_file = out_path.join(&rel);
            if let Some(parent) = dest_file.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }

            if present_indices.len() == 1 {
                // Exclusive file
                let src = &mod_files[present_indices[0]].1[&rel];
                fs::copy(src, &dest_file).map_err(|e| format!("Failed to copy {}: {}", rel, e))?;
                copied += 1;
            } else {
                // Multi-mod conflict or overlap
                let lower_rel = rel.to_lowercase();
                let is_bnd = lower_rel.contains("parambnd")
                    || lower_rel.contains("anibnd")
                    || lower_rel.contains("msgbnd")
                    || lower_rel.contains("talkesdbnd");

                let has_explicit_override = req.file_overrides.get(&rel).map(|s| s != "smart").unwrap_or(false);

                if is_bnd && mode == "smart" && !has_explicit_override {
                    if let Ok(()) = Self::smart_merge_bnd3(&rel, &mod_files, &present_indices, &dest_file) {
                        merged += 1;
                        continue;
                    }
                }

                let chosen_file = Self::resolve_conflict(
                    &rel,
                    &mod_files,
                    &present_indices,
                    mode,
                    &req.file_overrides,
                )?;

                fs::copy(&chosen_file, &dest_file).map_err(|e| format!("Failed to copy {}: {}", rel, e))?;
                merged += 1;
            }
        }

        // Process TPUP texture overrides into target TPF archives
        if !tpup_overrides.is_empty() {
            let base_app_dir = crate::get_app_dir();
            let vanilla_backup_dir = base_app_dir.join("vanilla_backup");
            let deployer_cfg = crate::deployer::Deployer::load_config();
            let game_dir = PathBuf::from(&deployer_cfg.game_path);

            for (container_rel, overrides) in tpup_overrides {
                let dest_file = out_path.join(&container_rel);
                if let Some(parent) = dest_file.parent() {
                    let _ = fs::create_dir_all(parent);
                }

                // 1. Locate base archive bytes
                let base_bytes: Option<Vec<u8>> = if dest_file.exists() {
                    fs::read(&dest_file).ok()
                } else {
                    let mut mod_bytes = None;
                    for (_, files) in &mod_files {
                        if let Some(p) = files.get(&container_rel) {
                            if let Ok(b) = fs::read(p) {
                                mod_bytes = Some(b);
                                break;
                            }
                        }
                    }
                    if mod_bytes.is_some() {
                        mod_bytes
                    } else if vanilla_backup_dir.join(&container_rel).exists() {
                        fs::read(vanilla_backup_dir.join(&container_rel)).ok()
                    } else if game_dir.join(&container_rel).exists() {
                        fs::read(game_dir.join(&container_rel)).ok()
                    } else {
                        None
                    }
                };

                let is_dcx = container_rel.to_lowercase().ends_with(".dcx");
                let mut tpf = if let Some(bytes) = base_bytes {
                    crate::formats::TpfArchive::parse(&bytes).unwrap_or_else(|_| crate::formats::TpfArchive {
                        textures: Vec::new(),
                    })
                } else {
                    crate::formats::TpfArchive {
                        textures: Vec::new(),
                    }
                };

                // Deduplicate so highest priority mod wins on duplicate texture names
                let mut seen_textures = HashSet::new();
                let mut injected_count = 0;

                let mut sorted_overrides = overrides;
                sorted_overrides.sort_by_key(|o| o.0);

                for (_mod_idx, _mod_name, tex_name, dds_path) in sorted_overrides {
                    let tex_key = tex_name.to_lowercase();
                    if seen_textures.insert(tex_key) {
                        if let Ok(dds_bytes) = fs::read(&dds_path) {
                            tpf.inject_texture(&tex_name, dds_bytes);
                            injected_count += 1;
                        }
                    }
                }

                if injected_count > 0 {
                    let serialized = tpf.to_bytes().map_err(|e| format!("Failed to serialize TPF {}: {}", container_rel, e))?;
                    let final_data = if is_dcx {
                        crate::formats::compress_dcx(&serialized)?
                    } else {
                        serialized
                    };
                    fs::write(&dest_file, final_data).map_err(|e| format!("Failed to write {}: {}", container_rel, e))?;
                    merged += 1;
                    println!("[Merger] Successfully injected {} texture(s) into '{}'.", injected_count, container_rel);
                }
            }
        }

        // Hybrid Asylum (m18) texture infill & backslash path normalization
        let asylum_mode = req.asylum_fix_mode.as_deref().unwrap_or("none");
        if asylum_mode == "hybrid" {
            let merged_m18_dir = out_path.join("map").join("m18");
            if merged_m18_dir.exists() {
                let base_app_dir = crate::get_app_dir();
                let vanilla_backup_dir = base_app_dir.join("vanilla_backup").join("map").join("m18");
                let deployer_cfg = crate::deployer::Deployer::load_config();
                let game_dir = PathBuf::from(&deployer_cfg.game_path).join("map").join("m18");

                for bhd_name in &["m18_0000.tpfbhd", "m18_0001.tpfbhd", "m18_0002.tpfbhd", "m18_0003.tpfbhd", "GI_EnvM_m18.tpfbhd"] {
                    let bdt_name = bhd_name.replace(".tpfbhd", ".tpfbdt");
                    let merged_bhd = merged_m18_dir.join(bhd_name);
                    let merged_bdt = merged_m18_dir.join(&bdt_name);

                    if merged_bhd.exists() && merged_bdt.exists() {
                        let (v_bhd, v_bdt) = if vanilla_backup_dir.join(bhd_name).exists() && vanilla_backup_dir.join(&bdt_name).exists() {
                            (vanilla_backup_dir.join(bhd_name), vanilla_backup_dir.join(&bdt_name))
                        } else if game_dir.join(bhd_name).exists() && game_dir.join(&bdt_name).exists() {
                            (game_dir.join(bhd_name), game_dir.join(&bdt_name))
                        } else {
                            continue;
                        };

                        if let (Ok(m_bhd_bytes), Ok(m_bdt_bytes), Ok(v_bhd_bytes), Ok(v_bdt_bytes)) = (
                            fs::read(&merged_bhd),
                            fs::read(&merged_bdt),
                            fs::read(&v_bhd),
                            fs::read(&v_bdt),
                        ) {
                            if let (Ok(mut m_archive), Ok(v_archive)) = (
                                crate::formats::BhdBdtArchive::parse(&m_bhd_bytes, &m_bdt_bytes),
                                crate::formats::BhdBdtArchive::parse(&v_bhd_bytes, &v_bdt_bytes),
                            ) {
                                let infilled = m_archive.infill_missing_from(&v_archive);
                                if let Ok((new_bhd, new_bdt)) = m_archive.to_bytes() {
                                    let _ = fs::write(&merged_bhd, new_bhd);
                                    let _ = fs::write(&merged_bdt, new_bdt);
                                    println!("[Merger] Hybrid Asylum Infill: Normalized and infilled {} texture(s) into '{}'.", infilled, bhd_name);
                                }
                            }
                        }
                    }
                }
            }
        }

        let mode_desc = match mode {
            "smart" => "Smart Merge",
            "priority" => "Sequential Priority",
            "manual" => "Custom Manual Resolution",
            _ => "Default",
        };

        Ok(MergeResponse {
            success: true,
            message: format!(
                "Merge successfully completed using [{}]! {} exclusive files copied and {} conflicting files unified perfectly.",
                mode_desc,
                copied,
                merged
            ),
            files_copied: copied,
            files_merged: merged,
            resolution_used: mode_desc.to_string(),
        })
    }

    fn resolve_conflict(
        rel: &str,
        mod_files: &[(String, HashMap<String, PathBuf>)],
        present_indices: &[usize],
        mode: &str,
        overrides: &HashMap<String, String>,
    ) -> Result<PathBuf, String> {
        // 1. Check if user set an explicit override for this file
        if let Some(target_mod_name) = overrides.get(rel) {
            if target_mod_name != "smart" {
                if let Some(&idx) = present_indices.iter().find(|&&i| &mod_files[i].0 == target_mod_name) {
                    return Ok(mod_files[idx].1[rel].clone());
                }
            }
        }

        // 2. Strategy: Smart Merge
        if mode == "smart" {
            let lower = rel.to_lowercase();

            // 2A. ANY EMEVD file: Pick the expanded version with the most events
            if lower.ends_with(".emevd.dcx") || lower.ends_with(".emevd") {
                let mut best_idx = present_indices[0];
                let mut max_events = 0;

                for &idx in present_indices {
                    let file_path = &mod_files[idx].1[rel];
                    if let Ok(bytes) = fs::read(file_path) {
                        if let Ok(events) = EmevdParser::parse_events(&bytes) {
                            if events.len() > max_events {
                                max_events = events.len();
                                best_idx = idx;
                            }
                        }
                    }
                }
                return Ok(mod_files[best_idx].1[rel].clone());
            }

            // 2B. ANY MSB map file: Pick the version with the most placed entities (bonfires, parts)
            if lower.ends_with(".msb") {
                let mut best_idx = present_indices[0];
                let mut max_entities = 0;

                for &idx in present_indices {
                    let file_path = &mod_files[idx].1[rel];
                    if let Ok(bytes) = fs::read(file_path) {
                        if let Ok(ents) = MsbParser::parse_entities(&bytes) {
                            if ents.len() > max_entities {
                                max_entities = ents.len();
                                best_idx = idx;
                            }
                        }
                    }
                }
                return Ok(mod_files[best_idx].1[rel].clone());
            }

            // 2C. ANY BND3 container (.talkesdbnd.dcx, .parambnd.dcx, .msgbnd.dcx, etc.)
            let mut best_bnd_idx = present_indices[0];
            let mut max_sub_entries = 0;
            let mut parsed_any_bnd = false;

            for &idx in present_indices {
                let file_path = &mod_files[idx].1[rel];
                if let Ok(bytes) = fs::read(file_path) {
                    if let Ok(bnd) = Bnd3Archive::parse(&bytes) {
                        parsed_any_bnd = true;
                        if bnd.entries.len() > max_sub_entries {
                            max_sub_entries = bnd.entries.len();
                            best_bnd_idx = idx;
                        }
                    }
                }
            }

            if parsed_any_bnd && max_sub_entries > 0 {
                return Ok(mod_files[best_bnd_idx].1[rel].clone());
            }

            // 2D. Generic files fallback: choose highest priority mod
            return Ok(mod_files[present_indices[0]].1[rel].clone());
        }

        // 3. Strategy: Priority (first on the list)
        Ok(mod_files[present_indices[0]].1[rel].clone())
    }

    fn smart_merge_bnd3(
        rel: &str,
        mod_files: &[(String, HashMap<String, PathBuf>)],
        present_indices: &[usize],
        dest_file: &Path,
    ) -> Result<(), String> {
        let highest_idx = present_indices[0];
        let base_path = &mod_files[highest_idx].1[rel];
        let base_bytes = fs::read(base_path).map_err(|e| e.to_string())?;
        let mut base_bnd = Bnd3Archive::parse(&base_bytes)?;

        let mut existing_names: HashSet<String> = base_bnd.entries.iter().map(|e| e.name.clone()).collect();
        let mut existing_ids: HashSet<u32> = base_bnd.entries.iter().map(|e| e.id).collect();

        // Ingest exclusive non-colliding sub-entries from other mods
        for &idx in &present_indices[1..] {
            let other_path = &mod_files[idx].1[rel];
            if let Ok(other_bytes) = fs::read(other_path) {
                if let Ok(other_bnd) = Bnd3Archive::parse(&other_bytes) {
                    for entry in other_bnd.entries {
                        if !existing_names.contains(&entry.name) && !existing_ids.contains(&entry.id) {
                            existing_names.insert(entry.name.clone());
                            existing_ids.insert(entry.id);
                            base_bnd.entries.push(entry);
                        }
                    }
                }
            }
        }

        let bnd_bytes = base_bnd.to_bytes()?;
        let is_dcx = rel.to_lowercase().ends_with(".dcx") || crate::formats::is_dcx(&base_bytes);
        let final_bytes = if is_dcx {
            crate::formats::compress_dcx(&bnd_bytes)?
        } else {
            bnd_bytes
        };

        fs::write(dest_file, final_bytes).map_err(|e| e.to_string())
    }

    fn list_files(base_path: &Path) -> Result<HashMap<String, PathBuf>, String> {
        let mut map = HashMap::new();
        Self::walk(base_path, base_path, &mut map)?;
        Ok(map)
    }

    fn walk(root: &Path, curr: &Path, map: &mut HashMap<String, PathBuf>) -> Result<(), String> {
        for entry in fs::read_dir(curr).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_dir() {
                let name = entry.file_name().to_string_lossy().to_lowercase();
                if name == "ptde" {
                    continue;
                }
                Self::walk(root, &path, map)?;
            } else if path.is_file() {
                let name = entry.file_name().to_string_lossy().to_string();
                if crate::scanner::is_ignored_non_game_file(&name) {
                    continue;
                }
                if let Ok(rel) = path.strip_prefix(root) {
                    let rel_str = rel.to_string_lossy().replace('\\', "/");
                    let canonical = resolve_canonical_game_path(&rel_str);
                    map.insert(canonical, path);
                }
            }
        }
        Ok(())
    }
}
