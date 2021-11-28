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
        static mut sys_ptr: *mut GbaSystem<$game> = (0 as *mut GbaSystem<$game>);

        // TODO Use some Mutex to secure this
        unsafe fn get_irq_arg() -> &'static mut GbaSystem<$game> {
            &mut *sys_ptr
        }

        use $crate::prelude::*;

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
                sys_ptr = &mut a as *mut GbaSystem<$game>;
            }
            init_irq_function();
            loop {
                $loop(&mut a);
                unsafe { VBlankIntrWait() };
            }
        }
    };
}
