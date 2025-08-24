use crate::game::players::GamePlayer;
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
    fn choose_point(&self) -> Point {
        todo!()
    }
    fn is_game_over(&self) -> bool {
        self.board.is_game_over()
    }
    fn process_shot(&mut self, p: Point) -> ShotResult {
        self.board.process_shot(p)
    }
}
impl Setup<Vec<ShipBlueprint>> for Computer {
    fn setup(&mut self, arg: Vec<ShipBlueprint>) {
        todo!()
    }
}
