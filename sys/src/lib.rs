#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[cfg(not(no_static_bindings))]
mod static_bindings {
    include!("./bindings.rs");
}
#[cfg(not(no_static_bindings))]
pub use static_bindings::*;

#[cfg(dynamic)]
pub mod dynamic;