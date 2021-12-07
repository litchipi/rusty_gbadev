use crate::graphics::{GbaGraphics, GraphicsConfiguration};
use crate::irq::{GbaIrq, IrqConfiguration};

use core::mem::{size_of, transmute_copy};

use gba::info;
use gba::prelude::{use_flash_128k, SaveAccess};

const SAVE_OFFSET: usize = 0;
const FLASH_WRITE_BLOCKSIZE: usize = 512;

pub trait GameState {
    type SaveType;

    fn get_gamesave(&self) -> Self::SaveType;
    fn load_gamesave(&mut self, data: Self::SaveType);
}

pub struct GbaSystem<T: GameState> {
    pub game: T,
    pub graphics: GbaGraphics,
    pub irq: GbaIrq,
}

impl<T: GameState> GbaSystem<T> {
    pub fn new(game: T, gconf: GraphicsConfiguration, iconf: IrqConfiguration) -> GbaSystem<T> {
        GbaSystem {
            game,
            graphics: gconf.into(),
            irq: iconf.into(),
        }
    }

    pub fn save(&self) {
        use_flash_128k();
        let access = SaveAccess::new().unwrap();

        let gsave = self.game.get_gamesave();
        let data: &[u8] = unsafe { transmute_copy(&gsave) };
        let datalen = size_of::<T::SaveType>();

        let mut current = SAVE_OFFSET;
        let end = SAVE_OFFSET + datalen;
        access.prepare_write(current..end).unwrap();

        while current != end {
            let cur_len = if end - current > FLASH_WRITE_BLOCKSIZE {
                FLASH_WRITE_BLOCKSIZE
            } else {
                end - current
            };
            access
                .write(current, &data[current..(current + cur_len)])
                .unwrap();
            current += cur_len;
        }
    }

    pub fn load(&mut self) {
        use_flash_128k();
        let access = SaveAccess::new().unwrap();

        let mut gsave = self.game.get_gamesave();
        let data: &mut [u8] = unsafe { transmute_copy(&gsave) };
        let datalen = size_of::<T::SaveType>();

        let mut current = SAVE_OFFSET;
        let end = SAVE_OFFSET + datalen;

        while current != end {
            let cur_len = if end - current > FLASH_WRITE_BLOCKSIZE {
                FLASH_WRITE_BLOCKSIZE
            } else {
                end - current
            };
            access
                .read(current, &mut data[current..(current + cur_len)])
                .unwrap();
            current += cur_len;
        }
        self.game.load_gamesave(gsave);
    }
}

#[macro_export]
macro_rules! gba_game {
    ($setup:ident, $loop:ident, $game:ident) => {
        use $crate::prelude::*;

        static mut SYS_PTR: *mut GbaSystem<$game> = 0 as *mut GbaSystem<$game>;

        #[inline]
        pub fn get_irq_arg() -> *mut GbaSystem<$game> {
            unsafe { SYS_PTR }
        }

        #[panic_handler]
        #[allow(unused)]
        fn panic(info: &PanicInfo) -> ! {
            fatal!("Panic: {:?}", info);
            loop {
                DISPCNT.read();
            }
        }

        #[no_mangle]
        pub fn main() -> ! {
            let mut a: GbaSystem<$game> = $setup();
            unsafe {
                SYS_PTR = &mut a as *mut GbaSystem<$game>;
            }
            init_irq_function();
            loop {
                $loop(&mut a);
                unsafe { VBlankIntrWait() };
            }
        }
    };
}

/*

pub struct GbaSystem {
    n : u8
}

static mut SYS_PTR: *mut GbaSystem = 0 as *mut GbaSystem;

unsafe fn get_sys() -> &'static mut GbaSystem {
    &mut *(SYS_PTR as *mut GbaSystem)
}

fn main() {
    let mut gba = GbaSystem { n : 3 };
    unsafe {
        SYS_PTR = &mut gba as *mut GbaSystem;
    }
    let mut sys2 = unsafe {get_sys()};
    println!("{} {}", gba.n, sys2.n);
    sys2.n += 1;
    println!("{} {}", gba.n, sys2.n);
}

 * */
