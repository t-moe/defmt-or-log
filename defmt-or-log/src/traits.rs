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

/// Copy of defmt::Debug2Format, except it behaves transparent if log is enabled
pub struct Debug2Format<'a, T: core::fmt::Debug + ?Sized>(pub &'a T);

#[cfg(not(feature = "defmt"))]
impl<T: core::fmt::Debug + ?Sized> core::fmt::Debug for Debug2Format<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "defmt")]
impl<T: core::fmt::Debug + ?Sized> defmt::Format for Debug2Format<'_, T> {
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::Debug2Format(self.0).format(f)
    }
}

/// Copy of defmt::Display2Format, except it behaves transparent if log is enabled
pub struct Display2Format<'a, T: core::fmt::Display + ?Sized>(pub &'a T);

#[cfg(not(feature = "defmt"))]
impl<T: core::fmt::Display + ?Sized> core::fmt::Display for Display2Format<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "defmt")]
impl<T: core::fmt::Display + ?Sized> defmt::Format for Display2Format<'_, T> {
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::Display2Format(self.0).format(f)
    }
}
