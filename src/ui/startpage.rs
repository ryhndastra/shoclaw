pub const STARTPAGE_HTML: &str = include_str!("../../assets/shoclaw_startpage.html");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startpage_html_integrity() {
        assert!(STARTPAGE_HTML.contains("<!DOCTYPE html>"));
        assert!(STARTPAGE_HTML.contains("Shoclaw"));
        assert!(STARTPAGE_HTML.contains("shoclaw-search-input"));
        assert!(STARTPAGE_HTML.contains("clock-time"));
        assert!(STARTPAGE_HTML.contains("window.ipc.postMessage"));
        assert!(STARTPAGE_HTML.contains("sendIPC"));
        assert!(STARTPAGE_HTML.contains("toggle_palette"));
    }
}
