use std::collections::HashMap;
use std::io::Read;
use flate2::read::ZlibDecoder;

pub fn decompress_dcx(data: &[u8]) -> Result<Vec<u8>, String> {
    if !is_dcx(data) {
        return Ok(data.to_vec());
    }
    // Search for DCA\0
    let mut dca_pos = None;
    for i in 0..data.len().saturating_sub(4) {
        if &data[i..i+4] == b"DCA\0" {
            dca_pos = Some(i);
            break;
        }
    }
    let dca_idx = dca_pos.ok_or_else(|| "DCA marker not found in DCX file".to_string())?;
    let compressed = &data[dca_idx + 8..];
    let mut decoder = ZlibDecoder::new(compressed);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|e| format!("zlib decompression failed: {}", e))?;
    Ok(decompressed)
}

pub fn is_dcx(data: &[u8]) -> bool {
    data.len() >= 4 && &data[0..4] == b"DCX\0"
}

pub fn compress_dcx(data: &[u8]) -> Result<Vec<u8>, String> {
    use std::io::Write;
    use flate2::write::ZlibEncoder;
    use flate2::Compression;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(data).map_err(|e| format!("zlib compression failed: {}", e))?;
    let compressed = encoder.finish().map_err(|e| format!("zlib finish failed: {}", e))?;

    let uncompressed_len = data.len() as u32;
    let compressed_len = compressed.len() as u32;

    // Standard FromSoftware DCX_DFLT header (76 bytes)
    let mut header = Vec::with_capacity(76 + compressed.len());
    header.extend_from_slice(b"DCX\0");
    header.extend_from_slice(&0x10000u32.to_be_bytes());
    header.extend_from_slice(&24u32.to_be_bytes());
    header.extend_from_slice(&36u32.to_be_bytes());
    header.extend_from_slice(&36u32.to_be_bytes());
    header.extend_from_slice(&44u32.to_be_bytes());
    header.extend_from_slice(b"DCS\0");
    header.extend_from_slice(&uncompressed_len.to_be_bytes());
    header.extend_from_slice(&compressed_len.to_be_bytes());
    header.extend_from_slice(b"DCP\0DFLT");
    header.extend_from_slice(&32u32.to_be_bytes());
    header.extend_from_slice(&9u32.to_be_bytes());
    header.extend_from_slice(&0u32.to_be_bytes());
    header.extend_from_slice(&0u32.to_be_bytes());
    header.extend_from_slice(&0u32.to_be_bytes());
    header.extend_from_slice(&0x10100u32.to_be_bytes());
    header.extend_from_slice(b"DCA\0");
    header.extend_from_slice(&8u32.to_be_bytes());
    header.extend_from_slice(&compressed);

    Ok(header)
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BndEntry {
    pub flags: u32,
    pub id: u32,
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Bnd3Archive {
    pub entries: Vec<BndEntry>,
}

impl Bnd3Archive {
    pub fn parse(raw: &[u8]) -> Result<Self, String> {
        let decompressed = decompress_dcx(raw)?;
        if decompressed.len() < 32 || &decompressed[0..4] != b"BND3" {
            return Err("Not a valid BND3 file".to_string());
        }

        let raw_file_count = u32::from_le_bytes(
            decompressed[16..20]
                .try_into()
                .map_err(|_| "Failed to read BND3 file count".to_string())?,
        ) as usize;

        // Cap allocation to the maximum possible entries that could physically fit in the buffer
        let max_possible_entries = (decompressed.len().saturating_sub(32)) / 24;
        let file_count = raw_file_count.min(max_possible_entries);
        let mut entries = Vec::with_capacity(file_count);

        for i in 0..file_count {
            let hdr_off = 32 + i * 24;
            if hdr_off + 24 > decompressed.len() {
                break;
            }
            let flags = u32::from_le_bytes(decompressed[hdr_off..hdr_off + 4].try_into().unwrap());
            let comp_sz = u32::from_le_bytes(decompressed[hdr_off + 4..hdr_off + 8].try_into().unwrap()) as usize;
            let data_off = u32::from_le_bytes(decompressed[hdr_off + 8..hdr_off + 12].try_into().unwrap()) as usize;
            let entry_id = u32::from_le_bytes(decompressed[hdr_off + 12..hdr_off + 16].try_into().unwrap());
            let name_off = u32::from_le_bytes(decompressed[hdr_off + 16..hdr_off + 20].try_into().unwrap()) as usize;

            // Safe extraction of name: check that name_off is within bounds
            let mut name = if name_off < decompressed.len() {
                let mut name_end = name_off;
                while name_end < decompressed.len() && decompressed[name_end] != 0 && (name_end - name_off) < 260 {
                    name_end += 1;
                }
                String::from_utf8_lossy(&decompressed[name_off..name_end]).to_string()
            } else {
                format!("entry_{}", entry_id)
            };
            if name.trim().is_empty() {
                name = format!("entry_{}", entry_id);
            }

            let data = if data_off < decompressed.len() {
                let end = data_off.saturating_add(comp_sz).min(decompressed.len());
                decompressed[data_off..end].to_vec()
            } else {
                Vec::new()
            };

            entries.push(BndEntry {
                flags,
                id: entry_id,
                name,
                data,
            });
        }

        Ok(Bnd3Archive { entries })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        let file_count = self.entries.len();
        let entry_headers_size = file_count * 24;
        let names_start = 32 + entry_headers_size;

        let mut names_blob = Vec::new();
        let mut name_offsets = Vec::with_capacity(file_count);

        for e in &self.entries {
            name_offsets.push((names_start + names_blob.len()) as u32);
            names_blob.extend_from_slice(e.name.as_bytes());
            names_blob.push(0); // null terminator
        }

        let names_end = names_start + names_blob.len();
        let mut data_start = names_end;
        if data_start % 16 != 0 {
            let pad = 16 - (data_start % 16);
            names_blob.resize(names_blob.len() + pad, 0);
            data_start += pad;
        }

        let mut file_data_offsets = Vec::with_capacity(file_count);
        let mut files_blob = Vec::new();

        for e in &self.entries {
            let mut curr_off = data_start + files_blob.len();
            if curr_off % 16 != 0 {
                let pad = 16 - (curr_off % 16);
                files_blob.resize(files_blob.len() + pad, 0);
                curr_off += pad;
            }
            file_data_offsets.push(curr_off as u32);
            files_blob.extend_from_slice(&e.data);
        }

        let total_size = 32 + entry_headers_size + names_blob.len() + files_blob.len();
        let mut out = Vec::with_capacity(total_size);

        // Header (32 bytes)
        out.extend_from_slice(b"BND3");
        out.extend_from_slice(b"07D7R6\0\0");
        out.extend_from_slice(&0x74u32.to_le_bytes()); // format flags
        out.extend_from_slice(&(file_count as u32).to_le_bytes());
        out.extend_from_slice(&(data_start as u32).to_le_bytes());
        out.extend_from_slice(&[0u8; 8]); // padding / zero

        // Entry headers (24 bytes each)
        for (i, e) in self.entries.iter().enumerate() {
            let sz = e.data.len() as u32;
            out.extend_from_slice(&e.flags.to_le_bytes());
            out.extend_from_slice(&sz.to_le_bytes());
            out.extend_from_slice(&file_data_offsets[i].to_le_bytes());
            out.extend_from_slice(&e.id.to_le_bytes());
            out.extend_from_slice(&name_offsets[i].to_le_bytes());
            out.extend_from_slice(&sz.to_le_bytes());
        }

        out.extend_from_slice(&names_blob);
        out.extend_from_slice(&files_blob);

        Ok(out)
    }
}

pub struct FmgParser;

impl FmgParser {
    pub fn parse(data: &[u8]) -> HashMap<i32, String> {
        let mut result = HashMap::new();
        if data.len() < 28 {
            return result;
        }

        let raw_range_cnt = u32::from_le_bytes(data[12..16].try_into().unwrap_or([0, 0, 0, 0])) as usize;
        let off_tbl = u32::from_le_bytes(data[20..24].try_into().unwrap_or([0, 0, 0, 0])) as usize;

        if off_tbl >= data.len() {
            return result;
        }

        let max_ranges = (data.len().saturating_sub(28)) / 12;
        let range_cnt = raw_range_cnt.min(max_ranges);

        for r in 0..range_cnt {
            let r_off = 28 + r * 12;
            if r_off + 12 > data.len() {
                break;
            }
            let first_idx = i32::from_le_bytes(data[r_off..r_off + 4].try_into().unwrap_or([0, 0, 0, 0])) as usize;
            let first_id = i32::from_le_bytes(data[r_off + 4..r_off + 8].try_into().unwrap_or([0, 0, 0, 0]));
            let last_id = i32::from_le_bytes(data[r_off + 8..r_off + 12].try_into().unwrap_or([0, 0, 0, 0]));

            if last_id < first_id {
                continue;
            }
            let count = (last_id as i64 - first_id as i64 + 1) as usize;
            if count > 50_000 || count > data.len() {
                continue;
            }

            for i in 0..count {
                let tid = first_id.saturating_add(i as i32);
                let s_idx = first_idx.saturating_add(i);
                let pos = off_tbl.saturating_add(s_idx * 4);
                if pos + 4 > data.len() {
                    continue;
                }
                let str_off = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap_or([0, 0, 0, 0])) as usize;
                if str_off == 0 || str_off >= data.len() {
                    result.insert(tid, String::new());
                } else {
                    let mut null_pos = str_off;
                    while null_pos + 1 < data.len() {
                        if data[null_pos] == 0 && data[null_pos + 1] == 0 {
                            break;
                        }
                        null_pos += 2;
                    }
                    if str_off <= null_pos && null_pos <= data.len() {
                        let utf16_slice: Vec<u16> = data[str_off..null_pos]
                            .chunks_exact(2)
                            .map(|c| u16::from_le_bytes([c[0], c[1]]))
                            .collect();
                        let text = String::from_utf16_lossy(&utf16_slice);
                        result.insert(tid, text);
                    }
                }
            }
        }

        result
    }
}

pub struct ParamParser;

impl ParamParser {
    /// Extracts row IDs from a .param file in Dark Souls 1 / Remastered (12-byte descriptor: id, data_off, name_off)
    #[allow(dead_code)]
    pub fn parse_row_ids(data: &[u8]) -> Vec<u32> {
        let mut row_ids = Vec::new();
        if data.len() < 48 {
            return row_ids;
        }

        let mut row_count = u16::from_le_bytes(data[10..12].try_into().unwrap_or([0, 0])) as usize;
        if row_count == 0 {
            row_count = u16::from_le_bytes(data[8..10].try_into().unwrap_or([0, 0])) as usize;
        }
        let row_index_start = 48usize;
        let max_rows = (data.len().saturating_sub(row_index_start)) / 12;
        let count = row_count.min(max_rows);

        for i in 0..count {
            let entry_off = row_index_start + i * 12;
            if entry_off + 4 > data.len() {
                break;
            }
            let row_id = u32::from_le_bytes(data[entry_off..entry_off + 4].try_into().unwrap_or([0, 0, 0, 0]));
            row_ids.push(row_id);
        }

        row_ids
    }

    /// Extracts row IDs and their raw row payload bytes for deep delta analysis
    pub fn parse_rows(data: &[u8]) -> HashMap<u32, Vec<u8>> {
        let mut rows = HashMap::new();
        if data.len() < 48 {
            return rows;
        }

        let str_off = u32::from_le_bytes(data[0..4].try_into().unwrap_or([0, 0, 0, 0])) as usize;
        let mut row_count = u16::from_le_bytes(data[10..12].try_into().unwrap_or([0, 0])) as usize;
        if row_count == 0 {
            row_count = u16::from_le_bytes(data[8..10].try_into().unwrap_or([0, 0])) as usize;
        }
        let row_index_start = 48usize;
        let max_rows = (data.len().saturating_sub(row_index_start)) / 12;
        let count = row_count.min(max_rows);

        let mut descriptors = Vec::with_capacity(count);
        for i in 0..count {
            let entry_off = row_index_start + i * 12;
            if entry_off + 12 > data.len() {
                break;
            }
            let row_id = u32::from_le_bytes(data[entry_off..entry_off + 4].try_into().unwrap_or([0, 0, 0, 0]));
            let data_off = u32::from_le_bytes(data[entry_off + 4..entry_off + 8].try_into().unwrap_or([0, 0, 0, 0])) as usize;
            descriptors.push((row_id, data_off));
        }

        let string_boundary = if str_off > 0 && str_off <= data.len() { str_off } else { data.len() };

        for i in 0..descriptors.len() {
            let (id, data_off) = descriptors[i];
            let next_data_off = if i + 1 < descriptors.len() {
                descriptors[i + 1].1
            } else {
                string_boundary
            };

            if data_off < data.len() && next_data_off <= data.len() && next_data_off >= data_off {
                rows.insert(id, data[data_off..next_data_off].to_vec());
            }
        }

        rows
    }
}

pub struct EmevdParser;

#[derive(Debug, Clone)]
pub struct EmevdEventInfo {
    pub id: u32,
    pub instruction_count: u32,
}

impl EmevdParser {
    pub fn parse_events(raw: &[u8]) -> Result<Vec<EmevdEventInfo>, String> {
        let decompressed = decompress_dcx(raw)?;
        if decompressed.len() < 24 || &decompressed[0..4] != b"EVD\0" {
            return Err("Not a valid EMEVD file".to_string());
        }

        let raw_count = u32::from_le_bytes(decompressed[16..20].try_into().unwrap_or([0, 0, 0, 0])) as usize;
        let events_offset = u32::from_le_bytes(decompressed[20..24].try_into().unwrap_or([0, 0, 0, 0])) as usize;

        if events_offset >= decompressed.len() {
            return Ok(Vec::new());
        }

        let max_events = (decompressed.len().saturating_sub(events_offset)) / 28;
        let event_count = raw_count.min(max_events);

        let mut events = Vec::with_capacity(event_count);
        for i in 0..event_count {
            let off = events_offset + i * 28;
            if off + 12 > decompressed.len() {
                break;
            }
            let eid = u32::from_le_bytes(decompressed[off..off + 4].try_into().unwrap_or([0, 0, 0, 0]));
            let icnt = u32::from_le_bytes(decompressed[off + 4..off + 8].try_into().unwrap_or([0, 0, 0, 0]));
            events.push(EmevdEventInfo { id: eid, instruction_count: icnt });
        }
        Ok(events)
    }
}

pub struct MsbParser;

#[derive(Debug, Clone)]
pub struct MsbEntity {
    pub name: String,
    pub entity_id: i32,
}

impl MsbParser {
    pub fn parse_entities(data: &[u8]) -> Result<Vec<MsbEntity>, String> {
        if data.len() < 64 {
            return Err("MSB file too small".to_string());
        }

        // Locate PARTS_PARAM_ST section marker
        let marker = b"PARTS_PARAM_ST\0";
        let mut marker_pos = None;
        for i in 0..data.len().saturating_sub(marker.len()) {
            if &data[i..i + marker.len()] == marker {
                marker_pos = Some(i);
                break;
            }
        }

        let mut entities = Vec::new();
        let marker_idx = match marker_pos {
            Some(p) => p,
            None => return Ok(entities),
        };

        // Scan backwards from marker to find section header
        let mut sec_hdr_opt = None;
        let search_start = marker_idx.saturating_sub(64);
        for p in (search_start..marker_idx).rev() {
            if p + 8 <= data.len() {
                let name_off = u32::from_le_bytes(data[p..p + 4].try_into().unwrap_or([0, 0, 0, 0])) as usize;
                if name_off == marker_idx {
                    sec_hdr_opt = Some(p);
                    break;
                }
            }
        }

        let sec_hdr = match sec_hdr_opt {
            Some(p) => p,
            None => return Ok(entities),
        };

        if sec_hdr + 8 > data.len() {
            return Ok(entities);
        }

        let count = u32::from_le_bytes(data[sec_hdr + 4..sec_hdr + 8].try_into().unwrap_or([0, 0, 0, 0])) as usize;
        if count <= 1 {
            return Ok(entities);
        }

        let real_count = count.saturating_sub(1);
        let offsets_start = sec_hdr + 8;
        let max_possible = (data.len().saturating_sub(offsets_start)) / 4;
        let safe_count = real_count.min(max_possible);

        for i in 0..safe_count {
            let pos = offsets_start + i * 4;
            if pos + 4 > data.len() {
                break;
            }
            let entry_off = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap_or([0, 0, 0, 0])) as usize;
            if entry_off < data.len() && entry_off + 0xa4 <= data.len() {
                // Name at offset 0x64
                let name_slice = &data[entry_off + 0x64..];
                let mut name_end = 0;
                while name_end < name_slice.len() && name_slice[name_end] != 0 && name_end < 48 {
                    name_end += 1;
                }
                let name = String::from_utf8_lossy(&name_slice[..name_end]).to_string();

                // Entity ID at offset 0xa0
                let entity_id = i32::from_le_bytes(data[entry_off + 0xa0..entry_off + 0xa4].try_into().unwrap_or([0, 0, 0, 0]));
                if entity_id > 0 {
                    entities.push(MsbEntity { name, entity_id });
                }
            }
        }

        Ok(entities)
    }
}

