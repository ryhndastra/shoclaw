use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder},
};
use wry::{Rect, WebView, WebViewBuilder};

#[cfg(target_os = "linux")]
use tao::platform::unix::WindowExtUnix;
#[cfg(target_os = "linux")]
use wry::WebViewBuilderExtUnix;

use crate::tab::TabManager;
use super::navigation::{execute_navigation, normalize_url, NavigationAction};

pub struct BrowserWindow {
    pub window: Window,
    pub tabs: TabManager,
    pub current_title: Arc<Mutex<String>>,
    pub audio_states: Arc<Mutex<HashMap<u64, bool>>>,
}

impl BrowserWindow {
    pub fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        initial_url: &str,
        max_active_tabs: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let window = WindowBuilder::new()
            .with_title("Shoclaw")
            .with_inner_size(LogicalSize::new(1280.0, 800.0))
            .with_min_inner_size(LogicalSize::new(640.0, 480.0))
            .build(event_loop)?;

        let current_title = Arc::new(Mutex::new("Shoclaw".to_string()));
        let audio_states = Arc::new(Mutex::new(HashMap::new()));
        let tabs = TabManager::new(max_active_tabs);

        let mut browser_window = Self {
            window,
            tabs,
            current_title,
            audio_states,
        };

        browser_window.open_tab(initial_url)?;
        Ok(browser_window)
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

    /// Spawns a WebKit WebView instance for a given tab ID and URL.
    fn spawn_webview(&self, tab_id: u64, url: &str) -> Result<WebView, Box<dyn std::error::Error>> {
        let title_clone = self.current_title.clone();
        let audio_states_clone = self.audio_states.clone();

        // Audio detection script injected into all pages
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
            })();
        "#;

        let webview_builder = WebViewBuilder::new()
            .with_url(url)
            .with_devtools(cfg!(debug_assertions))
            .with_initialization_script(audio_script)
            .with_document_title_changed_handler(move |title| {
                if let Ok(mut lock) = title_clone.lock() {
                    *lock = title;
                }
            })
            .with_ipc_handler(move |req| {
                let body = req.body();
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
                    if val.get("type").and_then(|v| v.as_str()) == Some("audio_state") {
                        let playing = val.get("playing").and_then(|v| v.as_bool()).unwrap_or(false);
                        println!("[Audio Guard] Tab {} audio playing: {}", tab_id, playing);
                        if let Ok(mut states) = audio_states_clone.lock() {
                            states.insert(tab_id, playing);
                        }
                    }
                }
            });

        #[cfg(target_os = "linux")]
        let webview = {
            let vbox = self.window
                .default_vbox()
                .ok_or("Failed to retrieve default_vbox from window")?;
            webview_builder.build_gtk(vbox)?
        };

        #[cfg(not(target_os = "linux"))]
        let webview = webview_builder.build(&self.window)?;

        Ok(webview)
    }

    /// Opens a new tab, spawns its WebKit view, and hibernates excess tabs.
    pub fn open_tab(&mut self, url: &str) -> Result<u64, Box<dyn std::error::Error>> {
        self.sync_audio_states();

        // Hide currently active webview
        if let Some(active_tab) = self.tabs.active_tab() {
            if let Some(wv) = &active_tab.webview {
                let _ = wv.set_visible(false);
            }
        }

        let (new_id, hibernated_ids) = self.tabs.create_tab(url);
        let webview = self.spawn_webview(new_id, url)?;
        let _ = webview.set_visible(true);

        if let Some(tab) = self.tabs.get_tab_mut(new_id) {
            tab.attach_webview(webview);
        }

        // Drop WebKit instances for any tabs that exceeded max_active_tabs
        for hib_id in hibernated_ids {
            if let Some(tab) = self.tabs.get_tab_mut(hib_id) {
                println!(
                    "[RAM Guard] Hibernating Tab {} ({}) to 0 MB RAM",
                    tab.id, tab.url
                );
                tab.detach_webview(); // WebKit instance dropped
            }
        }

        self.dump_memory_summary();
        Ok(new_id)
    }

    /// Switches to an existing tab, waking it from sleep if needed.
    pub fn switch_tab(&mut self, target_id: u64) -> Result<bool, Box<dyn std::error::Error>> {
        if self.tabs.active_tab_id == Some(target_id) {
            return Ok(true);
        }

        self.sync_audio_states();

        // Hide current active tab's webview
        if let Some(curr_active) = self.tabs.active_tab() {
            if let Some(wv) = &curr_active.webview {
                let _ = wv.set_visible(false);
            }
        }

        // If target tab is sleeping (webview dropped), reconstruct WebKit instance
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
                .unwrap_or_else(|| "https://google.com".to_string());

            println!(
                "[RAM Guard] Waking sleeping Tab {} ({}) from cache",
                target_id, url
            );
            let webview = self.spawn_webview(target_id, &url)?;
            let _ = webview.set_visible(true);

            if let Some(tab) = self.tabs.get_tab_mut(target_id) {
                tab.attach_webview(webview);
            }
        } else if let Some(tab) = self.tabs.get_tab(target_id) {
            if let Some(wv) = &tab.webview {
                let _ = wv.set_visible(true);
            }
        }

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

        self.dump_memory_summary();
        Ok(success)
    }

    /// Closes the active tab and switches to a neighbor.
    pub fn close_active_tab(&mut self) -> Result<Option<u64>, Box<dyn std::error::Error>> {
        let active_id = match self.tabs.active_tab_id {
            Some(id) => id,
            None => return Ok(None),
        };

        if let Ok(mut states) = self.audio_states.lock() {
            states.remove(&active_id);
        }

        let next_active = self.tabs.close_tab(active_id);

        if let Some(next_id) = next_active {
            // Wake or show next active tab
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
                    .unwrap_or_else(|| "https://google.com".to_string());
                let webview = self.spawn_webview(next_id, &url)?;
                let _ = webview.set_visible(true);

                if let Some(tab) = self.tabs.get_tab_mut(next_id) {
                    tab.attach_webview(webview);
                }
            } else if let Some(tab) = self.tabs.get_tab(next_id) {
                if let Some(wv) = &tab.webview {
                    let _ = wv.set_visible(true);
                }
            }
        }

        self.dump_memory_summary();
        Ok(next_active)
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

    pub fn navigate_active(&self, input: &str, search_template: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(wv) = self.active_webview() {
            let target_url = normalize_url(input, search_template);
            execute_navigation(wv, &NavigationAction::LoadUrl(target_url))?;
        }
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

    pub fn handle_resize(&self, size: LogicalSize<f64>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(wv) = self.active_webview() {
            wv.set_bounds(Rect {
                position: tao::dpi::Position::Logical(LogicalPosition::new(0.0, 0.0)),
                size: tao::dpi::Size::Logical(size),
            })?;
        }
        Ok(())
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
