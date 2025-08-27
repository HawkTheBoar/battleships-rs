#[derive(Clone, Copy)]
pub enum Rotation {
    None,
    Vertical,
    Horizontal,
    VerticalAndHorizontal,
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
