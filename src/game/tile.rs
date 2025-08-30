use crossterm::style::Stylize;
use ratatui::{
    style::{Color, Style},
    text::Text,
};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Tile {
    Ship(u8),
    SunkenShip,
    Hit,
    Miss,
    Empty,
    Hidden,
}
impl Tile {
    pub fn to_styled(&self) -> Text {
        match *self {
            Self::Ship(_) => Text::from(" ").style(Style::new().bg(Color::Gray)), // grey
            Self::SunkenShip => Text::from(" ").style(Style::new().bg(Color::Red)), // dark_red
            Self::Hit => Text::from(" ").style(Style::new().bg(Color::LightRed)), // red()
            Self::Miss => Text::from("*").style(Style::new().fg(Color::DarkGray)), // dark_grey()
            Self::Empty => Text::from(" ").style(Style::new().bg(Color::Blue)),   // dark_blue()
            Self::Hidden => Text::from("?").style(Style::new().fg(Color::Gray).bg(Color::Blue)), // blue()
        }
    }
}
