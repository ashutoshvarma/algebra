use crate::{curves::CurveParameters, GroupAffine, GroupProjective};
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_std::vec::Vec;
use std::cell::Cell;

pub trait CrossBoundary {
    // native boundary instance for passing call to native host
    const NATIVE_BOUNDARY: Cell<Option<&'static dyn NativeBoundary>> = Cell::new(None);

    fn set_native_boundary(nb: &'static dyn NativeBoundary) {
        Self::NATIVE_BOUNDARY.set(Some(nb));
    }

    fn get_boundary() -> Option<&'static dyn NativeBoundary> {
        // Self::NATIVE_BOUNDARY::get()
        Self::NATIVE_BOUNDARY.get()
    }
}

impl<C: AffineCurve> CrossBoundary for GroupAffine<C> {}
impl<C: ProjectiveCurve> CrossBoundary for GroupProjective<C> {}

pub enum CallId {
    // variable_base::multi_scalar_mul
    VBMul,
    // fixed_base::multi_scalar_mul
    FBMul,
}

pub trait NativeBoundary {
    fn call(
        &self,
        id: CallId,
        args: Option<Vec<&[u8]>>,
        cp: Vec<u8>,
    ) -> Result<Option<Vec<&[u8]>>, &'static str>;
}

pub enum Boundary {}

#[derive(Clone)]
pub struct DummyBoundary;

// impl NativeBoundary for DummyBoundary {
//     fn call(
//         &self,
//         id: CallId,
//         args: Option<Vec<&[u8]>>,
//         cp: Vec<u8>,
//     ) -> Result<Option<Vec<&[u8]>>, &'static str> {
//         match id {
//             CallId::VBMul => {
//                 let match 
//             }
//             _ => panic!(),
//         }
//     }
// }

// impl DummyBoundary {
//     fn pass_vb_multi_scalar_mul<G: AffineCurve>(sbases: &[u8], sscalars: &[u8]) -> Vec<u8> {}
// }
