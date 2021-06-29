// use ark_ec::{AffineCurve, ProjectiveCurve};
// use ark_ff::{BigInteger, FpParameters, PrimeField};
// use ark_std::vec::Vec;
// use ark_std::{cfg_iter, cfg_iter_mut};

// #[cfg(feature = "parallel")]
// use rayon::prelude::*;

// pub struct FixedBaseMSM;

// impl FixedBaseMSM {
//     pub fn get_mul_window_size(num_scalars: usize) -> usize {
//         0
//     }

//     pub fn get_window_table<T: ProjectiveCurve>(
//         scalar_size: usize,
//         window: usize,
//         g: T,
//     ) -> Vec<Vec<T::Affine>> {
//     }

//     pub fn windowed_mul<T: ProjectiveCurve>(
//         outerc: usize,
//         window: usize,
//         multiples_of_g: &[Vec<T::Affine>],
//         scalar: &T::ScalarField,
//     ) -> T {
//     }

//     pub fn multi_scalar_mul<T: ProjectiveCurve>(
//         scalar_size: usize,
//         window: usize,
//         table: &[Vec<T::Affine>],
//         v: &[T::ScalarField],
//     ) -> Vec<T> {
//     }
// }
