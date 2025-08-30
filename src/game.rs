pub mod cursor;
pub mod player_board;
pub mod players;
pub mod point;
mod rotation;
pub mod ship;
mod tile;

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::game::player_board::board_view::{self, BoardView};
use crate::game::players::GamePlayer;
use crate::game::ship::ShipBlueprint;

pub struct GameResult {
    pub winner: CurrentPlayer,
    pub winner_name: Option<String>,
}
pub trait GameMode {
    fn run(self) -> GameResult;
}
pub trait Setup<T> {
    fn setup(&mut self, arg: T);
}
#[derive(Clone, Copy)]
pub enum CurrentPlayer {
    First = 1,
    Second = 2,
}

pub struct SinglePlayer {
    player1: Box<RefCell<dyn GamePlayer>>,
    player2: Box<RefCell<dyn GamePlayer>>,
    current_player: CurrentPlayer,
}

impl SinglePlayer {
    pub fn new<T, U>(player1: T, player2: U) -> Self
    where
        T: GamePlayer + 'static,
        U: GamePlayer + 'static,
    {
        Self {
            player1: Box::new(RefCell::new(player1)),
            player2: Box::new(RefCell::new(player2)),
            current_player: CurrentPlayer::First,
        }
    }
    fn current(&self) -> &Box<RefCell<dyn GamePlayer>> {
        match self.current_player {
            CurrentPlayer::First => &self.player1,
            CurrentPlayer::Second => &self.player2,
        }
    }
    fn opponent(&self) -> &Box<RefCell<dyn GamePlayer>> {
        match self.current_player {
            CurrentPlayer::First => &self.player2,
            CurrentPlayer::Second => &self.player1,
        }
    }
    pub fn switch(&mut self) {
        self.current_player = match self.current_player {
            CurrentPlayer::First => CurrentPlayer::Second,
            CurrentPlayer::Second => CurrentPlayer::First,
        }
    }
    pub fn is_game_over(&self) -> bool {
        self.player1.borrow().is_game_over() || self.player2.borrow().is_game_over()
    }
}
impl GameMode for SinglePlayer {
    fn run(mut self) -> GameResult {
        loop {
            {
                let mut curr = self.current().borrow_mut();
                let mut opp = self.opponent().borrow_mut();
                let point = curr.choose_point();
                let Ok(shot) = opp.process_shot(point) else {
                    // write error and continue
                    continue;
                };
                curr.update_view_board(shot, point)
                    .expect("Out of bounds, unable to show this shot");
            }
            if self.is_game_over() {
                return GameResult {
                    winner: self.current_player,
                    winner_name: self.current().borrow().get_name().clone(),
                };
                // TODO: write winner
            }
            self.switch();
        }
        // restore terminal
    }
}
impl Setup<Vec<ShipBlueprint>> for SinglePlayer {
    fn setup(&mut self, ships: Vec<ShipBlueprint>) {
        // TODO: use lifetimes or this clone is okay?
        self.player1.borrow_mut().setup(ships.clone());
        self.player2.borrow_mut().setup(ships);
        // let player = self.player();
    }
}
