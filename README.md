<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="Shoclaw Browser Logo" />
</p>

<h1 align="center">Shoclaw</h1>

<p align="center">
  <strong>Ultra-lightweight, customizable, and modern minimalist web browser built in Rust.</strong><br>
  Engineered for developers who run heavy environments alongside dozens of tabs without memory thrashing.
</p>

<p align="center">
  <a href="#overview">Overview</a> &bull;
  <a href="#core-features">Core Features</a> &bull;
  <a href="#architecture">Architecture</a> &bull;
  <a href="#quickstart">Quickstart</a> &bull;
  <a href="#configuration">Configuration</a> &bull;
  <a href="#documentation">Documentation</a>
</p>

---

## Overview

Modern web browsers have grown into monolithic resource hogs. Running an IDE, Docker containers, language servers, and compiler toolchains alongside 30 to 50 open tabs in Chromium or Gecko-based browsers readily consumes 4 GB to 8 GB+ of RAM, causing system paging and CPU throttling.

**Shoclaw** solves this problem by pairing a compiled, native Rust core with system-level WebKit rendering and an uncompromising **Aggressive Tab Hibernation Engine**. Inactive tabs beyond a configurable limit are completely dropped from RAM while preserving metadata, keeping memory usage constant regardless of tab count.

### Design Principles
* **Zero Electron:** Native Rust binary with direct OS WebKit bindings. No Node.js runtime, no V8 wrapper overhead.
* **Instant Startup:** Cold launch time below 100 milliseconds.
* **Full-Bleed Web Canvas:** Screen area dedicated to web content; no cluttered toolbars or thick titlebars.
* **Aggressive Memory Guard:** Only active tabs occupy RAM. Sleeping tabs consume less than 5 KB of metadata.
* **100% Web Compatibility:** Hardware-accelerated rendering capable of playing 4K video, complex SPAs, and streaming media via WebKit.

---

## Core Features

### 1. Aggressive Tab Hibernation Engine
* **LRU Tab Disposal:** Only a user-defined number of tabs (`max_active_tabs`, default: 3) stay alive in memory.
* **Zero Inactive RAM:** Background tabs beyond the limit have their underlying WebKit instances destroyed.
* **Instant Wake-Up:** Clicking or switching to a sleeping tab instantly respawns its view from local disk cache with preserved scroll state.
* **Audio Playback Guard:** Any background tab actively streaming audio or video is strictly exempted from hibernation.

### 2. Hybrid UI System
Adaptable interface modes configurable dynamically or via keyboard shortcuts:
* **Hidden (Zen Maximalist):** 100% web content canvas. Window chrome and toolbars are hidden.
* **Autohide (Smart Edge):** The vertical sidebar smoothly slides in when the cursor touches the screen's left edge and collapses when moving away.
* **Compact (Icon Only):** A 48px fixed vertical strip displaying only tab favicons and status badges.
* **Expanded (Full Pinned):** A 220px fixed vertical sidebar displaying tab favicons, page titles, audio indicators, and sleep status.

### 3. Floating Command Palette (Raycast / Spotlight Omnibar)
* Triggered via global keybinding (`Ctrl+K` or `Ctrl+T`).
* Center-modal overlay with clean dark styling and background blur.
* Supports fuzzy searching across open tabs, direct URL navigation, search engine queries, and internal action commands (e.g. `> sleep all`).

### 4. Native Content Shield
* Network-level tracker and advertisement blocker powered by the Rust `adblock` engine.
* Drops unwanted requests before WebKit allocates DOM trees or downloads scripts, reducing memory consumption by 30% to 50%.

---

## Architecture

Shoclaw uses a dual-layer architecture separating native process management from web rendering:

```
+--------------------------------------------------------------+
| Shoclaw Host (Rust Native Application)                       |
|   - Config Engine (serde / toml)                             |
|   - Event Loop (tao)                                         |
|   - Tab & Memory Manager (LRU Hibernation)                   |
|   - Network Filter (adblock-rs)                              |
+------------------------------+-------------------------------+
                               |
               +---------------+---------------+
               |                               |
               v                               v
+------------------------------+ +-----------------------------+
| UI Overlay (WebKitGTK)       | | Content Viewports           |
|   - Floating Command Palette | |   - Active WebKit instance  |
|   - Vertical Tab Strip       | |   - Hardware accelerated    |
|   - Dynamic Viewport Bounds  | |   - Direct GPU rendering    |
+------------------------------+ +-----------------------------+
```

| Layer | Component | Description |
| :--- | :--- | :--- |
| **Language** | Rust 2021 Edition | Zero-cost abstractions, memory safety, native performance. |
| **Windowing** | `tao` | Cross-platform window and event loop management. |
| **Rendering** | `wry` (WebKitGTK 4.1) | OS-native webview bindings with hardware video acceleration. |
| **Config** | `serde` + `toml` | Human-readable configuration with automatic fallback defaults. |

---

## Quickstart

### Prerequisites (NixOS / Nix Flakes)
Shoclaw includes a self-contained `flake.nix` that provisions all required system libraries (`webkitgtk_4_1`, `gtk3`, `glib`, `libsoup_3`, `pkg-config`, `openssl`).

1. **Clone the repository:**
   ```bash
   git clone https://github.com/ryhndastra/shoclaw.git
   cd shoclaw
   ```

2. **Enter the development environment:**
   ```bash
   nix develop
   ```

3. **Run unit tests:**
   ```bash
   cargo test
   ```

4. **Build and launch:**
   ```bash
   cargo run --release
   ```

---

## Configuration

Shoclaw reads configuration from `~/.config/shoclaw/config.toml`. If absent, defaults are loaded automatically.

```toml
[general]
# Options: "hidden", "autohide", "compact", "expanded"
sidebar_mode = "autohide"
homepage = "https://duckduckgo.com"
search_engine = "https://duckduckgo.com/?q={}"

[performance]
# Maximum tabs kept alive in RAM simultaneously
max_active_tabs = 3
# Inactive threshold in minutes before hibernation
hibernate_after_minutes = 10

[appearance]
theme = "dark"
font = "Inter"
border_radius = "12px"
blur_effect = true

[keybindings]
command_palette = "Ctrl+K"
new_tab = "Ctrl+T"
close_tab = "Ctrl+W"
toggle_sidebar = "Ctrl+B"
next_tab = "Ctrl+Tab"
prev_tab = "Ctrl+Shift+Tab"
```

---

## Documentation

Detailed architectural and requirements documentation is available in the `docs/` directory:
* [Product Requirements Document (PRD)](docs/PRD.md): Detailed specifications, personas, and metrics.
* [System Architecture](docs/ARCHITECTURE.md): Component diagrams, state machine, and IPC protocol.
* [Master Tasklist & Roadmap](docs/TASKLIST.md): Granular implementation progress across all 8 development phases.

---

## License

Dual-licensed under either Apache License, Version 2.0 or MIT license at your option.
