use stm32f0xx_hal::{
    gpio::gpioa::{PA11, PA12},
    gpio::gpiob::{PB3},
    gpio::{Alternate, AF4, Output, PushPull},
    spi::Spi,
    stm32::{SPI1}
};

pub type CAN_TX_PIN = PA12<Alternate<AF4>>;
pub type CAN_RX_PIN = PA11<Alternate<AF4>>;
pub type USR_LED_PIN = PB3<Output<PushPull>>;


#[cfg(feature = "master")]
pub const CAN_ID: u32 = 101;
#[cfg(feature = "tr")]
pub const CAN_ID: u32 = 102;
#[cfg(feature = "bl")]
pub const CAN_ID: u32 = 103;
#[cfg(feature = "br")]
pub const CAN_ID: u32 = 104;


pub enum UAVCAN_PRIORITY {
    UcpExceptional = 0,
    UcpImmediate = 1,
    UcpFast = 2,
    UcpHigh = 3,
    UcpNominal = 4,
    UcpLow = 5,
    UCP_Slow = 6,
    UcpOptional = 7
}

pub fn get_uavcan_id(port: u32, node_id: u32, priority: UAVCAN_PRIORITY) -> u32{
    let prio = priority as u32;
    if prio < 7 && port < 32767 && node_id < 127 {
        prio << 26 | port << 8 | node_id
    }
    else{
        0
    }

}

