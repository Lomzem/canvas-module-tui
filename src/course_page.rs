use ratatui::{buffer::Buffer, layout::Rect};

use crate::menu::Menu;

impl Menu {
    pub fn render_course(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
    }
}
