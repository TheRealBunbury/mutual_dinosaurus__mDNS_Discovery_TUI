use crossterm::event::KeyEvent;
use ratatui::prelude::{Buffer, Rect};

pub trait DiscoveryWidget: Sized + std::fmt::Debug {
    fn title(&self) -> String;
    fn controls(&self) -> String;
    fn process_key_event(&mut self, key_event: &KeyEvent);
    fn render(&self, area: Rect, buf: &mut Buffer, selected: bool);
}
