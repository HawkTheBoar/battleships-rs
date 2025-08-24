#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Tile {
    Ship(u8),
    SunkenShip,
    Hit,
    Miss,
    Empty,
    Hidden,
}
