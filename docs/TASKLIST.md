# Master Tasklist & Implementation Roadmap

## Project: Shoclaw Browser
**Current Phase:** Phase 1 (Foundation & Environment)  
**Status Key:**
- `[ ]` Pending
- `[-]` In Progress
- `[x]` Completed

---

### Phase 1: Environment & Tooling Setup
- [x] Create project repository structure and documentation folder
- [x] Author comprehensive Product Requirements Document (`docs/PRD.md`)
- [x] Author Technical Architecture Document (`docs/ARCHITECTURE.md`)
- [x] Author Master Tasklist & Roadmap (`docs/TASKLIST.md`)
- [x] Initialize living changelog and session tracker (`session.md`)
- [x] Configure `flake.nix` with required C/System dependencies (`pkg-config`, `glib`, `gtk3`, `webkitgtk_4_1`, `openssl`, `libsoup_3`)
- [x] Initialize Rust Cargo workspace and `Cargo.toml` with pinned dependencies (`tao`, `wry`, `serde`, `toml`)
- [x] Verify clean build inside Nix development shell (`nix develop --command cargo check`)

---

### Phase 2: Configuration & Data Modeling
- [x] Define `Config` data structures (`general`, `performance`, `appearance`, `keybindings`)
- [x] Implement TOML config loader with automatic default fallback (`~/.config/shoclaw/config.toml`)
- [x] Define `Tab` struct with lifecycle states (`Active`, `Idle`, `Sleeping`)
- [x] Implement `TabManager` skeleton (tab collection, active index pointer, LRU tracking)
- [x] Write unit tests for config parsing and tab state transitions

---

### Phase 3: Core Window & WebKit Runtime
- [x] Implement `tao` native window initialization with custom window decorations
- [x] Integrate `wry` WebKit runtime attached to the native window
- [x] Configure WebKit preferences (hardware acceleration, media codecs, cookies persistence)
- [x] Handle window resize events and dynamic viewport bounds
- [x] Build basic navigation controls (load URL, back, forward, reload)

---

### Phase 4: Aggressive Tab Hibernation Engine
- [ ] Implement `max_active_tabs` enforcement in `TabManager`
- [ ] Implement WebKit teardown (`drop(webview)`) on tab hibernation
- [ ] Implement instant WebKit reconstruction on wake-from-sleep
- [ ] Implement Audio Guard: detect media playback to prevent active streaming tabs from sleeping
- [ ] Measure and benchmark RAM consumption across 10, 20, and 50 open tabs

---

### Phase 5: Hybrid UI & Shoclaw Startpage
- [ ] Design and embed local "Shoclaw Startpage" (embedded zero-latency HTML/CSS/JS with logo, digital clock, search bar, and shortcuts)
- [ ] Design sleek, dark-themed HTML/CSS/JS assets for the sidebar (embedded via `include_str!`)
- [ ] Implement Mode `expanded` (220px fixed vertical sidebar with tab titles and status)
- [ ] Implement Mode `compact` (48px fixed icon-only vertical sidebar)
- [ ] Implement Mode `hidden` (100% full-bleed web canvas)
- [ ] Implement Mode `autohide` (mouse edge hover trigger for slide-in sidebar)
- [ ] Implement IPC protocol for bidirectional communication between Rust host and UI shell

---

### Phase 6: Floating Command Palette (Raycast / Spotlight Omnibar)
- [ ] Design glassmorphic, centered modal UI for the Command Palette
- [ ] Implement global shortcut listener (`Ctrl+K` / `Ctrl+T`) to toggle the palette
- [ ] Implement fuzzy search filtering over open tabs
- [ ] Implement URL detection vs. default search engine query dispatcher
- [ ] Implement command execution mode (prefix `>` for internal browser actions)

---

### Phase 7: Content Shield & Adblocker
- [ ] Integrate `adblock` crate into the request interception pipeline
- [ ] Implement local EasyList / filter rule caching
- [ ] Block tracking beacons, advertising scripts, and telemetry requests
- [ ] Measure page load speed improvements and memory reductions

---

### Phase 8: Polish, Storage & Release
- [ ] Implement local SQLite database for search history and bookmarks
- [ ] Add session restore (reopen tabs from previous session in sleeping state)
- [ ] Package standalone release binary (`cargo build --release`)
- [ ] Comprehensive performance profiling and leak checks
