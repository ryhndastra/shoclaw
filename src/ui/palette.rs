pub const PALETTE_HTML: &str = include_str!("../../assets/shoclaw_palette.html");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_html_integrity() {
        assert!(PALETTE_HTML.contains("Shoclaw Omnibar"));
        assert!(PALETTE_HTML.contains("palette-modal"));
        assert!(PALETTE_HTML.contains("palette-input"));
        assert!(PALETTE_HTML.contains("results-list"));
        assert!(PALETTE_HTML.contains("window.__shoclaw_open_palette"));
    }

    #[test]
    fn test_palette_built_in_commands_present() {
        assert!(PALETTE_HTML.contains("> sleep all"));
        assert!(PALETTE_HTML.contains("> sidebar expanded"));
        assert!(PALETTE_HTML.contains("> sidebar compact"));
        assert!(PALETTE_HTML.contains("> sidebar hidden"));
        assert!(PALETTE_HTML.contains("> new tab"));
        assert!(PALETTE_HTML.contains("> close tab"));
        assert!(PALETTE_HTML.contains("> reload"));
        assert!(PALETTE_HTML.contains("> back"));
        assert!(PALETTE_HTML.contains("> forward"));
        assert!(PALETTE_HTML.contains("> quit"));
    }

    #[test]
    fn test_palette_ipc_actions_wired() {
        assert!(PALETTE_HTML.contains("type: 'close_palette'"));
        assert!(PALETTE_HTML.contains("type: 'switch_tab'"));
        assert!(PALETTE_HTML.contains("type: 'navigate'"));
        assert!(PALETTE_HTML.contains("type: 'command'"));
    }
}
