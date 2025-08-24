use crate::game::point::Point;
use crate::game::rotation::Rotation;
use std::rc::Rc;
use std::sync::atomic::AtomicU8;

static NEXT_SHIP_ID: AtomicU8 = AtomicU8::new(0);
pub struct Ship {
    pub id: u8,
    pub name: String,
    parts_alive: usize,
    pub parts: Vec<Point>,
}
impl Ship {
    pub fn new(points: Vec<Point>, name: String) -> Self {
        let id = NEXT_SHIP_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if id == u8::MAX {
            panic!("out of SHIP_IDs");
        }
        Self {
            id,
            parts_alive: points.len(),
            parts: points,
            name,
        }
    }
    pub fn hit(&mut self) -> Option<&Self> {
        self.parts_alive -= 1;
        if self.parts_alive > 0 {
            return None;
        }
        Some(self)
    }
    pub fn is_alive(&self) -> bool {
        self.parts_alive > 0
    }
}
#[derive(Clone)]
pub struct ShipBlueprint {
    pub parts: Vec<Point>,
    pub name: String,
}
impl ShipBlueprint {
    pub fn new(parts: Vec<Point>, name: String) -> Self {
        Self { parts, name }
    }
}
