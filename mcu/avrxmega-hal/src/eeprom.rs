//! EEPROM
//!
//! # Example
//!
//! ```
//! const BOOT_COUNT_OFFSET: u16 = 0;
//!
//! let dp = avrxmega_hal::Peripherals::take().unwrap();
//! let mut eeprom = Eeprom::new(dp.NVMCTRL);
//!
//! let mut boot_count = eeprom.read_byte(BOOT_COUNT_OFFSET);
//! boot_count = boot_count.wrapping_add(1);
//! eeprom.write_byte(BOOT_COUNT_OFFSET, boot_count);
//!
//! ufmt::uwriteln!(&mut serial, "Boot count: {}", boot_count).unwrap();
//! ```


use core::arch::asm;
pub use avr_hal_generic::eeprom::{EepromOps, OutOfBoundsError};
pub type Eeprom = avr_hal_generic::eeprom::Eeprom<crate::Avrxmega, crate::pac::NVMCTRL>;

#[cfg(any(
    feature = "attiny204",
    feature = "attiny404"
))]
avr_hal_generic::impl_eeprom_xmega! {
    hal: crate::Avrxmega,
    peripheral: crate::pac::NVMCTRL,
    capacity: 128,
    addr_start: 0x1400,
    addr_width: u16,
}

#[cfg(any(
    feature = "attiny804",
    feature = "attiny1604"
))]
avr_hal_generic::impl_eeprom_xmega! {
    hal: crate::Avrxmega,
    peripheral: crate::pac::NVMCTRL,
    capacity: 256,
    addr_start: 0x1400,
    addr_width: u16,
}

