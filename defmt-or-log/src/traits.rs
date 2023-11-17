#[cfg(not(any(feature = "defmt", feature = "log")))]
/// A trait that describes a type which can be logged with defmt or log, depending on the current configuration.
pub trait FormatOrDebug {}

#[cfg(feature = "defmt")]
/// A trait that describes a type which can be logged with defmt or log, depending on the current configuration.
pub trait FormatOrDebug: defmt::Format {}

#[cfg(feature = "defmt")]
impl<T> FormatOrDebug for T where T: defmt::Format {}

#[cfg(feature = "log")]
/// A trait that describes a type which can be logged with defmt or log, depending on the current configuration.
pub trait FormatOrDebug: core::fmt::Debug {}

#[cfg(feature = "log")]
impl<T> FormatOrDebug for T where T: core::fmt::Debug {}
