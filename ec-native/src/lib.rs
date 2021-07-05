#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unused, future_incompatible, nonstandard_style, rust_2018_idioms)]
#![forbid(unsafe_code)]
#![allow(
    clippy::op_ref,
    clippy::suspicious_op_assign_impl,
    clippy::many_single_char_names
)]

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate ark_std;

mod ark_ec;
// for now simply re-export all
pub use ark_ec::*;

pub mod boundary;
pub mod curves;
pub mod msm;
pub mod serialize;
pub mod wrapped;
