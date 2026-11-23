use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabState {
    Active,
    Idle,
    Sleeping,
}

#[derive(Debug)]
pub struct Tab {
    pub id: u64,
    pub title: String,
    pub url: String,
    pub state: TabState,
    pub scroll_pos: (i32, i32),
    pub is_pinned: bool,
    pub is_audio_playing: bool,
    pub last_accessed: Instant,
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
    }

    pub fn is_sleeping(&self) -> bool {
        self.state == TabState::Sleeping
    }

    pub fn can_hibernate(&self) -> bool {
        // Pinned tabs or tabs actively playing audio must never hibernate
        !self.is_pinned && !self.is_audio_playing && self.state != TabState::Active
    }
}
