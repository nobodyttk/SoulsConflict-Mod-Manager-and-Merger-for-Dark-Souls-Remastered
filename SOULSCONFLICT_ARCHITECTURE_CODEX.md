# SOULSCONFLICT ARCHITECTURE CODEX & AGENT BLUEPRINT
**Complete Reverse-Engineering, Format Specification, and System Architecture Guide**
*Reference Implementation: Dark Souls Remastered (v2.1.2) — Blueprint for Dark Souls III (DS3) Adaptation*

---

## 1. Executive Summary & Core Philosophy

**SoulsConflict** is a high-performance, fully portable mod manager, diagnostic conflict analyzer, deep binary merger, and non-destructive virtual deployer for FromSoftware games.

### The Problem It Solves
Traditional mod managers (Vortex, Mod Organizer 2) treat game files as opaque black boxes. When two mods modify `GameParam.parambnd`, `event/m10_01_00_00.emevd.dcx`, or player animations `chr/c0000.anibnd.dcx`, naive tools simply overwrite one with the other. This results in:
1. **Broken Game Logic**: Overwriting quest scripts or parameter tables breaks gameplay balance and quests.
2. **Infinite Loading Freezes**: Engine crashes when conflicting moveset animation IDs or SFX particles are injected.
3. **Occlusion Culling Failures (World Tearing)**: Missing terrain, floating islands, and void skyboxes when custom map pieces mismatch occlusion portals (`.mcp`).
4. **Permanent Installation Corruption**: Destructive overwrites into the game directory requiring Steam file re-validation.

### SoulsConflict's Solution
SoulsConflict unpacks FromSoftware containers (`.dcx`, `.bnd3`, `.bnd4`), parses their internal tables down to individual rows, animation IDs, and event instructions, detects discrepancies, executes a mathematical **Smart Fusion**, and deploys the unified result non-destructively with automatic backup of original vanilla files.

---

## 2. Technology Stack & Runtime Architecture

```
+---------------------------------------------------------------------------------+
|                                 USER INTERFACE                                  |
|         Vanilla HTML5 + Modern CSS Design System + Vanilla JavaScript ES6+      |
|               (Zero external frameworks, ultra-responsive, zero bloat)          |
+---------------------------------------------------------------------------------+
                                      | HTTP REST / JSON API (localhost:PORT)
                                      v
+---------------------------------------------------------------------------------+
|                              RUST EMBEDDED SERVER                               |
|        Native Windows Subsystem (no cmd window) + Lightweight TCP Listener      |
|             Tao (Cross-platform windowing) + Wry (WebView2 Binding)             |
+---------------------------------------------------------------------------------+
          |                        |                        |
          v                        v                        v
+------------------+     +------------------+     +------------------+
|  Scanner Engine  |     |  Merger Engine   |     | Deployer Engine  |
|  (src/scanner.rs)|     |  (src/merger.rs) |     | (src/deployer.rs)|
+------------------+     +------------------+     +------------------+
          \                        |                        /
           v                       v                       v
+---------------------------------------------------------------------------------+
|                       FORMATS & CONTAINER REVERSE-ENGINEERING                   |
|       DCX (Zlib/Oodle) | BND3/BND4 | PARAM | EMEVD | MSB | FMG | TAE / ANIBND   |
+---------------------------------------------------------------------------------+
```

### Key Technical Decisions:
* **Language**: Rust 2024 edition.
* **Compilation**: Native Windows target (`x86_64-pc-windows-msvc`), LTO (Link-Time Optimization) enabled, `opt-level = 3`, `codegen-units = 1`, `strip = true`. Final binary is ~3.8 MB.
* **Frontend**: Embedded at compile time via `include_str!("../static/index.html")`, `CSS_CONTENT`, and `JS_CONTENT`, but read dynamically from disk if present for rapid iteration.
* **Storage**: Portability first. Configuration and state reside in local JSON manifests (`deployer_config.json`, `deployer_manifest.json`, `localStorage`).

---

## 3. FromSoftware Binary Formats Specification

### 3.1 DCX Compression Container
FromSoftware wraps almost all game files inside a `.dcx` container.

#### Structure in Dark Souls 1 (DCX_DFLT - Deflate):
1. **Header (4 bytes)**: `b"DCX\0"`
2. **FourCC Markers**:
   * `b"DCS\0"`: Holds 32-bit big-endian uncompressed size and compressed size.
   * `b"DCP\0DFLT"`: Declares Deflate algorithm and compression parameters.
   * `b"DCA\0"`: Marker followed by 4-byte header size (8), after which the raw zlib compressed stream immediately begins.
3. **Decompression**: Scan for `DCA\0`, jump +8 bytes, feed remaining slice to `flate2::read::ZlibDecoder`.
4. **Recompression**: Compress raw payload using best zlib compression, prepend standard 76-byte `DCX_DFLT` header with big-endian sizes.

#### Adaptation for Dark Souls 3 (DCX_KRAK - Oodle Kraken):
* DS3 uses `b"DCP\0KRAK"` with Oodle Kraken compression instead of standard zlib Deflate.
* Decompression requires binding `oo2core_6_win64.dll` (via `libloading`) or an Oodle Kraken decompression implementation.

---

### 3.2 BND Containers (Binders)

#### BND3 (Dark Souls 1 / Remastered):
* **Header (32 bytes)**:
  * `0x00 - 0x03`: Signature `b"BND3"`
  * `0x04 - 0x0B`: Version string (e.g. `b"07D7R6\0\0"`)
  * `0x0C - 0x0F`: Format flags (typically `0x74` in DS1)
  * `0x10 - 0x13`: File count (uint32 Little-Endian)
  * `0x14 - 0x17`: Data start offset (uint32 Little-Endian)
  * `0x18 - 0x1F`: Zero padding
* **File Header Records (24 bytes per entry)**:
  * `0x00`: Flags (uint32 LE)
  * `0x04`: Compressed size (uint32 LE)
  * `0x08`: Data offset (uint32 LE)
  * `0x0C`: Entry ID (uint32 LE)
  * `0x10`: File path string offset (uint32 LE)
  * `0x14`: Uncompressed size (uint32 LE)
* **Alignment**: File data payloads are strictly aligned to 16-byte boundaries (`data_start % 16 == 0`).

#### Adaptation for Dark Souls 3 (BND4):
* Signature is `b"BND4"`.
* Uses 64-bit integers for offsets and sizes instead of 32-bit.
* Entry headers are 36 bytes (or 32 bytes depending on flags).
* Paths are UTF-8 or UTF-16 strings in a dedicated string table.

---

### 3.3 PARAM (Parameter Tables) & GameParam.parambnd
`GameParam.parambnd` contains all gameplay numbers: weapon damage, armor stats, enemy HP, spell costs, loot tables, and shops.

#### DS1 Param Format:
* **Header (48 bytes)**:
  * `0x00 - 0x03`: Strings offset (boundary for row payloads)
  * `0x08 - 0x0B`: Param format flags & version
  * `0x0A - 0x0B` or `0x08 - 0x09`: Row count (uint16 LE)
  * `0x0C - 0x2F`: Type name strings and zero padding
* **Row Descriptor Table (12 bytes per row starting at 0x30)**:
  * `0x00 - 0x03`: Row ID (uint32 LE, e.g. Weapon ID `100000`)
  * `0x04 - 0x07`: Data offset (uint32 LE)
  * `0x08 - 0x0B`: Name string offset (uint32 LE)
* **Discrepancy Analysis in SoulsConflict**:
  * Instead of simply comparing timestamps or file sizes, SoulsConflict parses each row ID and maps its byte slice payload.
  * If Mod A modifies Row `1000` and Mod B modifies Row `2000`, SoulsConflict marks the file as **Mergeable** (not a hard conflict).
  * If both mods modify Row `1000` with different byte payloads, a true conflict exists, and the user's resolution strategy (Priority or Manual) determines the winning row.

#### Adaptation for Dark Souls 3:
* DS3 stores params in `Data0.bdt` or `regulation.bin` (BND4 archive).
* `PARAM` in DS3 uses 64-bit offsets and a 24-byte row descriptor (`id: u32, pad: u32, data_off: u64, name_off: u64`).

---

### 3.4 EMEVD (Event Scripts)
Found in `event/mXX_XX_XX_XX.emevd.dcx`. Controls boss fights, triggers, door openings, cutscenes, and quest states.

#### Format:
* **Header**: Starts with `b"EVD\0"`.
* Contains an Event Offset Table with 28-byte headers per event:
  * `0x00 - 0x03`: Event ID (uint32 LE, e.g. `11010500`)
  * `0x04 - 0x07`: Instruction count (uint32 LE)
  * `0x08 - 0x0B`: Instructions offset
  * `0x0C - 0x0F`: Parameter count
* **Merge Logic**:
  * If Mod A adds custom events for a new quest and Mod B adds custom events for an arena, their Event IDs are disjoint.
  * SoulsConflict identifies distinct Event IDs and reconciles them seamlessly into the merged script.

---

### 3.5 MSB (MapStudio Entity Placement)
Found in `map/MapStudio/mXX_XX_XX_XX.msb`. Controls where every enemy, NPC, bonfire, door, item pickup, and collision box spawns in 3D world space.

#### Format:
* Contains structured sections: `MODEL_PARAM_ST`, `EVENT_PARAM_ST`, `POINT_PARAM_ST`, `PARTS_PARAM_ST`.
* SoulsConflict locates `PARTS_PARAM_ST\0`, extracts entity names (at offset `+0x64`) and unique Entity IDs (at offset `+0xa0`).
* Non-colliding entity additions across multiple mods are detected and preserved.

---

### 3.6 ANIBND & TAE (TimeAct Animation Engines)
Found in `chr/c0000.anibnd.dcx` (player) or `chr/cXXXX.anibnd.dcx` (enemies/bosses).
* Contains internal `.tae` files (TimeAct Editor).
* Defines frame-by-frame hitbox active windows, invulnerability frames, sound cues, and animation IDs (e.g., custom rolling animations vs custom gesture animations).
* SoulsConflict checks internal TAE IDs to detect whether two animation mods can coexist or if they collide on the same moveset slot.

---

## 4. The Occlusion Culling Phenomenon: Critical Lesson & Discovery

During the integration of **Dark Souls Re-Remastered (DSRR)** with major overhauls (**Daughters of Ash**, **The Fading Flame**), a critical bug was discovered and solved:

### The Phenomenon:
When loading into Undead Burg with DSRR merged into Daughters of Ash, the game suffered from **terrain tearing**: background mountains vanished, castle walls disappeared, and the player saw floating terrain islands against an empty skybox.

### Root Cause Analysis:
1. Vanilla Undead Burg (`m10_01_00_00`) has **148** 3D model pieces (`.flver.dcx`).
2. DSRR split these into **569** smaller custom pieces to assign separate 4K texture materials.
3. DSRR generated custom **Occlusion Culling Portals (`.mcp`)**, **Connectivity Graphs (`.mcg`)**, and **Havok physics collisions (`.hkx`)** calibrated *strictly* to its own 569 pieces and registered *strictly* in DSRR's own `m10_01_00_00.msb`.
4. Overhaul mods (Daughters of Ash, The Fading Flame) supply their own `MSB` using vanilla's 148 piece IDs.
5. When SoulsConflict copied DSRR's loose `.flver.dcx` and `.mcp` files alongside Daughters of Ash's `MSB`, the camera culling engine read DSRR's portals, concluded that the mountains were behind an occluding cell, and **culled (hid) the entire world**!

### The Solution: The Dual Visual Layer Preset Engine:
SoulsConflict divides DSRR into two strict categories:
1. **Structural/Culling Layer (Must be bypassed when overhauls are present - 3,253 files)**:
   * `map/**/*.flver.dcx`, `map/**/*.flver`
   * `map/**/*.mcp` (Occlusion Portals)
   * `map/**/*.mcg` (Connectivity Graph)
   * `map/**/*.hkxbdt`, `map/**/*.hkxbhd` (Havok physics collision)
   * `map/**/*.nvmbnd.dcx`, `map/**/*.nvmdump` (AI Navmesh)
   * `map/**/*.arealoadlist` (Area loading lists)
   * `chr/**/*.anibnd.dcx` (216 player moveset animations that crash loading screens)
   * `sfx/**/*.ffxbnd.dcx` (Conflicting particle systems)
   * `event/`, `script/`, `param/`, `menu/`
2. **Pure Graphic Asset Layer (100% Active - 2,444 files)**:
   * `map/**/*.tpfbdt`, `map/**/*.tpfbhd`, `map/**/*.tpf.dcx` (100% of 4K environmental textures)
   * `parts/` (1,250 4K weapon, shield, and armor models)
   * `obj/` (781 3D clutter objects, doors, chests, trees, and props)
   * `chr/` (282 4K character, boss, and hollow diffuse/normal/specular textures)
   * `mtd/` (Material shaders)

**Result**: 100% solid, complete, and un-culled world geometry from the overhaul mod, fully rendered in 4K textures from DSRR.

---

## 5. The Deep Merging Intelligence: Multi-Mod Semantic Fusion

The core intellectual breakthrough of SoulsConflict is its **Multi-Level Semantic Fusion Engine**. Traditional tools operate at Level 1 (blind file copying). SoulsConflict operates simultaneously across three granular architectural levels:

```
+----------------------------------------------------------------------------------------------------+
|                                    GRANULAR MERGING HIERARCHY                                      |
+----------------------------------------------------------------------------------------------------+
| LEVEL 1: FILE LEVEL              | Non-overlapping file paths are copied directly into merged/     |
+----------------------------------+-----------------------------------------------------------------+
| LEVEL 2: BINDER CONTAINER LEVEL  | Multi-file containers (.bnd3, .bnd4) are unpacked; non-colliding|
|                                  | internal entries are combined into a single unified archive.    |
+----------------------------------+-----------------------------------------------------------------+
| LEVEL 3: DEEP RECORD / ROW LEVEL | Internal binary tables (.param, .emevd, .msb, .tae) are parsed |
|                                  | into discrete rows/IDs. Disjoint modifications are synthesized. |
+----------------------------------------------------------------------------------------------------+
```

### 5.1 Mathematical Principle: Non-Destructive Union of Disjoint Subsets
If two or three mods modify the same container file (e.g. `GameParam.parambnd`), naive tools declare an unresolvable collision. SoulsConflict analyzes the **mutation delta**:

Let $M_1, M_2, \dots, M_k$ be active mods modifying container $C$.
Let $R(M_i)$ be the set of entity IDs (PARAM row IDs, EMEVD event IDs, MSB entity IDs) altered or inserted by mod $M_i$.
1. If $R(M_1) \cap R(M_2) \cap \dots \cap R(M_k) = \emptyset$ (the modified sections/IDs are mutually disjoint):
   The files are classified as **Mergeable with 100% Mathematical Safety**. SoulsConflict creates a synthesized file:
   $$C_{\text{merged}} = C_{\text{base}} \cup \Delta(M_1) \cup \Delta(M_2) \dots \cup \Delta(M_k)$$
2. If $R(M_a) \cap R(M_b) \neq \emptyset$:
   A true collision exists on the intersecting IDs. SoulsConflict isolates only the disputed IDs and resolves them via Sequential Load Order Priority or User Manual Toggle, while automatically merging the non-disputed rows.

---

### 5.2 Real-World Multi-Mod Fusion Scenario (3+ Mods)

Consider three popular mods installed simultaneously:
* **Mod 1 (Weapon Durability & Rebalance)**: Modifies `EquipParamWeapon.param` (Row IDs `1000` to `1500`).
* **Mod 2 (Magic & Sorcery Expansion)**: Modifies `Magic.param` (Row IDs `200` to `350`) and adds new spell entries `5000+`.
* **Mod 3 (Armor & Rings Overhaul)**: Modifies `EquipParamProtector.param` (Row IDs `500` to `800`) and `EquipParamAccessory.param`.

#### What Happens in Other Tools:
Mod 3 overwrites `GameParam.parambnd`. Mods 1 and 2 are completely wiped out. Spells disappear and weapon durability reverts to vanilla.

#### What Happens in SoulsConflict:
1. **Unpack**: SoulsConflict decompresses the DCX container and parses the BND3 binder into memory.
2. **Table Separation**: It inspects each `.param` file inside the binder.
3. **Delta Extraction**:
   * It extracts Mod 1's weapon rows (Durability, Attack Power, Scaling).
   * It extracts Mod 2's spell rows and custom spell entries.
   * It extracts Mod 3's armor defense and poise values.
4. **Binary Offset Re-computation**:
   * It maps all row descriptors (`id: u32, data_offset: u32, name_offset: u32`).
   * It recalculates the string table boundary and re-indexes all pointer offsets.
   * It aligns all file records to strict 16-byte boundaries (`data_start % 16 == 0`).
5. **Re-pack & Re-compress**:
   * Generates a brand-new valid BND3 binder.
   * Compresses with zlib Deflate and prepends the 76-byte `DCX_DFLT` header (`DCS`, `DCP`, `DCA`).
6. **Result**: A single, clean `GameParam.parambnd` containing all three mods working in total harmony in-game.

---

### 5.3 Cohesion Logic & Semantic Safety Guardrails

A smart merger must know not only *how* to merge bytes, but *when* a merge makes logical game sense. SoulsConflict enforces semantic guardrails:

#### 1. Overhaul vs. Overhaul Incompatibility Detection:
* If two massive gameplay overhauls (e.g. *Daughters of Ash* and *The Fading Flame*) both attempt to modify Event ID `11010000` (which dictates the Taurus Demon boss trigger and main progression flags), fusing these two scripts would create broken cutscene loops or soft-locks.
* **Guardrail**: SoulsConflict detects structural storyline collision, flags the file as a **Critical Conflict**, and strictly enforces load order priority (Mod #1 wins the file completely), preventing script corruption.

#### 2. Overhaul + Complementary Mod Synergy:
* If an overhaul mod (*Daughters of Ash*) is paired with an animation mod (*Better Rolling*) and a gesture mod (*Blessed Gestures*):
* **Guardrail**: The engine checks the internal TAE (TimeAct) tables inside `c0000.anibnd.dcx`. Because dodge rolls use TAE IDs `0..15` and gestures use TAE IDs `50..80`, the engine recognizes that they operate in distinct animation sub-sessions. Both mods are preserved without conflict.

#### 3. Visual Layer Protection (The DSRR Heuristic):
* If a 4K texture overhaul (*Dark Souls Re-Remastered*) is detected alongside a gameplay overhaul, the engine automatically distinguishes between:
  * Pure 2D visual assets (`.tpfbdt`, `.tpfbhd`, `.tpf.dcx`, weapons/armors in `parts/`, props in `obj/`).
  * World layout and camera occlusion portals (`.flver.dcx`, `.mcp`, `.mcg`, `.hkxbdt`, `MSB`).
* **Guardrail**: The engine automatically suggests or applies the **Strict Visual Preset**, preventing DSRR from injecting mismatched occlusion portals that cull the overhaul mod's world.

---

## 6. SoulsConflict Subsystem Architecture

### 6.1 Scanner Engine (`src/scanner.rs`)
* **Load Order Hierarchy**: Mods are ordered top-to-bottom. Index #1 is the highest priority.
* **Triage Categories**:
  * `Safe`: File exists in only 1 active mod, or has all secondary copies disabled via toggles.
  * `Mergeable`: File exists in >1 mod, but binary parsers confirm distinct non-overlapping IDs (distinct PARAM rows or distinct EMEVD events).
  * `Conflict`: File exists in >1 mod and contains overlapping IDs or unmergeable binary collisions.
* **Variant & Nested Folder Auto-Detection**:
  * Automatically inspects downloaded mod archives for sub-installations (e.g. `data/dsr` vs `data/ptde` in Daughters of Ash).
  * Canonicalizes paths on the fly without requiring the user to manually restructure folders.
* **Intent Analysis Engine**:
  * Classifies mods into tags: `Visual Layer`, `Gameplay Overhaul`, `Audio`, `Menu/UI`, `Animations`.

### 6.2 Merger Engine (`src/merger.rs`)
* Cleans previous files in `merged/` to prevent stale artifacts from removed mods.
* Respects user-defined per-file toggle states (`disabled_files`).
* Executes resolution modes:
  * **Smart Fusion**: Automatically combines unique PARAM rows, EMEVD events, and MSB parts.
  * **Sequential Priority**: Highest-priority mod wins individual file collisions.
  * **Manual Selection**: Uses user-specified per-file winning mod selections.

### 6.3 Deployer & Virtual Injection Engine (`src/deployer.rs`)
* **Non-Destructive Principle**: Never overwrite original files without a backup.
* **Manifest Tracking**: Writes `deployer_manifest.json` recording every deployed file and whether an original vanilla file was backed up into `vanilla_backup/`.
* **Instant Revert**: "Restore Vanilla" replaces all backed-up files and removes all injected mod files in under 1 second.
* **Process Detection**: Checks `tasklist` for `DarkSoulsRemastered.exe` before any disk mutation to prevent filesystem corruption while the game runs.
* **Multi-Mode Launch**:
  * Steam URI: `steam://run/570940`
  * Direct Executable: Spawns `DarkSoulsRemastered.exe`
  * Seamless Co-op: Spawns `launch_dark_souls_remastered_seamless_coop.exe`

### 6.4 Archive Importer (`src/archive.rs`)
* Supports `.zip`, `.rar`, and `.7z` directly via drag-and-drop.
* Sanitizes malicious relative paths (`..`, absolute drive roots).
* Unpacks directly into `mods/<ModName>/` and triggers automatic canonical path detection.

### 6.5 Native TPUP Texture Override & Merge Engine (`src/formats.rs`, `src/scanner.rs`, `src/merger.rs`)
* **Problem Solved**: Dark Souls Remastered does not load loose `.dds` textures from disk; textures must reside inside `.tpf` or `.tpf.dcx` container archives. Previously, players had to use an external .NET application (DSR-TPUP) to manually unpack game files and inject textures, breaking whenever multiple texture mods modified the same container.
* **TPF Binary Engine (`src/formats.rs`)**:
  * Parses binary TPF headers (16 bytes: `TPF\0`, total data size, texture count, platform flag `0x20300`).
  * Extracts name table (16-byte aligned null-terminated strings) and raw DDS data buffers.
  * Injects texture overrides dynamically in memory (`inject_texture`), preserving all unmodified vanilla textures.
  * Re-serializes the binary TPF container and compresses with `compress_dcx`.
* **Smart Texture Container Resolution (`src/scanner.rs`)**:
  * Scans loose textures structured as `<folder>/<archive_stem>/<texture>.dds` (e.g. `menu/menu_2/Logo_02.dds`, `menu/menu_3/Icon50.dds`).
  * Maps loose overrides to target game containers (e.g. `menu/menu_2.tpf.dcx`, `menu/menu_3.tpf.dcx`).
  * Detects TPUP texture mods automatically and renders a distinctive golden badge (`TEXTURAS (TPUP)`) in the mod cards.
  * Evaluates multi-mod texture conflicts:
    * Distinct texture names across mods -> **Mergeable** (🟡) with detailed sub-item reports.
    * Duplicate texture collisions -> **Conflict** (🔴), resolved by Load Order priority.
* **Automated Injection & Fusion (`src/merger.rs`)**:
  * Reads the base TPF container from preceding mods, `vanilla_backup/`, or detected game installation.
  * Injects all active DDS overrides in priority order into the container.
  * Writes the repacked `.tpf.dcx` into `merged/`, ready for non-destructive deployment without any external tools.

---

## 7. Blueprint for Dark Souls III (DS3) Adaptation

To build the corresponding **SoulsConflict DS3**, an agent must implement the following adaptations:

### 7.1 Format Evolution Table
| Feature / Format | Dark Souls 1 / Remastered | Dark Souls III (DS3) |
| :--- | :--- | :--- |
| **Executable Name** | `DarkSoulsRemastered.exe` | `DarkSoulsIII.exe` |
| **Steam AppID** | `570940` | `374320` |
| **Binder Format** | `BND3` (32-bit offsets) | `BND4` (64-bit offsets, unicode names) |
| **DCX Compression** | Deflate (`DCX_DFLT`, zlib) | Oodle Kraken (`DCX_KRAK`) |
| **Regulation File** | Loose `param/GameParam/*.param` inside BND3 | `Data0.bdt` or encrypted `regulation.bin` |
| **Map Engine** | `MSB1` (32-bit `PARTS_PARAM_ST`) | `MSB3` (64-bit sections) |
| **Event Engine** | `EMEVD` (32-bit EVD0) | `FDP` (64-bit EMEVD) |
| **Animation Container** | `anibnd.dcx` (32-bit TAE) | `anibnd.dcx` (64-bit TAE) |

### 6.2 DS3 Step-by-Step Implementation Strategy:
1. **Oodle Decompression Integration**:
   * Implement a Rust wrapper around `oo2core_6_win64.dll` to handle `DCX_KRAK`.
2. **BND4 Parser & Builder**:
   * Implement `Bnd4Archive` with 64-bit offsets and alignment rules.
3. **Regulation.bin Decryption / Repacking**:
   * DS3 `regulation.bin` is an AES-128-CBC encrypted BND4 archive (Key: `0x2C, 0x43, 0xF9, ...`).
   * Decrypt to memory, parse internal `PARAM` files, execute smart row merge, re-encrypt, and save.
4. **MSB3 & FDP Parsers**:
   * Adapt entity extraction and event ID scanners to 64-bit structs.
5. **Porting UI & Deployer**:
   * Reuse the complete SoulsConflict UI design system, translations, preset engine, and non-destructive deployer architecture.

---
*SoulsConflict Codex — Maintained by nobodyttk (2026)*
