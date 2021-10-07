
pub use stm32l4xx_hal as hal;
pub use hal::{
    gpio::gpioa::{PA11, PA12, PA5},
    gpio::{Alternate,
           AF9,
           Output,
           PushPull,
           Floating,
           Input,
           GpioExt},
    prelude::*,
    stm32,
    stm32::{RCC, FLASH},
    flash::FlashExt,
    time::Hertz,
    serial::{self, Config, Serial},
    dma::{self, consts, DMAFrame, FrameReader, FrameSender},
    rcc::{ClockSecuritySystem, CrystalBypass, MsiFreq},

};
use cortex_m::{
    asm::delay,
    interrupt::{CriticalSection, free as disable_interrupts},
};
use cortex_m::peripheral::SCB;
use stm32l4xx_hal::serial::{Tx, Rx};
use cortex_m::interrupt::{Mutex, free};
use core::cell::RefCell;


pub static SERIAL_TX: Mutex<RefCell<Option<Tx<stm32::USART1>>>> = Mutex::new(RefCell::new(None));
pub static SERIAL_RX: Mutex<RefCell<Option<Rx<stm32::USART1>>>> = Mutex::new(RefCell::new(None));

pub unsafe fn setup_peripherals() -> (FLASH, SCB, PA5<Output<PushPull>>){

    let dev = hal::stm32::Peripherals::take().unwrap();
    let mut flash = dev.FLASH.constrain();
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut rcc_reg = dev.RCC;
    rcc_reg.apb1enr1.modify(|_, w| w.can1en().set_bit());

    let mut rcc = rcc_reg.constrain();

    let mut gpioa = dev.GPIOA.split(&mut rcc.ahb2);
    let mut pwr = dev.PWR.constrain(&mut rcc.apb1r1);

    //let clocks = rcc.cfgr.hclk(8.mhz()).freeze(&mut flash.acr, &mut pwr);
    let clocks = rcc
        .cfgr
        .lse(CrystalBypass::Disable, ClockSecuritySystem::Disable)
        .hclk(8.mhz())
        .freeze(&mut flash.acr, &mut pwr);

    let usr_pin = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    // The Serial API is highly generic
    // TRY the commented out, different pin configurations
    let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    // let tx = gpiob.pb6.into_af7(&mut gpiob.moder, &mut gpiob.afrl);

    let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    // let rx = gpiob.pb7.into_af7(&mut gpiob.moder, &mut gpiob.afrl);

    // TRY using a different USART peripheral here
    let serial = Serial::usart1(
        dev.USART1,
        (tx, rx),
        Config::default().baudrate(115200.bps()),
        clocks,
        &mut rcc.apb2,
    );
    let (mut tx, rx) = serial.split();

    free(|cs| {
        SERIAL_TX.borrow(cs).replace(Some(tx));
        SERIAL_RX.borrow(cs).replace(Some(rx));
    });

    (hal::stm32::Peripherals::steal().FLASH, cp.SCB, usr_pin)
}