use ark_std::vec::Vec;
use std::cell::Cell;

pub trait CrossBoundary {
    // native boundary instance for passing call to native host
    const NATIVE_BOUNDARY: Cell<Option<&'static dyn NativeBoundary>>;

    // No idea why, but `set` on Cell is not changing the value
    fn set_native_boundary(nb: Option<&'static dyn NativeBoundary>) {
        Self::NATIVE_BOUNDARY.set(nb);
    }

    fn get_boundary() -> Option<&'static dyn NativeBoundary> {
        // Self::NATIVE_BOUNDARY::get()
        Self::NATIVE_BOUNDARY.get()
    }
}

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
    ) -> Result<Option<Vec<Vec<u8>>>, &'static str>;
}
