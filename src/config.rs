use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SidebarMode {
    Hidden,
    Autohide,
    Compact,
    Expanded,
}

impl Default for SidebarMode {
    fn default() -> Self {
        SidebarMode::Autohide
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default)]
    pub sidebar_mode: SidebarMode,
    #[serde(default = "default_homepage")]
    pub homepage: String,
    #[serde(default = "default_search_engine")]
    pub search_engine: String,
}

fn default_homepage() -> String {
    "https://duckduckgo.com".to_string()
}

fn default_search_engine() -> String {
    "https://duckduckgo.com/?q={}".to_string()
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            sidebar_mode: SidebarMode::default(),
            homepage: default_homepage(),
            search_engine: default_search_engine(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    #[serde(default = "default_max_active_tabs")]
    pub max_active_tabs: usize,
    #[serde(default = "default_hibernate_after_minutes")]
    pub hibernate_after_minutes: u64,
}

fn default_max_active_tabs() -> usize {
    3
}

fn default_hibernate_after_minutes() -> u64 {
    10
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_active_tabs: default_max_active_tabs(),
            hibernate_after_minutes: default_hibernate_after_minutes(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_font")]
    pub font: String,
    #[serde(default = "default_border_radius")]
    pub border_radius: String,
    #[serde(default = "default_blur_effect")]
    pub blur_effect: bool,
}

fn default_theme() -> String {
    "dark".to_string()
}

fn default_font() -> String {
    "Inter".to_string()
}

fn default_border_radius() -> String {
    "12px".to_string()
}

fn default_blur_effect() -> bool {
    true
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            font: default_font(),
            border_radius: default_border_radius(),
            blur_effect: default_blur_effect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingsConfig {
    #[serde(default = "default_cmd_palette")]
    pub command_palette: String,
    #[serde(default = "default_new_tab")]
    pub new_tab: String,
    #[serde(default = "default_close_tab")]
    pub close_tab: String,
    #[serde(default = "default_toggle_sidebar")]
    pub toggle_sidebar: String,
    #[serde(default = "default_next_tab")]
    pub next_tab: String,
    #[serde(default = "default_prev_tab")]
    pub prev_tab: String,
}

fn default_cmd_palette() -> String {
    "Ctrl+K".to_string()
}

fn default_new_tab() -> String {
    "Ctrl+T".to_string()
}

fn default_close_tab() -> String {
    "Ctrl+W".to_string()
}

fn default_toggle_sidebar() -> String {
    "Ctrl+B".to_string()
}

fn default_next_tab() -> String {
    "Ctrl+Tab".to_string()
}

fn default_prev_tab() -> String {
    "Ctrl+Shift+Tab".to_string()
}

impl Default for KeybindingsConfig {
    fn default() -> Self {
        Self {
            command_palette: default_cmd_palette(),
            new_tab: default_new_tab(),
            close_tab: default_close_tab(),
            toggle_sidebar: default_toggle_sidebar(),
            next_tab: default_next_tab(),
            prev_tab: default_prev_tab(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub performance: PerformanceConfig,
    #[serde(default)]
    pub appearance: AppearanceConfig,
    #[serde(default)]
    pub keybindings: KeybindingsConfig,
}

impl Config {
    /// Resolves the default configuration file location (`~/.config/shoclaw/config.toml`).
    pub fn default_config_path() -> PathBuf {
        let base_dir = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
            .unwrap_or_else(|| PathBuf::from("."));

        base_dir.join("shoclaw").join("config.toml")
    }

    /// Loads the configuration from the default path or falls back to standard defaults.
    pub fn load() -> Self {
        let path = Self::default_config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(config) = toml::from_str::<Config>(&content) {
                    return config;
                }
            }
        }
        Config::default()
    }

    /// Loads configuration from a given file path.
    pub fn from_file(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Serializes and writes the current configuration to a given file path.
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_values() {
        let config = Config::default();
        assert_eq!(config.general.sidebar_mode, SidebarMode::Autohide);
        assert_eq!(config.general.homepage, "https://duckduckgo.com");
        assert_eq!(config.performance.max_active_tabs, 3);
        assert_eq!(config.performance.hibernate_after_minutes, 10);
        assert_eq!(config.appearance.theme, "dark");
        assert_eq!(config.appearance.blur_effect, true);
        assert_eq!(config.keybindings.command_palette, "Ctrl+K");
    }

    #[test]
    fn test_toml_serialization_roundtrip() {
        let original = Config::default();
        let toml_str = toml::to_string_pretty(&original).expect("Failed to serialize to TOML");
        let parsed: Config = toml::from_str(&toml_str).expect("Failed to deserialize TOML");

        assert_eq!(parsed.general.sidebar_mode, original.general.sidebar_mode);
        assert_eq!(parsed.performance.max_active_tabs, original.performance.max_active_tabs);
        assert_eq!(parsed.keybindings.command_palette, original.keybindings.command_palette);
    }

    #[test]
    fn test_partial_toml_uses_defaults() {
        let partial_toml = r#"
            [general]
            sidebar_mode = "compact"
            
            [performance]
            max_active_tabs = 5
        "#;

        let parsed: Config = toml::from_str(partial_toml).expect("Failed to parse partial TOML");
        assert_eq!(parsed.general.sidebar_mode, SidebarMode::Compact);
        assert_eq!(parsed.general.homepage, "https://duckduckgo.com");
        assert_eq!(parsed.performance.max_active_tabs, 5);
        assert_eq!(parsed.performance.hibernate_after_minutes, 10);
        assert_eq!(parsed.appearance.theme, "dark");
    }
}
