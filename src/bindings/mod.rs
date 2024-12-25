#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::module_inception)]

#[cfg(not(feature = "debug"))]
mod bindings;
#[cfg(feature = "debug")]
mod bindings_debug;

#[cfg(not(feature = "debug"))]
pub use self::bindings::*;
#[cfg(feature = "debug")]
pub use self::bindings_debug::*;
