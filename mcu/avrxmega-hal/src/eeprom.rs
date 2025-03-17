// pub use avr_hal_generic::eeprom::{EepromOps, OutOfBoundsError};
// pub type Eeprom = avr_hal_generic::eeprom::Eeprom<crate::Avrxmega, crate::pac::NVMCTRL>;

// impl EepromOps<crate::Avrxmega> for Eeprom {
//     const CAPACITY: u16 = 128; // or 256, TODO

//     fn raw_read_byte(&self, address: u16) -> u8 {
//         self.p.read(address)
//     }

//     fn raw_write_byte(&mut self, address: u16, data: u8) {
//     }

//     fn raw_erase_byte(&mut self, address: u16) {
//     }
// }