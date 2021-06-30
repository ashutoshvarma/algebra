#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unused, future_incompatible, nonstandard_style, rust_2018_idioms)]
#![forbid(unsafe_code)]
#![allow(
    clippy::op_ref,
    clippy::suspicious_op_assign_impl,
    clippy::many_single_char_names
)]

// #[macro_use]
// extern crate ark_std;

pub use ark_ec_orig::{
    prepare_g1, prepare_g2, AffineCurve, CurveCycle, PairingEngine, PairingFriendlyCycle,
    ProjectiveCurve,
};

pub use ark_ec_orig::group;
pub use ark_ec_orig::wnaf;

pub use ark_ec_orig::models;
pub use ark_ec_orig::models::*;

pub use ark_ec_orig::msm;
