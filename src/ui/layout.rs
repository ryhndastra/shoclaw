use crate::config::SidebarMode;
use wry::{
    dpi::{LogicalPosition, LogicalSize},
    Rect,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiLayout {
    pub mode: SidebarMode,
    pub is_autohide_revealed: bool,
}

impl UiLayout {
    pub fn new(mode: SidebarMode) -> Self {
        Self {
            mode,
            is_autohide_revealed: false,
        }
    }

    pub fn sidebar_width(&self) -> f64 {
        match self.mode {
            SidebarMode::Expanded => 250.0,
            SidebarMode::Compact => 48.0,
            SidebarMode::Hidden => 0.0,
            SidebarMode::Autohide => {
                if self.is_autohide_revealed {
                    250.0
                } else {
                    0.0
                }
            }
        }
    }

    pub fn is_sidebar_visible(&self) -> bool {
        self.sidebar_width() > 0.0
    }

    pub fn toggle_mode(&mut self) -> SidebarMode {
        self.mode = match self.mode {
            SidebarMode::Expanded => SidebarMode::Compact,
            SidebarMode::Compact => SidebarMode::Expanded,
            SidebarMode::Hidden => SidebarMode::Expanded,
            SidebarMode::Autohide => SidebarMode::Expanded,
        };
        self.mode
    }

    pub fn toggle_hidden(&mut self) -> SidebarMode {
        self.mode = match self.mode {
            SidebarMode::Hidden => SidebarMode::Expanded,
            _ => SidebarMode::Hidden,
        };
        self.mode
    }

    pub fn compute_bounds(
        &self,
        window_width: f64,
        window_height: f64,
    ) -> (Rect, Rect) {
        let sb_width = self.sidebar_width();
        let content_width = (window_width - sb_width).max(0.0);

        let sidebar_rect = Rect {
            position: LogicalPosition::new(0.0, 0.0).into(),
            size: LogicalSize::new(sb_width, window_height).into(),
        };

        let content_rect = Rect {
            position: LogicalPosition::new(sb_width, 0.0).into(),
            size: LogicalSize::new(content_width, window_height).into(),
        };

        (sidebar_rect, content_rect)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_widths() {
        let mut layout = UiLayout::new(SidebarMode::Expanded);
        assert_eq!(layout.sidebar_width(), 250.0);
        assert!(layout.is_sidebar_visible());

        layout.mode = SidebarMode::Compact;
        assert_eq!(layout.sidebar_width(), 48.0);
        assert!(layout.is_sidebar_visible());

        layout.mode = SidebarMode::Hidden;
        assert_eq!(layout.sidebar_width(), 0.0);
        assert!(!layout.is_sidebar_visible());

        layout.mode = SidebarMode::Autohide;
        layout.is_autohide_revealed = false;
        assert_eq!(layout.sidebar_width(), 0.0);
        layout.is_autohide_revealed = true;
        assert_eq!(layout.sidebar_width(), 250.0);
    }

    #[test]
    fn test_layout_toggle_cycle() {
        let mut layout = UiLayout::new(SidebarMode::Expanded);
        assert_eq!(layout.toggle_mode(), SidebarMode::Compact);
        assert_eq!(layout.toggle_mode(), SidebarMode::Expanded);
        assert_eq!(layout.toggle_mode(), SidebarMode::Compact);
    }

    #[test]
    fn test_layout_toggle_hidden() {
        let mut layout = UiLayout::new(SidebarMode::Expanded);
        assert_eq!(layout.toggle_hidden(), SidebarMode::Hidden);
        assert_eq!(layout.sidebar_width(), 0.0);
        assert!(!layout.is_sidebar_visible());

        assert_eq!(layout.toggle_hidden(), SidebarMode::Expanded);
        assert_eq!(layout.sidebar_width(), 250.0);
        assert!(layout.is_sidebar_visible());
    }

    #[test]
    fn test_compute_bounds() {
        let layout = UiLayout::new(SidebarMode::Expanded);
        let (sb, content) = layout.compute_bounds(1280.0, 800.0);

        let sb_size = sb.size.to_logical::<f64>(1.0);
        assert_eq!(sb_size.width, 250.0);
        assert_eq!(sb_size.height, 800.0);

        let content_pos = content.position.to_logical::<f64>(1.0);
        let content_size = content.size.to_logical::<f64>(1.0);
        assert_eq!(content_pos.x, 250.0);
        assert_eq!(content_size.width, 1030.0);
        assert_eq!(content_size.height, 800.0);
    }
}
