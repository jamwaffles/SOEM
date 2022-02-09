#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

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

pub mod ethercatbase;
pub mod ethercatcoe;
pub mod ethercatconfig;
pub mod ethercatdc;
pub mod ethercateoe;
pub mod ethercatfoe;
pub mod ethercatmain;
pub mod ethercatprint;
pub mod ethercatsoe;
pub mod ethercattype;

pub mod test;
