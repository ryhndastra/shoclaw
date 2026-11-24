# Master Tasklist & Implementation Roadmap

## Project: Shoclaw Browser
**Current Phase:** Phase 7 (Content Shield & Adblocker)  
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
- [x] Implement `max_active_tabs` enforcement in `TabManager`
- [x] Implement WebKit teardown (`drop(webview)`) on tab hibernation
- [x] Implement instant WebKit reconstruction on wake-from-sleep
- [x] Implement Audio Guard: detect media playback to prevent active streaming tabs from sleeping
- [x] Measure and benchmark RAM consumption across 10, 20, and 50 open tabs

---

### Phase 5: Hybrid UI & Shoclaw Startpage
- [x] Design and embed local "Shoclaw Startpage" (`assets/shoclaw_startpage.html`) with embedded claw emblem, digital clock, search engine switcher, and quick shortcuts (pure HTML/CSS, zero external CDN frameworks)
- [x] Design sleek, dark-themed 250px vertical sidebar (`assets/shoclaw_sidebar_250px.html`) with dynamic tab list, active/idle/sleeping (0 MB) status badges, audio indicators, and navigation controls
- [x] Implement Mode `expanded` (250px fixed vertical sidebar with tab titles and status)
- [x] Implement Mode `compact` (48px fixed icon-only vertical sidebar)
- [x] Implement Mode `hidden` (100% full-bleed web canvas)
- [x] Implement Mode `autohide` (mouse edge hover trigger for slide-in sidebar)
- [x] Implement IPC protocol (`AppEvent`) for bidirectional communication between Rust host and UI shell (`new_tab`, `switch_tab`, `close_tab`, `toggle_sidebar`, `navigate`, `go_back`, `go_forward`, `reload`, `audio_state`, `title_changed`)

---

### Phase 6: Floating Command Palette (Raycast / Spotlight Omnibar)
- [x] Design glassmorphic, centered modal UI for the Command Palette (`assets/shoclaw_palette.html`)
- [x] Implement global shortcut listener (`Ctrl+K` / `Ctrl+T`) to toggle the palette across Tao window, sidebar, startpage, and content webviews
- [x] Implement fuzzy search filtering over open tabs (with real-time memory state badges: Active, Idle, 0 MB Sleeping)
- [x] Implement URL detection vs. default search engine query dispatcher
- [x] Implement command execution mode (prefix `>` for internal browser actions: `> sleep all`, `> sidebar expanded`, `> sidebar compact`, `> sidebar hidden`, `> new tab`, `> close tab`, `> reload`, `> back`, `> forward`, `> quit`)

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
