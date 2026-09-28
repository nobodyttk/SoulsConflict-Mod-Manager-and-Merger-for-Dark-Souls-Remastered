#![windows_subsystem = "windows"]

mod formats;
mod scanner;
mod merger;
mod deployer;
mod archive;

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::webview::{WebViewBuilder, WebContext};

use scanner::{Scanner, ModInput, resolve_canonical_game_path};
use merger::{Merger, MergeRequest};
use deployer::{Deployer, DeployerConfig};
use serde::{Deserialize, Serialize};

const HTML_CONTENT: &str = include_str!("../static/index.html");
const CSS_CONTENT: &str = include_str!("../static/style.css");
const JS_CONTENT: &str = include_str!("../static/app.js");

#[derive(Deserialize)]
struct ScanApiRequest {
    #[serde(default)]
    mods: Vec<ModInput>,
    #[serde(default)]
    mod_a: String,
    #[serde(default)]
    mod_b: String,
}

#[derive(Deserialize)]
struct OpenFolderRequest {
    target: String,
}

#[derive(Deserialize)]
struct CreateFolderRequest {
    #[serde(default)]
    name: String,
}

#[derive(Deserialize)]
struct FixModRequest {
    target: String,
}

#[derive(Deserialize)]
struct SetPathRequest {
    #[serde(default)]
    path: String,
}

#[derive(Deserialize)]
struct DeployApiRequest {
    #[serde(default)]
    active_mods: Vec<deployer::DeployedModInfo>,
}

#[derive(Deserialize)]
struct LaunchApiRequest {
    #[serde(default)]
    mode: Option<String>,
}

#[derive(Serialize)]
struct FolderInfo {
    id: String,
    name: String,
    relative_path: String,
    full_path: String,
    file_count: usize,
    exists: bool,
    subfolder_detected: Option<String>,
    variants: Vec<String>,
    selected_variant: Option<String>,
    has_structure_issues: bool,
}

#[derive(Serialize)]
struct MultiStatusResponse {
    mods: Vec<FolderInfo>,
    merged: FolderInfo,
}

pub fn get_app_dir() -> PathBuf {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let p_str = parent.to_string_lossy().to_lowercase();
            if !p_str.ends_with(r"\target\debug")
                && !p_str.ends_with(r"/target/debug")
                && !p_str.ends_with(r"\target\release")
                && !p_str.ends_with(r"/target/release")
            {
                return parent.to_path_buf();
            }
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn unwrap_single_child_dir(dir: &Path) -> PathBuf {
    let mut current = dir.to_path_buf();
    loop {
        if !current.is_dir() {
            break;
        }
        if let Ok(entries) = fs::read_dir(&current) {
            let items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            let dirs: Vec<_> = items.iter().filter(|e| e.path().is_dir()).collect();
            let files: Vec<_> = items.iter().filter(|e| e.path().is_file()).collect();

            // If there's exactly 1 directory and NO files, and this directory is NOT a canonical game folder
            if dirs.len() == 1 && files.is_empty() {
                let name = dirs[0].file_name().to_string_lossy().to_string().to_lowercase();
                if !is_canonical_game_dir(&name) {
                    current = dirs[0].path();
                    continue;
                }
            }
        }
        break;
    }
    current
}

fn main() {
    let port = 4545;
    let addr = format!("127.0.0.1:{}", port);
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to start server on port {}: {}", port, e);
            return;
        }
    };

    // Ensure default directories exist relative to the portable app root
    let base_dir = get_app_dir();
    let mods_root = base_dir.join("mods");
    let merged_dir = base_dir.join("merged");

    let _ = fs::create_dir_all(&mods_root);
    let _ = fs::create_dir_all(&merged_dir);

    let url = format!("http://127.0.0.1:{}", port);
    println!("============================================================");
    println!("   🔥 SoulsConflict - Mod Conflict Checker & Merger        ");
    println!("   Version: v2.0.3 • Running in Native Window (Portable)     ");
    println!("   Base Folder: {}", base_dir.display());
    println!("   Local server running at: {}", url);
    println!("   Close the window to terminate the application           ");
    println!("============================================================");

    // Run the TCP server in a background thread
    thread::spawn(move || {
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
        }
    });

    // Create the native window
    let event_loop = EventLoop::new();
    let icon_data = include_bytes!("../img/icon.ico");
    let icon_img = image::load_from_memory(icon_data).unwrap().into_rgba8();
    let (icon_width, icon_height) = icon_img.dimensions();
    let window_icon = tao::window::Icon::from_rgba(icon_img.into_raw(), icon_width, icon_height).ok();

    let window = WindowBuilder::new()
        .with_title("SoulsConflict")
        .with_inner_size(tao::dpi::LogicalSize::new(1200.0, 800.0))
        .with_window_icon(window_icon)
        .build(&event_loop)
        .unwrap();

    let webview_data_dir = std::env::temp_dir().join("SoulsConflictWebView");
    let mut web_context = WebContext::new(Some(webview_data_dir));

    let _webview = WebViewBuilder::new(window)
        .unwrap()
        .with_url(&url)
        .unwrap()
        .with_web_context(&mut web_context)
        .build()
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}

fn count_files_recursive(dir: &Path) -> usize {
    if !dir.exists() {
        return 0;
    }
    let mut count = 0;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.is_dir() {
                count += count_files_recursive(&p);
            } else if p.is_file() {
                count += 1;
            }
        }
    }
    count
}

pub fn is_canonical_game_dir(name: &str) -> bool {
    matches!(
        name,
        "chr" | "parts" | "obj" | "event" | "map" | "param" | "script" | "msg" | "ffx" | "menu" | "sound" | "action" | "sfx" | "font" | "facegen"
    )
}

fn detect_variants(dir: &Path) -> Vec<String> {
    if !dir.exists() {
        return Vec::new();
    }
    let effective = unwrap_single_child_dir(dir);
    let Ok(entries) = fs::read_dir(&effective) else {
        return Vec::new();
    };

    let mut dirs = Vec::new();
    let mut root_game_files = 0;

    for entry in entries.filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            let lower = name.to_lowercase();
            if is_canonical_game_dir(&lower) {
                return Vec::new();
            }
            dirs.push(name);
        } else if p.is_file() {
            root_game_files += 1;
        }
    }

    if dirs.len() >= 2 && root_game_files == 0 {
        dirs.sort();
        return dirs;
    }

    Vec::new()
}

fn check_mod_structure_issues(dir: &Path, variants: &[String]) -> bool {
    if !dir.exists() {
        return false;
    }
    if !variants.is_empty() {
        return true;
    }
    if let Ok(entries) = fs::read_dir(dir) {
        let items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        let dirs: Vec<_> = items.iter().filter(|e| e.path().is_dir()).collect();
        let files: Vec<_> = items.iter().filter(|e| e.path().is_file()).collect();

        // If wrapped in single folder that is not a canonical dir
        if dirs.len() == 1 && files.is_empty() {
            let sub_name = dirs[0].file_name().to_string_lossy().to_string();
            if !is_canonical_game_dir(&sub_name.to_lowercase()) {
                return true;
            }
        }

        // If files are directly in root and would need a subfolder like chr/
        for f in files {
            let fname = f.file_name().to_string_lossy().to_string();
            let canonical = resolve_canonical_game_path(&fname);
            if canonical != fname {
                return true;
            }
        }
    }
    false
}

fn normalize_mod_path(p: &Path, variant: Option<&str>) -> PathBuf {
    if !p.exists() {
        return p.to_path_buf();
    }

    // 1. If explicit variant is provided and exists
    if let Some(v) = variant {
        if !v.trim().is_empty() {
            let cand_direct = p.join(v.trim());
            if cand_direct.exists() {
                return unwrap_single_child_dir(&cand_direct);
            }
            let eff = unwrap_single_child_dir(p);
            let cand_eff = eff.join(v.trim());
            if cand_eff.exists() {
                return unwrap_single_child_dir(&cand_eff);
            }
        }
    }

    // 2. If directory has variants, default to the first one
    let eff = unwrap_single_child_dir(p);
    let variants = detect_variants(&eff);
    if !variants.is_empty() {
        let cand = eff.join(&variants[0]);
        if cand.exists() {
            return unwrap_single_child_dir(&cand);
        }
    }

    eff
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FixModResult {
    pub success: bool,
    pub organized_count: usize,
    pub is_variant: bool,
    pub message: String,
}

fn fix_mod_structure_internal(dir: &Path) -> Result<FixModResult, String> {
    if !dir.exists() {
        return Err(format!("Folder does not exist: {}", dir.display()));
    }

    let variants = detect_variants(dir);
    if !variants.is_empty() {
        let mut count = 0;
        for v in &variants {
            let v_path = dir.join(v);
            count += reorganize_folder_files(&v_path)?;
        }
        return Ok(FixModResult {
            success: true,
            organized_count: count,
            is_variant: true,
            message: format!("{} file(s) organized in variant folders.", count),
        });
    }

    let count = reorganize_folder_files(dir)?;
    Ok(FixModResult {
        success: true,
        organized_count: count,
        is_variant: false,
        message: format!("{} file(s) organized.", count),
    })
}

pub fn auto_organize_mod_if_needed(dir: &Path) -> usize {
    if !dir.exists() {
        return 0;
    }
    let variants = detect_variants(dir);
    let total = if !variants.is_empty() {
        let mut count = 0;
        for v in &variants {
            let v_path = dir.join(v);
            count += reorganize_folder_files(&v_path).unwrap_or(0);
        }
        count
    } else {
        reorganize_folder_files(dir).unwrap_or(0)
    };
    if total > 0 {
        println!("[Auto-Organize] Reorganized {} loose/misplaced file(s) in '{}' into canonical DS1 folders.", total, dir.display());
    }
    total
}

fn reorganize_folder_files(target_dir: &Path) -> Result<usize, String> {
    let mut files_to_move: Vec<(PathBuf, PathBuf)> = Vec::new();
    collect_files_to_reorganize(target_dir, target_dir, &mut files_to_move)?;

    let mut moved_count = 0;
    for (src, dest) in files_to_move {
        if src != dest {
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            if fs::rename(&src, &dest).is_err() {
                fs::copy(&src, &dest).map_err(|e| e.to_string())?;
                let _ = fs::remove_file(&src);
            }
            moved_count += 1;
        }
    }

    clean_empty_dirs(target_dir);
    Ok(moved_count)
}

fn collect_files_to_reorganize(root: &Path, current: &Path, list: &mut Vec<(PathBuf, PathBuf)>) -> Result<(), String> {
    if !current.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(current).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_to_reorganize(root, &path, list)?;
        } else if path.is_file() {
            if let Ok(rel) = path.strip_prefix(root) {
                let rel_str = rel.to_string_lossy().replace('\\', "/");
                let canonical = resolve_canonical_game_path(&rel_str);
                let dest = root.join(&canonical);
                list.push((path, dest));
            }
        }
    }
    Ok(())
}

fn clean_empty_dirs(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.is_dir() {
                clean_empty_dirs(&p);
                let _ = fs::remove_dir(&p);
            }
        }
    }
}

fn get_subfolder_name(p: &Path) -> Option<String> {
    let unwrapped = unwrap_single_child_dir(p);
    if &unwrapped != p {
        return Some(unwrapped.file_name().unwrap_or_default().to_string_lossy().to_string());
    }
    None
}

fn build_folder_info(id: &str, _name: &str, rel_path: &str) -> FolderInfo {
    let base_dir = get_app_dir();
    let full = base_dir.join(rel_path);
    let full_path = full.canonicalize().unwrap_or(full.clone()).to_string_lossy().to_string();
    let exists = full.exists();
    let file_count = count_files_recursive(&full);
    let subfolder_detected = get_subfolder_name(&full);
    let variants = detect_variants(&full);
    let selected_variant = variants.first().cloned();
    let has_structure_issues = check_mod_structure_issues(&full, &variants);

    // If a clear mod subfolder was detected, use it as display name!
    let display_name = if let Some(ref sub) = subfolder_detected {
        sub.clone()
    } else {
        id.to_string()
    };

    FolderInfo {
        id: id.to_string(),
        name: display_name,
        relative_path: rel_path.to_string(),
        full_path,
        file_count,
        exists,
        subfolder_detected,
        variants,
        selected_variant,
        has_structure_issues,
    }
}

fn get_all_mod_folders() -> Vec<FolderInfo> {
    let base_dir = get_app_dir();
    let mods_dir = base_dir.join("mods");
    let mut list = Vec::new();

    if let Ok(entries) = fs::read_dir(&mods_dir) {
        // Detect any loose archives (.zip, .rar, .7z) placed directly in mods/
        let mut archives = Vec::new();
        for entry in entries.filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.is_file() {
                if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    if ext_lower == "zip" || ext_lower == "rar" || ext_lower == "7z" {
                        archives.push(p);
                    }
                }
            }
        }

        // Auto-extract any loose archives into their own mod folder named after the archive
        for arch in archives {
            let file_stem = arch.file_stem().unwrap_or_default().to_string_lossy().to_string();
            println!("[Archive] Found archive in mods/: '{}'. Auto-extracting into native mod folder...", arch.display());
            match crate::archive::import_mod_archive(&arch, &mods_dir) {
                Ok((folder_name, count)) => {
                    println!("[Archive] Successfully extracted '{}' ({} files) into 'mods/{}'. Removing archive file.", file_stem, count, folder_name);
                    let _ = fs::remove_file(&arch);
                }
                Err(e) => {
                    eprintln!("[Archive] Failed to auto-extract archive '{}': {}", arch.display(), e);
                }
            }
        }

        // Now process directories in mods/
        if let Ok(dir_entries) = fs::read_dir(&mods_dir) {
            let mut dirs: Vec<_> = dir_entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .collect();

            dirs.sort_by_key(|e| e.file_name());

            for entry in dirs {
                let folder_name = entry.file_name().to_string_lossy().to_string();
                // Ignore hidden or internal folders if any
                if folder_name.starts_with('.') {
                    continue;
                }
                let rel_path = format!("mods/{}", folder_name);
                list.push(build_folder_info(&folder_name, &folder_name, &rel_path));
            }
        }
    }

    list
}

fn next_available_mod_name() -> String {
    let base_dir = get_app_dir();
    let mods_dir = base_dir.join("mods");

    for i in 1..=999 {
        let candidate = format!("Mod_{}", i);
        if !mods_dir.join(&candidate).exists() {
            return candidate;
        }
    }
    format!("Mod_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn read_http_request(stream: &mut TcpStream) -> Result<(String, String, String, Vec<u8>), std::io::Error> {
    let mut buf = Vec::with_capacity(16384);
    let mut temp = [0u8; 65536];
    let mut header_end = None;

    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(60)));

    while header_end.is_none() {
        let n = match stream.read(&mut temp) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        buf.extend_from_slice(&temp[..n]);
        if let Some(pos) = find_subslice(&buf, b"\r\n\r\n") {
            header_end = Some(pos);
            break;
        }
    }

    let header_pos = match header_end {
        Some(pos) => pos,
        None => return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Incomplete headers")),
    };

    let header_bytes = &buf[..header_pos];
    let header_str = String::from_utf8_lossy(header_bytes);
    
    let mut lines = header_str.lines();
    let first_line = lines.next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid HTTP first line"));
    }
    let method = parts[0].to_uppercase();
    let path = parts[1].to_string();

    let mut content_length: usize = 0;
    for line in lines {
        let lower = line.to_lowercase();
        if lower.starts_with("content-length:") {
            if let Some(val_str) = line.split(':').nth(1) {
                if let Ok(val) = val_str.trim().parse::<usize>() {
                    content_length = val;
                }
            }
        }
    }

    let mut body_bytes = buf[header_pos + 4..].to_vec();
    while body_bytes.len() < content_length {
        let to_read = (content_length - body_bytes.len()).min(temp.len());
        let n = match stream.read(&mut temp[..to_read]) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        body_bytes.extend_from_slice(&temp[..n]);
    }

    let body_str = if path.starts_with("/api/import_mod_archive") {
        String::new()
    } else {
        String::from_utf8_lossy(&body_bytes).to_string()
    };

    Ok((method, path, body_str, body_bytes))
}

fn url_decode(s: &str) -> String {
    let mut result = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""), 16) {
                result.push(byte);
                i += 3;
                continue;
            }
        } else if bytes[i] == b'+' {
            result.push(b' ');
            i += 1;
            continue;
        }
        result.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&result).to_string()
}

fn handle_client(mut stream: TcpStream) {
    let (method, path, body, body_bytes) = match read_http_request(&mut stream) {
        Ok(req) => req,
        Err(_) => return,
    };

    if method == "OPTIONS" {
        send_response(&mut stream, "204 No Content", "text/plain", b"");
        return;
    }

    let request = &body;

    match (method.as_str(), path.as_str()) {
        ("GET", "/") => {
            send_response(&mut stream, "200 OK", "text/html; charset=utf-8", HTML_CONTENT.as_bytes());
        }
        ("GET", "/style.css") => {
            send_response(&mut stream, "200 OK", "text/css; charset=utf-8", CSS_CONTENT.as_bytes());
        }
        ("GET", "/app.js") => {
            send_response(&mut stream, "200 OK", "application/javascript; charset=utf-8", JS_CONTENT.as_bytes());
        }
        ("GET", "/logo.jpg") => {
            let logo_data = include_bytes!("../img/logo.jpg");
            send_response(&mut stream, "200 OK", "image/jpeg", logo_data);
        }
        ("GET", "/api/status") => {
            let status = MultiStatusResponse {
                mods: get_all_mod_folders(),
                merged: build_folder_info("merged", "Merged Folder", "merged"),
            };
            let json = serde_json::to_string(&status).unwrap();
            send_response(&mut stream, "200 OK", "application/json", json.as_bytes());
        }
        ("POST", p) if p.starts_with("/api/import_mod_archive") => {
            let filename = if let Some(q_pos) = p.find('?') {
                let query = &p[q_pos + 1..];
                query.split('&')
                    .find_map(|pair| {
                        let mut parts = pair.splitn(2, '=');
                        let k = parts.next()?;
                        let v = parts.next()?;
                        if k == "name" {
                            Some(url_decode(v))
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| "imported_mod.zip".to_string())
            } else {
                "imported_mod.zip".to_string()
            };

            let base_dir = get_app_dir();
            let mods_dir = base_dir.join("mods");
            let _ = fs::create_dir_all(&mods_dir);

            if body_bytes.is_empty() {
                let resp = serde_json::json!({ "success": false, "message": "Uploaded archive file is empty." });
                send_response(&mut stream, "400 Bad Request", "application/json", resp.to_string().as_bytes());
            } else {
                let ext = Path::new(&filename)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("zip");
                let clean_name = crate::archive::sanitize_folder_name(&filename);
                let temp_name = format!(
                    "mod_import_{}_{}.{}",
                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
                    clean_name,
                    ext
                );
                let temp_path = std::env::temp_dir().join(&temp_name);

                if let Err(e) = fs::write(&temp_path, &body_bytes) {
                    let resp = serde_json::json!({ "success": false, "message": format!("Failed to buffer archive: {}", e) });
                    send_response(&mut stream, "500 Internal Server Error", "application/json", resp.to_string().as_bytes());
                } else {
                    match crate::archive::import_mod_archive_named(&temp_path, &mods_dir, Some(&filename)) {
                        Ok((mod_name, count)) => {
                            let _ = fs::remove_file(&temp_path);
                            let resp = serde_json::json!({
                                "success": true,
                                "mod_name": mod_name,
                                "extracted_count": count,
                                "message": format!("Mod '{}' imported successfully ({} files organized).", mod_name, count)
                            });
                            send_response(&mut stream, "200 OK", "application/json", resp.to_string().as_bytes());
                        }
                        Err(e) => {
                            let _ = fs::remove_file(&temp_path);
                            let resp = serde_json::json!({ "success": false, "message": e });
                            send_response(&mut stream, "400 Bad Request", "application/json", resp.to_string().as_bytes());
                        }
                    }
                }
            }
        }
        ("POST", "/api/create_mod_folder") => {
            let body = extract_body(&request);
            let req: Result<CreateFolderRequest, _> = serde_json::from_str(&body);
            let name = match req {
                Ok(r) if !r.name.trim().is_empty() => r.name.trim().to_string(),
                _ => next_available_mod_name(),
            };

            let base_dir = get_app_dir();
            let target_dir = base_dir.join("mods").join(&name);
            let _ = fs::create_dir_all(&target_dir);
            let full_path = target_dir.canonicalize().unwrap_or(target_dir);

            #[cfg(target_os = "windows")]
            {
                use std::os::windows::process::CommandExt;
                let _ = Command::new("explorer.exe")
                    .arg(&full_path)
                    .creation_flags(0x08000000)
                    .spawn();
            }

            let response = format!("{{\"success\":true,\"name\":\"{}\",\"path\":\"mods/{}\"}}", name, name);
            send_response(&mut stream, "200 OK", "application/json", response.as_bytes());
        }
        ("POST", "/api/delete_mod_folder") => {
            let body = extract_body(&request);
            let req: Result<OpenFolderRequest, _> = serde_json::from_str(&body);
            if let Ok(r) = req {
                let base_dir = get_app_dir();
                let target = base_dir.join("mods").join(r.target.trim());
                if target.exists() && target.is_dir() {
                    let _ = fs::remove_dir_all(&target);
                }
            }
            send_response(&mut stream, "200 OK", "application/json", b"{\"success\":true}");
        }
        ("GET", "/api/deployer/status") => {
            let status = Deployer::get_status();
            let json = serde_json::to_string(&status).unwrap();
            send_response(&mut stream, "200 OK", "application/json", json.as_bytes());
        }
        ("POST", "/api/deployer/set_path") => {
            let body = extract_body(&request);
            let req: Result<SetPathRequest, _> = serde_json::from_str(&body);
            if let Ok(r) = req {
                if !r.path.trim().is_empty() {
                    let cfg = DeployerConfig { game_path: r.path.trim().to_string() };
                    let _ = Deployer::save_config(&cfg);
                }
            }
            let status = Deployer::get_status();
            let json = serde_json::to_string(&status).unwrap();
            send_response(&mut stream, "200 OK", "application/json", json.as_bytes());
        }
        ("POST", "/api/deployer/deploy") => {
            let body = extract_body(&request);
            let req: Result<DeployApiRequest, _> = serde_json::from_str(&body);
            let active_mods = req.map(|r| r.active_mods).unwrap_or_default();
            match Deployer::deploy_mods(active_mods) {
                Ok(result) => {
                    let json = serde_json::to_string(&result).unwrap();
                    send_response(&mut stream, "200 OK", "application/json", json.as_bytes());
                }
                Err(err) => {
                    let resp = serde_json::json!({
                        "success": false,
                        "error_code": err.error_code,
                        "message": err.message,
                        "detail": err.detail
                    });
                    send_response(&mut stream, "400 Bad Request", "application/json", resp.to_string().as_bytes());
                }
            }
        }
        ("POST", "/api/deployer/restore") => {
            match Deployer::restore_vanilla() {
                Ok(result) => {
                    let json = serde_json::to_string(&result).unwrap();
                    send_response(&mut stream, "200 OK", "application/json", json.as_bytes());
                }
                Err(err) => {
                    let resp = serde_json::json!({
                        "success": false,
                        "error_code": err.error_code,
                        "message": err.message,
                        "detail": err.detail
                    });
                    send_response(&mut stream, "400 Bad Request", "application/json", resp.to_string().as_bytes());
                }
            }
        }
        ("POST", "/api/deployer/launch") => {
            let body = extract_body(&request);
            let req: Result<LaunchApiRequest, _> = serde_json::from_str(&body);
            let mode = req.ok().and_then(|r| r.mode);
            match Deployer::launch_game(mode.as_deref()) {
                Ok(result) => {
                    let json = serde_json::to_string(&result).unwrap();
                    send_response(&mut stream, "200 OK", "application/json", json.as_bytes());
                }
                Err(e) => {
                    let resp = serde_json::json!({
                        "success": false,
                        "message": e
                    });
                    send_response(&mut stream, "400 Bad Request", "application/json", resp.to_string().as_bytes());
                }
            }
        }
        ("POST", "/api/deployer/open_game_folder") => {
            let cfg = Deployer::load_config();
            let target = PathBuf::from(&cfg.game_path);
            if target.exists() {
                #[cfg(target_os = "windows")]
                {
                    use std::os::windows::process::CommandExt;
                    let _ = Command::new("explorer.exe")
                        .arg(&target)
                        .creation_flags(0x08000000)
                        .spawn();
                }
            }
            send_response(&mut stream, "200 OK", "application/json", b"{\"success\":true}");
        }
        ("POST", "/api/fix_mod_structure") => {
            let body = extract_body(&request);
            let req: Result<FixModRequest, _> = serde_json::from_str(&body);
            let base_dir = get_app_dir();
            match req {
                Ok(r) => {
                    let target_dir = base_dir.join("mods").join(r.target.trim());
                    match fix_mod_structure_internal(&target_dir) {
                        Ok(res) => {
                            let json = serde_json::to_string(&res).unwrap();
                            send_response(&mut stream, "200 OK", "application/json", json.as_bytes());
                        }
                        Err(e) => {
                            let resp = serde_json::json!({
                                "success": false,
                                "message": e
                            });
                            send_response(&mut stream, "400 Bad Request", "application/json", resp.to_string().as_bytes());
                        }
                    }
                }
                Err(e) => {
                    let resp = serde_json::json!({
                        "success": false,
                        "message": e.to_string()
                    });
                    send_response(&mut stream, "400 Bad Request", "application/json", resp.to_string().as_bytes());
                }
            }
        }
        ("POST", "/api/open_folder") => {
            let body = extract_body(&request);
            let req: Result<OpenFolderRequest, _> = serde_json::from_str(&body);
            let base_dir = get_app_dir();

            let target_dir = match req {
                Ok(r) => {
                    let t = r.target.trim();
                    if t == "merged" {
                        base_dir.join("merged")
                    } else if t == "mods" {
                        base_dir.join("mods")
                    } else if t.starts_with("mods/") || t.starts_with("mods\\") {
                        base_dir.join(t)
                    } else {
                        // could be an id like "ModC"
                        base_dir.join("mods").join(t)
                    }
                }
                Err(_) => base_dir.join("mods"),
            };

            let _ = fs::create_dir_all(&target_dir);
            let full_path = target_dir.canonicalize().unwrap_or(target_dir);

            #[cfg(target_os = "windows")]
            {
                use std::os::windows::process::CommandExt;
                let _ = Command::new("explorer.exe")
                    .arg(&full_path)
                    .creation_flags(0x08000000)
                    .spawn();
            }

            send_response(&mut stream, "200 OK", "application/json", b"{\"success\":true}");
        }
        ("POST", "/api/scan") => {
            let body = extract_body(&request);
            match serde_json::from_str::<ScanApiRequest>(&body) {
                Ok(req) => {
                    let base_dir = get_app_dir();
                    let mut resolved_inputs: Vec<ModInput> = Vec::new();

                    if !req.mods.is_empty() {
                        for m in req.mods {
                            let p = if m.path.trim().is_empty() {
                                base_dir.join("mods").join(&m.name)
                            } else {
                                PathBuf::from(m.path.trim())
                            };
                            let norm = normalize_mod_path(&p, m.variant.as_deref());
                            resolved_inputs.push(ModInput {
                                name: m.name,
                                path: norm.to_string_lossy().to_string(),
                                variant: m.variant,
                            });
                        }
                    } else {
                        // Fallback to legacy 2-mod fields
                        let p_a = if req.mod_a.trim().is_empty() { base_dir.join("mods").join("ModA") } else { PathBuf::from(req.mod_a.trim()) };
                        let p_b = if req.mod_b.trim().is_empty() { base_dir.join("mods").join("ModB") } else { PathBuf::from(req.mod_b.trim()) };
                        resolved_inputs.push(ModInput { name: "Mod A".into(), path: normalize_mod_path(&p_a, None).to_string_lossy().into(), variant: None });
                        resolved_inputs.push(ModInput { name: "Mod B".into(), path: normalize_mod_path(&p_b, None).to_string_lossy().into(), variant: None });
                    }

                    match Scanner::scan_multi(&resolved_inputs) {
                        Ok(result) => {
                            let json = serde_json::to_string(&result).unwrap();
                            send_response(&mut stream, "200 OK", "application/json", json.as_bytes());
                        }
                        Err(err) => {
                            send_response(&mut stream, "400 Bad Request", "text/plain", err.as_bytes());
                        }
                    }
                }
                Err(err) => {
                    send_response(&mut stream, "400 Bad Request", "text/plain", err.to_string().as_bytes());
                }
            }
        }
        ("POST", "/api/merge") => {
            let body = extract_body(&request);
            match serde_json::from_str::<MergeRequest>(&body) {
                Ok(mut req) => {
                    let base_dir = get_app_dir();

                    for m in &mut req.mods {
                        let p = if m.path.trim().is_empty() {
                            base_dir.join("mods").join(&m.name)
                        } else {
                            PathBuf::from(m.path.trim())
                        };
                        m.path = normalize_mod_path(&p, m.variant.as_deref()).to_string_lossy().to_string();
                    }

                    if req.mods.is_empty() {
                        let p_a = if req.mod_a.trim().is_empty() { base_dir.join("mods").join("ModA") } else { PathBuf::from(req.mod_a.trim()) };
                        let p_b = if req.mod_b.trim().is_empty() { base_dir.join("mods").join("ModB") } else { PathBuf::from(req.mod_b.trim()) };
                        req.mod_a = normalize_mod_path(&p_a, None).to_string_lossy().to_string();
                        req.mod_b = normalize_mod_path(&p_b, None).to_string_lossy().to_string();
                    }

                    if req.output_dir.trim().is_empty() {
                        req.output_dir = base_dir.join("merged").to_string_lossy().to_string();
                    }

                    match Merger::merge(&req) {
                        Ok(res) => {
                            let json = serde_json::to_string(&res).unwrap();
                            send_response(&mut stream, "200 OK", "application/json", json.as_bytes());
                        }
                        Err(err) => {
                            send_response(&mut stream, "400 Bad Request", "text/plain", err.as_bytes());
                        }
                    }
                }
                Err(err) => {
                    send_response(&mut stream, "400 Bad Request", "text/plain", err.to_string().as_bytes());
                }
            }
        }
        _ => {
            send_response(&mut stream, "404 Not Found", "text/plain", b"Page not found");
        }
    }
}

fn extract_body(request: &str) -> String {
    if let Some(pos) = request.find("\r\n\r\n") {
        request[pos + 4..].to_string()
    } else if let Some(pos) = request.find("\n\n") {
        request[pos + 2..].to_string()
    } else {
        request.to_string()
    }
}

fn send_response(stream: &mut TcpStream, status: &str, content_type: &str, body: &[u8]) {
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
        status,
        content_type,
        body.len()
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.write_all(body);
    let _ = stream.flush();
}
