use ark_std::vec::Vec;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[allow(nonstandard_style)]
static mut _BOUNDARY: Option<&'static (dyn NativeBoundary)> = None;
#[allow(nonstandard_style)]
static mut _BOUNDARY_FALLBACK: bool = false;

pub struct Boundary;
impl Boundary {
    #[allow(dead_code)]
    pub fn set_native_boundary(nb: Option<&'static (dyn NativeBoundary)>) {
        unsafe {
            _BOUNDARY = nb;
        }
    }

    #[allow(dead_code)]
    pub fn set_native_fallback(fall: bool) {
        unsafe {
            _BOUNDARY_FALLBACK = fall;
        }
    }

    #[allow(dead_code)]
    pub fn get_native_boundary() -> Option<&'static (dyn NativeBoundary)> {
        unsafe { _BOUNDARY }
    }

    #[allow(dead_code)]
    pub fn get_native_fallback() -> bool {
        unsafe { _BOUNDARY_FALLBACK }
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
