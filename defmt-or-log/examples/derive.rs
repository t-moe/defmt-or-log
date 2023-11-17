use defmt_or_log::*; // Import the log macros and the FormatOrDebug trait
use defmt_or_log_macros::*; // Optional: Import the derive macro

#[derive_format_or_debug] // Shorthand for:
                          // #[cfg_attr(feature="defmt", derive(defmt::Format))]
                          // #[cfg_attr(feature="log", derive(core::fmt::Debug))]
                          // Additonally derives FormatOrDebug manually if neither feature is enabled
struct S {
    a: u32,
    b: bool,
}

// A function that takes something which can be formatted
fn do_something<T: FormatOrDebug>(t: T) {
    error!("T is {:?}", t); // logs with defmt or log depending on the current configuration
}

fn main() {
    // <Here you would set up your global_logger for log or defmt>

    let s = S { a: 1, b: true };
    do_something(s); // log some struct

    #[cfg(any(feature = "defmt", feature = "log"))]
    // The blanked implementations of FormatOrDebug are only available if one of the features is enabled
    do_something(42); // log a primitive
}
