#!/usr/bin/env python3
"""
build_ui_assets.py
Generates pure, lightweight, framework-free HTML/CSS for:
1. assets/shoclaw_startpage.html
2. assets/shoclaw_sidebar_250px.html

Zero external JS frameworks (no Tailwind JIT runtime), zero external fonts,
hardware-friendly dark glass styling, and embedding the user's custom claw emblem.
"""

import os

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LOGO_B64_PATH = os.path.join(BASE_DIR, "assets", "logo_base64.txt")

with open(LOGO_B64_PATH, "r") as f:
    LOGO_B64 = f.read().strip()

LOGO_DATA_URI = f"data:image/png;base64,{LOGO_B64}"

STARTPAGE_HTML = f"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Shoclaw Start</title>
  <style>
    :root {{
      --bg-deep: #080B16;
      --bg-surface: #0E0E12;
      --surface-glass: rgba(255, 255, 255, 0.05);
      --surface-glass-hover: rgba(255, 255, 255, 0.09);
      --surface-glass-active: rgba(255, 255, 255, 0.12);
      --border-specular: rgba(255, 255, 255, 0.14);
      --border-subtle: rgba(255, 255, 255, 0.06);
      --text-main: #E7E4F0;
      --text-muted: #ACA9B5;
      --text-dim: #76747F;
      --accent: #C5C2EE;
      --accent-rgb: 197, 194, 238;
      --ease: cubic-bezier(0.2, 0, 0, 1);
    }}

    *, *::before, *::after {{
      box-sizing: border-box;
      margin: 0;
      padding: 0;
    }}

    html, body {{
      width: 100%;
      height: 100%;
      min-height: 100vh;
      background-color: var(--bg-deep);
      color: var(--text-main);
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Inter", "Helvetica Neue", Arial, sans-serif;
      font-size: 14px;
      line-height: 1.5;
      -webkit-font-smoothing: antialiased;
      overflow-x: hidden;
      overflow-y: auto;
      user-select: none;
      -webkit-user-select: none;
    }}

    .ambient-bg {{
      position: relative;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      padding: 24px 32px;
      background:
        radial-gradient(ellipse 75% 55% at 50% -10%, rgba(44, 43, 89, 0.55), transparent 70%),
        radial-gradient(ellipse 55% 45% at 15% 85%, rgba(23, 21, 67, 0.45), transparent 65%),
        radial-gradient(ellipse 50% 40% at 85% 75%, rgba(30, 34, 62, 0.4), transparent 65%),
        var(--bg-deep);
    }}

    /* Top bar */
    .top-bar {{
      width: 100%;
      max-width: 1100px;
      margin: 0 auto;
      display: flex;
      align-items: center;
      justify-content: space-between;
      z-index: 20;
    }}

    .clock-group {{
      display: flex;
      align-items: center;
      gap: 12px;
    }}

    .clock-time {{
      font-family: "JetBrains Mono", ui-monospace, SFMono-Regular, monospace;
      font-size: 16px;
      font-weight: 600;
      color: var(--text-main);
      letter-spacing: -0.3px;
    }}

    .clock-dot {{
      width: 4px;
      height: 4px;
      border-radius: 50%;
      background: rgba(255, 255, 255, 0.25);
    }}

    .clock-status {{
      font-size: 13.5px;
      color: var(--text-muted);
    }}

    /* Buttons & interactive primitives */
    button {{
      font: inherit;
      color: inherit;
      background: none;
      border: none;
      cursor: pointer;
    }}

    .glass-btn {{
      display: inline-flex;
      align-items: center;
      gap: 8px;
      padding: 6px 14px;
      border-radius: 9999px;
      background: var(--surface-glass);
      border: 1px solid var(--border-specular);
      color: var(--text-muted);
      font-size: 12.5px;
      font-weight: 500;
      transition: background-color 160ms var(--ease), color 160ms var(--ease), border-color 160ms var(--ease);
    }}

    .glass-btn:hover {{
      background: var(--surface-glass-hover);
      color: var(--text-main);
      border-color: rgba(var(--accent-rgb), 0.35);
    }}

    /* Quick settings popover */
    .popover-container {{
      position: relative;
    }}

    .popover {{
      display: none;
      position: absolute;
      right: 0;
      top: 100%;
      margin-top: 8px;
      width: 240px;
      padding: 16px;
      border-radius: 14px;
      background: #141722;
      border: 1px solid var(--border-specular);
      box-shadow: 0 16px 40px rgba(0, 0, 0, 0.5);
      flex-direction: column;
      gap: 12px;
      z-index: 50;
    }}

    .popover.is-open {{
      display: flex;
    }}

    .popover-row {{
      display: flex;
      align-items: center;
      justify-content: space-between;
      font-size: 12.5px;
    }}

    .popover-badge {{
      padding: 2px 8px;
      border-radius: 9999px;
      background: rgba(255, 255, 255, 0.08);
      border: 1px solid rgba(255, 255, 255, 0.12);
      font-family: monospace;
      font-size: 11px;
      color: var(--accent);
    }}

    /* Main Center Stack */
    .center-stack {{
      width: 100%;
      max-width: 680px;
      margin: auto auto;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 32px;
      padding: 32px 0;
    }}

    /* Brand Header */
    .brand-section {{
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 14px;
    }}

    .brand-logo-card {{
      width: 72px;
      height: 72px;
      border-radius: 18px;
      background: rgba(255, 255, 255, 0.04);
      border-top: 1px solid var(--border-specular);
      border-left: 1px solid var(--border-specular);
      border-right: 1px solid var(--border-subtle);
      border-bottom: 1px solid var(--border-subtle);
      display: flex;
      align-items: center;
      justify-content: center;
      box-shadow: 0 12px 30px rgba(0, 0, 0, 0.35);
      transition: transform 200ms var(--ease);
    }}

    .brand-logo-card:hover {{
      transform: translateY(-2px);
    }}

    .brand-logo-img {{
      width: 50px;
      height: 50px;
      object-fit: contain;
      filter: drop-shadow(0 4px 10px rgba(0, 0, 0, 0.5));
    }}

    .brand-wordmark {{
      font-size: 32px;
      font-weight: 700;
      color: var(--text-main);
      letter-spacing: -0.6px;
    }}

    /* Center Omnibar Search Bar */
    .search-section {{
      width: 100%;
    }}

    .search-pill {{
      position: relative;
      display: flex;
      align-items: center;
      gap: 10px;
      width: 100%;
      height: 52px;
      padding: 0 14px 0 10px;
      border-radius: 9999px;
      background: rgba(20, 23, 33, 0.88);
      border: 1px solid var(--border-specular);
      box-shadow: 0 16px 40px rgba(0, 0, 0, 0.4), inset 0 1px 1px rgba(255, 255, 255, 0.1);
      transition: border-color 180ms var(--ease), box-shadow 180ms var(--ease), transform 180ms var(--ease);
    }}

    .search-pill:focus-within {{
      border-color: var(--accent);
      box-shadow: 0 0 0 2px rgba(var(--accent-rgb), 0.25), 0 20px 48px rgba(0, 0, 0, 0.5);
      transform: translateY(-1px);
    }}

    .engine-btn {{
      width: 36px;
      height: 36px;
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      background: rgba(255, 255, 255, 0.06);
      border: 1px solid rgba(255, 255, 255, 0.08);
      color: var(--text-muted);
      flex-shrink: 0;
      transition: background-color 160ms var(--ease), color 160ms var(--ease);
    }}

    .engine-btn:hover {{
      background: rgba(255, 255, 255, 0.12);
      color: var(--text-main);
    }}

    .search-input {{
      flex: 1;
      height: 100%;
      background: transparent;
      border: none;
      outline: none;
      font-size: 15px;
      color: var(--text-main);
      user-select: text;
      -webkit-user-select: text;
    }}

    .search-input::placeholder {{
      color: rgba(172, 169, 181, 0.65);
    }}

    .kbd-shortcut {{
      font-family: "JetBrains Mono", ui-monospace, SFMono-Regular, monospace;
      font-size: 11px;
      padding: 3px 8px;
      border-radius: 6px;
      background: rgba(255, 255, 255, 0.06);
      border: 1px solid rgba(255, 255, 255, 0.1);
      color: var(--text-muted);
      user-select: none;
    }}

    /* Shortcuts Grid (2x4) */
    .shortcuts-section {{
      width: 100%;
    }}

    .shortcuts-grid {{
      display: grid;
      grid-template-columns: repeat(4, 1fr);
      gap: 14px;
      width: 100%;
    }}

    @media (max-width: 640px) {{
      .shortcuts-grid {{
        grid-template-columns: repeat(2, 1fr);
      }}
    }}

    .shortcut-card {{
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      gap: 10px;
      height: 104px;
      padding: 14px;
      border-radius: 16px;
      background: rgba(255, 255, 255, 0.04);
      border-top: 1px solid var(--border-specular);
      border-left: 1px solid var(--border-specular);
      border-right: 1px solid var(--border-subtle);
      border-bottom: 1px solid var(--border-subtle);
      color: var(--text-main);
      text-decoration: none;
      box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
      transition: transform 180ms var(--ease), background-color 180ms var(--ease), border-color 180ms var(--ease);
    }}

    .shortcut-card:hover {{
      transform: translateY(-2px);
      background: rgba(255, 255, 255, 0.08);
      border-color: rgba(var(--accent-rgb), 0.35);
    }}

    .shortcut-icon-box {{
      width: 40px;
      height: 40px;
      border-radius: 10px;
      display: flex;
      align-items: center;
      justify-content: center;
      background: rgba(255, 255, 255, 0.06);
      border: 1px solid rgba(255, 255, 255, 0.08);
      color: var(--text-main);
    }}

    .shortcut-label {{
      font-size: 12px;
      font-weight: 500;
      color: var(--text-main);
    }}

    /* Add Shortcut Modal */
    .modal-overlay {{
      display: none;
      position: fixed;
      inset: 0;
      background: rgba(0, 0, 0, 0.65);
      align-items: center;
      justify-content: center;
      z-index: 100;
    }}

    .modal-overlay.is-open {{
      display: flex;
    }}

    .modal-box {{
      width: 100%;
      max-width: 380px;
      padding: 24px;
      border-radius: 18px;
      background: #141722;
      border: 1px solid var(--border-specular);
      box-shadow: 0 24px 60px rgba(0, 0, 0, 0.6);
      display: flex;
      flex-direction: column;
      gap: 16px;
    }}

    .modal-title {{
      font-size: 16px;
      font-weight: 600;
    }}

    .modal-input {{
      width: 100%;
      height: 40px;
      padding: 0 12px;
      border-radius: 8px;
      background: rgba(255, 255, 255, 0.05);
      border: 1px solid var(--border-specular);
      color: var(--text-main);
      font-size: 13.5px;
      outline: none;
    }}

    .modal-input:focus {{
      border-color: var(--accent);
    }}

    .modal-actions {{
      display: flex;
      justify-content: flex-end;
      gap: 8px;
      margin-top: 8px;
    }}

    .btn-secondary {{
      padding: 8px 14px;
      border-radius: 8px;
      background: rgba(255, 255, 255, 0.06);
      color: var(--text-muted);
      font-size: 13px;
    }}

    .btn-primary {{
      padding: 8px 16px;
      border-radius: 8px;
      background: var(--accent);
      color: #0E0E12;
      font-size: 13px;
      font-weight: 600;
    }}

    /* Bottom footer */
    .footer-bar {{
      width: 100%;
      max-width: 1100px;
      margin: 0 auto;
      display: flex;
      align-items: center;
      justify-content: space-between;
      color: var(--text-dim);
      font-size: 12px;
      z-index: 10;
    }}
  </style>
</head>
<body>
  <div class="ambient-bg">
    <!-- Top Bar -->
    <header class="top-bar">
      <div class="clock-group">
        <time id="shoclaw-clock" class="clock-time">00:00</time>
        <span class="clock-dot"></span>
        <p id="shoclaw-status-text" class="clock-status">Loading...</p>
      </div>

      <div class="popover-container">
        <button id="quick-settings-btn" type="button" class="glass-btn" aria-label="Quick Settings">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"></path>
            <circle cx="12" cy="12" r="3"></circle>
          </svg>
          <span>Quick settings</span>
        </button>

        <div id="quick-settings-popover" class="popover">
          <div class="popover-row">
            <span>24-hour clock</span>
            <button id="toggle-24h-btn" type="button" class="popover-badge">ON</button>
          </div>
          <div class="popover-row">
            <span>Search engine</span>
            <span id="popover-engine-label" class="popover-badge">Google</span>
          </div>
        </div>
      </div>
    </header>

    <!-- Centerpiece -->
    <main class="center-stack">
      <!-- Brand Header -->
      <section class="brand-section">
        <div id="shoclaw-logo-slot" class="brand-logo-card">
          <img src="{LOGO_DATA_URI}" alt="Shoclaw" class="brand-logo-img" />
        </div>
        <h1 class="brand-wordmark">Shoclaw</h1>
      </section>

      <!-- Center Omnibar -->
      <section class="search-section">
        <form id="shoclaw-search-form" class="search-pill" role="search">
          <button id="engine-switcher-btn" type="button" class="engine-btn" title="Click to cycle search engine">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="11" cy="11" r="8"></circle>
              <path d="m21 21-4.3-4.3"></path>
            </svg>
          </button>
          <input id="shoclaw-search-input" class="search-input" type="search" placeholder="Search the web with Shoclaw or enter URL..." autocomplete="off" spellcheck="false" />
          <kbd class="kbd-shortcut">Ctrl + K</kbd>
        </form>
      </section>

      <!-- Quick Access Dock -->
      <section class="shortcuts-section">
        <div id="shortcuts-grid" class="shortcuts-grid">
          <!-- YouTube -->
          <a href="https://www.youtube.com" class="shortcut-card" data-url="https://www.youtube.com">
            <span class="shortcut-icon-box">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <rect x="2" y="5" width="20" height="14" rx="4"></rect>
                <polygon points="10 9 15 12 10 15 10 9" fill="currentColor"></polygon>
              </svg>
            </span>
            <span class="shortcut-label">YouTube</span>
          </a>

          <!-- GitHub -->
          <a href="https://github.com" class="shortcut-card" data-url="https://github.com">
            <span class="shortcut-icon-box">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"></path>
                <path d="M9 18c-4.51 2-5-2-7-2"></path>
              </svg>
            </span>
            <span class="shortcut-label">GitHub</span>
          </a>

          <!-- Reddit -->
          <a href="https://reddit.com" class="shortcut-card" data-url="https://reddit.com">
            <span class="shortcut-icon-box">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="9"></circle>
                <circle cx="9" cy="11.5" r="1.5" fill="currentColor"></circle>
                <circle cx="15" cy="11.5" r="1.5" fill="currentColor"></circle>
                <path d="M9 15.5c1 1 2.5 1.5 3 1.5s2-.5 3-1.5"></path>
              </svg>
            </span>
            <span class="shortcut-label">Reddit</span>
          </a>

          <!-- Google -->
          <a href="https://www.google.com" class="shortcut-card" data-url="https://www.google.com">
            <span class="shortcut-icon-box">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="11" cy="11" r="8"></circle>
                <path d="m21 21-4.3-4.3"></path>
              </svg>
            </span>
            <span class="shortcut-label">Google</span>
          </a>

          <!-- Wikipedia -->
          <a href="https://wikipedia.org" class="shortcut-card" data-url="https://wikipedia.org">
            <span class="shortcut-icon-box">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1-2.5-2.5Z"></path>
                <path d="M6 6h10"></path>
                <path d="M6 10h10"></path>
              </svg>
            </span>
            <span class="shortcut-label">Wikipedia</span>
          </a>

          <!-- Rust Docs -->
          <a href="https://doc.rust-lang.org/std/" class="shortcut-card" data-url="https://doc.rust-lang.org/std/">
            <span class="shortcut-icon-box">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="16 18 22 12 16 6"></polyline>
                <polyline points="8 6 2 12 8 18"></polyline>
              </svg>
            </span>
            <span class="shortcut-label">Rust Docs</span>
          </a>

          <!-- MDN Web -->
          <a href="https://developer.mozilla.org" class="shortcut-card" data-url="https://developer.mozilla.org">
            <span class="shortcut-icon-box">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 2L2 7l10 5 10-5-10-5z"></path>
                <path d="m2 17 10 5 10-5"></path>
                <path d="m2 12 10 5 10-5"></path>
              </svg>
            </span>
            <span class="shortcut-label">MDN Web</span>
          </a>

          <!-- Add shortcut -->
          <button id="add-shortcut-btn" type="button" class="shortcut-card">
            <span class="shortcut-icon-box">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 5v14"></path>
                <path d="M5 12h14"></path>
              </svg>
            </span>
            <span class="shortcut-label">+ Add</span>
          </button>
        </div>
      </section>
    </main>

    <!-- Footer -->
    <footer class="footer-bar">
      <span>Zero Electron · Native Rust Runtime</span>
      <span>Shoclaw Browser v0.1.0</span>
    </footer>
  </div>

  <!-- Add Shortcut Modal -->
  <div id="add-modal" class="modal-overlay">
    <div class="modal-box">
      <h3 class="modal-title">Add Quick Shortcut</h3>
      <form id="add-shortcut-form">
        <div style="display:flex; flex-direction:column; gap:10px;">
          <input id="shortcut-name-input" class="modal-input" type="text" placeholder="Shortcut Name (e.g. GitHub)" required />
          <input id="shortcut-url-input" class="modal-input" type="url" placeholder="URL (e.g. https://...)" required />
        </div>
        <div class="modal-actions">
          <button id="cancel-modal-btn" type="button" class="btn-secondary">Cancel</button>
          <button type="submit" class="btn-primary">Add</button>
        </div>
      </form>
    </div>
  </div>

  <script>
    (function () {{
      const clockEl = document.getElementById('shoclaw-clock');
      const statusEl = document.getElementById('shoclaw-status-text');
      const settingsBtn = document.getElementById('quick-settings-btn');
      const settingsPopover = document.getElementById('quick-settings-popover');
      const toggle24hBtn = document.getElementById('toggle-24h-btn');
      const popoverEngineLabel = document.getElementById('popover-engine-label');
      const searchForm = document.getElementById('shoclaw-search-form');
      const searchInput = document.getElementById('shoclaw-search-input');
      const engineBtn = document.getElementById('engine-switcher-btn');
      const addBtn = document.getElementById('add-shortcut-btn');
      const addModal = document.getElementById('add-modal');
      const cancelModalBtn = document.getElementById('cancel-modal-btn');
      const addForm = document.getElementById('add-shortcut-form');
      const shortcutsGrid = document.getElementById('shortcuts-grid');

      const engines = [
        {{ name: 'Google', url: 'https://www.google.com/search?q=' }},
        {{ name: 'DuckDuckGo', url: 'https://duckduckgo.com/?q=' }},
        {{ name: 'Brave', url: 'https://search.brave.com/search?q=' }},
        {{ name: 'Bing', url: 'https://www.bing.com/search?q=' }}
      ];

      let engineIndex = 0;
      let use24Hour = true;

      try {{
        const stored24h = localStorage.getItem('shoclaw_24h');
        if (stored24h !== null) use24Hour = stored24h === 'true';
        const storedEngine = localStorage.getItem('shoclaw_engine');
        if (storedEngine !== null) engineIndex = parseInt(storedEngine, 10) % engines.length;
      }} catch (e) {{}}

      function updateClock() {{
        const now = new Date();
        const hours = now.getHours();
        let greeting = 'Good evening';
        if (hours < 12) greeting = 'Good morning';
        else if (hours < 18) greeting = 'Good afternoon';

        const timeStr = now.toLocaleTimeString([], {{
          hour: '2-digit',
          minute: '2-digit',
          hour12: !use24Hour
        }});
        const dateStr = now.toLocaleDateString([], {{
          weekday: 'short',
          month: 'short',
          day: 'numeric'
        }});

        clockEl.textContent = timeStr;
        statusEl.textContent = greeting + ' · ' + dateStr;
        toggle24hBtn.textContent = use24Hour ? 'ON' : 'OFF';
      }}

      function updateEngineUI() {{
        const cur = engines[engineIndex];
        engineBtn.setAttribute('title', 'Search engine: ' + cur.name + ' (click to cycle)');
        popoverEngineLabel.textContent = cur.name;
      }}

      updateClock();
      updateEngineUI();
      setInterval(updateClock, 10000);

      // Keyboard navigation
      document.addEventListener('keydown', function (e) {{
        if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {{
          e.preventDefault();
          searchInput.focus();
        }} else if (e.key === 'Escape') {{
          settingsPopover.classList.remove('is-open');
          addModal.classList.remove('is-open');
        }}
      }});

      settingsBtn.addEventListener('click', function () {{
        settingsPopover.classList.toggle('is-open');
      }});

      toggle24hBtn.addEventListener('click', function () {{
        use24Hour = !use24Hour;
        try {{ localStorage.setItem('shoclaw_24h', String(use24Hour)); }} catch (e) {{}}
        updateClock();
      }});

      engineBtn.addEventListener('click', function () {{
        engineIndex = (engineIndex + 1) % engines.length;
        try {{ localStorage.setItem('shoclaw_engine', String(engineIndex)); }} catch (e) {{}}
        updateEngineUI();
      }});

      function doNavigate(target) {{
        if (window.ipc && typeof window.ipc.postMessage === 'function') {{
          window.ipc.postMessage(JSON.stringify({{ type: 'navigate', url: target }}));
        }} else {{
          window.location.href = target;
        }}
      }}

      searchForm.addEventListener('submit', function (e) {{
        e.preventDefault();
        const q = searchInput.value.trim();
        if (!q) return;

        const isUrl = /^(https?:\\/\\/)?([\\w-]+\\.)+[\\w-]{{2,}}(\\/.*)?$/i.test(q);
        if (isUrl) {{
          const target = /^https?:\\/\\//i.test(q) ? q : 'https://' + q;
          doNavigate(target);
        }} else {{
          doNavigate(engines[engineIndex].url + encodeURIComponent(q));
        }}
      }});

      // Shortcut card clicks
      document.addEventListener('click', function (e) {{
        const card = e.target.closest('[data-url]');
        if (card) {{
          e.preventDefault();
          doNavigate(card.getAttribute('data-url'));
        }}
      }});

      addBtn.addEventListener('click', function () {{
        addModal.classList.add('is-open');
        document.getElementById('shortcut-name-input').focus();
      }});

      cancelModalBtn.addEventListener('click', function () {{
        addModal.classList.remove('is-open');
      }});

      addForm.addEventListener('submit', function (e) {{
        e.preventDefault();
        const name = document.getElementById('shortcut-name-input').value.trim();
        const url = document.getElementById('shortcut-url-input').value.trim();
        if (!name || !url) return;

        const target = /^https?:\\/\\//i.test(url) ? url : 'https://' + url;
        const a = document.createElement('a');
        a.href = target;
        a.className = 'shortcut-card';
        a.setAttribute('data-url', target);
        a.innerHTML =
          '<span class="shortcut-icon-box" style="font-weight:600;font-size:15px;">' +
          name.charAt(0).toUpperCase() +
          '</span>' +
          '<span class="shortcut-label">' + name.replace(/</g, '&lt;') + '</span>';

        shortcutsGrid.insertBefore(a, addBtn);
        addForm.reset();
        addModal.classList.remove('is-open');
      }});
    }})();
  </script>
</body>
</html>
"""

SIDEBAR_HTML = f"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Shoclaw Sidebar</title>
  <style>
    :root {{
      --sc-bg: #0B0E17;
      --sc-glass: rgba(255, 255, 255, 0.05);
      --sc-glass-hover: rgba(255, 255, 255, 0.09);
      --sc-glass-active: rgba(255, 255, 255, 0.12);
      --sc-border: rgba(255, 255, 255, 0.10);
      --sc-border-specular: rgba(255, 255, 255, 0.16);
      --sc-border-subtle: rgba(255, 255, 255, 0.05);
      --sc-text: #E7E4F0;
      --sc-text-muted: #9F9CAE;
      --sc-text-dim: #6E6C7A;
      --sc-accent: #C5C2EE;
      --sc-ease: cubic-bezier(0.2, 0, 0, 1);
    }}

    *, *::before, *::after {{
      box-sizing: border-box;
      margin: 0;
      padding: 0;
    }}

    html, body {{
      width: 100%;
      height: 100vh;
      min-height: 100vh;
      background: var(--sc-bg);
      color: var(--sc-text);
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Inter", sans-serif;
      font-size: 13px;
      line-height: 1.4;
      -webkit-font-smoothing: antialiased;
      overflow: hidden;
      user-select: none;
      -webkit-user-select: none;
    }}

    button {{
      font: inherit;
      color: inherit;
      background: none;
      border: none;
      cursor: pointer;
    }}

    input {{
      font: inherit;
      color: inherit;
      background: none;
      border: none;
      outline: none;
    }}

    /* Main Responsive Liquid Glass Sidebar */
    .shoclaw-sidebar {{
      width: 100%;
      height: 100vh;
      display: flex;
      flex-direction: column;
      gap: 10px;
      padding: 12px 10px 10px;
      background:
        radial-gradient(ellipse 120% 45% at 0% 0%, rgba(44, 43, 89, 0.35), transparent 70%),
        var(--sc-bg);
      border-right: 1px solid var(--sc-border);
      box-shadow: inset -1px 0 0 rgba(255, 255, 255, 0.04);
      transition: padding 160ms var(--sc-ease);
    }}

    /* 1. Header controls */
    .sidebar-header {{
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 6px;
      padding: 0 2px;
      flex-shrink: 0;
    }}

    .brand-slot {{
      width: 26px;
      height: 26px;
      flex-shrink: 0;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      border-radius: 7px;
      background: var(--sc-glass);
      border: 1px solid var(--sc-border);
      color: var(--sc-accent);
      cursor: pointer;
    }}

    .brand-logo-img {{
      width: 18px;
      height: 18px;
      object-fit: contain;
      border-radius: 4px;
    }}

    .nav-group {{
      display: flex;
      align-items: center;
      gap: 2px;
    }}

    .icon-btn {{
      width: 28px;
      height: 28px;
      border-radius: 8px;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      color: var(--sc-text-muted);
      transition: background-color 160ms var(--sc-ease), color 160ms var(--sc-ease), transform 160ms var(--sc-ease);
    }}

    .icon-btn:hover {{
      background-color: var(--sc-glass-hover);
      color: var(--sc-text);
    }}

    .icon-btn:active {{
      transform: scale(0.94);
    }}

    /* 2. Compact URL bar */
    .url-bar {{
      position: relative;
      display: flex;
      align-items: center;
      gap: 7px;
      height: 34px;
      padding: 0 8px 0 10px;
      border-radius: 9999px;
      background-color: var(--sc-glass);
      border-top: 1px solid var(--sc-border-specular);
      border-left: 1px solid var(--sc-border-specular);
      border-right: 1px solid var(--sc-border-subtle);
      border-bottom: 1px solid var(--sc-border-subtle);
      flex-shrink: 0;
      transition: background-color 180ms var(--sc-ease), border-color 180ms var(--sc-ease);
    }}

    .url-bar:focus-within {{
      background-color: rgba(255, 255, 255, 0.08);
      border-color: rgba(197, 194, 238, 0.45);
    }}

    .url-lock {{
      color: var(--sc-text-dim);
      display: inline-flex;
      align-items: center;
      flex-shrink: 0;
    }}

    .url-input {{
      flex: 1;
      min-width: 0;
      font-size: 12px;
      color: var(--sc-text);
      user-select: text;
      -webkit-user-select: text;
    }}

    .url-input::placeholder {{
      color: var(--sc-text-dim);
    }}

    .url-copy-btn {{
      width: 22px;
      height: 22px;
      border-radius: 6px;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      color: var(--sc-text-muted);
      opacity: 0.65;
      transition: opacity 150ms var(--sc-ease), background-color 150ms var(--sc-ease);
    }}

    .url-copy-btn:hover {{
      opacity: 1;
      background-color: rgba(255, 255, 255, 0.12);
      color: var(--sc-text);
    }}

    /* 3. Pinned Favorites Grid (4-column) */
    .pinned-grid {{
      display: grid;
      grid-template-columns: repeat(4, 1fr);
      gap: 6px;
      flex-shrink: 0;
    }}

    .pinned-tile {{
      height: 40px;
      border-radius: 10px;
      display: flex;
      align-items: center;
      justify-content: center;
      background-color: var(--sc-glass);
      border-top: 1px solid var(--sc-border-specular);
      border-left: 1px solid var(--sc-border-specular);
      border-right: 1px solid var(--sc-border-subtle);
      border-bottom: 1px solid var(--sc-border-subtle);
      color: var(--sc-text-muted);
      transition: transform 160ms var(--sc-ease), background-color 160ms var(--sc-ease), color 160ms var(--sc-ease);
    }}

    .pinned-tile:hover {{
      transform: translateY(-1px);
      background-color: var(--sc-glass-hover);
      color: var(--sc-text);
    }}

    .pinned-tile:active {{
      transform: scale(0.96);
    }}

    .divider {{
      height: 1px;
      background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.10), transparent);
      margin: 2px 2px;
      flex-shrink: 0;
    }}

    /* 4. Open Tabs Section */
    .tabs-section {{
      flex: 1;
      min-height: 0;
      display: flex;
      flex-direction: column;
      gap: 8px;
    }}

    .new-tab-btn {{
      width: 100%;
      height: 34px;
      padding: 0 10px;
      border-radius: 9999px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      background-color: var(--sc-glass);
      border-top: 1px solid var(--sc-border-specular);
      border-left: 1px solid var(--sc-border-specular);
      border-right: 1px solid var(--sc-border-subtle);
      border-bottom: 1px solid var(--sc-border-subtle);
      color: var(--sc-text);
      font-size: 12.5px;
      font-weight: 500;
      flex-shrink: 0;
      transition: background-color 160ms var(--sc-ease), border-color 160ms var(--sc-ease);
    }}

    .new-tab-btn:hover {{
      background-color: var(--sc-glass-hover);
      border-color: rgba(197, 194, 238, 0.35);
    }}

    .new-tab-left {{
      display: inline-flex;
      align-items: center;
      gap: 7px;
    }}

    .kbd-hint {{
      font-family: "JetBrains Mono", ui-monospace, SFMono-Regular, monospace;
      font-size: 10px;
      font-weight: 500;
      color: var(--sc-text-muted);
      padding: 2px 6px;
      border-radius: 5px;
      background-color: rgba(255, 255, 255, 0.06);
      border: 1px solid rgba(255, 255, 255, 0.08);
    }}

    .tabs-scroll {{
      flex: 1;
      min-height: 0;
      overflow-y: auto;
      display: flex;
      flex-direction: column;
      gap: 4px;
      padding-right: 2px;
      scrollbar-width: thin;
      scrollbar-color: rgba(255, 255, 255, 0.14) transparent;
    }}

    .tabs-scroll::-webkit-scrollbar {{
      width: 4px;
    }}

    .tabs-scroll::-webkit-scrollbar-thumb {{
      background: rgba(255, 255, 255, 0.14);
      border-radius: 9999px;
    }}

    /* Tab Items */
    .tab-item {{
      position: relative;
      display: flex;
      align-items: center;
      gap: 8px;
      width: 100%;
      height: 34px;
      padding: 0 6px 0 10px;
      border-radius: 10px;
      border: 1px solid transparent;
      background-color: transparent;
      color: var(--sc-text-muted);
      cursor: pointer;
      flex-shrink: 0;
      transition: background-color 140ms var(--sc-ease), color 140ms var(--sc-ease), border-color 140ms var(--sc-ease);
    }}

    .tab-item:hover {{
      background-color: var(--sc-glass);
      color: var(--sc-text);
    }}

    .tab-item.is-active {{
      background-color: var(--sc-glass-active);
      border-top: 1px solid var(--sc-border-specular);
      border-left: 1px solid var(--sc-border-specular);
      border-right: 1px solid var(--sc-border-subtle);
      border-bottom: 1px solid var(--sc-border-subtle);
      color: var(--sc-text);
      font-weight: 500;
      box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.08);
    }}

    .tab-item.is-sleeping {{
      opacity: 0.65;
    }}

    .tab-item.is-sleeping:hover {{
      opacity: 0.95;
    }}

    .tab-favicon {{
      width: 16px;
      height: 16px;
      flex-shrink: 0;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      color: inherit;
    }}

    .tab-title {{
      flex: 1;
      min-width: 0;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      font-size: 12.5px;
    }}

    .audio-badge {{
      width: 8px;
      height: 8px;
      border-radius: 50%;
      background: #00d2b4;
      box-shadow: 0 0 6px #00d2b4;
      flex-shrink: 0;
    }}

    .sleep-badge {{
      font-family: monospace;
      font-size: 10px;
      color: var(--sc-text-dim);
      padding: 1px 4px;
      border-radius: 4px;
      background: rgba(255, 255, 255, 0.04);
      flex-shrink: 0;
    }}

    .tab-close {{
      width: 20px;
      height: 20px;
      flex-shrink: 0;
      border-radius: 6px;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      color: var(--sc-text-muted);
      opacity: 0;
      transition: opacity 120ms var(--sc-ease), background-color 120ms var(--sc-ease);
    }}

    .tab-item:hover .tab-close {{
      opacity: 1;
    }}

    .tab-close:hover {{
      background-color: rgba(255, 255, 255, 0.14);
      color: var(--sc-text);
    }}

    /* 5. Footer */
    .sidebar-footer {{
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 6px 4px 0;
      border-top: 1px solid rgba(255, 255, 255, 0.08);
      flex-shrink: 0;
    }}

    .ram-guard-text {{
      font-family: "JetBrains Mono", ui-monospace, SFMono-Regular, monospace;
      font-size: 10.5px;
      color: var(--sc-text-dim);
      letter-spacing: 0.02em;
    }}

    .footer-actions {{
      display: flex;
      align-items: center;
      gap: 2px;
    }}

    .footer-btn {{
      width: 26px;
      height: 26px;
      border-radius: 6px;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      color: var(--sc-text-muted);
      transition: background-color 140ms var(--sc-ease), color 140ms var(--sc-ease);
    }}

    .footer-btn:hover {{
      background-color: var(--sc-glass-hover);
      color: var(--sc-text);
    }}

    /* Compact mode (48px dock) */
    body.mode-compact .shoclaw-sidebar {{
      padding: 10px 4px;
      align-items: center;
      gap: 8px;
    }}

    body.mode-compact .nav-group,
    body.mode-compact .url-bar,
    body.mode-compact .pinned-grid,
    body.mode-compact .divider,
    body.mode-compact .kbd-hint,
    body.mode-compact .new-tab-left span,
    body.mode-compact .tab-title,
    body.mode-compact .tab-close,
    body.mode-compact .ram-guard-text,
    body.mode-compact .sleep-badge {{
      display: none !important;
    }}

    body.mode-compact .sidebar-header {{
      flex-direction: column;
      align-items: center;
      justify-content: center;
      gap: 8px;
      width: 100%;
      padding: 0;
    }}

    body.mode-compact .brand-slot {{
      width: 30px;
      height: 30px;
    }}

    body.mode-compact .new-tab-btn {{
      width: 36px;
      height: 36px;
      padding: 0;
      justify-content: center;
      border-radius: 9px;
    }}

    body.mode-compact .tab-item {{
      width: 36px;
      height: 36px;
      padding: 0;
      justify-content: center;
      border-radius: 9px;
    }}

    body.mode-compact .tab-favicon {{
      width: 16px;
      height: 16px;
    }}

    body.mode-compact .sidebar-footer {{
      justify-content: center;
      width: 100%;
      padding: 6px 0 0;
    }}
  </style>
</head>
<body>
  <aside class="shoclaw-sidebar" aria-label="Shoclaw Sidebar">
    <!-- Header -->
    <header class="sidebar-header">
      <div id="shoclaw-sidebar-logo-slot" class="brand-slot" title="Shoclaw Home" data-action="go-home">
        <img src="{LOGO_DATA_URI}" alt="Shoclaw" class="brand-logo-img" />
      </div>

      <nav class="nav-group" aria-label="Navigation">
        <button type="button" class="icon-btn" data-action="go-back" title="Back (Alt+Left)">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m15 18-6-6 6-6"></path>
          </svg>
        </button>
        <button type="button" class="icon-btn" data-action="go-forward" title="Forward (Alt+Right)">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m9 18 6-6-6-6"></path>
          </svg>
        </button>
        <button type="button" class="icon-btn" data-action="reload" title="Reload (Ctrl+R)">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12a9 9 0 1 1-2.64-6.36"></path>
            <path d="M21 3v6h-6"></path>
          </svg>
        </button>
      </nav>

      <button type="button" class="icon-btn" data-action="toggle-sidebar" title="Toggle sidebar (Ctrl+B)">
        <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="4" width="18" height="16" rx="3"></rect>
          <path d="M9 4v16"></path>
        </svg>
      </button>
    </header>

    <!-- Address Bar -->
    <form id="sidebar-url-form" class="url-bar" role="search">
      <span class="url-lock" title="Secure">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="4" y="11" width="16" height="10" rx="2"></rect>
          <path d="M8 11V7a4 4 0 0 1 8 0v4"></path>
        </svg>
      </span>
      <input id="sidebar-url-input" class="url-input" type="text" placeholder="Search or enter URL..." autocomplete="off" spellcheck="false" />
      <button type="button" id="copy-url-btn" class="url-copy-btn" title="Copy URL">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="9" y="9" width="13" height="13" rx="2"></rect>
          <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
        </svg>
      </button>
    </form>

    <!-- Pinned Favorites -->
    <section class="pinned-grid" aria-label="Pinned Favorites">
      <button type="button" class="pinned-tile" data-action="open-pinned" data-url="https://github.com" title="GitHub">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"></path>
          <path d="M9 18c-4.51 2-5-2-7-2"></path>
        </svg>
      </button>
      <button type="button" class="pinned-tile" data-action="open-pinned" data-url="https://www.youtube.com" title="YouTube">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <rect x="2" y="5" width="20" height="14" rx="4"></rect>
          <polygon points="10 9 15 12 10 15 10 9" fill="currentColor"></polygon>
        </svg>
      </button>
      <button type="button" class="pinned-tile" data-action="open-pinned" data-url="https://reddit.com" title="Reddit">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="9"></circle>
          <circle cx="9" cy="11.5" r="1.5" fill="currentColor"></circle>
          <circle cx="15" cy="11.5" r="1.5" fill="currentColor"></circle>
          <path d="M9 15.5c1 1 2.5 1.5 3 1.5s2-.5 3-1.5"></path>
        </svg>
      </button>
      <button type="button" class="pinned-tile" data-action="open-pinned" data-url="https://google.com" title="Google">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8"></circle>
          <path d="m21 21-4.3-4.3"></path>
        </svg>
      </button>
    </section>

    <div class="divider"></div>

    <!-- Open Tabs -->
    <section class="tabs-section" aria-label="Open Tabs">
      <button type="button" id="new-tab-btn" class="new-tab-btn" data-action="new-tab">
        <span class="new-tab-left">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M5 12h14"></path>
            <path d="M12 5v14"></path>
          </svg>
          <span>New Tab</span>
        </span>
        <kbd class="kbd-hint">Ctrl+T</kbd>
      </button>

      <div id="tabs-list" class="tabs-scroll" role="tablist">
        <!-- Rendered dynamically -->
      </div>
    </section>

    <!-- Footer -->
    <footer class="sidebar-footer">
      <div id="ram-guard-info" class="ram-guard-text">1 Active · 0 Sleeping</div>
      <div class="footer-actions">
        <button type="button" class="footer-btn" data-action="open-settings" title="Settings">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="3"></circle>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
          </svg>
        </button>
      </div>
    </footer>
  </aside>

  <script>
    (function () {{
      const tabsList = document.getElementById('tabs-list');
      const urlInput = document.getElementById('sidebar-url-input');
      const urlForm = document.getElementById('sidebar-url-form');
      const copyUrlBtn = document.getElementById('copy-url-btn');
      const ramGuardInfo = document.getElementById('ram-guard-info');

      function sendIPC(msg) {{
        if (window.ipc && typeof window.ipc.postMessage === 'function') {{
          window.ipc.postMessage(JSON.stringify(msg));
        }}
      }}

      // Dynamic Tab Updater called from Rust
      window.__shoclaw_update_tabs = function (tabs, activeId) {{
        tabsList.innerHTML = '';
        let activeCount = 0;
        let sleepingCount = 0;

        tabs.forEach(function (tab) {{
          const isActive = tab.id === activeId;
          const isSleeping = tab.state === 'Sleeping';

          if (isSleeping) sleepingCount++;
          else activeCount++;

          const tabEl = document.createElement('div');
          tabEl.className = 'tab-item' + (isActive ? ' is-active' : '') + (isSleeping ? ' is-sleeping' : '');
          tabEl.setAttribute('role', 'tab');
          tabEl.setAttribute('data-action', 'switch-tab');
          tabEl.setAttribute('data-tab-id', tab.id);

          let indicators = '';
          if (tab.is_audio_playing) {{
            indicators += '<span class="audio-badge" title="Playing audio"></span>';
          }}
          if (isSleeping) {{
            indicators += '<span class="sleep-badge" title="Sleeping (0 MB RAM)">0MB</span>';
          }}

          tabEl.innerHTML =
            '<span class="tab-favicon">' +
              '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">' +
                '<circle cx="12" cy="12" r="9"></circle>' +
              '</svg>' +
            '</span>' +
            '<span class="tab-title" title="' + tab.title.replace(/"/g, '&quot;') + '">' + tab.title.replace(/</g, '&lt;') + '</span>' +
            indicators +
            '<button type="button" class="tab-close" data-action="close-tab" data-tab-id="' + tab.id + '" title="Close tab">' +
              '<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">' +
                '<path d="M18 6 6 18"></path><path d="m6 6 12 12"></path>' +
              '</svg>' +
            '</button>';

          tabsList.appendChild(tabEl);

          if (isActive) {{
            if (document.activeElement !== urlInput) {{
              urlInput.value = tab.url;
            }}
          }}
        }});

        if (ramGuardInfo) {{
          ramGuardInfo.textContent = activeCount + ' Active · ' + sleepingCount + ' Sleeping';
        }}
      }};

      window.__shoclaw_set_mode = function (mode) {{
        document.body.classList.remove('mode-expanded', 'mode-compact', 'mode-hidden', 'mode-autohide');
        document.body.classList.add('mode-' + mode);
      }};

      // Delegated click handler
      document.addEventListener('click', function (e) {{
        const closeBtn = e.target.closest('[data-action="close-tab"]');
        if (closeBtn) {{
          e.stopPropagation();
          const tabId = parseInt(closeBtn.getAttribute('data-tab-id'), 10);
          sendIPC({{ type: 'close_tab', id: tabId }});
          return;
        }}

        const switchTab = e.target.closest('[data-action="switch-tab"]');
        if (switchTab) {{
          const tabId = parseInt(switchTab.getAttribute('data-tab-id'), 10);
          sendIPC({{ type: 'switch_tab', id: tabId }});
          return;
        }}

        const actionEl = e.target.closest('[data-action]');
        if (!actionEl) return;

        const action = actionEl.getAttribute('data-action');
        switch (action) {{
          case 'new-tab':
            sendIPC({{ type: 'new_tab' }});
            break;
          case 'go-back':
            sendIPC({{ type: 'go_back' }});
            break;
          case 'go-forward':
            sendIPC({{ type: 'go_forward' }});
            break;
          case 'reload':
            sendIPC({{ type: 'reload' }});
            break;
          case 'toggle-sidebar':
            sendIPC({{ type: 'toggle_sidebar' }});
            break;
          case 'go-home':
            if (document.body.classList.contains('mode-compact')) {{
              sendIPC({{ type: 'toggle_sidebar' }});
            }} else {{
              sendIPC({{ type: 'navigate', url: 'shoclaw://start' }});
            }}
            break;
          case 'open-pinned': {{
            const url = actionEl.getAttribute('data-url');
            if (url) sendIPC({{ type: 'navigate', url: url }});
            break;
          }}
          case 'open-settings':
            sendIPC({{ type: 'navigate', url: 'shoclaw://start' }});
            break;
        }}
      }});

      urlForm.addEventListener('submit', function (e) {{
        e.preventDefault();
        const value = urlInput.value.trim();
        if (!value) return;
        sendIPC({{ type: 'navigate', url: value }});
      }});

      copyUrlBtn.addEventListener('click', function () {{
        if (urlInput.value && navigator.clipboard && navigator.clipboard.writeText) {{
          navigator.clipboard.writeText(urlInput.value).catch(function () {{}});
        }}
      }});
    }})();
  </script>
</body>
</html>
"""

def main():
    startpage_path = os.path.join(BASE_DIR, "assets", "shoclaw_startpage.html")
    sidebar_path = os.path.join(BASE_DIR, "assets", "shoclaw_sidebar_250px.html")

    with open(startpage_path, "w", encoding="utf-8") as f:
        f.write(STARTPAGE_HTML)
    print(f"Generated {startpage_path} ({len(STARTPAGE_HTML)} bytes)")

    with open(sidebar_path, "w", encoding="utf-8") as f:
        f.write(SIDEBAR_HTML)
    print(f"Generated {sidebar_path} ({len(SIDEBAR_HTML)} bytes)")

if __name__ == "__main__":
    main()
