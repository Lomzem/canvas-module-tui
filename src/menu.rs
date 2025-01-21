use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{buffer::Buffer, layout::Rect, widgets::{Paragraph, Widget}, DefaultTerminal};

use crate::courses::Course;

pub struct Menu {
    quit: bool,
    page: Page,
    course_list: Vec<Course>,
    module_list: Option<usize>,
}

pub enum Page {
    Course,
    Module,
}

impl Menu {
    pub fn default(course_list: Vec<Course>) -> Self {
        Self {
            quit: false,
            page: Page::Course,
            course_list,
            module_list: None,
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<(), io::Error> {
        while !self.quit {
            terminal.draw(|frame| {
                frame.render_widget(&self, frame.area());
            })?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.quit = true,
            _ => {}
        }
    }
}

impl Widget for &Menu {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Paragraph::new("HI").render(area, buf);
    }
}
