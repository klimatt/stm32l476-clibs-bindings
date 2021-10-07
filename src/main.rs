#![no_std]
#![no_main]

use core::borrow::{Borrow, BorrowMut};
use core::iter::StepBy;
use core::ops::{Range, Deref, DerefMut};

use cortex_m::asm;
use cortex_m::asm::delay;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m::peripheral::SCB;
use rtt_target::{rprintln, rtt_init_print};
mod stm32l4_peripherals;
mod bindings;

use stm32l4_peripherals as stm32p;

use cortex_m::register::control::Spsel::Msp;

use bindings::shtp::*;
use embedded_hal::prelude::_embedded_hal_blocking_serial_Write;


#[macro_use(block)]
extern crate nb;

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


//static mut SERIAL_TX: Tx<stm32p::stm32::USART1> = ;



extern "C" fn open(hal: *mut sh2_Hal_s) -> i32 {
    rprintln!("open");
    0
}
extern "C" fn close(hal: *mut sh2_Hal_s) {
    rprintln!("close");
    //0
}
extern "C" fn read(hal: *mut sh2_Hal_s, data: *mut u8, len: u32, t: *mut u32) -> i32 {
    rprintln!("read");
    0
}
extern "C" fn write(hal: *mut sh2_Hal_s, data: *mut u8, len: u32) -> i32 {
    rprintln!("write");
    free(|cs| {
        let mut tx_ref = stm32p::SERIAL_TX.borrow(cs).borrow_mut();
        if let Some(ref mut t) = tx_ref.deref_mut() {
            t.bwrite_all(&[0x01]).unwrap();
        }
    });


    0
}
extern "C" fn getTimeUs(hal: *mut sh2_Hal_s) -> u32 {
    rprintln!("getTimeUs");
    0
}

extern "C" fn eventHandler(cookie: *mut cty::c_void, pEvent: *mut sh2_AsyncEvent_t){
    rprintln!("eventHandler");
}



#[entry]
fn main() -> ! {
    // To not have main optimize to abort in release mode, remove when you add code
    rtt_init_print!();
    rprintln!("init");
    let (mut flash_regs, mut scb, mut usr_led) = unsafe { stm32p::setup_peripherals()};
    rprintln!("sh2_inst");
    let mut sh2_inst = sh2_Hal_t {
        open: Some(open),
        close: Some(close),
        read: Some(read),
        write: Some(write),
        getTimeUs: Some(getTimeUs)
    };
    rprintln!("sh2_open");
    let status = unsafe{ sh2_open(&mut sh2_inst, core::prelude::v1::Some(eventHandler),  core::ptr::null_mut()) };

    rprintln!(" res: {}", status);
    loop{
        asm::delay(8_000_000);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use core::ptr::{null, NonNull};
use stm32l4xx_hal::serial::Tx;
use cortex_m::interrupt::{Mutex, free};
use core::cell::RefCell;

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rprintln!("Panic {:?}", _info);
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}