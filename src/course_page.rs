use ratatui::{buffer::Buffer, layout::Rect, widgets::{Paragraph, Widget}};

use crate::menu::Page;

impl Widget for &Page {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Paragraph::new("hi").render(area, buf);
    }
}
