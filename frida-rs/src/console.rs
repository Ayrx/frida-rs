pub use crate::plumbing::console::{error, log, warn};

///Write to the console of a Frida-based application.
///
///The exact behaviour depends on how Frida is integrated. This is equivalent
///to calling `console.log(line)` in the JavaScript API.
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

///Write to the console of a Frida-based application.
///
///The exact behaviour depends on how Frida is integrated. This is equivalent
///to calling `console.warn(line)` in the JavaScript API.
#[macro_export]
macro_rules! console_warn {
    ($($t:tt)*) => (warn(&format_args!($($t)*).to_string()))
}

///Write to the console of a Frida-based application.
///
///The exact behaviour depends on how Frida is integrated. This is equivalent
///to calling `console.err(line)` in the JavaScript API.
#[macro_export]
macro_rules! console_error {
    ($($t:tt)*) => (error(&format_args!($($t)*).to_string()))
}
