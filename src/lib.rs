#![no_std]
#![no_main]

pub mod graphics;
pub mod irq;
pub mod system;

// TODO     feature-conditional message display macros
//      Only allow displaying messages if the feature is allowed in Cargo.toml
//      Will reduce the size of the ROM by stripping the static strings

pub mod prelude {
    pub use core::panic::PanicInfo;

    pub use gba::mmio_addresses as addr;
    pub use gba::prelude::*;
    pub use gba::{debug, error, fatal, info, warning};

    pub use crate::graphics::{colors, GraphicsConfiguration};
    pub use crate::irq::{Irq, IrqConfiguration};
    pub use crate::system::GbaSystem;
    pub use crate::{gba_game, set_irq_functions};
}
