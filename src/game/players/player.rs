use crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout};

use crate::game::cursor::Cursor;
use crate::game::player_board::board_builder::BoardBuilder;
use crate::game::player_board::board_view::BoardView;
use crate::game::player_board::{BoardError, HEIGHT, ViewBoard, WIDTH, board_view};
use crate::game::players::GamePlayer;
use crate::game::{
    Setup,
    player_board::{PlayerBoard, ShotResult},
    point::Point,
    ship::ShipBlueprint,
};
use std::cell::RefCell;
use std::rc::Rc;
pub struct Player {
    board: PlayerBoard,
    opponent_board: ViewBoard,
    terminal: Rc<RefCell<ratatui::DefaultTerminal>>,
    last_cursor_pos: Option<Point>,
    name: String,
}
impl Player {
    pub fn new(terminal: Rc<RefCell<ratatui::DefaultTerminal>>, name: String) -> Self {
        Self {
            board: PlayerBoard::new(),
            opponent_board: ViewBoard::new(),
            terminal,
            last_cursor_pos: None,
            name,
        }
    }
    // opponent_board is passed in so that you can select points in choose_point
    fn render_view(&self, opponent_board: &BoardView) {
        // TWO BOARD VIEWS FIRST OPPONENT, SECOND SELF
        let self_board = BoardView::new(self.board.get_grid(), None, "Your ships");
        self.terminal.borrow_mut().draw(|f| {
            let outer = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(40)])
                .split(f.area())[0];
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(outer);
            opponent_board.render(f, layout[0]);
            self_board.render(f, layout[1]);
        });
    }
}
impl GamePlayer for Player {
    fn choose_point(&mut self) -> Point {
        let last_pos = match self.last_cursor_pos {
            Some(pos) => pos,
            None => Point::new(0, 0),
        };
        let mut opponent_board = BoardView::new(
            self.opponent_board.get_grid(),
            Some(Cursor::new(last_pos.x, last_pos.y, WIDTH, HEIGHT)),
            "choose a point",
        );
        loop {
            self.render_view(&opponent_board);
            let event = crossterm::event::read();
            let Ok(Event::Key(e)) = event else { continue };
            let res = opponent_board.handle_key(e);
            let Ok(Some(placement)) = res else { continue };
            self.last_cursor_pos = Some(placement);
            return placement;
        }
    }
    fn is_game_over(&self) -> bool {
        self.board.is_game_over()
    }
    fn process_shot(&mut self, p: Point) -> Result<ShotResult, BoardError> {
        self.board.process_shot(p)
    }
    fn update_view_board(&mut self, shot: ShotResult, p: Point) -> Result<(), BoardError> {
        self.opponent_board.register_shot(shot, p)
    }
    fn get_name(&self) -> &String {
        &self.name
    }
}
impl Setup<Vec<ShipBlueprint>> for Player {
    fn setup(&mut self, ships: Vec<ShipBlueprint>) {
        for ship in ships.iter() {
            // TODO: add selecting of coordinates to put the ship

            let mut builder = BoardBuilder::new(&self.board, ship);
            let (pos, rot) = loop {
                self.terminal
                    .borrow_mut()
                    .draw(|f| builder.render(f, f.area()));
                let event = crossterm::event::read();
                let Ok(Event::Key(e)) = event else { continue };
                let res = builder.handle_key(e);
                let Ok(Some(placement)) = res else { continue };
                break placement;
            };
            // this wont ever be an error
            self.board.place_ship(ship, pos, rot).unwrap();
        }
    }
}
