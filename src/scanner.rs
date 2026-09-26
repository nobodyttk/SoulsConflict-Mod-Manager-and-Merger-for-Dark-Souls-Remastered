use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

use crate::formats::{Bnd3Archive, BndEntry, EmevdParser, FmgParser, MsbParser, ParamParser};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConflictLevel {
    Safe,       // 🟢 Safe (exclusive or identical)
    Mergeable,  // 🟡 Mergeable (same container, but different sub-IDs)
    Conflict,   // 🔴 Critical conflict (same ID or non-mergeable file)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubItemReport {
    pub name: String,
    pub level: ConflictLevel,
    pub detail: String,
    pub mod_items: HashMap<String, usize>,
    pub overlapping_items: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileReport {
    pub relative_path: String,
    pub level: ConflictLevel,
    pub file_type: String,
    pub summary: String,
    pub present_in_mods: Vec<String>,
    pub details: Vec<String>,
    pub sub_items: Vec<SubItemReport>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanSummary {
    pub total_mods: usize,
    pub mod_counts: HashMap<String, usize>,
    pub total_examined: usize,
    pub safe_count: usize,
    pub mergeable_count: usize,
    pub conflict_count: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanResult {
    pub summary: ScanSummary,
    pub files: Vec<FileReport>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModInput {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub variant: Option<String>,
}

pub struct Scanner;

impl Scanner {
    #[allow(dead_code)]
    pub fn scan_mods(mod_a_dir: &str, mod_b_dir: &str) -> Result<ScanResult, String> {
        let inputs = vec![
            ModInput { name: "Mod A".to_string(), path: mod_a_dir.to_string(), variant: None },
            ModInput { name: "Mod B".to_string(), path: mod_b_dir.to_string(), variant: None },
        ];
        Self::scan_multi(&inputs)
    }

    pub fn scan_multi(mods: &[ModInput]) -> Result<ScanResult, String> {
        if mods.len() < 2 {
            return Err("Select at least 2 mod folders to analyze conflicts.".to_string());
        }

        let mut mod_files: Vec<(String, HashMap<String, PathBuf>)> = Vec::new();
        let mut mod_counts = HashMap::new();

        for m in mods {
            let path = Path::new(&m.path);
            if !path.exists() {
                return Err(format!("Mod folder '{}' not found: {}", m.name, m.path));
            }
            let files = Self::list_files_recursive(path)?;
            mod_counts.insert(m.name.clone(), files.len());
            mod_files.push((m.name.clone(), files));
        }

        let mut all_rel_paths: HashSet<String> = HashSet::new();
        for (_, files) in &mod_files {
            for rel in files.keys() {
                all_rel_paths.insert(rel.clone());
            }
        }

        let mut reports = Vec::new();
        let mut safe_count = 0;
        let mut mergeable_count = 0;
        let mut conflict_count = 0;

        for rel in all_rel_paths {
            let present_mods: Vec<&(String, HashMap<String, PathBuf>)> = mod_files
                .iter()
                .filter(|(_, files)| files.contains_key(&rel))
                .collect();

            let present_names: Vec<String> = present_mods.iter().map(|(name, _)| name.clone()).collect();
            let file_type = Self::detect_type(&rel);

            if present_mods.len() == 1 {
                safe_count += 1;
                reports.push(FileReport {
                    relative_path: rel.clone(),
                    level: ConflictLevel::Safe,
                    file_type,
                    summary: format!("Exclusive to {}", present_names[0]),
                    present_in_mods: present_names.clone(),
                    details: vec![format!("This file exists only in {}. It will be copied directly to the merged folder.", present_names[0])],
                    sub_items: vec![],
                });
            } else {
                let mut datas: Vec<Vec<u8>> = Vec::new();
                for (_, files) in &present_mods {
                    let full_path = &files[&rel];
                    let d = fs::read(full_path).map_err(|e| format!("Error reading {}: {}", full_path.display(), e))?;
                    datas.push(d);
                }

                let all_identical = datas.windows(2).all(|w| w[0] == w[1]);

                if all_identical {
                    safe_count += 1;
                    reports.push(FileReport {
                        relative_path: rel.clone(),
                        level: ConflictLevel::Safe,
                        file_type,
                        summary: format!("Identical in {} mods", present_names.len()),
                        present_in_mods: present_names.clone(),
                        details: vec![format!("Exactly identical file in: {}.", present_names.join(", "))],
                        sub_items: vec![],
                    });
                } else {
                    let lower_rel = rel.to_lowercase();

                    if lower_rel.ends_with(".emevd.dcx") || lower_rel.ends_with(".emevd") {
                        // Deep EMEVD inspection
                        let report = Self::inspect_emevd_multi(&rel, &file_type, &present_names, &datas);
                        match report.level {
                            ConflictLevel::Safe => safe_count += 1,
                            ConflictLevel::Mergeable => mergeable_count += 1,
                            ConflictLevel::Conflict => conflict_count += 1,
                        }
                        reports.push(report);
                    } else if lower_rel.ends_with(".msb") {
                        // Deep MSB inspection
                        let report = Self::inspect_msb_multi(&rel, &file_type, &present_names, &datas);
                        match report.level {
                            ConflictLevel::Safe => safe_count += 1,
                            ConflictLevel::Mergeable => mergeable_count += 1,
                            ConflictLevel::Conflict => conflict_count += 1,
                        }
                        reports.push(report);
                    } else {
                        // Try parsing all as BND3 archives (parambnd, talkesdbnd, msgbnd, etc.)
                        let mut bnds: Vec<Bnd3Archive> = Vec::new();
                        let mut all_bnd = true;
                        for d in &datas {
                            match Bnd3Archive::parse(d) {
                                Ok(bnd) => bnds.push(bnd),
                                Err(_) => {
                                    all_bnd = false;
                                    break;
                                }
                            }
                        }

                        if all_bnd {
                            let report = Self::inspect_bnd3_multi(&rel, &file_type, &present_names, &bnds);
                            match report.level {
                                ConflictLevel::Safe => safe_count += 1,
                                ConflictLevel::Mergeable => mergeable_count += 1,
                                ConflictLevel::Conflict => conflict_count += 1,
                            }
                            reports.push(report);
                        } else {
                            // Loose file collision across multiple mods
                            conflict_count += 1;
                            reports.push(FileReport {
                                relative_path: rel.clone(),
                                level: ConflictLevel::Conflict,
                                file_type,
                                summary: format!("Direct Collision ({} mods)", present_names.len()),
                                present_in_mods: present_names.clone(),
                                details: vec![
                                    format!("The mods [{}] modify this file with different content.", present_names.join(", ")),
                                    "Not an automatically mergeable container; the mod version with the highest priority will prevail.".to_string(),
                                ],
                                sub_items: vec![],
                            });
                        }
                    }
                }
            }
        }

        // Sort by severity: Conflict first, then Mergeable, then Safe
        reports.sort_by(|a, b| {
            let rank = |lvl: &ConflictLevel| match lvl {
                ConflictLevel::Conflict => 0,
                ConflictLevel::Mergeable => 1,
                ConflictLevel::Safe => 2,
            };
            rank(&a.level).cmp(&rank(&b.level)).then(a.relative_path.cmp(&b.relative_path))
        });

        let summary = ScanSummary {
            total_mods: mods.len(),
            mod_counts,
            total_examined: reports.len(),
            safe_count,
            mergeable_count,
            conflict_count,
        };

        Ok(ScanResult { summary, files: reports })
    }

    fn detect_type(path: &str) -> String {
        let p = path.to_lowercase();
        if p.contains("msgbnd") { "Texts / Menus (MSGBND)".to_string() }
        else if p.contains("parambnd") { "Game Parameters (PARAMBND)".to_string() }
        else if p.contains("emevd") { "Event Scripts (EMEVD)".to_string() }
        else if p.contains("talkesdbnd") { "NPC Dialogues / Menus (TALKESD)".to_string() }
        else if p.ends_with(".msb") { "MapStudio / Entities (MSB)".to_string() }
        else if p.ends_with(".dcx") { "DCX Container".to_string() }
        else { "Loose File".to_string() }
    }

    fn inspect_emevd_multi(
        rel: &str,
        file_type: &str,
        mod_names: &[String],
        datas: &[Vec<u8>],
    ) -> FileReport {
        let mut events_per_mod: Vec<(String, HashMap<u32, u32>)> = Vec::new();
        let mut all_eids: HashSet<u32> = HashSet::new();

        for (idx, d) in datas.iter().enumerate() {
            match EmevdParser::parse_events(d) {
                Ok(events) => {
                    let map: HashMap<u32, u32> = events.into_iter().map(|e| (e.id, e.instruction_count)).collect();
                    for &eid in map.keys() {
                        all_eids.insert(eid);
                    }
                    events_per_mod.push((mod_names[idx].clone(), map));
                }
                Err(_) => {
                    return FileReport {
                        relative_path: rel.to_string(),
                        level: ConflictLevel::Conflict,
                        file_type: file_type.to_string(),
                        summary: "Failed to decode EMEVD".to_string(),
                        present_in_mods: mod_names.to_vec(),
                        details: vec!["Corrupted EMEVD file or unsupported format.".to_string()],
                        sub_items: vec![],
                    };
                }
            }
        }

        let mut collided_ids = Vec::new();
        let mut exclusive_events = Vec::new();

        for &eid in &all_eids {
            let present: Vec<(&String, u32)> = events_per_mod
                .iter()
                .filter_map(|(name, map)| map.get(&eid).map(|&icnt| (name, icnt)))
                .collect();

            if present.len() > 1 {
                let first_icnt = present[0].1;
                let differs = present.iter().any(|p| p.1 != first_icnt);
                if differs {
                    collided_ids.push(eid);
                }
            } else if present.len() == 1 {
                exclusive_events.push((eid, present[0].0.clone()));
            }
        }

        if !collided_ids.is_empty() {
            let sample_ids = &collided_ids[..collided_ids.len().min(6)];
            FileReport {
                relative_path: rel.to_string(),
                level: ConflictLevel::Conflict,
                file_type: file_type.to_string(),
                summary: format!("Collision of {} Script Event IDs", collided_ids.len()),
                present_in_mods: mod_names.to_vec(),
                details: vec![
                    format!("The mods modify the same scripts with conflicting instructions in Event IDs: {:?}", sample_ids),
                    "To avoid anomalous behavior on the map, the version with the highest priority will prevail.".to_string(),
                ],
                sub_items: vec![],
            }
        } else {
            FileReport {
                relative_path: rel.to_string(),
                level: ConflictLevel::Mergeable,
                file_type: file_type.to_string(),
                summary: format!("100% Mergeable Events ({} new IDs)", exclusive_events.len()),
                present_in_mods: mod_names.to_vec(),
                details: vec![
                    format!("No Event ID collides! Identified {} new exclusive events distributed across the mods.", exclusive_events.len()),
                    "The scripts can be merged completely safely via event injection.".to_string(),
                ],
                sub_items: vec![],
            }
        }
    }

    fn inspect_msb_multi(
        rel: &str,
        file_type: &str,
        mod_names: &[String],
        datas: &[Vec<u8>],
    ) -> FileReport {
        let mut entities_per_mod: Vec<(String, HashMap<i32, String>)> = Vec::new();
        let mut all_entity_ids: HashSet<i32> = HashSet::new();

        for (idx, d) in datas.iter().enumerate() {
            match MsbParser::parse_entities(d) {
                Ok(ents) => {
                    let map: HashMap<i32, String> = ents.into_iter().map(|e| (e.entity_id, e.name)).collect();
                    for &eid in map.keys() {
                        all_entity_ids.insert(eid);
                    }
                    entities_per_mod.push((mod_names[idx].clone(), map));
                }
                Err(_) => {
                    return FileReport {
                        relative_path: rel.to_string(),
                        level: ConflictLevel::Conflict,
                        file_type: file_type.to_string(),
                        summary: "Failed to read MSB".to_string(),
                        present_in_mods: mod_names.to_vec(),
                        details: vec!["Could not read map entities.".to_string()],
                        sub_items: vec![],
                    };
                }
            }
        }

        let mut hard_collisions = Vec::new();
        let mut compatible_new_entities = Vec::new();

        for &eid in &all_entity_ids {
            let present: Vec<(&String, &String)> = entities_per_mod
                .iter()
                .filter_map(|(mname, map)| map.get(&eid).map(|pname| (mname, pname)))
                .collect();

            if present.len() > 1 {
                let first_name = present[0].1;
                let differs = present.iter().any(|p| p.1 != first_name);
                if differs {
                    let dispute = present.iter().map(|(m, p)| format!("{}: '{}'", m, p)).collect::<Vec<_>>().join(" vs ");
                    hard_collisions.push((eid, dispute));
                }
            } else if present.len() == 1 {
                compatible_new_entities.push((eid, present[0].0.clone(), present[0].1.clone()));
            }
        }

        if !hard_collisions.is_empty() {
            FileReport {
                relative_path: rel.to_string(),
                level: ConflictLevel::Conflict,
                file_type: file_type.to_string(),
                summary: format!("Critical Collision of {} Entity IDs in Map", hard_collisions.len()),
                present_in_mods: mod_names.to_vec(),
                details: vec![
                    format!("Dispute for Entity ID {}: {}", hard_collisions[0].0, hard_collisions[0].1),
                    "If merged without remapping, event scripts will trigger the wrong object in the game!".to_string(),
                ],
                sub_items: vec![],
            }
        } else {
            FileReport {
                relative_path: rel.to_string(),
                level: ConflictLevel::Mergeable,
                file_type: file_type.to_string(),
                summary: format!("100% Mergeable Map ({} free entities)", compatible_new_entities.len()),
                present_in_mods: mod_names.to_vec(),
                details: vec![
                    format!("All Entity IDs are compatible! Added {} entities/bonfires with free IDs.", compatible_new_entities.len()),
                    "Map parts can be unified with no cross-collision danger.".to_string(),
                ],
                sub_items: vec![],
            }
        }
    }

    fn inspect_bnd3_multi(
        rel: &str,
        file_type: &str,
        mod_names: &[String],
        bnds: &[Bnd3Archive],
    ) -> FileReport {
        let mut mod_submaps: Vec<HashMap<String, &BndEntry>> = Vec::new();
        let mut all_sub_names: HashSet<String> = HashSet::new();

        for bnd in bnds {
            let mut map = HashMap::new();
            for entry in &bnd.entries {
                all_sub_names.insert(entry.name.clone());
                map.insert(entry.name.clone(), entry);
            }
            mod_submaps.push(map);
        }

        let mut sub_items = Vec::new();
        let mut has_hard_conflict = false;
        let mut has_mergeable = false;

        for sub_name in &all_sub_names {
            let mut present_indices: Vec<usize> = Vec::new();
            for (idx, map) in mod_submaps.iter().enumerate() {
                if map.contains_key(sub_name) {
                    present_indices.push(idx);
                }
            }

            if present_indices.len() <= 1 {
                continue;
            }

            let first_idx = present_indices[0];
            let first_data = &mod_submaps[first_idx][sub_name].data;
            let sub_identical = present_indices[1..].iter().all(|&idx| {
                &mod_submaps[idx][sub_name].data == first_data
            });

            if sub_identical {
                continue;
            }

            if sub_name.ends_with(".fmg") {
                let mut fmgs: Vec<(String, HashMap<i32, String>)> = Vec::new();
                let mut mod_items_map = HashMap::new();

                for &idx in &present_indices {
                    let name = &mod_names[idx];
                    let data = &mod_submaps[idx][sub_name].data;
                    let fmg = FmgParser::parse(data);
                    mod_items_map.insert(name.clone(), fmg.len());
                    fmgs.push((name.clone(), fmg));
                }

                let mut collided_ids = HashSet::new();
                for i in 0..fmgs.len() {
                    for j in (i + 1)..fmgs.len() {
                        let (_name_a, fmg_a) = &fmgs[i];
                        let (_name_b, fmg_b) = &fmgs[j];
                        for (&id, text_a) in fmg_a {
                            if let Some(text_b) = fmg_b.get(&id) {
                                if text_a != text_b && !text_a.is_empty() && !text_b.is_empty() {
                                    collided_ids.insert(id);
                                }
                            }
                        }
                    }
                }

                if !collided_ids.is_empty() {
                    has_hard_conflict = true;
                    sub_items.push(SubItemReport {
                        name: sub_name.clone(),
                        level: ConflictLevel::Conflict,
                        detail: format!("Conflict in {} texts with same ID between mods.", collided_ids.len()),
                        mod_items: mod_items_map,
                        overlapping_items: collided_ids.len(),
                    });
                } else {
                    has_mergeable = true;
                    sub_items.push(SubItemReport {
                        name: sub_name.clone(),
                        level: ConflictLevel::Mergeable,
                        detail: "Safely mergeable (all IDs are compatible between mods).".to_string(),
                        mod_items: mod_items_map,
                        overlapping_items: 0,
                    });
                }
            } else if sub_name.ends_with(".param") {
                let mut mod_items_map = HashMap::new();
                for &idx in &present_indices {
                    let name = &mod_names[idx];
                    let data = &mod_submaps[idx][sub_name].data;
                    let rows = ParamParser::parse_row_ids(data);
                    mod_items_map.insert(name.clone(), rows.len());
                }

                has_mergeable = true;
                sub_items.push(SubItemReport {
                    name: sub_name.clone(),
                    level: ConflictLevel::Mergeable,
                    detail: "Mergeable parameters. IDs will be combined respecting mod priority.".to_string(),
                    mod_items: mod_items_map,
                    overlapping_items: 0,
                });
            } else if sub_name.ends_with(".esd") {
                // Talk script ESD entry
                has_mergeable = true;
                let mut mod_items_map = HashMap::new();
                for &idx in &present_indices {
                    mod_items_map.insert(mod_names[idx].clone(), mod_submaps[idx][sub_name].data.len());
                }
                sub_items.push(SubItemReport {
                    name: sub_name.clone(),
                    level: ConflictLevel::Mergeable,
                    detail: "Dialogue/Menu script (.esd) present in multiple mods. On merge, priority mod prevails.".to_string(),
                    mod_items: mod_items_map,
                    overlapping_items: 1,
                });
            } else {
                has_hard_conflict = true;
                let mut mod_items_map = HashMap::new();
                for &idx in &present_indices {
                    mod_items_map.insert(mod_names[idx].clone(), mod_submaps[idx][sub_name].data.len());
                }
                sub_items.push(SubItemReport {
                    name: sub_name.clone(),
                    level: ConflictLevel::Conflict,
                    detail: "Internal sub-file has non-mergeable conflicting changes.".to_string(),
                    mod_items: mod_items_map,
                    overlapping_items: 1,
                });
            }
        }

        let level = if has_hard_conflict {
            ConflictLevel::Conflict
        } else if has_mergeable {
            ConflictLevel::Mergeable
        } else {
            ConflictLevel::Safe
        };

        let summary = match level {
            ConflictLevel::Conflict => format!("Container with Internal Conflict ({} mods)", mod_names.len()),
            ConflictLevel::Mergeable => format!("100% Mergeable Container ({} mods)", mod_names.len()),
            ConflictLevel::Safe => format!("Safe Container ({} mods)", mod_names.len()),
        };

        FileReport {
            relative_path: rel.to_string(),
            level,
            file_type: file_type.to_string(),
            summary,
            present_in_mods: mod_names.to_vec(),
            details: vec![format!("Modified in {} mods: [{}].", mod_names.len(), mod_names.join(", "))],
            sub_items,
        }
    }

    fn list_files_recursive(base_path: &Path) -> Result<HashMap<String, PathBuf>, String> {
        let mut map = HashMap::new();
        Self::walk_dir(base_path, base_path, &mut map)?;
        Ok(map)
    }

    fn walk_dir(root: &Path, current: &Path, map: &mut HashMap<String, PathBuf>) -> Result<(), String> {
        if !current.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(current).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.is_dir() {
                Self::walk_dir(root, &path, map)?;
            } else if path.is_file() {
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

pub fn resolve_canonical_game_path(rel_path: &str) -> String {
    let normalized = rel_path.replace('\\', "/");
    let lower = normalized.to_lowercase();

    let standard_dirs = [
        "chr/",
        "parts/",
        "obj/",
        "event/",
        "map/mapstudio/",
        "map/",
        "param/drawparam/",
        "param/",
        "script/talk/",
        "script/",
        "msg/english/",
        "msg/portuguese/",
        "msg/spanish/",
        "msg/french/",
        "msg/german/",
        "msg/italian/",
        "msg/japanese/",
        "msg/korean/",
        "msg/polish/",
        "msg/russian/",
        "msg/tchinese/",
        "msg/",
        "ffx/",
        "menu/",
        "sound/",
        "action/",
        "sfx/",
        "font/",
        "facegen/",
    ];

    for std_dir in standard_dirs {
        if let Some(pos) = lower.find(std_dir) {
            return normalized[pos..].to_string();
        }
    }

    let file_name = Path::new(&normalized)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&normalized);
    let name_lower = file_name.to_lowercase();

    // 1. Character models & animations (cXXXX.*)
    if name_lower.starts_with('c') && name_lower.chars().nth(1).map_or(false, |c| c.is_ascii_digit()) {
        if name_lower.ends_with(".anibnd.dcx") || name_lower.ends_with(".anibnd")
            || name_lower.ends_with(".chrbnd.dcx") || name_lower.ends_with(".chrbnd")
            || name_lower.ends_with(".tpf.dcx") || name_lower.ends_with(".tpf")
        {
            return format!("chr/{}", file_name);
        }
    }

    // 2. Armor and weapon parts (wp_*, am_*, bd_*, lg_*, hd_*)
    if name_lower.starts_with("wp_") || name_lower.starts_with("am_")
        || name_lower.starts_with("bd_") || name_lower.starts_with("lg_")
        || name_lower.starts_with("hd_")
    {
        if name_lower.ends_with(".partsbnd.dcx") || name_lower.ends_with(".partsbnd")
            || name_lower.ends_with(".tpf.dcx") || name_lower.ends_with(".tpf")
        {
            return format!("parts/{}", file_name);
        }
    }

    // 3. Objects (oXXXX.*)
    if name_lower.starts_with('o') && name_lower.chars().nth(1).map_or(false, |c| c.is_ascii_digit()) {
        if name_lower.ends_with(".objbnd.dcx") || name_lower.ends_with(".objbnd")
            || name_lower.ends_with(".tpf.dcx") || name_lower.ends_with(".tpf")
        {
            return format!("obj/{}", file_name);
        }
    }

    // 4. Events
    if name_lower.ends_with(".emevd.dcx") || name_lower.ends_with(".emevd") {
        return format!("event/{}", file_name);
    }

    // 5. Maps (MSB)
    if name_lower.ends_with(".msb") {
        return format!("map/MapStudio/{}", file_name);
    }

    // 6. Dialogue scripts (Talk ESD)
    if name_lower.ends_with(".talkesdbnd.dcx") || name_lower.ends_with(".talkesdbnd") || name_lower.ends_with(".esd") {
        return format!("script/talk/{}", file_name);
    }

    // 7. Parameters
    if name_lower.ends_with(".parambnd.dcx") || name_lower.ends_with(".parambnd") || name_lower.ends_with(".param") {
        return format!("param/{}", file_name);
    }

    // 8. Texts & localization (FMG)
    if name_lower.ends_with(".msgbnd.dcx") || name_lower.ends_with(".msgbnd") || name_lower.ends_with(".fmg") {
        return format!("msg/ENGLISH/{}", file_name);
    }

    // 9. Special effects (FFX)
    if name_lower.ends_with(".ffxbnd.dcx") || name_lower.ends_with(".ffxbnd") || name_lower.ends_with(".ffx") {
        return format!("ffx/{}", file_name);
    }

    // 10. Menus & UI
    if name_lower.starts_with("menu") || name_lower.ends_with(".gfx") || name_lower.ends_with(".swf") {
        return format!("menu/{}", file_name);
    }

    // 11. Sounds
    if name_lower.ends_with(".fsb") || name_lower.ends_with(".fev") || name_lower.ends_with(".fdp") {
        return format!("sound/{}", file_name);
    }

    normalized
}
