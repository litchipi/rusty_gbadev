use gba::mmio_addresses as addr;
use gba::mmio_types::{DisplayStatus, InterruptFlags, TimerControl};

const SYSTEM_CLOCK : u32 = 16780000;

#[inline]
fn secs_to_cfg(nsecs: f32) -> (u16, u8) {
    let mut ncycles = (nsecs * (SYSTEM_CLOCK as f32)) as u32;
    let mut prescaler = 0;

    let u16max = u16::MAX as u32;

    if ncycles > u16max {
        ncycles = ncycles >> 6;
        prescaler = 1;
    }
    if ncycles > u16max {
        ncycles = ncycles >> 2;
        prescaler = 2;
    }
    if ncycles > u16max {
        ncycles = ncycles >> 2;
        prescaler = 3;
    }
    if ncycles > u16max {
        panic!("Number of cycles too high, consider using a custom prescaler");
    }

    (ncycles as u16, prescaler)
}

#[derive(Debug)]
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
    pub fn enable_selected_irq(&mut self) {
        let flags = InterruptFlags::new()
            .with_vblank(true)
            .with_hblank(self.enable_hblank_irq)
            .with_vcount(self.enable_vcount_irq)
            .with_timer0(self.enable_timer0_irq)
            .with_timer1(self.enable_timer1_irq);

        let display_status = DisplayStatus::new()
            .with_vblank_irq_enabled(true)
            .with_hblank_irq_enabled(self.enable_hblank_irq)
            .with_vcount_irq_enabled(self.enable_vcount_irq);

        unsafe {
            addr::DISPSTAT.write(display_status);
            addr::IE.write(flags)
        };
    }

    pub fn set_timer_irq(&mut self, timer: u8, state: bool) {
        match timer {
            0 => self.enable_timer0_irq = state,
            1 => self.enable_timer1_irq = state,
            _ => unreachable!(),
        }
        self.enable_selected_irq()
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

    pub fn set_timer_raw(&mut self, timer: u8, init_val: u16, prescaler: u8) {
        start_timer(timer, init_val, prescaler);
        self.set_timer_irq(timer, true);
    }

    pub fn set_timer_secs(&mut self, timer: u8, nsecs: f32) {
        let (ncycles, prescaler) = secs_to_cfg(nsecs);
        start_timer(timer, ncycles, prescaler);
        self.set_timer_irq(timer, true);
    }
}

fn start_timer(timer: u8, init_val: u16, prescaler: u8) {
    let (rld, ctl) = match timer {
        0 => (addr::TIMER0_RELOAD, addr::TIMER0_CONTROL),
        1 => (addr::TIMER1_RELOAD, addr::TIMER1_CONTROL),
        _ => unreachable!(),
    };
    rld.write(init_val);
    ctl.write(
        TimerControl::new()
            .with_irq_on_overflow(true)
            .with_enabled(true)
            .with_prescaler_selection(prescaler),
    );
}

impl From<IrqConfiguration> for GbaIrq {
    fn from(_c: IrqConfiguration) -> GbaIrq {
        let mut i = GbaIrq {
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
                let irq_arg = &mut *get_irq_arg();
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
