use crate::game::point::Point;

pub struct Cursor {
    position: Point,
    max_y: usize,
    max_x: usize,
}

impl Cursor {
    pub fn new(x: usize, y: usize, max_x: usize, max_y: usize) -> Self {
        Self {
            position: Point::new(x, y),
            max_x,
            max_y,
        }
    }
    pub fn move_by(&mut self, dx: isize, dy: isize) -> Result<(), &str> {
        let pos = self.position;
        let (x, y) = (pos.x, pos.y);
        if x > self.max_x || y > self.max_y {
            return Err("out of bounds");
        }
        let (Some(new_x), Some(new_y)) = (x.checked_add_signed(dx), y.checked_add_signed(dy))
        else {
            return Err("out of bounds");
        };

        self.position = Point::new(new_x, new_y);
        Ok(())
    }
    // returns (x, y) coordinates
    pub fn pos(&self) -> Point {
        self.position
    }
}
