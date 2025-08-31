#[derive(Clone, Copy, PartialEq)]
pub enum Rotation {
    None = 0,
    Horizontal = 90,
    Vertical = 180,
    VerticalAndHorizontal = 270,
}
impl Rotation {
    pub fn next(self) -> Rotation {
        match self {
            Self::None => Self::Vertical,
            Self::Vertical => Self::Horizontal,
            Self::Horizontal => Self::VerticalAndHorizontal,
            Self::VerticalAndHorizontal => Self::None,
        }
    }
}
