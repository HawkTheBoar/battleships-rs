use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use crate::game::point::Point;
use crate::game::rotation::Rotation;
use crate::game::ship::{Ship, ShipBlueprint};
use crate::game::tile::Tile;
#[derive(Debug, PartialEq, Eq)]
pub enum BoardError {
    ShipPlacementError(PlacementError),
}
#[derive(Debug, PartialEq, Eq)]
pub enum PlacementError {
    ShipOverlap,
    OutOfBounds,
}
impl Display for PlacementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Error for BoardError {}

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

pub enum ShotResult {}
pub struct PlayerBoard {
    grid: [[Tile; WIDTH]; HEIGHT],
    pub ships: HashMap<u8, Ship>,
}
impl PlayerBoard {
    pub fn new() -> Self {
        Self {
            grid: [[Tile::Empty; WIDTH]; HEIGHT],
            ships: HashMap::new(),
        }
    }
    pub fn place_ship(
        &mut self,
        blueprint: &ShipBlueprint,
        pos: Point,
        rotation: Rotation,
    ) -> Result<(), BoardError> {
        // check if can place
        let mut points: Vec<Point> = Vec::with_capacity(blueprint.parts.capacity());
        for point in blueprint.parts.iter() {
            let (x, y) = (point.x + pos.x, point.y + pos.y);

            // TODO: Implement checking for rotated ships
            if x >= WIDTH || y >= HEIGHT {
                return Err(BoardError::ShipPlacementError(PlacementError::OutOfBounds));
            }
            if self.grid[y][x] != Tile::Empty {
                return Err(BoardError::ShipPlacementError(PlacementError::ShipOverlap));
            }
            points.push(Point::new(x, y));
        }

        // TODO: Optimize this so there is no clone
        // Maybe use Rc? i dont know anything better xdd
        let s = Ship::new(points.clone(), blueprint.name.clone());
        // create and insert the tiles into the board
        for p in points {
            self.grid[p.y][p.x] = Tile::Ship(s.id)
        }
        self.ships.insert(s.id, s);
        Ok(())
    }
    // return some state enum or tile whatever
    pub fn process_shot(&mut self, p: Point) -> ShotResult {
        todo!()
    }
    pub fn is_game_over(&self) -> bool {
        self.ships.iter().all(|(_, ship)| !ship.is_alive())
    }
}

#[cfg(test)]
mod test {
    use crate::game::{
        player_board::{BoardError, PlacementError, PlayerBoard},
        point::Point,
        rotation::Rotation,
        ship::ShipBlueprint,
    };
    fn default_setup() -> (PlayerBoard, ShipBlueprint) {
        let board: PlayerBoard = PlayerBoard::new();
        let blueprint: ShipBlueprint =
            ShipBlueprint::new(vec![Point::new(0, 0)], String::from("dot"));
        (board, blueprint)
    }
    fn big_ship_setup() -> (PlayerBoard, ShipBlueprint) {
        let board = PlayerBoard::new();
        let bp = ShipBlueprint::new(
            vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(1, 1),
            ],
            String::from("BigDih"),
        );
        (board, bp)
    }
    #[test]
    fn can_place_ship() {
        let (mut board, blueprint) = default_setup();
        let res = board.place_ship(&blueprint, Point::new(0, 0), Rotation::None);
        assert!(res.is_ok())
    }
    #[test]
    fn can_place_bigger_ship() {
        let (mut board, bp) = big_ship_setup();
        let res = board.place_ship(&bp, Point::new(0, 0), Rotation::None);
        assert!(res.is_ok())
    }
    #[test]
    fn cannot_place_bigger_ship_out_of_bounds() {
        let (mut board, bp) = big_ship_setup();
        let res = board.place_ship(&bp, Point::new(9, 0), Rotation::None);
        assert_eq!(
            res.expect_err("should be err"),
            BoardError::ShipPlacementError(PlacementError::OutOfBounds)
        );
        let res2 = board.place_ship(&bp, Point::new(9, 0), Rotation::None);
        assert_eq!(
            res2.expect_err("should be err"),
            BoardError::ShipPlacementError(PlacementError::OutOfBounds)
        );
    }
    #[test]
    fn cannot_place_ship_out_of_bounds_x() {
        let (mut board, blueprint) = default_setup();
        let res = board.place_ship(&blueprint, Point::new(10, 0), Rotation::None);
        assert_eq!(
            res.expect_err("should be err"),
            BoardError::ShipPlacementError(PlacementError::OutOfBounds)
        );
    }
    #[test]
    fn cannot_place_ship_out_of_bounds_y() {
        let (mut board, blueprint) = default_setup();
        let res = board.place_ship(&blueprint, Point::new(0, 10), Rotation::None);
        assert_eq!(
            res.expect_err("should be err"),
            BoardError::ShipPlacementError(PlacementError::OutOfBounds)
        );
    }
    #[test]
    fn cannot_place_ship_on_another_ship() {
        let (mut board, blueprint) = default_setup();
        let res = board.place_ship(&blueprint, Point::new(0, 0), Rotation::None);
        let res2 = board.place_ship(&blueprint, Point::new(0, 0), Rotation::None);
        assert!(res.is_ok());
        assert_eq!(
            res2.expect_err("should be err"),
            BoardError::ShipPlacementError(PlacementError::ShipOverlap)
        );
    }
    #[test]
    fn cannot_place_bigger_ship_on_ship() {
        let (mut board, bp) = big_ship_setup();
        let (_, sm) = default_setup();
        let res = board.place_ship(&sm, Point::new(1, 1), Rotation::None);
        assert!(res.is_ok());
        let res2 = board.place_ship(&bp, Point::new(0, 0), Rotation::None);
        assert_eq!(
            res2.expect_err("should be err"),
            BoardError::ShipPlacementError(PlacementError::ShipOverlap)
        );
    }
}
