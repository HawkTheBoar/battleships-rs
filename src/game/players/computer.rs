use rand::random_range;

use crate::game::player_board::board_view::BoardView;
use crate::game::player_board::{BoardError, HEIGHT, WIDTH};
use crate::game::players::GamePlayer;
use crate::game::rotation::Rotation;
use crate::game::{
    Setup,
    player_board::{PlayerBoard, ShotResult},
    point::Point,
    ship::ShipBlueprint,
};

pub struct Computer {
    board: PlayerBoard,
}
impl Computer {
    pub fn new() -> Self {
        Self {
            board: PlayerBoard::new(),
        }
    }
}
impl GamePlayer for Computer {
    fn choose_point(&mut self) -> Point {
        let x = random_range(0..WIDTH);
        let y = random_range(0..HEIGHT);
        Point::new(x, y)
    }
    fn is_game_over(&self) -> bool {
        self.board.is_game_over()
    }
    fn process_shot(&mut self, p: Point) -> Result<ShotResult, BoardError> {
        self.board.process_shot(p)
    }
    fn update_view_board(&mut self, _: ShotResult, _: Point) -> Result<(), BoardError> {
        // this computer just chooses random positions each time
        Ok(())
    }
}
impl Setup<Vec<ShipBlueprint>> for Computer {
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
