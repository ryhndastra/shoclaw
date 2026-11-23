use super::tab_item::{Tab, TabState};

#[derive(Debug)]
pub struct TabManager {
    pub tabs: Vec<Tab>,
    pub active_tab_id: Option<u64>,
    next_id: u64,
    pub max_active_tabs: usize,
}

impl TabManager {
    pub fn new(max_active_tabs: usize) -> Self {
        Self {
            tabs: Vec::new(),
            active_tab_id: None,
            next_id: 1,
            max_active_tabs: max_active_tabs.max(1),
        }
    }

    pub fn create_tab(&mut self, url: impl Into<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        // Transition current active tab to Idle
        if let Some(active_id) = self.active_tab_id {
            if let Some(active_tab) = self.get_tab_mut(active_id) {
                active_tab.mark_idle();
            }
        }

        let new_tab = Tab::new(id, url);
        self.tabs.push(new_tab);
        self.active_tab_id = Some(id);

        self.enforce_hibernation_limit();
        id
    }

    pub fn switch_to_tab(&mut self, id: u64) -> bool {
        if !self.tabs.iter().any(|t| t.id == id) {
            return false;
        }

        // Demote current active tab to Idle
        if let Some(curr_active) = self.active_tab_id {
            if curr_active != id {
                if let Some(tab) = self.get_tab_mut(curr_active) {
                    tab.mark_idle();
                }
            }
        }

        // Promote requested tab to Active (waking it up if sleeping)
        if let Some(tab) = self.get_tab_mut(id) {
            tab.mark_active();
            self.active_tab_id = Some(id);
        }

        self.enforce_hibernation_limit();
        true
    }

    pub fn close_tab(&mut self, id: u64) -> Option<u64> {
        let index = self.tabs.iter().position(|t| t.id == id)?;
        self.tabs.remove(index);

        if self.tabs.is_empty() {
            self.active_tab_id = None;
            return None;
        }

        // If the closed tab was active, activate adjacent tab
        if self.active_tab_id == Some(id) {
            let next_index = if index >= self.tabs.len() {
                self.tabs.len() - 1
            } else {
                index
            };
            let next_id = self.tabs[next_index].id;
            self.tabs[next_index].mark_active();
            self.active_tab_id = Some(next_id);
            self.enforce_hibernation_limit();
            Some(next_id)
        } else {
            self.active_tab_id
        }
    }

    pub fn enforce_hibernation_limit(&mut self) {
        let loaded_count = self.tabs.iter().filter(|t| !t.is_sleeping()).count();
        if loaded_count <= self.max_active_tabs {
            return;
        }

        let excess = loaded_count - self.max_active_tabs;

        // Gather candidate tabs that can hibernate, sorted by oldest access (LRU)
        let mut candidates: Vec<(u64, std::time::Instant)> = self
            .tabs
            .iter()
            .filter(|t| t.can_hibernate())
            .map(|t| (t.id, t.last_accessed))
            .collect();

        candidates.sort_by_key(|&(_, accessed)| accessed);

        let to_hibernate: Vec<u64> = candidates.into_iter().take(excess).map(|(id, _)| id).collect();

        for id in to_hibernate {
            if let Some(tab) = self.get_tab_mut(id) {
                tab.mark_sleeping();
            }
        }
    }

    pub fn get_tab(&self, id: u64) -> Option<&Tab> {
        self.tabs.iter().find(|t| t.id == id)
    }

    pub fn get_tab_mut(&mut self, id: u64) -> Option<&mut Tab> {
        self.tabs.iter_mut().find(|t| t.id == id)
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.active_tab_id.and_then(|id| self.get_tab(id))
    }

    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    pub fn sleeping_tab_count(&self) -> usize {
        self.tabs.iter().filter(|t| t.is_sleeping()).count()
    }

    pub fn active_memory_count(&self) -> usize {
        self.tabs.iter().filter(|t| !t.is_sleeping()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_creation_and_activation() {
        let mut manager = TabManager::new(3);
        let tab1 = manager.create_tab("https://duckduckgo.com");
        let tab2 = manager.create_tab("https://github.com");

        assert_eq!(manager.tab_count(), 2);
        assert_eq!(manager.active_tab_id, Some(tab2));
        assert_eq!(manager.get_tab(tab1).unwrap().state, TabState::Idle);
        assert_eq!(manager.get_tab(tab2).unwrap().state, TabState::Active);
    }

    #[test]
    fn test_lru_hibernation_limit() {
        // Limit to 2 active tabs
        let mut manager = TabManager::new(2);
        let tab1 = manager.create_tab("https://tab1.com");
        let tab2 = manager.create_tab("https://tab2.com");

        // Currently 2 tabs loaded
        assert_eq!(manager.active_memory_count(), 2);
        assert_eq!(manager.sleeping_tab_count(), 0);

        // Open 3rd tab: tab1 should be put to sleep
        let tab3 = manager.create_tab("https://tab3.com");
        assert_eq!(manager.tab_count(), 3);
        assert_eq!(manager.active_memory_count(), 2);
        assert_eq!(manager.sleeping_tab_count(), 1);
        assert_eq!(manager.get_tab(tab1).unwrap().state, TabState::Sleeping);
        assert_eq!(manager.get_tab(tab2).unwrap().state, TabState::Idle);
        assert_eq!(manager.get_tab(tab3).unwrap().state, TabState::Active);

        // Switching back to sleeping tab1 should wake it up and hibernate tab2
        manager.switch_to_tab(tab1);
        assert_eq!(manager.get_tab(tab1).unwrap().state, TabState::Active);
        assert_eq!(manager.get_tab(tab2).unwrap().state, TabState::Sleeping);
        assert_eq!(manager.active_memory_count(), 2);
    }

    #[test]
    fn test_audio_guard_prevents_hibernation() {
        let mut manager = TabManager::new(1);
        let tab1 = manager.create_tab("https://youtube.com");
        
        // Mark tab1 as playing audio
        manager.get_tab_mut(tab1).unwrap().is_audio_playing = true;

        // Opening tab2 should NOT hibernate tab1 because of audio guard
        let tab2 = manager.create_tab("https://reddit.com");
        assert_eq!(manager.get_tab(tab1).unwrap().state, TabState::Idle);
        assert_eq!(manager.get_tab(tab2).unwrap().state, TabState::Active);
    }

    #[test]
    fn test_tab_closure_and_neighbor_activation() {
        let mut manager = TabManager::new(3);
        let _tab1 = manager.create_tab("https://tab1.com");
        let tab2 = manager.create_tab("https://tab2.com");
        let tab3 = manager.create_tab("https://tab3.com");

        // Close active tab3, should activate tab2
        assert_eq!(manager.close_tab(tab3), Some(tab2));
        assert_eq!(manager.active_tab_id, Some(tab2));
        assert_eq!(manager.get_tab(tab2).unwrap().state, TabState::Active);
        assert_eq!(manager.tab_count(), 2);
    }
}
