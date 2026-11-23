use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use wry::WebView;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavigationAction {
    LoadUrl(String),
    Reload,
    Back,
    Forward,
    Stop,
}

/// Normalizes user input into a valid URL or search engine query.
///
/// If the input is already a complete URL or resembles a domain name (e.g. `github.com`),
/// it is formatted into an HTTP/HTTPS URL. Otherwise, it is encoded as a search query.
pub fn normalize_url(input: &str, search_template: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return "about:blank".to_string();
    }

    // Direct protocol matches
    if trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
        || trimmed.starts_with("file://")
        || trimmed.starts_with("about:")
    {
        return trimmed.to_string();
    }

    // Localhost special casing
    if trimmed.starts_with("localhost") || trimmed.starts_with("127.0.0.1") {
        return format!("http://{}", trimmed);
    }

    // Check if input looks like a domain name (contains dot, no spaces)
    let has_whitespace = trimmed.chars().any(char::is_whitespace);
    let parts: Vec<&str> = trimmed.split('/').collect();
    let host_candidate = parts.first().copied().unwrap_or("");

    if !has_whitespace && host_candidate.contains('.') && host_candidate.len() > 3 {
        return format!("https://{}", trimmed);
    }

    // Fall back to search engine query
    let encoded_query = utf8_percent_encode(trimmed, NON_ALPHANUMERIC).to_string();
    if search_template.contains("{}") {
        search_template.replace("{}", &encoded_query)
    } else {
        format!("{}/?q={}", search_template.trim_end_matches('/'), encoded_query)
    }
}

/// Executes a navigation action on an active WebKit WebView instance.
pub fn execute_navigation(webview: &WebView, action: &NavigationAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        NavigationAction::LoadUrl(url) => {
            webview.load_url(url)?;
        }
        NavigationAction::Reload => {
            let _ = webview.evaluate_script("window.location.reload();");
        }
        NavigationAction::Back => {
            let _ = webview.evaluate_script("window.history.back();");
        }
        NavigationAction::Forward => {
            let _ = webview.evaluate_script("window.history.forward();");
        }
        NavigationAction::Stop => {
            let _ = webview.evaluate_script("window.stop();");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DUCKDUCKGO: &str = "https://duckduckgo.com/?q={}";

    #[test]
    fn test_direct_url_preserved() {
        assert_eq!(
            normalize_url("https://github.com/ryhndastra", DUCKDUCKGO),
            "https://github.com/ryhndastra"
        );
        assert_eq!(
            normalize_url("http://insecure.example.com", DUCKDUCKGO),
            "http://insecure.example.com"
        );
    }

    #[test]
    fn test_domain_names_auto_prefix_https() {
        assert_eq!(
            normalize_url("github.com", DUCKDUCKGO),
            "https://github.com"
        );
        assert_eq!(
            normalize_url("crates.io/crates/wry", DUCKDUCKGO),
            "https://crates.io/crates/wry"
        );
    }

    #[test]
    fn test_localhost_uses_http() {
        assert_eq!(
            normalize_url("localhost:3000", DUCKDUCKGO),
            "http://localhost:3000"
        );
        assert_eq!(
            normalize_url("127.0.0.1:8080/api", DUCKDUCKGO),
            "http://127.0.0.1:8080/api"
        );
    }

    #[test]
    fn test_search_queries_encoded() {
        assert_eq!(
            normalize_url("rust async await", DUCKDUCKGO),
            "https://duckduckgo.com/?q=rust%20async%20await"
        );
        assert_eq!(
            normalize_url("how to use wry?", DUCKDUCKGO),
            "https://duckduckgo.com/?q=how%20to%20use%20wry%3F"
        );
    }

    #[test]
    fn test_empty_input_defaults_to_blank() {
        assert_eq!(normalize_url("", DUCKDUCKGO), "about:blank");
        assert_eq!(normalize_url("   ", DUCKDUCKGO), "about:blank");
    }
}
