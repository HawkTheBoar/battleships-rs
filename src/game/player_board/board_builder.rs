use std::error::Error;

use crossterm::{
    event::{KeyCode, KeyEvent},
    style::{StyledContent, Stylize},
};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Row, Table, Widget},
};

use crate::game::{cursor::Cursor, player_board::HEIGHT, tile::Tile};
use crate::game::{
    player_board::{BoardError, PlayerBoard, WIDTH},
    point::Point,
    rotation::Rotation,
    ship::ShipBlueprint,
};
pub struct BoardBuilder<'a> {
    cursor: Cursor,
    rotation: Rotation,
    current_ship: &'a ShipBlueprint,
    board: &'a PlayerBoard,
}
impl<'a> BoardBuilder<'a> {
    pub fn new(board: &'a PlayerBoard, ship: &'a ShipBlueprint) -> Self {
        Self {
            board,
            cursor: Cursor::new(0, 0, WIDTH, HEIGHT),
            current_ship: ship,
            rotation: Rotation::Horizontal,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Result<Option<(Point, Rotation)>, BoardError> {
        match key.code {
            KeyCode::Left => self.move_cursor(-1, 0),
            KeyCode::Right => self.move_cursor(1, 0),
            KeyCode::Up => self.move_cursor(0, -1),
            KeyCode::Down => self.move_cursor(0, 1),
            KeyCode::Char('r') => self.rotate_ship(),
            KeyCode::Enter => return self.place_ship(),
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
        // Implement movement logic with boundary checks
        let _ = self.cursor.move_by(dx, dy);
    }

    fn rotate_ship(&mut self) {
        // Rotate the ship and check validity
        self.rotation = self.rotation.next();
        // Update bounds for cursor
    }

    fn place_ship(&self) -> Result<Option<(Point, Rotation)>, BoardError> {
        // Validate placement and return position if valid
        if let Err(err) =
            self.board
                .can_place_ship(self.current_ship, self.cursor.pos(), self.rotation)
        {
            Err(err)
        } else {
            Ok(Some((self.cursor.pos(), self.rotation)))
        }
    }
    pub fn render(&self, f: &mut Frame, rect: Rect) {
        let title = Line::from(" Position your ships! ");
        let block = Block::bordered().title(title).border_set(border::THICK);

        let cursor = self.cursor.pos();
        // TODO: maybe optimize this someday?
        let ship_preview = self
            .current_ship
            .parts
            .iter()
            .map(|p| Point::new(p.x + cursor.x, p.y + cursor.y))
            .collect::<Vec<Point>>();
        let can_place =
            match self
                .board
                .can_place_ship(self.current_ship, self.cursor.pos(), self.rotation)
            {
                Ok(_) => true,
                Err(_) => false,
            };
        let rows = self
            .board
            .grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                Row::new(
                    row.iter()
                        .enumerate()
                        .map(|(x, t)| {
                            if ship_preview.contains(&Point::new(x, y)) {
                                if can_place {
                                    return Tile::Ship(0).to_styled();
                                } else {
                                    return Tile::Ship(0)
                                        .to_styled()
                                        .style(Style::new().bg(Color::Red));
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
}
