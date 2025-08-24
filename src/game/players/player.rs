use crate::game::players::GamePlayer;
use crate::game::{
    Setup,
    player_board::{PlayerBoard, ShotResult},
    point::Point,
    rotation::Rotation,
    ship::ShipBlueprint,
};
use std::io::stdin;
pub struct Player {
    board: PlayerBoard,
}
impl Player {
    pub fn new() -> Self {
        Self {
            board: PlayerBoard::new(),
        }
    }
}
impl GamePlayer for Player {
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
    fn process_shot(&mut self, p: Point) -> ShotResult {
        self.board.process_shot(p)
    }
}
impl Setup<Vec<ShipBlueprint>> for Player {
    fn setup(&mut self, ships: Vec<ShipBlueprint>) {
        for ship in ships.iter() {
            // TODO: add selecting of coordinates to put the ship
            loop {
                let pos = self.choose_point();

                if self.board.place_ship(ship, pos, Rotation::None).is_ok() {
                    break;
                }
            }
        }
    }
}
