use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

use crate::formats::{Bnd3Archive, BndEntry, EmevdParser, FmgParser, MsbParser, ParamParser};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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
    #[serde(default)]
    pub active_val: Option<String>,
    #[serde(default)]
    pub inactive_val: Option<String>,
    #[serde(default)]
    pub mod_values: HashMap<String, String>,
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
    #[serde(default)]
    pub winner_mod: Option<String>,
    #[serde(default)]
    pub overwritten_mods: Vec<String>,
    #[serde(default)]
    pub disabled_in_mods: Vec<String>,
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
    #[serde(default)]
    pub disabled_files: Vec<String>,
}

pub struct Scanner;

impl Scanner {
    #[allow(dead_code)]
    pub fn scan_mods(mod_a_dir: &str, mod_b_dir: &str) -> Result<ScanResult, String> {
        let inputs = vec![
            ModInput { name: "Mod A".to_string(), path: mod_a_dir.to_string(), variant: None, disabled_files: Vec::new() },
            ModInput { name: "Mod B".to_string(), path: mod_b_dir.to_string(), variant: None, disabled_files: Vec::new() },
        ];
        Self::scan_multi(&inputs)
    }

    pub fn scan_multi(mods: &[ModInput]) -> Result<ScanResult, String> {
        if mods.len() < 2 {
            return Err("Select at least 2 mod folders to analyze conflicts.".to_string());
        }

        let mut mod_files: Vec<(String, HashMap<String, PathBuf>)> = Vec::new();
        let mut mod_counts = HashMap::new();
        let mut disabled_map: HashMap<String, HashSet<String>> = HashMap::new();

        for m in mods {
            let mut d_set = HashSet::new();
            for d in &m.disabled_files {
                d_set.insert(d.to_lowercase().replace('\\', "/"));
            }
            disabled_map.insert(m.name.clone(), d_set);
        }

        for m in mods {
            let path = Path::new(&m.path);
            if !path.exists() {
                return Err(format!("Mod folder '{}' not found: {}", m.name, m.path));
            }
            let files = Self::list_files_recursive(path)?;
            mod_counts.insert(m.name.clone(), files.len());
            mod_files.push((m.name.clone(), files));
        }

        // Group loose TPUP DDS files by their target archive container
        // container -> Vec<(mod_name, texture_name, rel_path, full_path)>
        let mut tpup_container_map: HashMap<String, Vec<(String, String, String, PathBuf)>> = HashMap::new();
        let mut tpup_loose_files: HashSet<String> = HashSet::new();

        for (m_name, files) in &mod_files {
            for (rel, path) in files {
                if let Some(info) = parse_tpup_override_path(rel) {
                    tpup_loose_files.insert(rel.clone());
                    tpup_container_map
                        .entry(info.target_archive)
                        .or_default()
                        .push((m_name.clone(), info.texture_name, rel.clone(), path.clone()));
                }
            }
        }

        let mut all_rel_paths: HashSet<String> = HashSet::new();
        for (_, files) in &mod_files {
            for rel in files.keys() {
                if !tpup_loose_files.contains(rel) {
                    all_rel_paths.insert(rel.clone());
                }
            }
        }
        for target_container in tpup_container_map.keys() {
            all_rel_paths.insert(target_container.clone());
        }

        let mut reports = Vec::new();
        let mut safe_count = 0;
        let mut mergeable_count = 0;
        let mut conflict_count = 0;

        for rel in all_rel_paths {
            // Check if this container is a TPUP texture target
            if let Some(entries) = tpup_container_map.get(&rel) {
                let mut present_names: Vec<String> = Vec::new();
                for (m_name, _) in &mod_files {
                    let has_loose = entries.iter().any(|e| &e.0 == m_name);
                    let has_container = mod_files.iter().find(|(name, _)| name == m_name).map(|(_, f)| f.contains_key(&rel)).unwrap_or(false);
                    if (has_loose || has_container) && !present_names.contains(m_name) {
                        present_names.push(m_name.clone());
                    }
                }

                let rel_lower = rel.to_lowercase().replace('\\', "/");
                let mut disabled_in: Vec<String> = Vec::new();
                let mut active_in: Vec<String> = Vec::new();

                for m in &present_names {
                    let d_set = disabled_map.get(m);
                    let is_container_disabled = d_set.map(|s| s.contains(&rel_lower)).unwrap_or(false);
                    if is_container_disabled {
                        disabled_in.push(m.clone());
                    } else {
                        active_in.push(m.clone());
                    }
                }

                // Group active textures
                let mut tex_mods: HashMap<String, Vec<String>> = HashMap::new();
                let mut tex_names_order: Vec<String> = Vec::new();

                for (m_name, tex_name, orig_rel, _) in entries {
                    if !active_in.contains(m_name) {
                        continue;
                    }
                    let orig_rel_lower = orig_rel.to_lowercase().replace('\\', "/");
                    let tex_lower = tex_name.to_lowercase();
                    if let Some(d_set) = disabled_map.get(m_name) {
                        if d_set.contains(&orig_rel_lower) || d_set.contains(&tex_lower) || d_set.contains(&format!("{}.dds", tex_lower)) {
                            continue;
                        }
                    }
                    if !tex_names_order.contains(tex_name) {
                        tex_names_order.push(tex_name.clone());
                    }
                    let list = tex_mods.entry(tex_name.clone()).or_default();
                    if !list.contains(m_name) {
                        list.push(m_name.clone());
                    }
                }

                let mut sub_items = Vec::new();
                let mut has_sub_conflict = false;
                let mut collision_count = 0;

                for tex in &tex_names_order {
                    let mods_touching = &tex_mods[tex];
                    let (sub_level, detail) = if mods_touching.len() > 1 {
                        has_sub_conflict = true;
                        collision_count += 1;
                        (
                            ConflictLevel::Conflict,
                            format!("Texture collision: '{}' modified in [{}]", tex, mods_touching.join(", ")),
                        )
                    } else {
                        (
                            ConflictLevel::Safe,
                            format!("Overridden from {}", mods_touching[0]),
                        )
                    };

                    let mut mod_items = HashMap::new();
                    for m in mods_touching {
                        mod_items.insert(m.clone(), 1);
                    }

                    sub_items.push(SubItemReport {
                        name: format!("{}.dds", tex),
                        level: sub_level,
                        detail,
                        active_val: mods_touching.first().cloned(),
                        inactive_val: mods_touching.get(1).cloned(),
                        mod_values: HashMap::new(),
                        mod_items,
                        overlapping_items: if mods_touching.len() > 1 { mods_touching.len() } else { 0 },
                    });
                }

                let effective_winner = active_in.first().cloned();
                let effective_overwritten: Vec<String> = if let Some(ref w) = effective_winner {
                    present_names.iter().filter(|m| *m != w).cloned().collect()
                } else {
                    present_names.iter().skip(1).cloned().collect()
                };

                let (level, summary, details) = if active_in.is_empty() {
                    safe_count += 1;
                    (
                        ConflictLevel::Safe,
                        "Disabled in all mods".to_string(),
                        vec!["This texture archive is disabled in all mods.".to_string()],
                    )
                } else if has_sub_conflict {
                    conflict_count += 1;
                    (
                        ConflictLevel::Conflict,
                        format!("Texture Collision ({} textures colliding)", collision_count),
                        vec![
                            format!("Duplicate textures modified across active mods [{}]. Highest priority will prevail.", active_in.join(", ")),
                        ],
                    )
                } else if active_in.len() > 1 {
                    mergeable_count += 1;
                    (
                        ConflictLevel::Mergeable,
                        format!("Smart Texture Merge ({} distinct DDS textures from {} mods)", sub_items.len(), active_in.len()),
                        vec![
                            format!("All {} distinct textures will be injected into {}.", sub_items.len(), rel),
                            format!("Mods contributing textures: [{}].", active_in.join(", ")),
                        ],
                    )
                } else {
                    safe_count += 1;
                    (
                        ConflictLevel::Safe,
                        format!("Texture Injection from {} ({} texture(s))", active_in[0], sub_items.len()),
                        vec![
                            format!("{} loose textures will be injected into {}.", sub_items.len(), rel),
                        ],
                    )
                };

                reports.push(FileReport {
                    relative_path: rel.clone(),
                    level,
                    file_type: "Textures / TPF Archive".to_string(),
                    summary,
                    present_in_mods: present_names,
                    details,
                    sub_items,
                    winner_mod: effective_winner,
                    overwritten_mods: effective_overwritten,
                    disabled_in_mods: disabled_in,
                });
                continue;
            }

            let present_mods: Vec<&(String, HashMap<String, PathBuf>)> = mod_files
                .iter()
                .filter(|(_, files)| files.contains_key(&rel))
                .collect();

            let present_names: Vec<String> = present_mods.iter().map(|(name, _)| name.clone()).collect();
            let rel_lower = rel.to_lowercase().replace('\\', "/");
            let disabled_in: Vec<String> = present_names
                .iter()
                .filter(|m| disabled_map.get(*m).map(|s| s.contains(&rel_lower)).unwrap_or(false))
                .cloned()
                .collect();
            let active_in: Vec<String> = present_names
                .iter()
                .filter(|m| !disabled_map.get(*m).map(|s| s.contains(&rel_lower)).unwrap_or(false))
                .cloned()
                .collect();

            let effective_winner = if !active_in.is_empty() {
                active_in.first().cloned()
            } else {
                present_names.first().cloned()
            };

            let effective_overwritten: Vec<String> = if let Some(ref w) = effective_winner {
                present_names.iter().filter(|m| *m != w).cloned().collect()
            } else {
                present_names.iter().skip(1).cloned().collect()
            };

            let file_type = Self::detect_type(&rel);

            if active_in.len() <= 1 {
                safe_count += 1;
                let (summary, details) = if active_in.is_empty() {
                    (
                        "Disabled in all mods".to_string(),
                        vec!["This file is disabled in all mods and will not be merged.".to_string()],
                    )
                } else if !disabled_in.is_empty() {
                    (
                        format!("Resolved: Used from {}", active_in[0]),
                        vec![format!(
                            "Conflict resolved via toggle. Active in {} (disabled in {}).",
                            active_in[0],
                            disabled_in.join(", ")
                        )],
                    )
                } else {
                    (
                        format!("Exclusive to {}", present_names[0]),
                        vec![format!(
                            "This file exists only in {}. It will be copied directly to the merged folder.",
                            present_names[0]
                        )],
                    )
                };

                reports.push(FileReport {
                    relative_path: rel.clone(),
                    level: ConflictLevel::Safe,
                    file_type,
                    summary,
                    present_in_mods: present_names.clone(),
                    details,
                    sub_items: vec![],
                    winner_mod: effective_winner,
                    overwritten_mods: effective_overwritten,
                    disabled_in_mods: disabled_in,
                });
                continue;
            } else {
                let active_mods: Vec<&(String, HashMap<String, PathBuf>)> = present_mods
                    .into_iter()
                    .filter(|(name, _)| active_in.contains(name))
                    .collect();

                // Check file sizes first to avoid reading gigabytes of data into RAM
                let mut sizes = Vec::new();
                for (_, files) in &active_mods {
                    let full_path = &files[&rel];
                    let sz = fs::metadata(full_path).map(|m| m.len()).unwrap_or(0);
                    sizes.push(sz);
                }

                let same_size = sizes.windows(2).all(|w| w[0] == w[1]);
                let lower_rel = rel.to_lowercase();
                let is_mergeable = lower_rel.ends_with(".emevd.dcx")
                    || lower_rel.ends_with(".emevd")
                    || lower_rel.ends_with(".msb")
                    || lower_rel.contains("parambnd")
                    || lower_rel.contains("msgbnd")
                    || lower_rel.contains("talkesdbnd")
                    || lower_rel.contains("anibnd")
                    || lower_rel.ends_with(".fmg")
                    || lower_rel.ends_with(".param")
                    || lower_rel.ends_with(".esd");

                // Fast path: if sizes differ and it's not a mergeable container, it's immediately a direct collision
                if !same_size && !is_mergeable {
                    conflict_count += 1;
                    reports.push(FileReport {
                        relative_path: rel.clone(),
                        level: ConflictLevel::Conflict,
                        file_type,
                        summary: format!("Direct Collision ({} mods)", active_in.len()),
                        present_in_mods: present_names.clone(),
                        details: vec![
                            format!("The active mods [{}] modify this file with different content.", active_in.join(", ")),
                            "Direct asset collision; the mod version with the highest priority will prevail.".to_string(),
                        ],
                        sub_items: vec![],
                        winner_mod: effective_winner,
                        overwritten_mods: effective_overwritten,
                        disabled_in_mods: disabled_in,
                    });
                    continue;
                }

                // If it's a huge asset file (> 50 MB) and not a mergeable container, don't read into RAM
                if !is_mergeable && sizes.iter().any(|&s| s > 50_000_000) {
                    conflict_count += 1;
                    reports.push(FileReport {
                        relative_path: rel.clone(),
                        level: ConflictLevel::Conflict,
                        file_type,
                        summary: format!("Large Asset Collision ({} mods)", active_in.len()),
                        present_in_mods: present_names.clone(),
                        details: vec![
                            format!("The active mods [{}] both provide this large asset archive.", active_in.join(", ")),
                            "Non-mergeable large asset; the mod version with highest priority will prevail.".to_string(),
                        ],
                        sub_items: vec![],
                        winner_mod: effective_winner,
                        overwritten_mods: effective_overwritten,
                        disabled_in_mods: disabled_in,
                    });
                    continue;
                }

                let mut datas: Vec<Vec<u8>> = Vec::new();
                for (_, files) in &active_mods {
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
                        winner_mod: effective_winner,
                        overwritten_mods: effective_overwritten,
                        disabled_in_mods: disabled_in,
                    });
                } else if lower_rel.ends_with(".emevd.dcx") || lower_rel.ends_with(".emevd") {
                    // Deep EMEVD inspection
                    let mut report = Self::inspect_emevd_multi(&rel, &file_type, &present_names, &datas);
                    report.winner_mod = effective_winner;
                    report.overwritten_mods = effective_overwritten;
                    report.disabled_in_mods = disabled_in;
                    match report.level {
                        ConflictLevel::Safe => safe_count += 1,
                        ConflictLevel::Mergeable => mergeable_count += 1,
                        ConflictLevel::Conflict => conflict_count += 1,
                    }
                    reports.push(report);
                } else if lower_rel.ends_with(".msb") {
                    // Deep MSB inspection
                    let mut report = Self::inspect_msb_multi(&rel, &file_type, &present_names, &datas);
                    report.winner_mod = effective_winner;
                    report.overwritten_mods = effective_overwritten;
                    report.disabled_in_mods = disabled_in;
                    match report.level {
                        ConflictLevel::Safe => safe_count += 1,
                        ConflictLevel::Mergeable => mergeable_count += 1,
                        ConflictLevel::Conflict => conflict_count += 1,
                    }
                    reports.push(report);
                } else if is_mergeable {
                    // Try parsing mergeable BND3 archives (parambnd, talkesdbnd, msgbnd, anibnd, etc.)
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
                        let report = Self::inspect_bnd3_multi(
                            &rel,
                            &file_type,
                            &present_names,
                            &bnds,
                            effective_winner.as_deref(),
                            &effective_overwritten,
                            &disabled_in,
                        );
                        match report.level {
                            ConflictLevel::Safe => safe_count += 1,
                            ConflictLevel::Mergeable => mergeable_count += 1,
                            ConflictLevel::Conflict => conflict_count += 1,
                        }
                        reports.push(report);
                    } else {
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
                            winner_mod: effective_winner,
                            overwritten_mods: effective_overwritten,
                            disabled_in_mods: disabled_in,
                        });
                    }
                } else {
                    // Loose asset collision across multiple mods
                    conflict_count += 1;
                    reports.push(FileReport {
                        relative_path: rel.clone(),
                        level: ConflictLevel::Conflict,
                        file_type,
                        summary: format!("Direct Collision ({} mods)", present_names.len()),
                        present_in_mods: present_names.clone(),
                        details: vec![
                            format!("The mods [{}] modify this file with different content.", present_names.join(", ")),
                            "Direct asset collision; the mod version with highest priority will prevail.".to_string(),
                        ],
                        sub_items: vec![],
                        winner_mod: effective_winner,
                        overwritten_mods: effective_overwritten,
                        disabled_in_mods: disabled_in,
                    });
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
        if p.contains(".tpf") || p.ends_with(".tpf.dcx") || p.ends_with(".tpf") { "Textures / TPF Archive".to_string() }
        else if p.contains("msgbnd") { "Texts / Menus (MSGBND)".to_string() }
        else if p.contains("parambnd") { "Game Parameters (PARAMBND)".to_string() }
        else if p.contains("anibnd") { "Character / Animations (ANIBND)".to_string() }
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
                        winner_mod: None,
                        overwritten_mods: Vec::new(),
                        disabled_in_mods: Vec::new(),
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
                winner_mod: None,
                overwritten_mods: Vec::new(),
                disabled_in_mods: Vec::new(),
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
                winner_mod: None,
                overwritten_mods: Vec::new(),
                disabled_in_mods: Vec::new(),
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
                        winner_mod: None,
                        overwritten_mods: Vec::new(),
                        disabled_in_mods: Vec::new(),
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
                winner_mod: None,
                overwritten_mods: Vec::new(),
                disabled_in_mods: Vec::new(),
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
                winner_mod: None,
                overwritten_mods: Vec::new(),
                disabled_in_mods: Vec::new(),
            }
        }
    }

    fn inspect_bnd3_multi(
        rel: &str,
        file_type: &str,
        mod_names: &[String],
        bnds: &[Bnd3Archive],
        winner_name: Option<&str>,
        overwritten_names: &[String],
        disabled_in_mods: &[String],
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

        let active_val_str = winner_name.map(|w| format!("Winner (#1): {}", w));
        let inactive_val_str = if !overwritten_names.is_empty() {
            Some(format!("Overwritten: {}", overwritten_names.join(", ")))
        } else {
            None
        };

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
                        active_val: active_val_str.clone(),
                        inactive_val: inactive_val_str.clone(),
                        mod_values: HashMap::new(),
                        mod_items: mod_items_map,
                        overlapping_items: collided_ids.len(),
                    });
                } else {
                    has_mergeable = true;
                    sub_items.push(SubItemReport {
                        name: sub_name.clone(),
                        level: ConflictLevel::Mergeable,
                        detail: "Safely mergeable (all text IDs are compatible between mods).".to_string(),
                        active_val: None,
                        inactive_val: None,
                        mod_values: HashMap::new(),
                        mod_items: mod_items_map,
                        overlapping_items: 0,
                    });
                }
            } else if sub_name.ends_with(".param") {
                let mut mod_items_map = HashMap::new();
                let mut parsed_params = Vec::new();
                for &idx in &present_indices {
                    let name = &mod_names[idx];
                    let data = &mod_submaps[idx][sub_name].data;
                    let rows = ParamParser::parse_rows(data);
                    mod_items_map.insert(name.clone(), rows.len());
                    parsed_params.push((name.clone(), rows));
                }

                let mut colliding_rows = Vec::new();
                for i in 0..parsed_params.len() {
                    for j in (i + 1)..parsed_params.len() {
                        let (_name_a, rows_a) = &parsed_params[i];
                        let (_name_b, rows_b) = &parsed_params[j];
                        for (id, bytes_a) in rows_a {
                            if let Some(bytes_b) = rows_b.get(id) {
                                if bytes_a != bytes_b && !colliding_rows.contains(id) {
                                    colliding_rows.push(*id);
                                }
                            }
                        }
                    }
                }

                if !colliding_rows.is_empty() {
                    colliding_rows.sort();
                    has_hard_conflict = true;
                    let sample_str = colliding_rows.iter().take(5).map(|id| id.to_string()).collect::<Vec<_>>().join(", ");
                    sub_items.push(SubItemReport {
                        name: format!("{}: Direct Row Discrepancy ({} rows)", sub_name, colliding_rows.len()),
                        level: ConflictLevel::Conflict,
                        detail: format!("Multiple mods set conflicting values for {} rows. Sample row IDs: [{}]", colliding_rows.len(), sample_str),
                        active_val: active_val_str.clone(),
                        inactive_val: inactive_val_str.clone(),
                        mod_values: HashMap::new(),
                        mod_items: mod_items_map,
                        overlapping_items: colliding_rows.len(),
                    });
                } else {
                    has_mergeable = true;
                    sub_items.push(SubItemReport {
                        name: sub_name.clone(),
                        level: ConflictLevel::Mergeable,
                        detail: "Mergeable parameters. All row IDs are unique or identical across mods.".to_string(),
                        active_val: None,
                        inactive_val: None,
                        mod_values: HashMap::new(),
                        mod_items: mod_items_map,
                        overlapping_items: 0,
                    });
                }
            } else if sub_name.ends_with(".hkx") {
                has_hard_conflict = true;
                let mut mod_items_map = HashMap::new();
                let mut mod_values = HashMap::new();
                for &idx in &present_indices {
                    let sz = mod_submaps[idx][sub_name].data.len();
                    mod_items_map.insert(mod_names[idx].clone(), sz);
                    mod_values.insert(mod_names[idx].clone(), format!("{:.1} KB", sz as f64 / 1024.0));
                }
                sub_items.push(SubItemReport {
                    name: format!("Animation: {}", sub_name),
                    level: ConflictLevel::Conflict,
                    detail: format!("Multiple mods provide different Havok animation/skeleton data for internal entry '{}'.", sub_name),
                    active_val: active_val_str.clone(),
                    inactive_val: inactive_val_str.clone(),
                    mod_values,
                    mod_items: mod_items_map,
                    overlapping_items: 1,
                });
            } else if sub_name.ends_with(".tae") {
                has_hard_conflict = true;
                let mut mod_items_map = HashMap::new();
                let mut mod_values = HashMap::new();
                for &idx in &present_indices {
                    let sz = mod_submaps[idx][sub_name].data.len();
                    mod_items_map.insert(mod_names[idx].clone(), sz);
                    mod_values.insert(mod_names[idx].clone(), format!("{:.1} KB", sz as f64 / 1024.0));
                }
                sub_items.push(SubItemReport {
                    name: format!("TimeAct Table: {}", sub_name),
                    level: ConflictLevel::Conflict,
                    detail: format!("Multiple mods alter TimeAct event timing/attacks in internal table '{}'.", sub_name),
                    active_val: active_val_str.clone(),
                    inactive_val: inactive_val_str.clone(),
                    mod_values,
                    mod_items: mod_items_map,
                    overlapping_items: 1,
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
                    active_val: active_val_str.clone(),
                    inactive_val: inactive_val_str.clone(),
                    mod_values: HashMap::new(),
                    mod_items: mod_items_map,
                    overlapping_items: 1,
                });
            } else {
                has_hard_conflict = true;
                let mut mod_items_map = HashMap::new();
                let mut mod_values = HashMap::new();
                for &idx in &present_indices {
                    let sz = mod_submaps[idx][sub_name].data.len();
                    mod_items_map.insert(mod_names[idx].clone(), sz);
                    mod_values.insert(mod_names[idx].clone(), format!("{:.1} KB", sz as f64 / 1024.0));
                }
                sub_items.push(SubItemReport {
                    name: sub_name.clone(),
                    level: ConflictLevel::Conflict,
                    detail: "Internal sub-file has non-mergeable conflicting changes.".to_string(),
                    active_val: active_val_str.clone(),
                    inactive_val: inactive_val_str.clone(),
                    mod_values,
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
            winner_mod: winner_name.map(|s| s.to_string()),
            overwritten_mods: overwritten_names.to_vec(),
            disabled_in_mods: disabled_in_mods.to_vec(),
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
                let name = entry.file_name().to_string_lossy().to_lowercase();
                if name == "ptde" {
                    continue;
                }
                Self::walk_dir(root, &path, map)?;
            } else if path.is_file() {
                let name = entry.file_name().to_string_lossy().to_string();
                if is_ignored_non_game_file(&name) {
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

pub fn is_ignored_non_game_file(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.ends_with(".exe")
        || lower.ends_with(".txt")
        || lower.ends_with(".md")
        || lower.ends_with(".pdf")
        || lower.ends_with(".zip")
        || lower.ends_with(".rar")
        || lower.ends_with(".7z")
        || lower.ends_with(".url")
        || lower.ends_with(".lnk")
        || lower == "thumbs.db"
        || lower == ".ds_store"
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

    // Check for loose menu subfolders (e.g. menu_0/, menu_2/) or font subfolders
    if (lower.starts_with("menu_") || lower.starts_with("menu\\")) && lower.contains('/') {
        return format!("menu/{}", normalized);
    }
    if (lower.starts_with("font_") || lower.starts_with("font\\")) && lower.contains('/') {
        return format!("font/{}", normalized);
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TpupOverrideInfo {
    pub target_archive: String,
    pub texture_name: String,
    pub is_dcx: bool,
}

pub fn parse_tpup_override_path(rel_path: &str) -> Option<TpupOverrideInfo> {
    let normalized = rel_path.replace('\\', "/");
    let lower = normalized.to_lowercase();

    let stem_path = if lower.ends_with(".dds2") {
        &normalized[..normalized.len() - 5]
    } else if lower.ends_with(".dds") {
        &normalized[..normalized.len() - 4]
    } else {
        return None;
    };

    let p = Path::new(stem_path);
    let parent = p.parent()?;
    let parent_str = parent.to_string_lossy().replace('\\', "/");
    if parent_str.is_empty() || parent_str == "." {
        return None;
    }

    // Exclude yabber-style unpack folders like menu_local-tpf-dcx
    if parent_str.ends_with("-tpf-dcx") || parent_str.ends_with("-tpf") {
        return None;
    }

    let texture_name = p.file_name()?.to_string_lossy().to_string();
    if texture_name.is_empty() {
        return None;
    }

    let is_plain_tpf = parent_str.starts_with("parts/") || parent_str.starts_with("other/");
    let target_archive = if is_plain_tpf {
        format!("{}.tpf", parent_str)
    } else {
        format!("{}.tpf.dcx", parent_str)
    };

    Some(TpupOverrideInfo {
        target_archive,
        texture_name,
        is_dcx: !is_plain_tpf,
    })
}

pub fn scan_mod_tpup_details(mod_dir: &Path) -> (bool, usize, Vec<String>) {
    let mut count = 0;
    let mut targets = HashSet::new();
    let mut has_heavy_logic = false;

    let _ = walk_dir_tpup(mod_dir, mod_dir, &mut count, &mut targets, &mut has_heavy_logic);

    let is_tpup = count > 0 && !has_heavy_logic;
    let mut target_list: Vec<String> = targets.into_iter().collect();
    target_list.sort();
    (is_tpup, count, target_list)
}

fn walk_dir_tpup(
    root: &Path,
    current: &Path,
    count: &mut usize,
    targets: &mut HashSet<String>,
    has_heavy_logic: &mut bool,
) -> std::io::Result<()> {
    if !current.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name == "ptde" || name.starts_with('.') {
                continue;
            }
            walk_dir_tpup(root, &p, count, targets, has_heavy_logic)?;
        } else if p.is_file() {
            let name = entry.file_name().to_string_lossy().to_string();
            let lower = name.to_lowercase();
            if lower.ends_with(".msb")
                || lower.ends_with(".emevd")
                || lower.ends_with(".emevd.dcx")
                || lower.ends_with(".param")
                || lower.contains("parambnd")
                || lower.ends_with(".anibnd.dcx")
                || lower.ends_with(".chrbnd.dcx")
            {
                *has_heavy_logic = true;
            }

            if let Ok(rel) = p.strip_prefix(root) {
                let rel_str = rel.to_string_lossy().replace('\\', "/");
                let canonical = resolve_canonical_game_path(&rel_str);
                if let Some(info) = parse_tpup_override_path(&canonical) {
                    *count += 1;
                    targets.insert(info.target_archive);
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bnd3_malformed_never_panics() {
        let mut garbage = b"BND307D7R6\0\0\0\0\0\0".to_vec();
        garbage.extend_from_slice(&999999u32.to_le_bytes()); // huge file count
        garbage.resize(200, 0xCC);
        let res = Bnd3Archive::parse(&garbage);
        assert!(res.is_ok() || res.is_err()); // never panics
    }

    #[test]
    fn test_real_mods_scan() {
        let p1 = r"mods/Dark Souls Re-Remastered 2.0 RC4 (full mod)-642-02032026-RC4-1770363811(1)";
        let p2 = r"mods/The Fading Flame 2.0.8-1043-2-0-8-1757450657";
        if Path::new(p1).exists() && Path::new(p2).exists() {
            let mods = vec![
                ModInput { name: "Dark Souls Re-Remastered".into(), path: p1.into(), variant: None, disabled_files: vec![] },
                ModInput { name: "The Fading Flame".into(), path: p2.into(), variant: None, disabled_files: vec![] },
            ];
            let res = Scanner::scan_multi(&mods).expect("Scan should not fail");
            assert!(res.summary.total_examined > 0);
        }
    }

    #[test]
    fn test_doa_dsrr_scan() {
        let p1 = r"mods/Dark Souls Re-Remastered 2.0 RC4 (full mod)-642-02032026-RC4-1770363811(1)";
        let p2 = r"mods/Daughters of Ash Installer-140-1-6-0-1671456763";
        if Path::new(p1).exists() && Path::new(p2).exists() {
            let mods = vec![
                ModInput { name: "Daughters of Ash".into(), path: p2.into(), variant: None, disabled_files: vec![] },
                ModInput { name: "Dark Souls Re-Remastered".into(), path: p1.into(), variant: None, disabled_files: vec![] },
            ];
            let res = Scanner::scan_multi(&mods).expect("Scan should not fail");
            println!("TOTAL EXAMINED: {}", res.summary.total_examined);
            println!("SAFE: {}", res.summary.safe_count);
            println!("MERGEABLE: {}", res.summary.mergeable_count);
            println!("CONFLICT: {}", res.summary.conflict_count);

            let mut doa_files = 0;
            let mut ptde_files = 0;
            for f in &res.files {
                if f.present_in_mods.contains(&"Daughters of Ash".to_string()) {
                    doa_files += 1;
                    if f.relative_path.contains("ptde") {
                        ptde_files += 1;
                    }
                }
            }
            println!("DOA FILES COUNT: {}", doa_files);
            println!("PTDE IN DETECTED: {}", ptde_files);
            let mut disabled_in_dsrr_strict = Vec::new();
            for f in &res.files {
                let lower = f.relative_path.to_lowercase().replace('\\', "/");
                let is_colliding = f.present_in_mods.len() > 1 && f.present_in_mods.contains(&"Dark Souls Re-Remastered".to_string());
                let is_map_structure = lower.starts_with("map/") && !(
                    lower.ends_with(".tpfbdt") || lower.ends_with(".tpfbhd") || lower.ends_with(".tpf.dcx") || lower.ends_with(".tpf")
                );
                let is_non_asset = lower.starts_with("event/")
                    || lower.starts_with("script/")
                    || lower.ends_with(".msb")
                    || lower.ends_with(".anibnd.dcx")
                    || lower.contains(".esd.")
                    || lower.ends_with(".esd.dcx")
                    || lower.ends_with(".chresdbnd.dcx")
                    || lower.starts_with("param/")
                    || lower.starts_with("menu/")
                    || lower.starts_with("sfx/")
                    || is_map_structure;

                if f.present_in_mods.contains(&"Dark Souls Re-Remastered".to_string()) && (is_colliding || is_non_asset) {
                    disabled_in_dsrr_strict.push(f.relative_path.clone());
                }
            }
            println!("STRICT DISABLED FILES IN DSRR: {}", disabled_in_dsrr_strict.len());

            // Run scan with strict preset applied
            let strict_mods = vec![
                ModInput { name: "Daughters of Ash".into(), path: p2.into(), variant: None, disabled_files: vec![] },
                ModInput { name: "Dark Souls Re-Remastered".into(), path: p1.into(), variant: None, disabled_files: disabled_in_dsrr_strict },
            ];
            let res_strict = Scanner::scan_multi(&strict_mods).expect("Strict scan should succeed");
            println!("STRICT TOTAL EXAMINED: {}", res_strict.summary.total_examined);
            println!("STRICT SAFE: {}", res_strict.summary.safe_count);
            println!("STRICT MERGEABLE: {}", res_strict.summary.mergeable_count);
            println!("STRICT CONFLICT: {}", res_strict.summary.conflict_count);
            assert_eq!(res_strict.summary.conflict_count, 0, "Strict preset must have 0 conflicts!");
            assert_eq!(res_strict.summary.mergeable_count, 0, "Strict preset must have 0 mergeable collisions!");
        }
    }

    #[test]
    fn test_tpup_mods_scan_and_merge() {
        let p1 = r"mods/Dark Bandai Namco Logo-139-1-0-1556525582";
        let p2 = r"mods/Merged HUDs-807-2-2-1750833124";
        let p3 = r"mods/Revamped Stat Buff Icons-20-2-0";

        if Path::new(p1).exists() && Path::new(p2).exists() && Path::new(p3).exists() {
            // Verify TPUP mod detection
            let (is_tpup1, count1, _) = scan_mod_tpup_details(Path::new(p1));
            assert!(is_tpup1, "Dark Bandai Namco Logo must be detected as TPUP");
            assert_eq!(count1, 1);

            let (is_tpup2, count2, _) = scan_mod_tpup_details(Path::new(p2));
            assert!(is_tpup2, "Merged HUDs must be detected as TPUP");
            assert_eq!(count2, 6);

            let (is_tpup3, count3, _) = scan_mod_tpup_details(Path::new(p3));
            assert!(is_tpup3, "Revamped Stat Buff Icons must be detected as TPUP");
            assert_eq!(count3, 1);

            let mods = vec![
                ModInput { name: "Dark Bandai Namco Logo".into(), path: p1.into(), variant: None, disabled_files: vec![] },
                ModInput { name: "Merged HUDs".into(), path: p2.into(), variant: None, disabled_files: vec![] },
                ModInput { name: "Revamped Stat Buff Icons".into(), path: p3.into(), variant: None, disabled_files: vec![] },
            ];

            let res = Scanner::scan_multi(&mods).expect("Scan TPUP mods");
            println!("TPUP SCAN TOTAL: {}", res.summary.total_examined);
            println!("TPUP SCAN MERGEABLE: {}", res.summary.mergeable_count);
            println!("TPUP SCAN SAFE: {}", res.summary.safe_count);
            println!("TPUP SCAN CONFLICT: {}", res.summary.conflict_count);

            for f in &res.files {
                println!(" - File: {}, Level: {:?}, Summary: {}", f.relative_path, f.level, f.summary);
                for sub in &f.sub_items {
                    println!("    * Sub-item: {}, Level: {:?}, Detail: {}", sub.name, sub.level, sub.detail);
                }
            }

            // Both menu_2.tpf.dcx and menu_3.tpf.dcx should be Mergeable!
            let m2 = res.files.iter().find(|f| f.relative_path.contains("menu_2")).expect("menu_2 found");
            assert_eq!(m2.level, ConflictLevel::Mergeable, "menu_2 must be Mergeable");

            let m3 = res.files.iter().find(|f| f.relative_path.contains("menu_3")).expect("menu_3 found");
            assert_eq!(m3.level, ConflictLevel::Mergeable, "menu_3 must be Mergeable");

            // Test merging
            let req = crate::merger::MergeRequest {
                mods: mods.clone(),
                output_dir: "target/test_tpup_merged".into(),
                resolution_mode: "smart".into(),
                file_overrides: HashMap::new(),
                mod_a: String::new(),
                mod_b: String::new(),
                priority: String::new(),
            };
            let merge_res = crate::merger::Merger::merge(&req).expect("Merge TPUP mods");
            println!("Merge result: {}", merge_res.message);
            assert!(merge_res.success);

            // Verify merged TPF archives exist and can be parsed
            let out_m2 = Path::new("target/test_tpup_merged/menu/menu_2.tpf.dcx");
            assert!(out_m2.exists(), "Merged menu_2.tpf.dcx must exist");
            let m2_bytes = std::fs::read(out_m2).expect("Read merged menu_2");
            let m2_tpf = crate::formats::TpfArchive::parse(&m2_bytes).expect("Parse merged menu_2");
            assert!(m2_tpf.textures.iter().any(|t| t.name.eq_ignore_ascii_case("Logo_02")), "Injected Logo_02 must exist in merged menu_2");
            assert!(m2_tpf.textures.iter().any(|t| t.name.eq_ignore_ascii_case("Menu07_1")), "Injected Menu07_1 must exist in merged menu_2");

            let out_m3 = Path::new("target/test_tpup_merged/menu/menu_3.tpf.dcx");
            assert!(out_m3.exists(), "Merged menu_3.tpf.dcx must exist");
            let m3_bytes = std::fs::read(out_m3).expect("Read merged menu_3");
            let m3_tpf = crate::formats::TpfArchive::parse(&m3_bytes).expect("Parse merged menu_3");
            assert!(m3_tpf.textures.iter().any(|t| t.name.eq_ignore_ascii_case("Icon50")), "Injected Icon50 must exist in merged menu_3");
            assert!(m3_tpf.textures.iter().any(|t| t.name.eq_ignore_ascii_case("Menu03")), "Injected Menu03 must exist in merged menu_3");
            assert!(m3_tpf.textures.iter().any(|t| t.name.eq_ignore_ascii_case("Menu07_7")), "Injected Menu07_7 must exist in merged menu_3");

            println!("TEST TPUP MODS SCAN AND MERGE PASSED WITH FLYING COLORS!");
        }
    }
}
