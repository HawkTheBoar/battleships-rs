use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Row, StatefulWidget, Table},
};

use crate::game::{
    cursor::Cursor,
    player_board::{self, BoardError, HEIGHT, WIDTH},
    point::Point,
    tile::Tile,
};

pub struct BoardView<'a> {
    grid: &'a [[Tile; WIDTH]; HEIGHT],
    cursor: Option<Cursor>,
    title: &'a str,
}

impl<'a> BoardView<'a> {
    pub fn new(grid: &'a [[Tile; WIDTH]; HEIGHT], cursor: Option<Cursor>, title: &'a str) -> Self {
        Self {
            grid,
            cursor,
            title,
        }
    }
    pub fn render(&self, f: &mut Frame, rect: Rect) {
        let title: Line = Line::from(self.title);
        let block = Block::bordered().title(title).border_set(border::THICK);

        let rows = self
            .grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                Row::new(
                    row.iter()
                        .enumerate()
                        .map(|(x, t)| {
                            // TODO: Or just push it into the vector which will be alot more performant
                            if let Some(cursor) = &self.cursor {
                                if cursor.pos() == Point::new(x, y) {
                                    return t.to_styled().style(Style::new().bg(Color::Yellow));
                                }
                            }
                            t.to_styled()
                        })
                        .collect::<Vec<Text>>(), // Collect as Text
                )
            })
            .collect::<Vec<Row>>();
        let widths = [Constraint::Length(1); WIDTH];
        let table = Table::new(rows, widths).block(block).column_spacing(0);
        f.render_widget(table, rect);
    }
    pub fn handle_key(&mut self, key: KeyEvent) -> Result<Option<(Point)>, BoardError> {
        if self.cursor.is_none() {
            return Ok(None);
        }
        match key.code {
            KeyCode::Left => self.move_cursor(-1, 0),
            KeyCode::Right => self.move_cursor(1, 0),
            KeyCode::Up => self.move_cursor(0, -1),
            KeyCode::Down => self.move_cursor(0, 1),
            KeyCode::Enter => return self.select(),
            KeyCode::Esc => {
                return Err(BoardError::ShipPlacementError(
                    super::PlacementError::PlacementExit,
                ));
            }
            _ => {}
        }
        Ok(None)
    }
    fn move_cursor(&mut self, dx: isize, dy: isize) {
        let _ = if let Some(cursor) = &mut self.cursor {
            cursor.move_by(dx, dy)
        } else {
            return;
        };
    }
    fn select(&self) -> Result<Option<(Point)>, BoardError> {
        if let Some(cursor) = &self.cursor {
            Ok(Some(cursor.pos()))
        } else {
            Err(BoardError::Shot(player_board::ShotError::AlreadyShot))
        }
    }
}
