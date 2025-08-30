use crossterm::event::{self, Event};
use ratatui::{Frame, layout::Rect, text::Text, widgets::Widget};

pub struct WaitForKey<'a> {
    text: Text<'a>,
}

impl<'a> WaitForKey<'a> {
    pub fn new(mut text: Text<'a>) -> Self {
        text = text
            .centered()
            .alignment(ratatui::layout::Alignment::Center);
        Self { text }
    }
    pub fn render(&self, f: &mut Frame, rect: Rect) {
        f.render_widget(self.text.clone(), rect);
    }
    pub fn wait(self, allowed: Option<Vec<char>>) -> char {
        loop {
            let Ok(Event::Key(key)) = event::read() else {
                continue;
            };
            // let Some(ch) = key.code.as_char() else {
            //     continue;
            // };
            // if let Some(ref allowed) = allowed {
            //     if !allowed.contains(&ch) {
            //         continue;
            //     }
            // }
            break 't';
            // break ch;
        }
    }
}
