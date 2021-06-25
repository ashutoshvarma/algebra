use crate::CurveParameters;
use ark_std::vec::Vec;
use crossbeam_utils::atomic::AtomicCell;

impl<P: CurveParameters> CrossBoundary for P {}

pub trait CrossBoundary {
    #[allow(nonstandard_style)]
    fn NATIVE_BOUNDARY() -> &'static AtomicCell<Option<&'static (dyn NativeBoundary + Sync)>> {
        static STATIC: AtomicCell<Option<&'static (dyn NativeBoundary + Sync)>> =
            AtomicCell::new(None);
        &STATIC
    }

    #[allow(nonstandard_style)]
    fn NATIVE_FALLBACK() -> &'static AtomicCell<bool> {
        static STATIC: AtomicCell<bool> = AtomicCell::new(false);
        &STATIC
    }

    fn set_native_boundary(nb: Option<&'static (dyn NativeBoundary + Sync)>) {
        Self::NATIVE_BOUNDARY().store(nb);
    }

    fn set_native_fallback(fall: bool) {
        Self::NATIVE_FALLBACK().store(fall);
    }

    fn get_native_boundary() -> Option<&'static (dyn NativeBoundary + Sync)> {
        Self::NATIVE_BOUNDARY().load()
    }
    fn get_native_fallback() -> bool {
        Self::NATIVE_FALLBACK().load()
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
        wrapped: bool,
    ) -> Result<Option<Vec<Vec<u8>>>, &'static str>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Curve;
    impl CrossBoundary for Curve {}

    struct NB;
    impl NativeBoundary for NB {
        fn call(
            &self,
            _: CallId,
            _: Option<Vec<&[u8]>>,
            _: Vec<u8>,
            _: bool,
        ) -> Result<Option<Vec<Vec<u8>>>, &'static str> {
            Ok(None)
        }
    }

    #[test]
    fn test_set_boundary() {
        Curve::set_native_boundary(Some(&NB));
        Curve::get_native_boundary().unwrap();

        Curve::set_native_fallback(true);
        assert_eq!(Curve::get_native_fallback(), true);
    }
}
