pub mod browser;
pub mod config;
pub mod tab;
pub mod ui;

use browser::{AppEvent, BrowserWindow};
use config::Config;
use tao::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    keyboard::{KeyCode, ModifiersState},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    {
        // Software renderer fallback when explicitly requested.
        if std::env::var_os("SHOCLAW_SOFTWARE_RENDER").is_some() {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
            println!("[Renderer] Software rendering forced via SHOCLAW_SOFTWARE_RENDER");
        }

        // Locate glib-networking TLS module on NixOS.
        if std::env::var_os("GIO_MODULE_DIR").is_none() {
            if let Ok(entries) = std::fs::read_dir("/nix/store") {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    if name_str.contains("glib-networking")
                        && !name_str.ends_with(".drv")
                        && !name_str.ends_with(".tar.xz")
                    {
                        let candidate = entry.path().join("lib/gio/modules");
                        if candidate.join("libgiognutls.so").exists() {
                            std::env::set_var("GIO_MODULE_DIR", &candidate);
                            break;
                        }
                    }
                }
            }
        }

        // Discover GStreamer plugins for media playback on NixOS.
        if std::env::var_os("GST_PLUGIN_SYSTEM_PATH_1_0").is_none() {
            let mut gst_paths = Vec::new();
            if let Ok(entries) = std::fs::read_dir("/nix/store") {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    if (name_str.contains("gst-plugins-base")
                        || name_str.contains("gst-plugins-good")
                        || name_str.contains("gst-plugins-bad")
                        || name_str.contains("gst-plugins-ugly")
                        || name_str.contains("gstreamer-"))
                        && !name_str.ends_with(".drv")
                        && !name_str.ends_with(".tar.xz")
                    {
                        let plugin_dir = entry.path().join("lib/gstreamer-1.0");
                        if plugin_dir.exists() && !gst_paths.contains(&plugin_dir.to_string_lossy().to_string()) {
                            gst_paths.push(plugin_dir.to_string_lossy().to_string());
                        }
                    }
                }
            }
            if !gst_paths.is_empty() {
                let joined = gst_paths.join(":");
                std::env::set_var("GST_PLUGIN_SYSTEM_PATH_1_0", joined);
            }
        }
    }

    println!("Shoclaw Initializing...");

    let config = Config::load();
    let event_loop = EventLoopBuilder::<AppEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let mut browser_window = BrowserWindow::new(
        &event_loop,
        proxy,
        &config.general.homepage,
        config.performance.max_active_tabs,
        config.general.sidebar_mode,
    )?;

    println!(
        "Shoclaw running with max_active_tabs = {}, sidebar_mode = {:?}",
        config.performance.max_active_tabs, config.general.sidebar_mode
    );

    let mut modifiers = ModifiersState::empty();
    let homepage = config.general.homepage.clone();
    let search_engine = config.general.search_engine.clone();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(app_event) => match app_event {
                AppEvent::NewTab => {
                    let _ = browser_window.open_tab(&homepage);
                }
                AppEvent::SwitchTab(id) => {
                    let _ = browser_window.switch_tab(id);
                }
                AppEvent::CloseTab(id) => {
                    if let Ok(Some(_)) = browser_window.close_tab(id) {
                        // Tab closed and neighbor activated
                    } else if browser_window.tabs.tab_count() == 0 {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                AppEvent::ToggleSidebar => {
                    let _ = browser_window.toggle_sidebar();
                }
                AppEvent::ToggleSidebarHidden => {
                    let _ = browser_window.toggle_sidebar_hidden();
                }
                AppEvent::Navigate(url) => {
                    let _ = browser_window.navigate_active(&url, &search_engine);
                }
                AppEvent::GoBack => {
                    let _ = browser_window.back_active();
                }
                AppEvent::GoForward => {
                    let _ = browser_window.forward_active();
                }
                AppEvent::Reload => {
                    let _ = browser_window.reload_active();
                }
                AppEvent::AudioState { tab_id, playing } => {
                    browser_window.set_tab_audio_state(tab_id, playing);
                }
                AppEvent::TitleChanged { tab_id, title } => {
                    browser_window.update_tab_title(tab_id, title);
                }
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("Shoclaw: Close requested, exiting cleanly.");
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(physical_size) => {
                    let scale = browser_window.window.scale_factor();
                    let logical_size = physical_size.to_logical::<f64>(scale);
                    let _ = browser_window.handle_resize(logical_size);
                }
                WindowEvent::ModifiersChanged(new_modifiers) => {
                    modifiers = new_modifiers;
                }
                WindowEvent::KeyboardInput {
                    event: key_event,
                    ..
                } => {
                    if key_event.state == ElementState::Pressed {
                        if modifiers.control_key() && key_event.physical_key == KeyCode::KeyT {
                            let _ = browser_window.open_tab(&homepage);
                        } else if modifiers.control_key() && key_event.physical_key == KeyCode::KeyW {
                            if browser_window.close_active_tab().ok().flatten().is_none() {
                                println!("No tabs left, exiting application.");
                                *control_flow = ControlFlow::Exit;
                            }
                        } else if modifiers.control_key() && !modifiers.shift_key() && key_event.physical_key == KeyCode::KeyB {
                            let _ = browser_window.toggle_sidebar();
                        } else if modifiers.control_key() && modifiers.shift_key() && key_event.physical_key == KeyCode::KeyB {
                            let _ = browser_window.toggle_sidebar_hidden();
                        } else if modifiers.control_key() && !modifiers.shift_key() && key_event.physical_key == KeyCode::Tab {
                            let _ = browser_window.next_tab();
                        } else if modifiers.control_key() && modifiers.shift_key() && key_event.physical_key == KeyCode::Tab {
                            let _ = browser_window.prev_tab();
                        } else if modifiers.control_key() && key_event.physical_key == KeyCode::KeyR {
                            let _ = browser_window.reload_active();
                        } else if modifiers.alt_key() && key_event.physical_key == KeyCode::ArrowLeft {
                            let _ = browser_window.back_active();
                        } else if modifiers.alt_key() && key_event.physical_key == KeyCode::ArrowRight {
                            let _ = browser_window.forward_active();
                        } else if modifiers.control_key() && key_event.physical_key == KeyCode::KeyH {
                            browser_window.dump_memory_summary();
                        }
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {}
            _ => {}
        }
    });
}
