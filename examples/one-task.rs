//! An application with one task
#![deny(unsafe_code)]
#![feature(const_fn)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f103xx;

use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Threshold};

app! {
    device: stm32f103xx,

    // Here resources are declared
    //
    // Resources are static variables that are safe to share across tasks
    resources: {
        // declaration of resources looks exactly like declaration of static
        // variables
        static ON: bool = false;
    },

    // Here tasks are declared
    //
    // Each task corresponds to an interrupt or an exception. Every time the
    // interrupt or exception becomes *pending* the corresponding task handler
    // will be executed.
    tasks: {
        // Here we declare that we'll use the SYS_TICK exception as a task
        SYS_TICK: {
            // Path to the task *handler*
            path: sys_tick,

            // This is the priority of the task.
            //
            // 1 is the lowest priority a task can have.
            // The maximum priority is determined by the number of priority bits
            // the device has. This device has 4 priority bits so 16 is the
            // maximum value.
            priority: 1,

            // These are the *resources* associated with this task
            //
            // The peripherals that the task needs can be listed here
            resources: [GPIOC, ON],
        },
    }
}

fn init(p: init::Peripherals, _r: init::Resources) {
    // power on GPIOC
    p.RCC.apb2enr.modify(|_, w| w.iopcen().enabled());

    // configure PC13 as output
    p.GPIOC.bsrr.write(|w| w.bs13().set());
    p.GPIOC
        .crh
        .modify(|_, w| w.mode13().output().cnf13().push());

    // configure the system timer to generate one interrupt every second
    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(8_000_000); // 1s
    p.SYST.enable_interrupt();
    p.SYST.enable_counter();
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

// This is the task handler of the SYS_TICK exception
//
// `r` is the resources this task has access to. `SYS_TICK::Resources` has one
// field per every resource declared in `app!`.
fn sys_tick(_t: &mut Threshold, r: SYS_TICK::Resources) {
    // toggle state
    **r.ON = !**r.ON;

    if **r.ON {
        // set the pin PC13 high
        r.GPIOC.bsrr.write(|w| w.bs13().set());
    } else {
        // set the pin PC13 low
        r.GPIOC.bsrr.write(|w| w.br13().reset());
    }
}