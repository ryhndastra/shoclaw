use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tao::{
    dpi::LogicalSize,
    event_loop::{EventLoopProxy, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder};

#[cfg(target_os = "linux")]
use gtk::prelude::*;
#[cfg(target_os = "linux")]
use tao::platform::unix::WindowExtUnix;
#[cfg(target_os = "linux")]
use wry::{WebViewBuilderExtUnix, WebViewExtUnix};

use crate::config::SidebarMode;
use crate::tab::{TabManager, TabState};
use crate::ui::UiLayout;
use super::navigation::{execute_navigation, normalize_url, NavigationAction};

#[derive(Debug, Clone)]
pub enum AppEvent {
    NewTab,
    SwitchTab(u64),
    CloseTab(u64),
    ToggleSidebar,
    ToggleSidebarHidden,
    TogglePalette,
    ClosePalette,
    ExecuteCommand(String),
    Navigate(String),
    GoBack,
    GoForward,
    Reload,
    Quit,
    AudioState { tab_id: u64, playing: bool },
    TitleChanged { tab_id: u64, title: String },
}

pub struct BrowserWindow {
    pub window: Window,
    pub tabs: TabManager,
    pub current_title: Arc<Mutex<String>>,
    pub audio_states: Arc<Mutex<HashMap<u64, bool>>>,
    pub layout: UiLayout,
    pub sidebar_webview: Option<WebView>,
    pub palette_webview: Option<WebView>,
    pub is_palette_open: bool,
    pub homepage: String,
    pub search_engine: String,
    pub proxy: EventLoopProxy<AppEvent>,
    #[cfg(target_os = "linux")]
    pub fixed: gtk::Fixed,
}

impl BrowserWindow {
    pub fn new(
        event_loop: &EventLoopWindowTarget<AppEvent>,
        proxy: EventLoopProxy<AppEvent>,
        initial_url: &str,
        max_active_tabs: usize,
        sidebar_mode: SidebarMode,
        homepage: &str,
        search_engine: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let window = WindowBuilder::new()
            .with_title("Shoclaw")
            .with_inner_size(LogicalSize::new(1280.0, 800.0))
            .with_min_inner_size(LogicalSize::new(640.0, 480.0))
            .build(event_loop)?;

        #[cfg(target_os = "linux")]
        let fixed = {
            let fixed = gtk::Fixed::new();
            let vbox = window
                .default_vbox()
                .ok_or("Failed to retrieve default_vbox from window")?;
            vbox.pack_start(&fixed, true, true, 0);
            fixed.show_all();
            fixed
        };

        let current_title = Arc::new(Mutex::new("Shoclaw".to_string()));
        let audio_states = Arc::new(Mutex::new(HashMap::new()));
        let tabs = TabManager::new(max_active_tabs);
        let layout = UiLayout::new(sidebar_mode);

        let mut browser_window = Self {
            window,
            tabs,
            current_title,
            audio_states,
            layout,
            sidebar_webview: None,
            palette_webview: None,
            is_palette_open: false,
            homepage: homepage.to_string(),
            search_engine: search_engine.to_string(),
            proxy,
            #[cfg(target_os = "linux")]
            fixed,
        };

        let sidebar_wv = browser_window.spawn_sidebar_webview()?;
        browser_window.sidebar_webview = Some(sidebar_wv);
        browser_window.open_tab(initial_url)?;
        let palette_wv = browser_window.spawn_palette_webview()?;
        browser_window.palette_webview = Some(palette_wv);
        browser_window.sync_sidebar_ui();

        Ok(browser_window)
    }

    /// Synchronizes content webview bounds with GTK Fixed container layout.
    fn sync_content_bounds(&self, webview: &WebView, content_rect: wry::Rect) {
        let _ = webview.set_bounds(content_rect);

        #[cfg(target_os = "linux")]
        {
            let widget = webview.webview();
            let scale_factor = widget.scale_factor() as f64;
            let (width, height) = content_rect.size.to_logical::<i32>(scale_factor).into();
            let (x, y) = content_rect.position.to_logical::<i32>(scale_factor).into();

            self.fixed.move_(&widget, x, y);
            widget.set_size_request(width, height);
        }
    }

    /// Synchronizes sidebar webview bounds with GTK Fixed container layout.
    fn sync_sidebar_bounds(&self, webview: &WebView, sidebar_rect: wry::Rect, visible: bool) {
        let _ = webview.set_bounds(sidebar_rect);
        let _ = webview.set_visible(visible);

        #[cfg(target_os = "linux")]
        {
            let widget = webview.webview();
            let scale_factor = widget.scale_factor() as f64;
            let (width, height) = sidebar_rect.size.to_logical::<i32>(scale_factor).into();
            let (x, y) = sidebar_rect.position.to_logical::<i32>(scale_factor).into();

            self.fixed.move_(&widget, x, y);
            widget.set_size_request(width, height);
            widget.set_visible(visible);
        }
    }

    /// Synchronizes command palette webview bounds with GTK Fixed container layout.
    fn sync_palette_bounds(&self, webview: &WebView, width: f64, height: f64, visible: bool) {
        let palette_rect = wry::Rect {
            position: wry::dpi::LogicalPosition::new(0.0, 0.0).into(),
            size: wry::dpi::LogicalSize::new(width, height).into(),
        };
        let _ = webview.set_bounds(palette_rect);
        let _ = webview.set_visible(visible);

        #[cfg(target_os = "linux")]
        {
            let widget = webview.webview();
            let scale_factor = widget.scale_factor() as f64;
            let (w, h) = palette_rect.size.to_logical::<i32>(scale_factor).into();
            self.fixed.move_(&widget, 0, 0);
            widget.set_size_request(w, h);
            widget.set_visible(visible);
        }
    }

    /// Spawns the dedicated vertical sidebar WebKit instance.
    fn spawn_sidebar_webview(&self) -> Result<WebView, Box<dyn std::error::Error>> {
        let proxy = self.proxy.clone();
        let window_size = self.window.inner_size();
        let scale = self.window.scale_factor();
        let logical = window_size.to_logical::<f64>(scale);
        let (sidebar_rect, _) = self.layout.compute_bounds(logical.width, logical.height);

        let builder = WebViewBuilder::new()
            .with_html(crate::ui::SIDEBAR_HTML)
            .with_devtools(cfg!(debug_assertions))
            .with_bounds(sidebar_rect)
            .with_ipc_handler(move |req| {
                let body = req.body();
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
                    if let Some(msg_type) = val.get("type").and_then(|v| v.as_str()) {
                        match msg_type {
                            "new_tab" | "new-tab" => {
                                let _ = proxy.send_event(AppEvent::NewTab);
                            }
                            "switch_tab" | "switch-tab" => {
                                if let Some(id) = val.get("id").and_then(|v| v.as_u64()) {
                                    let _ = proxy.send_event(AppEvent::SwitchTab(id));
                                }
                            }
                            "close_tab" | "close-tab" => {
                                if let Some(id) = val.get("id").and_then(|v| v.as_u64()) {
                                    let _ = proxy.send_event(AppEvent::CloseTab(id));
                                }
                            }
                            "toggle_sidebar" | "toggle-sidebar" => {
                                let _ = proxy.send_event(AppEvent::ToggleSidebar);
                            }
                            "toggle_palette" | "open-palette" => {
                                let _ = proxy.send_event(AppEvent::TogglePalette);
                            }
                            "navigate" | "open-pinned" | "open_pinned" => {
                                if let Some(target_url) = val.get("url").and_then(|v| v.as_str()) {
                                    let _ = proxy.send_event(AppEvent::Navigate(target_url.to_string()));
                                }
                            }
                            "go_back" | "go-back" => {
                                let _ = proxy.send_event(AppEvent::GoBack);
                            }
                            "go_forward" | "go-forward" => {
                                let _ = proxy.send_event(AppEvent::GoForward);
                            }
                            "reload" => {
                                let _ = proxy.send_event(AppEvent::Reload);
                            }
                            _ => {}
                        }
                    }
                }
            });

        #[cfg(target_os = "linux")]
        let webview = builder.build_gtk(&self.fixed)?;

        #[cfg(not(target_os = "linux"))]
        let webview = builder.build(&self.window)?;

        self.sync_sidebar_bounds(&webview, sidebar_rect, self.layout.is_sidebar_visible());
        Ok(webview)
    }

    /// Spawns the floating Command Palette (Raycast / Spotlight Omnibar) WebKit instance.
    fn spawn_palette_webview(&self) -> Result<WebView, Box<dyn std::error::Error>> {
        let proxy = self.proxy.clone();
        let window_size = self.window.inner_size();
        let scale = self.window.scale_factor();
        let logical = window_size.to_logical::<f64>(scale);
        let palette_rect = wry::Rect {
            position: wry::dpi::LogicalPosition::new(0.0, 0.0).into(),
            size: wry::dpi::LogicalSize::new(logical.width, logical.height).into(),
        };

        let builder = WebViewBuilder::new()
            .with_html(crate::ui::PALETTE_HTML)
            .with_devtools(cfg!(debug_assertions))
            .with_transparent(true)
            .with_bounds(palette_rect)
            .with_ipc_handler(move |req| {
                let body = req.body();
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
                    if let Some(msg_type) = val.get("type").and_then(|v| v.as_str()) {
                        match msg_type {
                            "close_palette" => {
                                let _ = proxy.send_event(AppEvent::ClosePalette);
                            }
                            "switch_tab" => {
                                if let Some(id) = val.get("id").and_then(|v| v.as_u64()) {
                                    let _ = proxy.send_event(AppEvent::SwitchTab(id));
                                }
                            }
                            "navigate" => {
                                if let Some(url) = val.get("url").and_then(|v| v.as_str()) {
                                    let _ = proxy.send_event(AppEvent::Navigate(url.to_string()));
                                }
                            }
                            "command" => {
                                if let Some(cmd) = val.get("command").and_then(|v| v.as_str()) {
                                    let _ = proxy.send_event(AppEvent::ExecuteCommand(cmd.to_string()));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            });

        #[cfg(target_os = "linux")]
        let webview = builder.build_gtk(&self.fixed)?;

        #[cfg(not(target_os = "linux"))]
        let webview = builder.build(&self.window)?;

        let _ = webview.set_visible(false);
        #[cfg(target_os = "linux")]
        {
            let widget = webview.webview();
            widget.set_visible(false);
        }

        Ok(webview)
    }

    /// Spawns a content WebKit WebView instance for a given tab ID and URL.
    fn spawn_content_webview(&self, tab_id: u64, url: &str) -> Result<WebView, Box<dyn std::error::Error>> {
        let proxy = self.proxy.clone();
        let title_clone = self.current_title.clone();
        let audio_states_clone = self.audio_states.clone();
        let window_size = self.window.inner_size();
        let scale = self.window.scale_factor();
        let logical = window_size.to_logical::<f64>(scale);
        let (_, content_rect) = self.layout.compute_bounds(logical.width, logical.height);

        // Media playback detection script
        let audio_script = r#"
            (function() {
                function checkMedia() {
                    const media = document.querySelectorAll('audio, video');
                    let playing = false;
                    for (const el of media) {
                        if (!el.paused && !el.ended && el.currentTime > 0) {
                            playing = true;
                            break;
                        }
                    }
                    if (window.ipc) {
                        window.ipc.postMessage(JSON.stringify({ type: 'audio_state', playing: playing }));
                    }
                }
                document.addEventListener('play', checkMedia, true);
                document.addEventListener('pause', checkMedia, true);
                document.addEventListener('ended', checkMedia, true);

                // Global Omnibar hotkey inside webview
                window.addEventListener('keydown', function(e) {
                    if ((e.ctrlKey || e.metaKey) && (e.key.toLowerCase() === 'k' || e.code === 'KeyK')) {
                        e.preventDefault();
                        if (window.ipc) {
                            window.ipc.postMessage(JSON.stringify({ type: 'toggle_palette' }));
                        }
                    }
                }, true);
            })();
        "#;

        let proxy_title = proxy.clone();
        let builder = if url == "shoclaw://start" {
            WebViewBuilder::new().with_html(crate::ui::STARTPAGE_HTML)
        } else {
            WebViewBuilder::new().with_url(url)
        };

        let builder = builder
            .with_devtools(cfg!(debug_assertions))
            .with_bounds(content_rect)
            .with_initialization_script(audio_script)
            .with_document_title_changed_handler(move |title| {
                if let Ok(mut lock) = title_clone.lock() {
                    *lock = title.clone();
                }
                let _ = proxy_title.send_event(AppEvent::TitleChanged {
                    tab_id,
                    title,
                });
            })
            .with_ipc_handler(move |req| {
                let body = req.body();
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
                    if let Some(msg_type) = val.get("type").and_then(|v| v.as_str()) {
                        match msg_type {
                            "audio_state" => {
                                let playing = val.get("playing").and_then(|v| v.as_bool()).unwrap_or(false);
                                if let Ok(mut states) = audio_states_clone.lock() {
                                    states.insert(tab_id, playing);
                                }
                                let _ = proxy.send_event(AppEvent::AudioState { tab_id, playing });
                            }
                            "navigate" => {
                                if let Some(target_url) = val.get("url").and_then(|v| v.as_str()) {
                                    let _ = proxy.send_event(AppEvent::Navigate(target_url.to_string()));
                                }
                            }
                            "toggle_palette" => {
                                let _ = proxy.send_event(AppEvent::TogglePalette);
                            }
                            _ => {}
                        }
                    }
                }
            });

        #[cfg(target_os = "linux")]
        let webview = builder.build_gtk(&self.fixed)?;

        #[cfg(not(target_os = "linux"))]
        let webview = builder.build(&self.window)?;

        #[cfg(target_os = "linux")]
        {
            use webkit2gtk::{WebContextExt, WebViewExt, WebsiteDataManagerExt};
            let wk = webview.webview();
            if let Some(ctx) = wk.context() {
                #[allow(deprecated)]
                ctx.set_tls_errors_policy(webkit2gtk::TLSErrorsPolicy::Ignore);
                if let Some(dm) = ctx.website_data_manager() {
                    dm.set_tls_errors_policy(webkit2gtk::TLSErrorsPolicy::Ignore);
                }
            }
            wk.connect_load_failed_with_tls_errors(|wv, failing_uri, cert, _errors| {
                println!("[TLS Guard] Intercepted/Invalid TLS certificate on {}, allowing exception...", failing_uri);
                if let Some(host) = failing_uri
                    .split("://")
                    .nth(1)
                    .and_then(|s| s.split('/').next())
                    .and_then(|s| s.split(':').next())
                {
                    if let Some(ctx) = wv.context() {
                        ctx.allow_tls_certificate_for_host(cert, host);
                    }
                }
                true
            });
        }

        Ok(webview)
    }

    /// Synchronizes reported audio states into the TabManager tabs.
    pub fn sync_audio_states(&mut self) {
        if let Ok(lock) = self.audio_states.lock() {
            for (&tab_id, &playing) in lock.iter() {
                if let Some(tab) = self.tabs.get_tab_mut(tab_id) {
                    tab.is_audio_playing = playing;
                }
            }
        }
    }

    /// Opens a new tab, spawns its WebKit view, and hibernates excess tabs.
    pub fn open_tab(&mut self, url: &str) -> Result<u64, Box<dyn std::error::Error>> {
        self.sync_audio_states();

        if let Some(active_tab) = self.tabs.active_tab() {
            if let Some(wv) = &active_tab.webview {
                let _ = wv.set_visible(false);
            }
        }

        let (new_id, hibernated_ids) = self.tabs.create_tab(url);
        let webview = self.spawn_content_webview(new_id, url)?;
        let window_size = self.window.inner_size();
        let scale = self.window.scale_factor();
        let logical = window_size.to_logical::<f64>(scale);
        let (_, content_rect) = self.layout.compute_bounds(logical.width, logical.height);
        self.sync_content_bounds(&webview, content_rect);
        let _ = webview.set_visible(true);
        #[cfg(target_os = "linux")]
        self.fixed.queue_resize();

        if let Some(tab) = self.tabs.get_tab_mut(new_id) {
            tab.attach_webview(webview);
            if url == "shoclaw://start" {
                tab.title = "Shoclaw Start".to_string();
            }
        }

        for hib_id in hibernated_ids {
            if let Some(tab) = self.tabs.get_tab_mut(hib_id) {
                println!(
                    "[RAM Guard] Hibernating Tab {} ({}) to 0 MB RAM",
                    tab.id, tab.url
                );
                tab.detach_webview();
            }
        }

        self.dump_memory_summary();
        self.sync_sidebar_ui();
        Ok(new_id)
    }

    /// Switches to an existing tab, waking it from sleep if needed.
    pub fn switch_tab(&mut self, target_id: u64) -> Result<bool, Box<dyn std::error::Error>> {
        if self.tabs.active_tab_id == Some(target_id) {
            return Ok(true);
        }

        self.sync_audio_states();

        if let Some(curr_active) = self.tabs.active_tab() {
            if let Some(wv) = &curr_active.webview {
                let _ = wv.set_visible(false);
            }
        }

        let window_size = self.window.inner_size();
        let scale = self.window.scale_factor();
        let logical = window_size.to_logical::<f64>(scale);
        let (_, content_rect) = self.layout.compute_bounds(logical.width, logical.height);

        let needs_wake = self
            .tabs
            .get_tab(target_id)
            .map(|t| t.webview.is_none())
            .unwrap_or(false);

        if needs_wake {
            let url = self
                .tabs
                .get_tab(target_id)
                .map(|t| t.url.clone())
                .unwrap_or_else(|| "shoclaw://start".to_string());

            println!(
                "[RAM Guard] Waking sleeping Tab {} ({}) from cache",
                target_id, url
            );
            let webview = self.spawn_content_webview(target_id, &url)?;
            self.sync_content_bounds(&webview, content_rect);
            let _ = webview.set_visible(true);

            if let Some(tab) = self.tabs.get_tab_mut(target_id) {
                tab.attach_webview(webview);
            }
        } else if let Some(tab) = self.tabs.get_tab(target_id) {
            if let Some(wv) = &tab.webview {
                self.sync_content_bounds(wv, content_rect);
                let _ = wv.set_visible(true);
            }
        }
        #[cfg(target_os = "linux")]
        self.fixed.queue_resize();

        let (success, hibernated_ids) = self.tabs.switch_to_tab(target_id);

        for hib_id in hibernated_ids {
            if let Some(tab) = self.tabs.get_tab_mut(hib_id) {
                println!(
                    "[RAM Guard] LRU Hibernated Tab {} to 0 MB RAM",
                    tab.id
                );
                tab.detach_webview();
            }
        }

        if let Some(active) = self.tabs.active_tab() {
            let formatted_title = format!("{} - Shoclaw", active.title);
            self.window.set_title(&formatted_title);
        }

        self.dump_memory_summary();
        self.sync_sidebar_ui();
        Ok(success)
    }

    /// Closes a tab by ID. If it was active, activates an adjacent tab.
    pub fn close_tab(&mut self, target_id: u64) -> Result<Option<u64>, Box<dyn std::error::Error>> {
        if let Ok(mut states) = self.audio_states.lock() {
            states.remove(&target_id);
        }

        let next_active = self.tabs.close_tab(target_id);

        if let Some(next_id) = next_active {
            let window_size = self.window.inner_size();
            let scale = self.window.scale_factor();
            let logical = window_size.to_logical::<f64>(scale);
            let (_, content_rect) = self.layout.compute_bounds(logical.width, logical.height);

            let needs_wake = self
                .tabs
                .get_tab(next_id)
                .map(|t| t.webview.is_none())
                .unwrap_or(false);

            if needs_wake {
                let url = self
                    .tabs
                    .get_tab(next_id)
                    .map(|t| t.url.clone())
                    .unwrap_or_else(|| "shoclaw://start".to_string());
                let webview = self.spawn_content_webview(next_id, &url)?;
                self.sync_content_bounds(&webview, content_rect);
                let _ = webview.set_visible(true);

                if let Some(tab) = self.tabs.get_tab_mut(next_id) {
                    tab.attach_webview(webview);
                }
            } else if let Some(tab) = self.tabs.get_tab(next_id) {
                if let Some(wv) = &tab.webview {
                    self.sync_content_bounds(wv, content_rect);
                    let _ = wv.set_visible(true);
                }
            }
            #[cfg(target_os = "linux")]
            self.fixed.queue_resize();

            if let Some(active) = self.tabs.active_tab() {
                let formatted_title = format!("{} - Shoclaw", active.title);
                self.window.set_title(&formatted_title);
            }
        }

        self.dump_memory_summary();
        self.sync_sidebar_ui();
        Ok(next_active)
    }

    /// Closes the currently active tab.
    pub fn close_active_tab(&mut self) -> Result<Option<u64>, Box<dyn std::error::Error>> {
        match self.tabs.active_tab_id {
            Some(id) => self.close_tab(id),
            None => Ok(None),
        }
    }

    pub fn next_tab(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(next_id) = self.tabs.next_tab_id() {
            self.switch_tab(next_id)?;
        }
        Ok(())
    }

    pub fn prev_tab(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(prev_id) = self.tabs.prev_tab_id() {
            self.switch_tab(prev_id)?;
        }
        Ok(())
    }

    pub fn active_webview(&self) -> Option<&WebView> {
        self.tabs.active_tab().and_then(|t| t.webview.as_ref())
    }

    pub fn navigate_active(&mut self, input: &str, search_template: &str) -> Result<(), Box<dyn std::error::Error>> {
        let target_url = normalize_url(input, search_template);
        if let Some(tab) = self.tabs.active_tab_mut() {
            tab.url = target_url.clone();
            if target_url == "shoclaw://start" {
                tab.title = "Shoclaw Start".to_string();
                if let Some(wv) = &tab.webview {
                    let _ = wv.load_html(crate::ui::STARTPAGE_HTML);
                }
            } else if let Some(wv) = &tab.webview {
                execute_navigation(wv, &NavigationAction::LoadUrl(target_url))?;
            }
        }
        self.sync_sidebar_ui();
        Ok(())
    }

    pub fn reload_active(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(wv) = self.active_webview() {
            execute_navigation(wv, &NavigationAction::Reload)?;
        }
        Ok(())
    }

    pub fn back_active(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(wv) = self.active_webview() {
            execute_navigation(wv, &NavigationAction::Back)?;
        }
        Ok(())
    }

    pub fn forward_active(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(wv) = self.active_webview() {
            execute_navigation(wv, &NavigationAction::Forward)?;
        }
        Ok(())
    }

    /// Cycles the sidebar layout mode between Expanded and Compact.
    pub fn toggle_sidebar(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let new_mode = self.layout.toggle_mode();
        let mode_str = match new_mode {
            SidebarMode::Expanded => "expanded",
            SidebarMode::Compact => "compact",
            SidebarMode::Hidden => "hidden",
            SidebarMode::Autohide => "autohide",
        };

        if let Some(sb) = &self.sidebar_webview {
            let script = format!("window.__shoclaw_set_mode('{}');", mode_str);
            let _ = sb.evaluate_script(&script);
        }

        let window_size = self.window.inner_size();
        let scale = self.window.scale_factor();
        let logical = window_size.to_logical::<f64>(scale);
        self.handle_resize(logical)?;
        self.sync_sidebar_ui();
        Ok(())
    }

    /// Toggles the sidebar between Hidden (0px full-bleed) and Expanded.
    pub fn toggle_sidebar_hidden(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let new_mode = self.layout.toggle_hidden();
        let mode_str = match new_mode {
            SidebarMode::Expanded => "expanded",
            SidebarMode::Compact => "compact",
            SidebarMode::Hidden => "hidden",
            SidebarMode::Autohide => "autohide",
        };

        if let Some(sb) = &self.sidebar_webview {
            let script = format!("window.__shoclaw_set_mode('{}');", mode_str);
            let _ = sb.evaluate_script(&script);
        }

        let window_size = self.window.inner_size();
        let scale = self.window.scale_factor();
        let logical = window_size.to_logical::<f64>(scale);
        self.handle_resize(logical)?;
        self.sync_sidebar_ui();
        Ok(())
    }

    /// Handles window resizing and geometry allocation for all active views.
    pub fn handle_resize(&self, size: LogicalSize<f64>) -> Result<(), Box<dyn std::error::Error>> {
        let (sidebar_rect, content_rect) = self.layout.compute_bounds(size.width, size.height);

        if let Some(sb) = &self.sidebar_webview {
            self.sync_sidebar_bounds(sb, sidebar_rect, self.layout.is_sidebar_visible());
        }

        if let Some(wv) = self.active_webview() {
            self.sync_content_bounds(wv, content_rect);
        }

        if let Some(pwv) = &self.palette_webview {
            self.sync_palette_bounds(pwv, size.width, size.height, self.is_palette_open);
        }

        #[cfg(target_os = "linux")]
        {
            use gtk::prelude::*;
            self.fixed.queue_resize();
        }

        Ok(())
    }

    /// Toggles the floating Command Palette (Omnibar) open or closed.
    pub fn toggle_palette(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_palette_open {
            self.hide_palette();
        } else {
            self.show_palette()?;
        }
        Ok(())
    }

    /// Shows the floating Command Palette and updates its list of open tabs.
    pub fn show_palette(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_palette_open = true;

        let window_size = self.window.inner_size();
        let scale = self.window.scale_factor();
        let logical = window_size.to_logical::<f64>(scale);

        if let Some(pwv) = &self.palette_webview {
            self.sync_palette_bounds(pwv, logical.width, logical.height, true);

            #[derive(serde::Serialize)]
            struct TabPaletteDto<'a> {
                id: u64,
                title: &'a str,
                url: &'a str,
                state: &'static str,
            }

            let tabs_dto: Vec<TabPaletteDto> = self
                .tabs
                .tabs
                .iter()
                .map(|t| TabPaletteDto {
                    id: t.id,
                    title: &t.title,
                    url: &t.url,
                    state: match t.state {
                        TabState::Active => "active",
                        TabState::Idle => "idle",
                        TabState::Sleeping => "sleeping",
                    },
                })
                .collect();

            let payload = serde_json::json!({
                "tabs": tabs_dto,
                "active_id": self.tabs.active_tab_id,
            });

            let script = format!("if (window.__shoclaw_open_palette) {{ window.__shoclaw_open_palette({}); }}", payload);
            let _ = pwv.evaluate_script(&script);

            #[cfg(target_os = "linux")]
            {
                let widget = pwv.webview();
                if let Some(w) = widget.window() {
                    w.raise();
                }
                widget.grab_focus();
                self.fixed.queue_resize();
            }
        }

        Ok(())
    }

    /// Hides the floating Command Palette and restores keyboard focus to content.
    pub fn hide_palette(&mut self) {
        self.is_palette_open = false;

        let window_size = self.window.inner_size();
        let scale = self.window.scale_factor();
        let logical = window_size.to_logical::<f64>(scale);

        if let Some(pwv) = &self.palette_webview {
            let _ = pwv.evaluate_script("if (window.__shoclaw_close_palette) { window.__shoclaw_close_palette(); }");
            self.sync_palette_bounds(pwv, logical.width, logical.height, false);
            #[cfg(target_os = "linux")]
            self.fixed.queue_resize();
        }

        if let Some(wv) = self.active_webview() {
            #[cfg(target_os = "linux")]
            wv.webview().grab_focus();
        }
    }

    /// Executes an internal browser command triggered from the Command Palette.
    pub fn execute_command(&mut self, cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.hide_palette();
        println!("[Command Palette] Executing command: {}", cmd);
        match cmd {
            "sleep_all" | "hibernate_all" => {
                self.hibernate_all_background_tabs();
            }
            "sidebar_expanded" => {
                self.layout.mode = SidebarMode::Expanded;
                self.update_sidebar_mode()?;
            }
            "sidebar_compact" => {
                self.layout.mode = SidebarMode::Compact;
                self.update_sidebar_mode()?;
            }
            "sidebar_hidden" => {
                self.layout.mode = SidebarMode::Hidden;
                self.update_sidebar_mode()?;
            }
            "new_tab" => {
                let home = self.homepage.clone();
                self.open_tab(&home)?;
            }
            "close_tab" => {
                self.close_active_tab()?;
            }
            "reload" => {
                self.reload_active()?;
            }
            "go_back" => {
                self.back_active()?;
            }
            "go_forward" => {
                self.forward_active()?;
            }
            "quit" => {
                let _ = self.proxy.send_event(AppEvent::Quit);
            }
            _ => {
                eprintln!("[Command Palette] Unknown command: {}", cmd);
            }
        }
        Ok(())
    }

    /// Helper to update the sidebar mode in layout and DOM.
    pub fn update_sidebar_mode(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mode_str = match self.layout.mode {
            SidebarMode::Expanded => "expanded",
            SidebarMode::Compact => "compact",
            SidebarMode::Hidden => "hidden",
            SidebarMode::Autohide => "autohide",
        };

        if let Some(sb) = &self.sidebar_webview {
            let script = format!("window.__shoclaw_set_mode('{}');", mode_str);
            let _ = sb.evaluate_script(&script);
        }

        let window_size = self.window.inner_size();
        let scale = self.window.scale_factor();
        let logical = window_size.to_logical::<f64>(scale);
        self.handle_resize(logical)?;
        self.sync_sidebar_ui();
        Ok(())
    }

    /// Force hibernates all background tabs to 0 MB RAM, keeping only active tab.
    pub fn hibernate_all_background_tabs(&mut self) {
        let active_id = self.tabs.active_tab_id;
        for tab in self.tabs.tabs.iter_mut() {
            if Some(tab.id) != active_id && !tab.is_audio_playing {
                println!("[RAM Guard] Force Hibernating Tab {} ({}) to 0 MB RAM", tab.id, tab.url);
                tab.detach_webview();
            }
        }
        self.dump_memory_summary();
        self.sync_sidebar_ui();
    }

    /// Synchronizes the current tab list and active selection with the sidebar UI shell.
    pub fn sync_sidebar_ui(&self) {
        if let Some(sb) = &self.sidebar_webview {
            let active_id = self.tabs.active_tab_id.unwrap_or(0);

            #[derive(serde::Serialize)]
            struct TabDto<'a> {
                id: u64,
                title: &'a str,
                url: &'a str,
                state: &'static str,
                is_audio_playing: bool,
            }

            let tabs_dto: Vec<TabDto> = self
                .tabs
                .tabs
                .iter()
                .map(|t| {
                    let state_str = match t.state {
                        TabState::Active => "Active",
                        TabState::Idle => "Idle",
                        TabState::Sleeping => "Sleeping",
                    };
                    TabDto {
                        id: t.id,
                        title: &t.title,
                        url: &t.url,
                        state: state_str,
                        is_audio_playing: t.is_audio_playing,
                    }
                })
                .collect();

            if let Ok(json) = serde_json::to_string(&tabs_dto) {
                let script = format!(
                    "if (window.__shoclaw_update_tabs) {{ window.__shoclaw_update_tabs({}, {}); }}",
                    json, active_id
                );
                let _ = sb.evaluate_script(&script);
            }
        }
    }

    /// Updates the audio playback state for a given tab.
    pub fn set_tab_audio_state(&mut self, tab_id: u64, playing: bool) {
        if let Ok(mut states) = self.audio_states.lock() {
            states.insert(tab_id, playing);
        }
        if let Some(tab) = self.tabs.get_tab_mut(tab_id) {
            tab.is_audio_playing = playing;
        }
        self.sync_sidebar_ui();
    }

    /// Updates tab title and synchronizes the window / sidebar.
    pub fn update_tab_title(&mut self, tab_id: u64, title: String) {
        if let Some(tab) = self.tabs.get_tab_mut(tab_id) {
            tab.title = title.clone();
        }
        if self.tabs.active_tab_id == Some(tab_id) {
            let formatted_title = format!("{} - Shoclaw", title);
            self.window.set_title(&formatted_title);
        }
        self.sync_sidebar_ui();
    }

    pub fn dump_memory_summary(&self) {
        println!("--------------------------------------------------");
        println!(
            "Shoclaw Memory Status: {} total tabs | {} loaded in RAM | {} sleeping (0 MB)",
            self.tabs.tab_count(),
            self.tabs.active_memory_count(),
            self.tabs.sleeping_tab_count()
        );
        for tab in &self.tabs.tabs {
            let marker = if self.tabs.active_tab_id == Some(tab.id) {
                "[ACTIVE]"
            } else if tab.is_sleeping() {
                "[SLEEPING (0 MB)]"
            } else {
                "[IDLE (WARM)]"
            };
            println!("  * Tab #{} {} - {}", tab.id, marker, tab.url);
        }
        println!("--------------------------------------------------");
    }
}
