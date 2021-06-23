pub mod interface;
pub use interface::{CallId, CrossBoundary, NativeBoundary};

pub mod dummy;
use dummy::DummyBoundary;

use crate::{GroupAffine, GroupProjective};
use ark_ec::{AffineCurve, ProjectiveCurve};
use std::cell::Cell;

impl<C: AffineCurve> CrossBoundary for GroupAffine<C> {
    const NATIVE_BOUNDARY: Cell<Option<&'static dyn NativeBoundary>> =
        Cell::new(Some(&DummyBoundary));
}
impl<C: ProjectiveCurve> CrossBoundary for GroupProjective<C> {
    const NATIVE_BOUNDARY: Cell<Option<&'static dyn NativeBoundary>> =
        Cell::new(Some(&DummyBoundary));
}
