mod game;
use game::players::{Computer, Player};
use game::point::Point;
use game::ship::ShipBlueprint;
use game::{GameMode, Setup, SinglePlayer};
fn main() {
    let mut terminal = ratatui::init();

    let p1: Player = Player::new(&mut terminal);
    let p2: Computer = Computer::new();
    let mut game = SinglePlayer::new(p1, p2);

    // TODO: Hardcode at first then add loading from config?
    let ships: Vec<ShipBlueprint> = vec![
        ShipBlueprint::new(vec![Point::new(0, 0)], String::from("dot")),
        ShipBlueprint::new(
            vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 1)],
            String::from("new"),
        ),
    ];
    game.setup(ships);
    println!("Running game");
    // game.run();
    ratatui::restore();
}
