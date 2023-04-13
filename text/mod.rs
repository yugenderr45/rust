use graphics::color::BLACK;
use graphics::{Context, Graphics};

/// Renders text
#[derive(Copy, Clone)]
pub struct Text {
    pub color: [f32; 4],
    pub font_size: u32,
    pub round: bool,
}

impl Text {


    /// Creates a new colored text
    pub fn new_color() -> Text {
        Text {
            color: BLACK,
            font_size: 32,
            round: false,
        }
    }

    /// A builder method indicating that the Text's position should be rounded upon drawing.
    pub fn round(mut self) -> Text {
        self.round = true;
        self
    }


}