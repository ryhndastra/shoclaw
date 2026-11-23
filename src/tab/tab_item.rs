use std::fmt;
use std::time::Instant;
use wry::WebView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabState {
    Active,
    Idle,
    Sleeping,
}

pub struct Tab {
    pub id: u64,
    pub title: String,
    pub url: String,
    pub state: TabState,
    pub scroll_pos: (i32, i32),
    pub is_pinned: bool,
    pub is_audio_playing: bool,
    pub last_accessed: Instant,
    pub webview: Option<WebView>,
}

impl fmt::Debug for Tab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tab")
            .field("id", &self.id)
            .field("title", &self.title)
            .field("url", &self.url)
            .field("state", &self.state)
            .field("scroll_pos", &self.scroll_pos)
            .field("is_pinned", &self.is_pinned)
            .field("is_audio_playing", &self.is_audio_playing)
            .field("has_webview", &self.webview.is_some())
            .finish()
    }
}

impl Tab {
    pub fn new(id: u64, url: impl Into<String>) -> Self {
        let url = url.into();
        Self {
            id,
            title: url.clone(),
            url,
            state: TabState::Active,
            scroll_pos: (0, 0),
            is_pinned: false,
            is_audio_playing: false,
            last_accessed: Instant::now(),
            webview: None,
        }
    }

    pub fn mark_active(&mut self) {
        self.state = TabState::Active;
        self.last_accessed = Instant::now();
    }

    pub fn mark_idle(&mut self) {
        if self.state == TabState::Active {
            self.state = TabState::Idle;
        }
    }

    pub fn mark_sleeping(&mut self) {
        self.state = TabState::Sleeping;
        self.webview = None; // Explicit drop: WebKit destroys DOM and releases RAM
    }

    pub fn is_sleeping(&self) -> bool {
        self.state == TabState::Sleeping
    }

    pub fn can_hibernate(&self) -> bool {
        // Pinned tabs or tabs actively playing audio must never hibernate
        !self.is_pinned && !self.is_audio_playing && self.state != TabState::Active
    }

    pub fn attach_webview(&mut self, webview: WebView) {
        self.webview = Some(webview);
        self.state = TabState::Active;
        self.last_accessed = Instant::now();
    }

    pub fn detach_webview(&mut self) {
        self.mark_sleeping();
    }
}
