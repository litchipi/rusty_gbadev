use gba::mmio_types::Color;

pub const BLACK: Color = Color::from_rgb(0, 0, 0);
pub const RED: Color = Color::from_rgb(31, 0, 0);
pub const GREEN: Color = Color::from_rgb(0, 31, 0);
pub const BLUE: Color = Color::from_rgb(0, 0, 31);
pub const YELLOW: Color = Color::from_rgb(31, 31, 0);
pub const PINK: Color = Color::from_rgb(31, 0, 31);

// TODO Macro to get Color from hexa (reducing the color depth if necessary)
// TODO Define more basic colors
