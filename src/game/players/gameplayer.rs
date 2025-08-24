use crate::game::{Setup, player_board::ShotResult, point::Point, ship::ShipBlueprint};

pub trait GamePlayer: Setup<Vec<ShipBlueprint>> {
    fn choose_point(&self) -> Point;
    fn is_game_over(&self) -> bool;
    fn process_shot(&mut self, p: Point) -> ShotResult;
}
