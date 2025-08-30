mod game;
use std::cell::RefCell;
use std::rc::Rc;

use game::players::{Computer, Player};
use game::point::Point;
use game::ship::ShipBlueprint;
use game::{GameMode, Setup, SinglePlayer};

use crate::game::PlayerVsPlayerMode;
fn main() {
    let terminal = ratatui::init();

    let term = Rc::new(RefCell::new(terminal));

    let p1: Player = Player::new(Rc::clone(&term), String::from("mistr"));
    let p2: Player = Player::new(Rc::clone(&term), String::from("tondik"));
    let mut game = PlayerVsPlayerMode::new(p1, p2, term);
    // let p2: Computer = Computer::new();
    // let mut game = SinglePlayer::new(p1, p2);

    // TODO: Hardcode at first then add loading from config?
    let ships: Vec<ShipBlueprint> = vec![
        // ShipBlueprint::new(vec![Point::new(0, 0)], String::from("dot")),
        ShipBlueprint::new(
            vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 1)],
            String::from("new"),
        ),
        ShipBlueprint::new(
            vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
            ],
            String::from("long boy"),
        ),
    ];
    game.setup(ships);
    let player = game.run();

    // TODO: Game Over screen
    ratatui::restore();
    println!(
        "Player {}: {} has won the game!",
        player.winner as i32, player.winner_name
    );
}
