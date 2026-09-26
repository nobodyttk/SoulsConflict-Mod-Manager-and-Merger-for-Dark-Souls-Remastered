use std::collections::HashMap;
use std::io::Read;
use flate2::read::ZlibDecoder;

pub fn decompress_dcx(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 4 || &data[0..4] != b"DCX\0" {
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

        let file_count = u32::from_le_bytes(decompressed[16..20].try_into().unwrap()) as usize;
        let mut entries = Vec::with_capacity(file_count);

        for i in 0..file_count {
            let hdr_off = 32 + i * 24;
            if hdr_off + 24 > decompressed.len() {
                break;
            }
            let flags = u32::from_le_bytes(decompressed[hdr_off..hdr_off+4].try_into().unwrap());
            let comp_sz = u32::from_le_bytes(decompressed[hdr_off+4..hdr_off+8].try_into().unwrap()) as usize;
            let data_off = u32::from_le_bytes(decompressed[hdr_off+8..hdr_off+12].try_into().unwrap()) as usize;
            let entry_id = u32::from_le_bytes(decompressed[hdr_off+12..hdr_off+16].try_into().unwrap());
            let name_off = u32::from_le_bytes(decompressed[hdr_off+16..hdr_off+20].try_into().unwrap()) as usize;

            // Extract name (null-terminated string)
            let mut name_end = name_off;
            while name_end < decompressed.len() && decompressed[name_end] != 0 {
                name_end += 1;
            }
            let name = String::from_utf8_lossy(&decompressed[name_off..name_end]).to_string();

            let data = if data_off + comp_sz <= decompressed.len() {
                decompressed[data_off..data_off + comp_sz].to_vec()
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
}

pub struct FmgParser;

impl FmgParser {
    pub fn parse(data: &[u8]) -> HashMap<i32, String> {
        let mut result = HashMap::new();
        if data.len() < 28 {
            return result;
        }

        let range_cnt = u32::from_le_bytes(data[12..16].try_into().unwrap()) as usize;
        let off_tbl = u32::from_le_bytes(data[20..24].try_into().unwrap()) as usize;

        if off_tbl > data.len() {
            return result;
        }

        for r in 0..range_cnt {
            let r_off = 28 + r * 12;
            if r_off + 12 > data.len() {
                break;
            }
            let first_idx = i32::from_le_bytes(data[r_off..r_off+4].try_into().unwrap()) as usize;
            let first_id = i32::from_le_bytes(data[r_off+4..r_off+8].try_into().unwrap());
            let last_id = i32::from_le_bytes(data[r_off+8..r_off+12].try_into().unwrap());

            if last_id < first_id {
                continue;
            }
            let count = (last_id - first_id + 1) as usize;

            for i in 0..count {
                let tid = first_id + i as i32;
                let s_idx = first_idx + i;
                let pos = off_tbl + s_idx * 4;
                if pos + 4 > data.len() {
                    continue;
                }
                let str_off = u32::from_le_bytes(data[pos..pos+4].try_into().unwrap()) as usize;
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
                    let utf16_slice: Vec<u16> = data[str_off..null_pos]
                        .chunks_exact(2)
                        .map(|c| u16::from_le_bytes([c[0], c[1]]))
                        .collect();
                    let text = String::from_utf16_lossy(&utf16_slice);
                    result.insert(tid, text);
                }
            }
        }

        result
    }
}

pub struct ParamParser;

impl ParamParser {
    /// Extracts row IDs from a .param file in Dark Souls 1 / Remastered
    pub fn parse_row_ids(data: &[u8]) -> Vec<u32> {
        let mut row_ids = Vec::new();
        if data.len() < 48 {
            return row_ids;
        }

        // DS1 Param header:
        // offset 0x08: row_count (u16)
        let row_count = u16::from_le_bytes(data[8..10].try_into().unwrap()) as usize;
        // offset 0x14: header size / row index start (usually 0x30 = 48)
        let row_index_start = 48usize;
        // Each row index entry in DS1 is 24 bytes: ID(4), data_offset(4), name_offset(4)...
        for i in 0..row_count {
            let entry_off = row_index_start + i * 24;
            if entry_off + 4 > data.len() {
                break;
            }
            let row_id = u32::from_le_bytes(data[entry_off..entry_off+4].try_into().unwrap());
            row_ids.push(row_id);
        }

        row_ids
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

        let event_count = u32::from_le_bytes(decompressed[16..20].try_into().unwrap()) as usize;
        let events_offset = u32::from_le_bytes(decompressed[20..24].try_into().unwrap()) as usize;

        let mut events = Vec::with_capacity(event_count);
        for i in 0..event_count {
            let off = events_offset + i * 28;
            if off + 12 > decompressed.len() {
                break;
            }
            let eid = u32::from_le_bytes(decompressed[off..off+4].try_into().unwrap());
            let icnt = u32::from_le_bytes(decompressed[off+4..off+8].try_into().unwrap());
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
            if &data[i..i+marker.len()] == marker {
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
                let name_off = u32::from_le_bytes(data[p..p+4].try_into().unwrap()) as usize;
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

        let count = u32::from_le_bytes(data[sec_hdr+4..sec_hdr+8].try_into().unwrap()) as usize;
        if count <= 1 {
            return Ok(entities);
        }

        let real_count = count.saturating_sub(1);
        let offsets_start = sec_hdr + 8;

        for i in 0..real_count {
            let pos = offsets_start + i * 4;
            if pos + 4 > data.len() {
                break;
            }
            let entry_off = u32::from_le_bytes(data[pos..pos+4].try_into().unwrap()) as usize;
            if entry_off + 0xa4 <= data.len() {
                // Name at offset 0x64
                let name_slice = &data[entry_off + 0x64..];
                let mut name_end = 0;
                while name_end < name_slice.len() && name_slice[name_end] != 0 && name_end < 48 {
                    name_end += 1;
                }
                let name = String::from_utf8_lossy(&name_slice[..name_end]).to_string();

                // Entity ID at offset 0xa0
                let entity_id = i32::from_le_bytes(data[entry_off+0xa0..entry_off+0xa4].try_into().unwrap());
                if entity_id > 0 {
                    entities.push(MsbEntity { name, entity_id });
                }
            }
        }

        Ok(entities)
    }
}

