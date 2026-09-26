import os
import zlib
import struct
import hashlib
from typing import Dict, List, Any, Optional

def get_file_hash(filepath: str) -> str:
    h = hashlib.sha256()
    with open(filepath, 'rb') as f:
        while chunk := f.read(65536):
            h.update(chunk)
    return h.hexdigest()

def is_dcx(data: bytes) -> bool:
    return len(data) >= 4 and data[:4] == b'DCX\x00'

def decompress_dcx(data: bytes) -> bytes:
    if not is_dcx(data):
        return data
    dca_idx = data.find(b'DCA\x00')
    if dca_idx == -1:
        raise ValueError("Invalid DCX file: missing DCA marker")
    return zlib.decompress(data[dca_idx + 8:])

def compress_dcx(data: bytes, original_header: Optional[bytes] = None) -> bytes:
    compressed = zlib.compress(data, 9)
    if original_header and b'DCA\x00' in original_header and b'DCS\x00' in original_header:
        dcs_idx = original_header.find(b'DCS\x00')
        dca_idx = original_header.find(b'DCA\x00')
        new_dcs = struct.pack('>4sII', b'DCS\x00', len(data), len(compressed))
        return original_header[:dcs_idx] + new_dcs + original_header[dcs_idx+12:dca_idx+8] + compressed
    else:
        # Standard DCX_DFLT header (76 bytes)
        hdr = bytearray()
        hdr += struct.pack('>4sIIII', b'DCX\x00', 0x10000, 24, 36, 36)
        hdr += struct.pack('>I', 44)
        hdr += struct.pack('>4sII', b'DCS\x00', len(data), len(compressed))
        hdr += struct.pack('>4s4sIIIIII', b'DCP\x00', b'DFLT', 32, 9, 0, 0, 0, 0x10100)
        hdr += struct.pack('>4sI', b'DCA\x00', 8)
        return bytes(hdr) + compressed

class BND3Entry:
    def __init__(self, flags: int, entry_id: int, name: str, data: bytes):
        self.flags = flags
        self.entry_id = entry_id
        self.name = name
        self.data = data

class BND3Archive:
    def __init__(self):
        self.magic = b'BND3'
        self.version = b'07D7R6\x00\x00'
        self.format_flags = 0x74
        self.entries: List[BND3Entry] = []
        self.raw_header: bytes = b''

    @classmethod
    def from_bytes(cls, raw: bytes) -> 'BND3Archive':
        data = decompress_dcx(raw)
        if len(data) < 32 or data[:4] != b'BND3':
            raise ValueError("Not a valid BND3 archive")
        
        bnd = cls()
        bnd.raw_header = data[:32]
        bnd.magic = data[:4]
        bnd.version = data[4:12]
        bnd.format_flags = struct.unpack_from('<I', data, 12)[0]
        file_count = struct.unpack_from('<I', data, 16)[0]

        for i in range(file_count):
            hdr_off = 32 + i * 24
            flags, comp_sz, data_off, entry_id, name_off, uncomp_sz = struct.unpack_from('<IIIIII', data, hdr_off)
            null_idx = data.find(b'\x00', name_off)
            name = data[name_off:null_idx].decode('shift_jis', errors='replace')
            entry_data = data[data_off:data_off + comp_sz]
            bnd.entries.append(BND3Entry(flags, entry_id, name, entry_data))
        
        return bnd

    def to_bytes(self) -> bytes:
        file_count = len(self.entries)
        entry_headers_size = file_count * 24
        names_start = 32 + entry_headers_size

        names_blob = bytearray()
        name_offsets = []
        for e in self.entries:
            name_offsets.append(names_start + len(names_blob))
            names_blob += e.name.encode('shift_jis', errors='replace') + b'\x00'

        names_end = names_start + len(names_blob)
        data_start = names_end
        if data_start % 16 != 0:
            pad = 16 - (data_start % 16)
            names_blob += b'\x00' * pad
            data_start += pad

        file_data_offsets = []
        files_blob = bytearray()
        for e in self.entries:
            curr_off = data_start + len(files_blob)
            if curr_off % 16 != 0:
                pad = 16 - (curr_off % 16)
                files_blob += b'\x00' * pad
                curr_off += pad
            file_data_offsets.append(curr_off)
            files_blob += e.data

        if self.raw_header and len(self.raw_header) >= 32:
            bnd_hdr = bytearray(self.raw_header[:32])
        else:
            bnd_hdr = bytearray(32)
            struct.pack_into('<4s8sIIIQ', bnd_hdr, 0, self.magic, self.version, self.format_flags, file_count, data_start, 0)
        
        struct.pack_into('<I', bnd_hdr, 16, file_count)
        struct.pack_into('<I', bnd_hdr, 20, data_start)

        entry_headers = bytearray()
        for i, e in enumerate(self.entries):
            d_len = len(e.data)
            entry_headers += struct.pack('<IIIIII', e.flags, d_len, file_data_offsets[i], e.entry_id, name_offsets[i], d_len)

        return bytes(bnd_hdr) + bytes(entry_headers) + bytes(names_blob) + bytes(files_blob)

class FMGParser:
    """Parser para arquivos de texto FMG (DSR/FromSoftware)"""
    @staticmethod
    def parse(data: bytes) -> Dict[int, str]:
        if len(data) < 28:
            return {}
        magic, unk1, ver, unk2, total_size, one, range_cnt, str_cnt, off_tbl = struct.unpack_from('<BBBBIIIII', data, 0)
        if off_tbl > len(data):
            return {}

        entries = {}
        for r_idx in range(range_cnt):
            r_off = 28 + r_idx * 12
            if r_off + 12 > len(data):
                break
            first_idx, first_id, last_id = struct.unpack_from('<iii', data, r_off)
            count = last_id - first_id + 1
            for i in range(count):
                tid = first_id + i
                s_idx = first_idx + i
                pos = off_tbl + s_idx * 4
                if pos + 4 > len(data):
                    continue
                str_off = struct.unpack_from('<I', data, pos)[0]
                if str_off == 0 or str_off >= len(data):
                    entries[tid] = ''
                else:
                    null_idx = str_off
                    while null_idx + 1 < len(data):
                        if data[null_idx:null_idx+2] == b'\x00\x00':
                            break
                        null_idx += 2
                    entries[tid] = data[str_off:null_idx].decode('utf-16le', errors='replace')
        return entries
