// Copied from esp-hal: https://github.com/esp-rs/esp-hal/blob/main/esp-hal-common/src/fmt.rs
#[cfg(all(feature = "defmt", feature = "log"))]
compile_error!("You may not enable both `defmt` and `log` features.");

#[cfg(all(feature = "at_least_one", not(feature = "defmt"), not(feature = "log")))]
compile_error!("You have to enable either the `defmt` or the `log` feature (because feature at_least_one is set).");

#[cfg(feature = "defmt")]
#[doc(hidden)]
#[macro_export]
macro_rules! defmt_or_core {
    ($macro:ident, $($x:tt)*) => {
        $crate::__private::defmt::$macro!($($x)*);
    };
}

#[cfg(not(feature = "defmt"))]
#[doc(hidden)]
#[macro_export]
macro_rules! defmt_or_core {
    ($macro:ident, $($x:tt)*) => {
        ::core::$macro!($($x)*);
    };
}

#[cfg(feature = "defmt")]
#[doc(hidden)]
#[macro_export]
macro_rules! maybe_defmt {
    ($macro:ident, $s:literal $(, $x:expr)* $(,)?) => {
        $crate::__private::defmt::$macro!($s $(, $x)*);
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "defmt"))]
macro_rules! maybe_defmt {
    ($macro:ident, $s:literal $(, $x:expr)* $(,)?) => {
        let _ = ($($x),*);
    };
}

#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! maybe_log {
    ($macro:ident, $s:literal $(, $x:expr)* $(,)?) => {
        $crate::__private::log::$macro!($s $(, $x)*);
    };
}

#[cfg(not(feature = "log"))]
#[doc(hidden)]
#[macro_export]
macro_rules! maybe_log {
    ($macro:ident, $s:literal $(, $x:expr)* $(,)?) => {
        let _ = ($($x),*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! defmt_or_log {
    ($macro:ident, $s:literal $(, $x:expr)* $(,)?) => {
        {
            let _ = ($($x),*);
            $crate::maybe_defmt!($macro, $s $(, $x)*);
            $crate::maybe_log!($macro, $s $(, $x)*);
        }
    };
}


#[macro_export]
macro_rules! assert {
    ($($x:tt)*) => {
        {
            $crate::defmt_or_core!(assert, $($x)*)
        }
    };
}

#[macro_export]
macro_rules! assert_eq {
    ($($x:tt)*) => {
        {
            $crate::defmt_or_core!(assert_eq, $($x)*)
        }
    };
}

#[macro_export]
macro_rules! assert_ne {
    ($($x:tt)*) => {
        {
            $crate::defmt_or_core!(assert_ne, $($x)*)
        }
    };
}

#[macro_export]
macro_rules! debug_assert {
    ($($x:tt)*) => {
        {
            $crate::defmt_or_core!(debug_assert, $($x)*)
        }
    };
}

#[macro_export]
macro_rules! debug_assert_eq {
    ($($x:tt)*) => {
        {
            $crate::defmt_or_core!(debug_assert_eq, $($x)*)
        }
    };
}

#[macro_export]
macro_rules! debug_assert_ne {
    ($($x:tt)*) => {
        {
            $crate::defmt_or_core!(debug_assert_ne, $($x)*)
        }
    };
}

#[macro_export]
macro_rules! todo {
    ($($x:tt)*) => {
        {
            $crate::defmt_or_core!(todo, $($x)*)
        }
    };
}

#[macro_export]
macro_rules! unreachable {
    ($($x:tt)*) => {
        {
            $crate::defmt_or_core!(unreachable, $($x)*)
        }
    };
}

#[macro_export]
macro_rules! panic {
    ($($x:tt)*) => {
        {
            $crate::defmt_or_core!(panic, $($x)*)
        }
    };
}

#[macro_export]
macro_rules! trace {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::defmt_or_log!(trace, $s $(, $x)*)
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::defmt_or_log!(debug, $s $(, $x)*)
        }
    };
}

#[macro_export]
macro_rules! info {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::defmt_or_log!(info, $s $(, $x)*)
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::defmt_or_log!(warn, $s $(,$x)*)
        }
    };
}

#[macro_export]
macro_rules! error {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::defmt_or_log!(error, $s $(, $x)*)
        }
    };
}

#[cfg(not(feature = "defmt"))]
#[macro_export]
macro_rules! intern {
    ($s:literal) => {
        $s
    };
}

#[cfg(feature = "defmt")]
#[macro_export]
macro_rules! intern {
    ($s:literal) => {
        $crate::__private::defmt::intern!($s)
    };
}

#[cfg(feature = "defmt")]
#[macro_export]
macro_rules! unwrap {
    ($($x:tt)*) => {
        $crate::__private::defmt::unwrap!($($x)*)
    };
}

#[cfg(not(feature = "defmt"))]
#[macro_export]
macro_rules! unwrap {
    ($arg:expr) => {
        match defmt_or_log::macros::Try::into_result($arg) {
            ::core::result::Result::Ok(t) => t,
            ::core::result::Result::Err(e) => {
                ::core::panic!("unwrap of `{}` failed: {:?}", ::core::stringify!($arg), e);
            }
        }
    };
    ($arg:expr, $($msg:expr),+ $(,)? ) => {
        match defmt_or_log::macros::Try::into_result($arg) {
            ::core::result::Result::Ok(t) => t,
            ::core::result::Result::Err(e) => {
                ::core::panic!("unwrap of `{}` failed: {}: {:?}", ::core::stringify!($arg), ::core::format_args!($($msg,)*), e);
            }
        }
    }
}

#[cfg(feature = "defmt")]
#[macro_export]
macro_rules! expect {
    ($($x:tt)*) => {
        $crate::__private::defmt::expect!($($x)*)
    };
}

#[cfg(not(feature = "defmt"))]
#[macro_export]
macro_rules! expect {
    ($x:expr, $msg:expr) => {
        $x.expect($msg)
    };
}

#[macro_export]
macro_rules! unimplemented {
    ($($x:tt)*) => {
        {
            $crate::defmt_or_core!(unimplemented, $($x)*)
        }
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NoneError;

pub trait Try {
    type Ok;
    type Error;
    fn into_result(self) -> Result<Self::Ok, Self::Error>;
}

impl<T> Try for Option<T> {
    type Ok = T;
    type Error = NoneError;

    #[inline]
    fn into_result(self) -> Result<T, NoneError> {
        self.ok_or(NoneError)
    }
}

impl<T, E> Try for Result<T, E> {
    type Ok = T;
    type Error = E;

    #[inline]
    fn into_result(self) -> Self {
        self
    }
}
