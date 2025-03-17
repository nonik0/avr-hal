// TODO: really need to figure out how to design handle using ADC0 and VREF peripherals
// should ADC manage both together or should they be separate, or maybe an optional reference to VREF?

#![allow(non_camel_case_types)]
//! Analog-to-Digital Converter
//!
//! # Example
//!
//! TODO
//! ```

use crate::port;
use avr_hal_generic::adc::{AdcChannel, ClockDivider};

/// Select the voltage reference for the ADC peripheral
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ReferenceVoltage {
    /// Internal 1.1V? reference.
    Internal = 0b00,
    /// VDD as reference voltage.
    VDD = 0b01,
}

impl Default for ReferenceVoltage {
    fn default() -> Self {
        Self::Internal
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u8)]
// pub enum Resolution {
//     _10bit = 0b0,
//     _12bit = 0b1,
// }

// impl Default for Resolution {
//     fn default() -> Self {
//         Self::_10bit
//     }
// }

/// Configuration for the ADC peripheral.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdcSettings {
    pub clock_divider: ClockDivider,
    pub ref_voltage: ReferenceVoltage,
    //pub resolution: Resolution,
}

/// Check the [`avr_hal_generic::adc::Adc`] documentation.
pub type Adc<CLOCK> = avr_hal_generic::adc::Adc<crate::Avrxmega, crate::pac::ADC0, CLOCK>;

/// Check the [`avr_hal_generic::adc::Channel`] documentation.
pub type Channel = avr_hal_generic::adc::Channel<crate::Avrxmega, crate::pac::ADC0>;

/// Additional channels
///
/// Some channels are not directly connected to pins.  This module provides types which can be used
/// to access them.
///
/// # Example
/// ```
/// let dp = avrxmega_hal::Peripherals::take().unwrap();
/// let mut adc = avrxmega_hal::Adc::new(dp.ADC, Default::default());
///
/// let value = adc.read_blocking(&channel::Vbg);
/// ```
pub mod channel {
    pub struct IntRef;
    pub struct TempSense;
    pub struct Gnd;
}

// #[cfg(any(feature = "attiny204", feature = "attiny404"))]

// #[cfg(any(feature = "attiny804", feature = "attiny1604"))]
impl avr_hal_generic::adc::AdcOps<crate::Avrxmega> for crate::pac::ADC0 {
    type Channel = crate::pac::adc0::muxpos::MUXPOS_A;
    type Settings = AdcSettings;

    #[inline]
    fn raw_init(&mut self, settings: Self::Settings) {
        self.ctrla.write(|w| w.enable().set_bit());
        // TODO: resolution
        self.ctrlc.write(|w| {
            match settings.ref_voltage {
                ReferenceVoltage::Internal => w.refsel().intref(),
                ReferenceVoltage::VDD => w.refsel().vddref(),
            };
            match settings.clock_divider {
                ClockDivider::Factor2 => w.presc().div2(),
                ClockDivider::Factor4 => w.presc().div4(),
                ClockDivider::Factor8 => w.presc().div8(),
                ClockDivider::Factor16 => w.presc().div16(),
                ClockDivider::Factor32 => w.presc().div32(),
                ClockDivider::Factor64 => w.presc().div64(),
                ClockDivider::Factor128 => w.presc().div128(),
                //ClockDivider::Div256 => w.presc().div256(),
            }
        });
    }

    #[inline]
    fn raw_read_adc(&self) -> u16 {
        self.res.read().bits()
    }

    #[inline]
    fn raw_is_converting(&self) -> bool {
        self.command.read().stconv().bit_is_set()
    }

    #[inline]
    fn raw_start_conversion(&mut self) {
        self.command.write(|w| w.stconv().set_bit());
    }

    #[inline]
    fn raw_set_channel(&mut self, channel: Self::Channel) {
        self.muxpos.modify(|_, w| w.muxpos().variant(channel));
    }

    #[inline]
    fn raw_enable_channel(&mut self, channel: Self::Channel) {
        self.muxpos.modify(|_, w| w.muxpos().variant(channel));
    }

    #[inline]
    fn raw_disable_channel(&mut self, _channel: Self::Channel) {
        // noop
    }
}

// TODO:
//[cfg(any(feature = "attiny204", feature = "attiny404"))]

#[cfg(any(feature = "attiny804", feature = "attiny1604"))]
crate::impl_avrxmega_adc! {
    hal: crate::Avrxmega,
    peripheral: crate::pac::ADC0,
    //settings: AdcSettings,
    // apply_settings: |peripheral, settings| { apply_settings(peripheral, settings) },
    channel_id: crate::pac::adc0::muxpos::MUXPOS_A,
    // set_channel: |peripheral, id| {
    //     peripheral.admux.modify(|_, w| w.mux().variant(id));
    // },
    pins: {
        port::PA0: (crate::pac::adc0::muxpos::MUXPOS_A::AIN0),
        port::PA1: (crate::pac::adc0::muxpos::MUXPOS_A::AIN1),
        port::PA2: (crate::pac::adc0::muxpos::MUXPOS_A::AIN2),
        port::PA3: (crate::pac::adc0::muxpos::MUXPOS_A::AIN3),
        port::PA4: (crate::pac::adc0::muxpos::MUXPOS_A::AIN4),
        port::PA5: (crate::pac::adc0::muxpos::MUXPOS_A::AIN5),
        port::PA6: (crate::pac::adc0::muxpos::MUXPOS_A::AIN6),
        port::PA7: (crate::pac::adc0::muxpos::MUXPOS_A::AIN7),
        port::PB1: (crate::pac::adc0::muxpos::MUXPOS_A::AIN10),
        port::PB0: (crate::pac::adc0::muxpos::MUXPOS_A::AIN11),
    },
    channels: {
        // channel::IntRef: crate::pac::adc0::muxpos::MUXPOS_A::INTREF,
        // channel::TempSense: crate::pac::adc0::muxpos::MUXPOS_A::TEMPSENSE,
        // channel::Gnd: crate::pac::adc0::muxpos::MUXPOS_A::GND,
    },
}

#[macro_export]
macro_rules! impl_avrxmega_adc {
    (
        hal: $HAL:ty,
        peripheral: $ADC:ty,
        //settings: $Settings:ty,
        //apply_settings: |$settings_periph_var:ident, $settings_var:ident| $apply_settings:block,
        channel_id: $Channel:ty,
        //set_channel: |$periph_var:ident, $chan_var:ident| $set_channel:block,
        pins: {
            $(
                $(#[$pin_attr:meta])*
                $pin:ty: ($pin_channel:expr$(, $didr:ident::$didr_method:ident)?),
            )+
        },
        $(channels: {
            $(
                $(#[$channel_attr:meta])*
                $channel_ty:ty: $channel:expr,
            )*
        },)?
    ) => {
        $(
        $(#[$pin_attr])*
        impl $crate::adc::AdcChannel<$HAL, $ADC> for $crate::port::Pin<$crate::port::mode::Analog, $pin> {
            #[inline]
            fn channel(&self) -> $Channel {
                $pin_channel
            }
        }
        )+

        $($(
        $(#[$channel_attr])*
        impl $crate::adc::AdcChannel<$HAL, $ADC> for $channel_ty {
            #[inline]
            fn channel(&self) -> $Channel {
                $channel
            }
        }

        /// Convert this channel into a generic "[`Channel`][adc-channel]" type.
        ///
        /// The generic channel type can be used to store multiple channels in an array.
        ///
        /// [adc-channel]: crate::adc::Channel
        $(#[$channel_attr])*
        impl $channel_ty {
            pub fn into_channel(self) -> $crate::adc::Channel<$HAL, $ADC> {
                $crate::adc::Channel::new(self)
            }
        }
        )*)?
    };
}
