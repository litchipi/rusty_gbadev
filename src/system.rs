use crate::graphics::{GbaGraphics, GraphicsConfiguration};
use crate::irq::{GbaIrq, IrqConfiguration};

pub struct GbaSystem<T> {
    pub game: T,
    pub graphics: GbaGraphics,
    pub irq: GbaIrq,
}

impl<T> GbaSystem<T> {
    pub fn new(game: T, gconf: GraphicsConfiguration, iconf: IrqConfiguration) -> GbaSystem<T> {
        GbaSystem {
            game,
            graphics: gconf.into(),
            irq: iconf.into(),
        }
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
