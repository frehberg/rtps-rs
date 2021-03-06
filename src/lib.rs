#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate bit_set;
extern crate bit_vec;
extern crate bytes;
extern crate tokio;
#[macro_use]
extern crate speedy_derive;
extern crate speedy;

#[macro_use]
mod serialization_test;
mod behavior;
mod common;
mod dds;
mod discovery;
mod messages;
mod structure;
