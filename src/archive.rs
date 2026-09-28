use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn sanitize_folder_name(raw: &str) -> String {
    let sanitized: String = raw
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            other => other,
        })
        .collect();

    let trimmed = sanitized.trim().trim_matches('.').trim();
    if trimmed.is_empty() {
        "Mod_Archive".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn extract_zip_file(zip_path: &Path, target_dir: &Path) -> Result<usize, String> {
    let file = fs::File::open(zip_path)
        .map_err(|e| format!("Failed to open ZIP file '{}': {}", zip_path.display(), e))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("Failed to read ZIP archive '{}': {}", zip_path.display(), e))?;

    let mut count = 0;
    fs::create_dir_all(target_dir)
        .map_err(|e| format!("Failed to create destination directory '{}': {}", target_dir.display(), e))?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)
            .map_err(|e| format!("Failed to read ZIP entry #{}: {}", i, e))?;

        let raw_name = entry.name().replace('\\', "/");
        let safe_rel: PathBuf = raw_name
            .split('/')
            .filter(|p| !p.is_empty() && *p != "." && *p != "..")
            .collect();

        if safe_rel.as_os_str().is_empty() {
            continue;
        }

        let dest_path = target_dir.join(&safe_rel);

        if entry.is_dir() {
            let _ = fs::create_dir_all(&dest_path);
        } else {
            if let Some(parent) = dest_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let mut outfile = fs::File::create(&dest_path)
                .map_err(|e| format!("Failed to create extracted file '{}': {}", dest_path.display(), e))?;
            std::io::copy(&mut entry, &mut outfile)
                .map_err(|e| format!("Failed to write extracted file '{}': {}", dest_path.display(), e))?;
            count += 1;
        }
    }

    Ok(count)
}

#[allow(dead_code)]
pub fn extract_zip_bytes(bytes: &[u8], target_dir: &Path) -> Result<usize, String> {
    let cursor = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor)
        .map_err(|e| format!("Failed to read ZIP archive from memory: {}", e))?;

    let mut count = 0;
    fs::create_dir_all(target_dir)
        .map_err(|e| format!("Failed to create destination directory '{}': {}", target_dir.display(), e))?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)
            .map_err(|e| format!("Failed to read ZIP entry #{}: {}", i, e))?;

        let raw_name = entry.name().replace('\\', "/");
        let safe_rel: PathBuf = raw_name
            .split('/')
            .filter(|p| !p.is_empty() && *p != "." && *p != "..")
            .collect();

        if safe_rel.as_os_str().is_empty() {
            continue;
        }

        let dest_path = target_dir.join(&safe_rel);

        if entry.is_dir() {
            let _ = fs::create_dir_all(&dest_path);
        } else {
            if let Some(parent) = dest_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let mut outfile = fs::File::create(&dest_path)
                .map_err(|e| format!("Failed to create extracted file '{}': {}", dest_path.display(), e))?;
            std::io::copy(&mut entry, &mut outfile)
                .map_err(|e| format!("Failed to write extracted file '{}': {}", dest_path.display(), e))?;
            count += 1;
        }
    }

    Ok(count)
}

pub fn extract_rar_file(rar_path: &Path, target_dir: &Path) -> Result<usize, String> {
    fs::create_dir_all(target_dir)
        .map_err(|e| format!("Failed to create destination directory '{}': {}", target_dir.display(), e))?;

    // 1. Try pure-Rust rars crate first
    let rars_res = (|| -> Result<usize, String> {
        let archive = rars::ArchiveReader::read_path(rar_path)
            .map_err(|e| format!("rars failed to open archive: {:?}", e))?;

        let mut count = 0;
        let dest = target_dir.to_path_buf();

        archive.extract_to(None, |meta| {
            let name_lossy = meta.name_lossy().replace('\\', "/");
            let safe_rel: PathBuf = name_lossy
                .split('/')
                .filter(|p| !p.is_empty() && *p != "." && *p != "..")
                .collect();

            if safe_rel.as_os_str().is_empty() {
                return Ok(Box::new(std::io::sink()) as Box<dyn std::io::Write>);
            }

            let out_path = dest.join(safe_rel);

            if meta.is_directory {
                let _ = fs::create_dir_all(&out_path);
                Ok(Box::new(std::io::sink()) as Box<dyn std::io::Write>)
            } else {
                if let Some(parent) = out_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                match fs::File::create(&out_path) {
                    Ok(file) => {
                        count += 1;
                        Ok(Box::new(file) as Box<dyn std::io::Write>)
                    }
                    Err(_) => Ok(Box::new(std::io::sink()) as Box<dyn std::io::Write>),
                }
            }
        }).map_err(|e| format!("rars extraction error: {:?}", e))?;

        Ok(count)
    })();

    if let Ok(c) = rars_res {
        if c > 0 {
            return Ok(c);
        }
    }

    // 2. Fallback to Windows built-in tar.exe (System32\tar.exe supports RAR via libarchive on Windows 10/11)
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        let mut cmd = Command::new("tar.exe");
        cmd.args(&["-xf", &rar_path.to_string_lossy(), "-C", &target_dir.to_string_lossy()])
            .creation_flags(0x08000000); // CREATE_NO_WINDOW

        if let Ok(status) = cmd.status() {
            if status.success() {
                let count = count_files_recursive(target_dir);
                if count > 0 {
                    return Ok(count);
                }
            }
        }

        // 3. Fallback to 7z if installed on user system
        let mut cmd7z = Command::new("7z");
        cmd7z.args(&["x", "-y", &format!("-o{}", target_dir.display()), &rar_path.to_string_lossy()])
            .creation_flags(0x08000000);

        if let Ok(status) = cmd7z.status() {
            if status.success() {
                let count = count_files_recursive(target_dir);
                if count > 0 {
                    return Ok(count);
                }
            }
        }
    }

    let extracted = count_files_recursive(target_dir);
    if extracted > 0 {
        Ok(extracted)
    } else {
        Err(format!("Failed to extract RAR archive '{}'. File may be corrupted or encrypted.", rar_path.display()))
    }
}

pub fn extract_archive(archive_path: &Path, target_dir: &Path) -> Result<usize, String> {
    let ext = archive_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "zip" => extract_zip_file(archive_path, target_dir),
        "rar" => extract_rar_file(archive_path, target_dir),
        "7z" => {
            #[cfg(target_os = "windows")]
            {
                use std::os::windows::process::CommandExt;
                // Try tar.exe first (built-in Windows 10/11)
                let mut cmd = Command::new("tar.exe");
                cmd.args(&["-xf", &archive_path.to_string_lossy(), "-C", &target_dir.to_string_lossy()])
                    .creation_flags(0x08000000);

                if let Ok(status) = cmd.status() {
                    if status.success() {
                        let count = count_files_recursive(target_dir);
                        if count > 0 {
                            return Ok(count);
                        }
                    }
                }

                // Try 7z if tar fails
                let mut cmd7z = Command::new("7z");
                cmd7z.args(&["x", "-y", &format!("-o{}", target_dir.display()), &archive_path.to_string_lossy()])
                    .creation_flags(0x08000000);

                if let Ok(status) = cmd7z.status() {
                    if status.success() {
                        let count = count_files_recursive(target_dir);
                        if count > 0 {
                            return Ok(count);
                        }
                    }
                }
            }
            Err(format!("Unsupported or failed to extract 7z archive: {}", archive_path.display()))
        }
        other => Err(format!("Unsupported archive extension '.{}'. Only .zip, .rar, and .7z are supported.", other)),
    }
}

pub fn import_mod_archive(archive_path: &Path, mods_dir: &Path) -> Result<(String, usize), String> {
    import_mod_archive_named(archive_path, mods_dir, None)
}

pub fn import_mod_archive_named(
    archive_path: &Path,
    mods_dir: &Path,
    custom_name: Option<&str>,
) -> Result<(String, usize), String> {
    let stem = match custom_name {
        Some(name) => Path::new(name)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        None => archive_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
    };

    let clean_name = sanitize_folder_name(&stem);
    let mut target_dir = mods_dir.join(&clean_name);

    // If target folder already exists, find a non-conflicting unique name
    if target_dir.exists() {
        for i in 1..=99 {
            let candidate_name = format!("{} ({})", clean_name, i);
            let candidate_path = mods_dir.join(&candidate_name);
            if !candidate_path.exists() {
                target_dir = candidate_path;
                break;
            }
        }
    }

    let count = extract_archive(archive_path, &target_dir)?;

    // Unwrap single non-canonical wrapper folder if present (e.g. ModName/ModName/files...)
    unwrap_single_wrapper_if_needed(&target_dir);

    // Auto-organize loose files into canonical Dark Souls folders
    crate::auto_organize_mod_if_needed(&target_dir);

    let final_mod_name = target_dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    Ok((final_mod_name, count))
}

pub fn count_files_recursive(dir: &Path) -> usize {
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

pub fn unwrap_single_wrapper_if_needed(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        let items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        let dirs: Vec<_> = items.iter().filter(|e| e.path().is_dir()).collect();
        let files: Vec<_> = items.iter().filter(|e| e.path().is_file()).collect();

        // If there's exactly 1 directory, 0 files, and the directory is NOT a canonical game folder
        if dirs.len() == 1 && files.is_empty() {
            let sub = dirs[0].path();
            let sub_name = sub.file_name().unwrap_or_default().to_string_lossy().to_string();
            let is_canonical = crate::is_canonical_game_dir(&sub_name.to_lowercase());

            // If it's a wrapper folder like "Artorias Armor - HD" inside "Artorias Armor - HD"
            if !is_canonical {
                // Move everything from sub to dir
                if sub.exists() {
                    let temp_parent = dir.join("__tmp_unwrap__");
                    if fs::rename(&sub, &temp_parent).is_ok() {
                        if let Ok(temp_entries) = fs::read_dir(&temp_parent) {
                            for item in temp_entries.filter_map(|e| e.ok()) {
                                let target = dir.join(item.file_name());
                                let _ = fs::rename(item.path(), target);
                            }
                        }
                        let _ = fs::remove_dir_all(&temp_parent);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_folder_name() {
        assert_eq!(sanitize_folder_name("Dark Souls: Remastered Mod"), "Dark Souls_ Remastered Mod");
        assert_eq!(sanitize_folder_name("Mod/With\\Slashes"), "Mod_With_Slashes");
        assert_eq!(sanitize_folder_name("...Trailing dots..."), "Trailing dots");
        assert_eq!(sanitize_folder_name(""), "Mod_Archive");
    }

    #[test]
    fn test_extract_zip_bytes() {
        let mut zip_buf = Vec::new();
        {
            let mut writer = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buf));
            use zip::write::SimpleFileOptions;
            let options = SimpleFileOptions::default();

            writer.start_file("c1000.chrbnd.dcx", options).unwrap();
            std::io::Write::write_all(&mut writer, b"mock chr data").unwrap();

            writer.start_file("parts/wp_a_0100.partsbnd.dcx", options).unwrap();
            std::io::Write::write_all(&mut writer, b"mock parts data").unwrap();

            writer.finish().unwrap();
        }

        let temp_dir = std::env::temp_dir().join(format!("ds1_test_archive_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let count = extract_zip_bytes(&zip_buf, &temp_dir).unwrap();
        assert_eq!(count, 2);
        assert!(temp_dir.join("c1000.chrbnd.dcx").exists());
        assert!(temp_dir.join("parts").join("wp_a_0100.partsbnd.dcx").exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_import_mod_archive_named() {
        let temp_base = std::env::temp_dir().join(format!("ds1_test_import_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let mods_dir = temp_base.join("mods");
        let _ = fs::create_dir_all(&mods_dir);

        let zip_path = temp_base.join("temp_upload.zip");
        {
            let file = fs::File::create(&zip_path).unwrap();
            let mut writer = zip::ZipWriter::new(file);
            use zip::write::SimpleFileOptions;
            let options = SimpleFileOptions::default();

            writer.start_file("c1000.chrbnd.dcx", options).unwrap();
            std::io::Write::write_all(&mut writer, b"mock chr data").unwrap();

            writer.finish().unwrap();
        }

        let (mod_name, count) = import_mod_archive_named(&zip_path, &mods_dir, Some("Cool DS1 Mod v1.0.zip")).unwrap();
        assert_eq!(mod_name, "Cool DS1 Mod v1.0");
        assert_eq!(count, 1);

        let mod_dir = mods_dir.join("Cool DS1 Mod v1.0");
        assert!(mod_dir.exists());
        // c1000.chrbnd.dcx is auto-organized into chr/c1000.chrbnd.dcx
        assert!(mod_dir.join("chr").join("c1000.chrbnd.dcx").exists());

        let _ = fs::remove_dir_all(&temp_base);
    }
}

