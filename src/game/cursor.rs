use std::error::Error;

use crate::game::point::Point;

pub struct Cursor {
    position: Point,
    max_y: usize,
    max_x: usize,
}
#[derive(PartialEq, Eq, Debug)]
pub enum CursorError {
    OutOfBounds,
}
impl Error for CursorError {}
impl std::fmt::Display for CursorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Cursor {
    pub fn new(x: usize, y: usize, max_x: usize, max_y: usize) -> Self {
        Self {
            position: Point::new(x, y),
            max_x,
            max_y,
        }
    }
    pub fn move_by(&mut self, dx: isize, dy: isize) -> Result<(), CursorError> {
        let pos = self.position;
        let (x, y) = (pos.x, pos.y);

        let (Some(new_x), Some(new_y)) = (x.checked_add_signed(dx), y.checked_add_signed(dy))
        else {
            return Err(CursorError::OutOfBounds);
        };

        if new_x >= self.max_x || new_y >= self.max_y {
            return Err(CursorError::OutOfBounds);
        }

        self.position = Point::new(new_x, new_y);
        Ok(())
    }
    // returns (x, y) coordinates
    pub fn pos(&self) -> Point {
        self.position
    }
}
#[cfg(test)]
mod test {
    use crate::game::cursor::{Cursor, CursorError};

    #[test]
    fn cannot_move_outside_of_bounds() {
        let mut c = Cursor::new(0, 0, 10, 10);
        let res = c.move_by(10, 2);
        assert_eq!(res, Err(CursorError::OutOfBounds))
    }
    #[test]
    fn can_move_inside_bounds() {
        let mut c1 = Cursor::new(0, 0, 10, 10);
        let res = c1.move_by(9, 9);
        assert!(res.is_ok());
        let mut c2 = Cursor::new(2, 5, 5, 6);
        let res2 = c2.move_by(-2, -5);
        assert!(res2.is_ok());
    }
}
