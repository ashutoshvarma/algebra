use ark_std::vec::Vec;
use crossbeam_utils::atomic::AtomicCell;
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub struct Boundary;
impl Boundary {
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

    #[allow(dead_code)]
    pub fn set_native_boundary(nb: Option<&'static (dyn NativeBoundary + Sync)>) {
        Self::NATIVE_BOUNDARY().store(nb);
    }

    #[allow(dead_code)]
    pub fn set_native_fallback(fall: bool) {
        Self::NATIVE_FALLBACK().store(fall);
    }

    #[allow(dead_code)]
    pub fn get_native_boundary() -> Option<&'static (dyn NativeBoundary + Sync)> {
        Self::NATIVE_BOUNDARY().load()
    }

    #[allow(dead_code)]
    pub fn get_native_fallback() -> bool {
        Self::NATIVE_FALLBACK().load()
    }
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CallId {
    // variable_base::multi_scalar_mul
    VBMul,
    // fixed_base::multi_scalar_mul
    FBMul,
    // ProjectiveCurve::batch_normalization
    ProjBN,
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
        Boundary::set_native_boundary(Some(&NB));
        Boundary::get_native_boundary().unwrap();

        Boundary::set_native_fallback(true);
        assert_eq!(Boundary::get_native_fallback(), true);
    }
}
