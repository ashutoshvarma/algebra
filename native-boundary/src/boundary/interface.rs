use ark_std::vec::Vec;
use std::cell::Cell;

pub trait CrossBoundary {
    // native boundary instance for passing call to native host
    const NATIVE_BOUNDARY: Cell<Option<&'static dyn NativeBoundary>>;
    const NATIVE_FALLBACK: Cell<bool> = Cell::new(false);

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
    // This methods call the native host with serialized args
    fn call(
        &self,
        id: CallId,
        args: Option<Vec<&[u8]>>,
        cp: Vec<u8>,
    ) -> Result<Option<Vec<Vec<u8>>>, &'static str>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Curve;
    impl CrossBoundary for Curve {
        const NATIVE_BOUNDARY: Cell<Option<&'static dyn NativeBoundary>> = Cell::new(None);
    }

    struct NB;
    impl NativeBoundary for NB {
        fn call(
            &self,
            _: CallId,
            _: Option<Vec<&[u8]>>,
            _: Vec<u8>,
        ) -> Result<Option<Vec<Vec<u8>>>, &'static str> {
            Ok(None)
        }
    }

    #[test]
    fn test_set_boundary() {
        // Curve::set_native_boundary(Some(&NB));
        // Curve::get_boundary().unwrap();
    //     Curve::NATIVE_FALLBACK = Cell::new(true);
    //     assert_eq!(Curve::NATIVE_FALLBACK.get(), true);
    }
}
