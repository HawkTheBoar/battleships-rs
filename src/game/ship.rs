use crate::game::point::Point;
use crate::game::rotation::Rotation;
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
    pub fn hit(&mut self) -> Option<&Vec<Point>> {
        self.parts_alive -= 1;
        if self.parts_alive > 0 {
            return None;
        }
        Some(&self.parts)
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
    pub fn rotate(&self, target: Rotation) -> Vec<Point> {
        if self.parts.is_empty() {
            return self.parts.clone();
        }
        let deg = target as i32;
        if deg == 0 {
            return self.parts.clone();
        }
        // Convert to signed coordinates
        let signed_points: Vec<(i32, i32)> = self
            .parts
            .iter()
            .map(|p| (p.x as i32, p.y as i32))
            .collect();

        // Find centroid
        let (sum_x, sum_y) = signed_points
            .iter()
            .fold((0, 0), |(sx, sy), &(x, y)| (sx + x, sy + y));
        let n = signed_points.len() as i32;
        let centroid = (sum_x / n, sum_y / n);

        // Rotate points around centroid
        let rad = (deg as f32).to_radians();
        let rotated_points: Vec<(i32, i32)> = signed_points
            .iter()
            .map(|&(x, y)| {
                let dx = x - centroid.0;
                let dy = y - centroid.1;

                let rotated_dx = (dx as f32 * rad.cos() - dy as f32 * rad.sin()).round() as i32;
                let rotated_dy = (dx as f32 * rad.sin() + dy as f32 * rad.cos()).round() as i32;

                (centroid.0 + rotated_dx, centroid.1 + rotated_dy)
            })
            .collect();

        // Find min coordinates to adjust for negative values
        let min_x = rotated_points.iter().map(|p| p.0).min().unwrap();
        let min_y = rotated_points.iter().map(|p| p.1).min().unwrap();

        // Convert back to unsigned coordinates
        // and return
        rotated_points
            .iter()
            .map(|&(x, y)| Point {
                x: (x - min_x) as usize,
                y: (y - min_y) as usize,
            })
            .collect()
    }
}
