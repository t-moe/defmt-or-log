//! Newtypes that implement styles of display that can not be expressed with a unified portable
//! expression in both defmt and Rust string formatting.

/// A newtype around byte slices produces hex output.
///
/// Its preferred output is `00 11 22 33`, but backends may also produce something like `[00, 11,
/// 22, 33]` (eg. while that is cheaper on defmt).
///
/// Instead of writing some variation of `info!("Found bytes {:02x}", data)` in defmt (or
/// `… {:02x?}", data`) for log), you can write `info!("Found bytes {}", Hex(&data))`.
pub struct Hex<T: AsRef<[u8]>>(pub T);

impl<T: AsRef<[u8]>> core::fmt::Display for Hex<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut first = true;
        for byte in self.0.as_ref() {
            if !first {
                write!(f, " ")?;
            }
            write!(f, "{:02x}", byte)?;
            first = false;
        }
        Ok(())
    }
}

#[cfg(feature = "defmt")]
impl<T: AsRef<[u8]>> defmt::Format for Hex<T> {
    fn format(&self, f: defmt::Formatter) {
        // If defmt ever gains a `:hexdump` or similar formatting, this can be upgraded.
        ::defmt::write!(f, "{=[u8]:02x}", self.0.as_ref())
    }
}

/// A newtype around byte slices used for that prefers interpreting the data as CBOR.
///
/// Its preferred output is CBOR Diagnostic Notation (EDN), but showing hex is also acceptable.
///
/// Instead of writing some variation of `info!("Found bytes {:cbor}", item)`, you can write
/// `info!("Found bytes {}", Cbor(&item))`.
///
/// Note that using this wrapper is not necessary when using a
/// [`cboritem::CborItem`](https://docs.rs/cboritem/latest/cboritem/struct.CborItem.html) as it
/// already does something similar on its own.
pub struct Cbor<T: AsRef<[u8]>>(pub T);

impl<T: AsRef<[u8]>> core::fmt::Display for Cbor<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Hex(self.0.as_ref()).fmt(f)
    }
}

#[cfg(feature = "defmt")]
impl<T: AsRef<[u8]>> defmt::Format for Cbor<T> {
    fn format(&self, f: defmt::Formatter) {
        ::defmt::write!(f, "{=[u8]:cbor}", self.0.as_ref())
    }
}

#[test]
fn test_hex() {
    extern crate alloc;
    use alloc::format;
    assert_eq!("", format!("{}", Hex([])));
    assert_eq!("00", format!("{}", Hex([0x00])));
    assert_eq!("00 01 02", format!("{}", Hex([0x00, 0x01, 0x02])));
}
