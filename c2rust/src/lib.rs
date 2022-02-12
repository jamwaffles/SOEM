#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod osal {
    pub mod linux {
        pub mod osal;
    } // mod linux
} // mod osal
pub mod oshw {
    pub mod linux {
        pub mod nicdrv;
        pub mod oshw;
    } // mod linux
} // mod oshw

pub mod base;
pub mod coe;
pub mod config;
pub mod dc;
pub mod eoe;
pub mod foe;
pub mod main;
pub mod print;
pub mod soe;
pub mod types;
