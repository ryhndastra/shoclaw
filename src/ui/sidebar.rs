pub const SIDEBAR_HTML: &str = include_str!("../../assets/shoclaw_sidebar_250px.html");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sidebar_html_integrity() {
        assert!(SIDEBAR_HTML.contains("Shoclaw Sidebar"));
        assert!(SIDEBAR_HTML.contains("__shoclaw_update_tabs"));
        assert!(SIDEBAR_HTML.contains("__shoclaw_set_mode"));
        assert!(SIDEBAR_HTML.contains("toggle_sidebar"));
        assert!(SIDEBAR_HTML.contains("switch_tab"));
        assert!(SIDEBAR_HTML.contains("new_tab"));
    }
}
