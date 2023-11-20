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

    // The entire code in this examples compiles, no matter whether feature log or defmt is enabled

    let s = S { a: 1, b: true };
    do_something(s); // log some struct. This even compiles if neither feature is enabled

    #[cfg(any(feature = "defmt", feature = "log"))]
    // The blanked implementations of FormatOrDebug are only available if one of the features is enabled
    // This might change in future versions of the crate. I haven't made up my mind yet...
    {
        let e = "not_an_int".parse::<i32>().unwrap_err(); //Example for a type for which core::fmt::Debug is implemented, but defmt::Format is not
        do_something(Debug2Format(&e)); // use the Debug2Format adapter, which behaves transparent if log is enabled, and uses defmt::Debug2Format if defmt is enabled

        do_something(42); // log a primitive (which is always possible, because core::fmt::Debug and defmt::Format are implemented for all primitives)
    }
}
