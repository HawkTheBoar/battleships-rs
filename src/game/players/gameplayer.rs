use crate::game::{
    Setup,
    player_board::{BoardError, ShotResult, board_view::BoardView},
    point::Point,
    ship::ShipBlueprint,
};

pub trait GamePlayer: Setup<Vec<ShipBlueprint>> {
    // TODO: Should i have just put the terminal to each method that requires it instead of putting
    // it inside the Player struct?
    fn choose_point(&mut self) -> Point;
    fn is_game_over(&self) -> bool;
    fn process_shot(&mut self, p: Point) -> Result<ShotResult, BoardError>;
    fn update_view_board(&mut self, shot: ShotResult, p: Point) -> Result<(), BoardError>;
    fn get_name(&self) -> &String;
    fn render(&self);
}
