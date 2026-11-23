# Product Requirements Document (PRD)

## Project: Shoclaw Browser
**Version:** 0.1.0-draft  
**Author:** sho & AI Engineering Assistant  
**Status:** In Inception / Architectural Design  
**Target Platform:** Linux (NixOS / Wayland / X11 first-class), cross-platform extensible (macOS, Windows)

---

## 1. Executive Summary & Vision

### 1.1 The Core Problem
Modern web browsers (Google Chrome, Mozilla Firefox, Brave, and newer forks like Zen Browser) have become monolithic and resource-intensive. When used by developers who run resource-heavy development environments (IDEs, Docker, Rust/Go/Node compilers, language servers) alongside 20–50+ open browser tabs, system RAM quickly exceeds 16GB–32GB, causing CPU throttling, paging/swapping, and sluggish workflows.

Current browser pain points:
1. **Excessive Memory Usage:** Opening 30+ tabs in Chromium or Gecko-based browsers readily consumes 4GB to 8GB+ of RAM because inactive tabs retain large DOM trees, script heaps, and media buffers in memory.
2. **Aggressive Vendor Bloat:** Mainstream browsers ship with AI sidebars, shopping assistants, tracking telemetry, and bloated user interfaces that alone consume hundreds of megabytes before opening a single page.
3. **Electron Disgust:** Electron-based browser experiments bundle entire Node.js runtimes and custom Chromium builds, resulting in slow startup times (>1.5s) and massive baseline RAM usage (~200MB+ for empty windows).
4. **Cluttered, Rigid UI:** Traditional multi-row tab bars and thick titlebars consume valuable vertical screen real estate, leaving less room for code and reading materials.

### 1.2 The Vision
**Shoclaw** is an ultra-lightweight, blazing-fast, and deeply customizable web browser built in native **Rust**. It combines a **Full-Bleed Web Canvas** with an adaptable **Hybrid UI** (collapsible vertical sidebar + floating Raycast-style command palette) and a ruthless **Tab Hibernation Engine**. 

Shoclaw operates under the principle that:
> *"The browser should disappear when you are reading, respond instantly when summoned, and never steal RAM from your development environment."*

---

## 2. Target Audience & Persona

* **Primary Persona:** Software engineers, developers, and power users who multitask heavily, keep dozens of documentation and research tabs open, and care about system performance and battery life.
* **Aesthetic & Workflow Preference:** Minimalist, keyboard-centric, distraction-free (reminiscent of Arc, Zen, Raycast, and modern tiling window managers like Hyprland / i3 / Sway).
* **Technical Stance:** Prefers config-file driven customization (TOML/JSON), dislikes telemetry and vendor lock-in, values native binaries over web wrappers.

---

## 3. Product Goals & Core Pillars

| Pillar | Principle | Target Metric |
| :--- | :--- | :--- |
| **Pillar 1: Extreme Lightness** | Zero vendor bloat; native Rust binary; aggressive memory management. | Idle RAM: < 150MB; 50 tabs open: < 600MB. |
| **Pillar 2: Instant Speed** | No Electron; no JS-heavy browser shell; compiled native code. | Cold startup time: < 100ms. UI interactions: 60/120fps. |
| **Pillar 3: Uncompromised Compatibility** | 100% web compatibility with modern standards, streaming video, and codecs. | Plays YouTube 4K, Google Docs, Figma without rendering issues. |
| **Pillar 4: Hybrid Minimalist Elegance** | 98% screen area for web content; customizable sidebar + spotlight command bar. | Distraction-free, zero visual noise when browsing. |
| **Pillar 5: User-Centric Customizability** | Configurable via `~/.config/shoclaw/config.toml` to suit any user workflow. | Keybindings, UI modes, tab thresholds, and search engines 100% editable. |

---

## 4. Feature Specifications

### 4.1 Tab Management & Aggressive Hibernation Engine
* **Max Active Tabs Limit (`max_active_tabs`):** Configurable ceiling (default: 3 tabs). When a user opens a 4th tab, the least-recently-used (LRU) inactive tab is automatically hibernated.
* **Hibernation Mechanics:**
  * When a tab hibernates, its underlying `WebView` process instance is destroyed (`drop(webview)`), releasing DOM trees and JS heaps from RAM.
  * The tab's metadata (URL, Page Title, Favicon, Scroll Coordinates, History stack) is retained in an ultra-lightweight Rust `Tab` struct (< 5 KB RAM per tab).
  * Clicking or switching to a sleeping tab instantly respawns the WebKit instance and restores the page from local disk cache.
* **Audio Guard:** Any tab actively playing audio/video is exempt from hibernation until media playback stops.

### 4.2 The Hybrid UI System
The user interface supports 4 interchangeable modes, switchable via configuration or keyboard shortcut:
1. **Mode `hidden` (Zen/Maximalist):**
   * Screen is 100% web content. No window chrome, no tab strip, no address bar.
   * All navigation is performed through the floating Command Palette (`Ctrl+K`).
2. **Mode `autohide` (Smart Edge):**
   * Default state is full-screen web.
   * Moving the cursor to the left edge of the screen reveals a sleek, smooth slide-in vertical sidebar.
   * Moving the mouse away auto-collapses the sidebar.
3. **Mode `compact` (Icon Only):**
   * Fixed ultra-slim sidebar (48px width) showing only tab favicons and status badges.
4. **Mode `expanded` (Full Pinned):**
   * Fixed sidebar (220px width) showing favicon, tab titles, audio indicators, and sleeping/active badges.

### 4.3 Floating Command Palette (Raycast / Spotlight Omnibar)
* Triggered via global keybinding (`Ctrl+K` or `Ctrl+T` or custom).
* **Modal Overlay:** Appears centered on top of the web canvas with a modern dark glassmorphic design, subtle border glow, and clean typography.
* **Omnibar Capabilities:**
  * **Direct URL:** Entering a valid URL (e.g. `github.com` or `localhost:3000`) navigates immediately.
  * **Search Engine:** Entering text defaults to querying the configured search engine (e.g., DuckDuckGo, Google, Brave).
  * **Fuzzy Tab Switcher:** Typing any keyword fuzzy-matches open tab titles and URLs, allowing single-keystroke tab switching without touching the mouse.
  * **Command Mode (Prefix `>`):**
    * `> sleep all` : Manually forces all background tabs to hibernate.
    * `> sidebar [mode]` : Dynamically switches UI sidebar mode.
    * `> clear cache` : Cleans temporary browsing cache.
    * `> bookmark add` : Saves current URL to bookmarks.

### 4.4 Rust-Native Content Blocking & Shield
* Utilizes Rust-native adblocking logic (Brave's `adblock-rs` engine).
* Request filtering occurs at the network interception layer before WebKit allocates DOM or parses scripts.
* Blocks tracking scripts, banner ads, and crypto-miners, reducing page weight by 30%–50%.

### 4.5 Configuration Engine (`config.toml`)
* Located at `~/.config/shoclaw/config.toml`.
* Hot-reloaded or loaded on startup.
* Fallback defaults provided out-of-the-box if file is missing.

---

## 5. Non-Functional Requirements

### 5.1 Performance
* **Cold Launch Time:** $\le 100\text{ ms}$ on SSD / modern Linux system.
* **Baseline Memory (Single Tab):** $\le 150\text{ MB}$.
* **Multi-Tab Memory (50 Tabs with Hibernation):** $\le 500\text{ MB}$.
* **Frame Rate:** 60fps / 120fps smooth animations for UI transitions and scrolling.

### 5.2 Security & Privacy
* **Zero Telemetry:** No analytical events, user tracking, or background pings to remote servers.
* **Sandboxed WebViews:** Strict separation between WebKit page processes and the native Rust host application.
* **Local Storage Only:** Bookmarks, history, and user settings stored strictly on the local filesystem.

---

## 6. Success Metrics & Criteria
* [x] The browser compiles into a single standalone binary.
* [x] Cold start is perceptual instant (< 100ms).
* [x] Memory usage with 30 tabs open remains under 500MB.
* [x] Capable of playing 4K 60fps YouTube videos without stutter or frame drops.
* [x] Hybrid UI transitions seamlessly between hidden, autohide, compact, and expanded states.
