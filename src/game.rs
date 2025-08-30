pub mod cursor;
pub mod player_board;
pub mod players;
pub mod point;
mod rotation;
pub mod ship;
mod tile;
pub mod ui;

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

use ratatui::text::Text;

use crate::game::player_board::board_view::{self, BoardView};
use crate::game::players::{GamePlayer, Player};
use crate::game::ship::ShipBlueprint;
use crate::game::ui::WaitForKey;

pub struct GameResult {
    pub winner: CurrentPlayer,
    pub winner_name: String,
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

pub struct SinglePlayer<T, U>
where
    T: GamePlayer,
    U: GamePlayer,
{
    player1: T,
    player2: U,
    current_player: CurrentPlayer,
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
            current_player: CurrentPlayer::First,
        }
    }
    fn current(&self) -> &dyn GamePlayer {
        match self.current_player {
            CurrentPlayer::First => &self.player1,
            CurrentPlayer::Second => &self.player2,
        }
    }
    fn opponent(&self) -> &dyn GamePlayer {
        match self.current_player {
            CurrentPlayer::First => &self.player2,
            CurrentPlayer::Second => &self.player1,
        }
    }
    // returns (current_player, opponent_player)
    fn players_mut(&mut self) -> (&mut dyn GamePlayer, &mut dyn GamePlayer) {
        match self.current_player {
            CurrentPlayer::First => (&mut self.player1, &mut self.player2),
            CurrentPlayer::Second => (&mut self.player2, &mut self.player1),
        }
    }
    pub fn switch(&mut self) {
        self.current_player = match self.current_player {
            CurrentPlayer::First => CurrentPlayer::Second,
            CurrentPlayer::Second => CurrentPlayer::First,
        }
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
    fn run(mut self) -> GameResult {
        loop {
            {
                let (curr, opp) = self.players_mut();
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
                    winner_name: self.current().get_name().clone(),
                };
                // TODO: write winner
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
        // TODO: use lifetimes or this clone is okay?
        self.player1.setup(ships.clone());
        self.player2.setup(ships);
        // let player = self.player();
    }
}

pub struct PlayerVsPlayerMode {
    game: SinglePlayer<Player, Player>,
    term: Rc<RefCell<ratatui::DefaultTerminal>>,
}

impl PlayerVsPlayerMode {
    pub fn new(
        player1: Player,
        player2: Player,
        term: Rc<RefCell<ratatui::DefaultTerminal>>,
    ) -> Self {
        Self {
            game: SinglePlayer::new(player1, player2),
            term,
        }
    }
}

impl GameMode for PlayerVsPlayerMode {
    fn run(mut self) -> GameResult {
        loop {
            {
                let (curr, opp) = self.game.players_mut();
                let point = curr.choose_point();
                let Ok(shot) = opp.process_shot(point) else {
                    // write error and continue
                    continue;
                };
                curr.update_view_board(shot, point)
                    .expect("Out of bounds, unable to show this shot");
            }
            if self.game.is_game_over() {
                return GameResult {
                    winner: self.game.current_player,
                    winner_name: self.game.current().get_name().clone(),
                };
                // TODO: write winner
            }
            self.game.switch();
            // TODO: Add a switch timer/click here
            let wait = WaitForKey::new(Text::from(format!(
                "Player {}: {}'s turn, press any key to continue.",
                self.game.current_player as i32,
                self.game.current().get_name(),
            )));
            sleep(Duration::from_secs(1));
            self.term.borrow_mut().draw(|f| wait.render(f, f.area()));
            wait.wait(None);
        }
    }
}
impl Setup<Vec<ShipBlueprint>> for PlayerVsPlayerMode {
    fn setup(&mut self, ships: Vec<ShipBlueprint>) {
        self.game.setup(ships);
    }
}
