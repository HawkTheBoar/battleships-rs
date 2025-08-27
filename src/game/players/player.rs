use crossterm::event::Event;
use ratatui::layout::Rect;

use crate::game::player_board::board_builder::BoardBuilder;
use crate::game::player_board::{BoardError, ViewBoard};
use crate::game::players::GamePlayer;
use crate::game::{
    Setup,
    player_board::{PlayerBoard, ShotResult},
    point::Point,
    rotation::Rotation,
    ship::ShipBlueprint,
};
use std::cell::RefCell;
use std::io::stdin;
pub struct Player<'a> {
    board: PlayerBoard,
    opponent_board: ViewBoard,
    terminal: &'a mut ratatui::DefaultTerminal,
}
impl<'a> Player<'a> {
    pub fn new(terminal: &'a mut ratatui::DefaultTerminal) -> Self {
        Self {
            board: PlayerBoard::new(),
            opponent_board: ViewBoard::new(),
            terminal,
        }
    }
}
impl<'a> GamePlayer for Player<'a> {
    fn choose_point(&self) -> Point {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let t: Vec<&str> = buf.split_whitespace().collect();
        Point {
            x: t.first().unwrap().parse().unwrap(),
            y: t.get(1).unwrap().parse().unwrap(),
        }
    }
    fn is_game_over(&self) -> bool {
        self.board.is_game_over()
    }
    fn process_shot(&mut self, p: Point) -> Result<ShotResult, BoardError> {
        self.board.process_shot(p)
    }
    fn update_view_board(&mut self, shot: ShotResult, p: Point) -> Result<(), BoardError> {
        self.opponent_board.register_shot(shot, p)
    }
}
impl<'a> Setup<Vec<ShipBlueprint>> for Player<'a> {
    fn setup(&mut self, ships: Vec<ShipBlueprint>) {
        let mut frame = self.terminal.get_frame();
        for ship in ships.iter() {
            // TODO: add selecting of coordinates to put the ship

            let mut builder = BoardBuilder::new(&self.board, ship);
            let (pos, rot) = loop {
                self.terminal.draw(|f| builder.render(f, f.area()));
                let event = crossterm::event::read();
                let Ok(Event::Key(e)) = event else { continue };
                let res = builder.handle_key(e);
                let Ok(Some(placement)) = res else { continue };
                break placement;
            };
            // this wont ever be an error
            self.board.place_ship(ship, pos, rot).unwrap();
        }
    }
}
