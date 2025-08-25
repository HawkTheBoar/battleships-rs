pub mod player_board;
pub mod players;
pub mod point;
mod rotation;
pub mod ship;
mod tile;

use crate::game::players::GamePlayer;
use crate::game::{
    player_board::PlayerBoard,
    point::Point,
    rotation::Rotation,
    ship::{Ship, ShipBlueprint},
};
pub trait GameMode {
    fn run(&mut self);
}
pub trait Setup<T> {
    fn setup(&mut self, arg: T);
}
pub struct SinglePlayer<T, U>
where
    T: GamePlayer,
    U: GamePlayer,
{
    player1: T,
    player2: U,
    current_player: usize, // Tracks current player (e.g., 1 or 2)
}

impl<T, U> SinglePlayer<T, U>
where
    T: GamePlayer,
    U: GamePlayer,
{
    pub fn new(player1: T, player2: U) -> Self {
        Self {
            player1,
            player2,
            current_player: 1,
        }
    }
    // returns (current player, opponent_player)
    pub fn players(&mut self) -> (&mut dyn GamePlayer, &mut dyn GamePlayer) {
        if self.is_current_p1() {
            (&mut self.player1, &mut self.player2)
        } else {
            (&mut self.player2, &mut self.player1)
        }
    }
    pub fn current_player_mut(&mut self) -> &mut dyn GamePlayer {
        if self.is_current_p1() {
            &mut self.player1
        } else {
            &mut self.player2
        }
    }
    pub fn opponent_player_mut(&mut self) -> &mut dyn GamePlayer {
        if !self.is_current_p1() {
            &mut self.player1
        } else {
            &mut self.player2
        }
    }
    pub fn switch(&mut self) {
        if self.is_current_p1() {
            self.current_player = 2;
        } else {
            self.current_player = 1;
        }
    }
    fn is_current_p1(&self) -> bool {
        self.current_player == 1
    }
    pub fn is_game_over(&self) -> bool {
        self.player1.is_game_over() || self.player2.is_game_over()
    }
}
impl<T, U> GameMode for SinglePlayer<T, U>
where
    T: GamePlayer,
    U: GamePlayer,
{
    fn run(&mut self) {
        loop {
            let (curr, opp) = self.players();
            let point = curr.choose_point();
            let Ok(shot) = opp.process_shot(point) else {
                // write error and continue
                println!("Processing error, continuing");
                continue;
            };
            curr.update_view_board(shot, point)
                .expect("Out of bounds, unable to show this shot");

            println!(
                "player {} shoots at {} {}",
                self.current_player, point.x, point.y
            );
            if self.is_game_over() {
                // TODO: write winner
                println!("Winner player: {}", self.current_player);
                break;
            }
            self.switch();
        }
    }
}
impl<T, U> Setup<Vec<ShipBlueprint>> for SinglePlayer<T, U>
where
    T: GamePlayer,
    U: GamePlayer,
{
    fn setup(&mut self, ships: Vec<ShipBlueprint>) {
        // TODO: use lifetimes this clone is okay but triggers me
        self.player1.setup(ships.clone());
        self.player2.setup(ships);
        // let player = self.player();
    }
}
