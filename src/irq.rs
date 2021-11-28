use gba::mmio_addresses as addr;
use gba::mmio_types::InterruptFlags;

use gba::debug;

pub enum Irq {
    VBlank,
    HBlank,
    VCount,
    Timer0,
    Timer1,
}

pub struct IrqConfiguration;
impl IrqConfiguration {
    pub fn default() -> IrqConfiguration {
        IrqConfiguration {}
    }
}

pub struct GbaIrq {
    enable_hblank_irq: bool,
    enable_vcount_irq: bool,
    enable_timer0_irq: bool,
    enable_timer1_irq: bool,
}

impl GbaIrq {
    pub fn enable_selected_irq(&self) {
        let flags = InterruptFlags::new()
            .with_vblank(true)
            .with_hblank(self.enable_hblank_irq)
            .with_vcount(self.enable_vcount_irq)
            .with_timer0(self.enable_timer0_irq)
            .with_timer1(self.enable_timer1_irq);
        debug!(
            "Enable IRQ {} {} {} {}",
            self.enable_hblank_irq,
            self.enable_vcount_irq,
            self.enable_timer0_irq,
            self.enable_timer1_irq
        );
        unsafe { addr::IE.write(flags) };
    }

    pub fn set_irq(&mut self, irq: Irq) {
        match irq {
            Irq::VBlank => {}
            Irq::HBlank => self.enable_hblank_irq = true,
            Irq::VCount => self.enable_vcount_irq = true,
            Irq::Timer0 => self.enable_timer0_irq = true,
            Irq::Timer1 => self.enable_timer1_irq = true,
        }
    }

    pub fn unset_irq(&mut self, irq: Irq) {
        match irq {
            Irq::VBlank => {}
            Irq::HBlank => self.enable_hblank_irq = false,
            Irq::VCount => self.enable_vcount_irq = false,
            Irq::Timer0 => self.enable_timer0_irq = false,
            Irq::Timer1 => self.enable_timer1_irq = false,
        }
    }

    // TODO Easy wrappers to handle timers
}

impl From<IrqConfiguration> for GbaIrq {
    fn from(_c: IrqConfiguration) -> GbaIrq {
        let i = GbaIrq {
            enable_hblank_irq: false,
            enable_vcount_irq: false,
            enable_timer0_irq: false,
            enable_timer1_irq: false,
        };
        i.enable_selected_irq();
        i
    }
}

#[macro_export]
macro_rules! set_irq_functions {
    ($vblank:ident, $hblank:ident, $vcount:ident, $timer0:ident, $timer1:ident) => {
        use $crate::prelude::addr;

        #[instruction_set(arm::a32)]
        extern "C" fn irq_handler_a32() {
            irq_handler_t32()
        }

        fn init_irq_function() {
            unsafe { addr::USER_IRQ_HANDLER.write(Some(irq_handler_a32)) };
            unsafe { addr::IME.write(true) };
        }

        fn irq_handler_t32() {
            unsafe { addr::IME.write(false) };
            let irq_tohandle = addr::IRQ_PENDING.read() & addr::IE.read();
            let mut intr_wait_flags = addr::INTR_WAIT_ACKNOWLEDGE.read();

            unsafe {
                let irq_arg = get_irq_arg();
                if irq_tohandle.vblank() {
                    $vblank(irq_arg);
                    intr_wait_flags.set_vblank(true);
                }
                if irq_tohandle.hblank() {
                    $hblank(irq_arg);
                    intr_wait_flags.set_hblank(true);
                }
                if irq_tohandle.vcount() {
                    $vcount(irq_arg);
                    intr_wait_flags.set_vcount(true);
                }
                if irq_tohandle.timer0() {
                    $timer0(irq_arg);
                    intr_wait_flags.set_timer0(true);
                }
                if irq_tohandle.timer1() {
                    $timer1(irq_arg);
                    intr_wait_flags.set_timer1(true);
                }
            }

            addr::IRQ_ACKNOWLEDGE.write(irq_tohandle);

            unsafe { addr::INTR_WAIT_ACKNOWLEDGE.write(intr_wait_flags) };
            unsafe { addr::IME.write(true) };
        }
    };
}

// pub fn start_timers() {
//     let init_val: u16 = u32::wrapping_sub(0x1_0000, 64) as u16;
//     const TIMER_SETTINGS: TimerControl = TimerControl::new()
//         .with_irq_on_overflow(true)
//         .with_enabled(true);
//
//     addr::TIMER0_RELOAD.write(init_val);
//     addr::TIMER0_CONTROL.write(TIMER_SETTINGS.with_prescaler_selection(3));
//     addr::TIMER1_RELOAD.write(init_val);
//     addr::TIMER1_CONTROL.write(TIMER_SETTINGS.with_prescaler_selection(1));
// }
//
