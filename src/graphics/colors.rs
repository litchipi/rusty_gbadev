use gba::mmio_types::Color;

pub const BLACK: Color = Color::from_rgb(0, 0, 0);
pub const WHITE: Color = Color::from_rgb(31, 31, 31);

pub const RED: Color = Color::from_rgb(31, 0, 0);
pub const ORANGE: Color = Color::from_rgb(31, 15, 0);
pub const YELLOW: Color = Color::from_rgb(31, 31, 0);
pub const LIME: Color = Color::from_rgb(15, 31, 0);
pub const GREEN: Color = Color::from_rgb(0, 31, 0);
pub const SPRING: Color = Color::from_rgb(0, 31, 15);
pub const CYAN: Color = Color::from_rgb(0, 31, 31);
pub const DODGERBLUE: Color = Color::from_rgb(0, 15, 31);
pub const BLUE: Color = Color::from_rgb(0, 0, 31);
pub const VIOLET: Color = Color::from_rgb(15, 0, 31);
pub const MAGENTA: Color = Color::from_rgb(31, 0, 31);
pub const PINK: Color = Color::from_rgb(31, 0, 15);

#[inline]
const fn hex_value(h: u8) -> u8 {
    match h {
        48 => 0,
        49 => 1,
        50 => 2,
        51 => 3,
        52 => 4,
        53 => 5,
        54 => 6,
        55 => 7,
        56 => 8,
        57 => 9,
        65 | 97 => 10,
        66 | 98 => 11,
        67 | 99 => 12,
        68 | 100 => 13,
        69 | 101 => 14,
        70 | 102 => 15,
        _ => unreachable!(),
    }
}

#[inline]
const fn hex_to_u8(hex0: u8, hex1: u8) -> u8 {
    (hex_value(hex0) << 4) + hex_value(hex1)
}

macro_rules! char_ind {
    ($str:expr, $ind:expr) => {
        $str.as_bytes()[$ind]
    };
}

pub const fn color_from_hex(hex: &'static str) -> Color {
    let r: u8 = (hex_to_u8(char_ind!(hex, 0), char_ind!(hex, 1)) >> 3) as u8;
    let g: u8 = (hex_to_u8(char_ind!(hex, 2), char_ind!(hex, 3)) >> 3) as u8;
    let b: u8 = (hex_to_u8(char_ind!(hex, 4), char_ind!(hex, 5)) >> 3) as u8;
    Color::from_rgb(r, g, b)
}
