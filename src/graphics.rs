use gba::mmio_addresses as addr;
use gba::mmio_types::{Color, DisplayControl};

pub mod colors;

pub struct GraphicsConfiguration {
    display_mode: u16,
    with_display_bg: [bool; 4],
}

impl GraphicsConfiguration {
    pub fn default() -> GraphicsConfiguration {
        GraphicsConfiguration {
            display_mode: 3,
            with_display_bg: [false, false, true, false],
        }
    }
}

pub struct GbaGraphics {
    pixel: usize,
    ctl: DisplayControl,
}

impl From<GraphicsConfiguration> for GbaGraphics {
    fn from(c: GraphicsConfiguration) -> GbaGraphics {
        let h = GbaGraphics {
            pixel: 0,
            ctl: DisplayControl::new()
                .with_display_mode(c.display_mode)
                .with_display_bg0(c.with_display_bg[0])
                .with_display_bg1(c.with_display_bg[1])
                .with_display_bg2(c.with_display_bg[2])
                .with_display_bg3(c.with_display_bg[3]),
        };
        h.apply_config();
        h.fill_screen(colors::BLACK);
        h
    }
}

impl GbaGraphics {
    pub fn apply_config(&self) {
        addr::DISPCNT.write(self.ctl);
    }

    pub fn fill_screen(&self, color: Color) {
        addr::mode3::dma3_clear_to(color);
    }

    pub fn write_pixel(&mut self, color: Color) {
        unsafe {
            (0x0600_0000 as *mut Color)
                .wrapping_add(self.pixel)
                .write_volatile(color);
        }
        self.pixel += 1;
        if self.pixel == (addr::mode3::WIDTH * addr::mode3::HEIGHT) {
            self.pixel = 0;
        }
    }
}

// TODO Implement traits
//      Drawable
//          set_visible(visible: bool)
//          draw()
//
//      Sprite : Drawable
//          set_animation_frame(nframe: u32)
//          set_orientation(orientation: SpriteOrientation)
//          get_hitbox() -> (ScreenCoord, ScreenCoord)
//                          // R G B
//          alter_color(channel: char, alteration: i8)
//
//      Player interface : Drawable
//
//      Background : Drawable
//          Self::Identifier

