pub mod dummy;
pub use dummy::DummyBoundary;

pub mod handler;
pub use handler::SimpleNativeCallHandler;

use crate::{
    curves::*,
    serialize::{NonCanonicalDeserialize, NonCanonicalSerialize},
};
use crate::ark_ec::{
    models::{
        short_weierstrass_jacobian::{GroupAffine as SWAffine, GroupProjective as SWProjective},
        twisted_edwards_extended::{GroupAffine as EDAffine, GroupProjective as EDProjective},
    },
    AffineCurve, ModelParameters, ProjectiveCurve, SWModelParameters, TEModelParameters,
};
use ark_std::{convert::TryInto, vec::Vec};
use crossbeam_utils::atomic::AtomicCell;
use num_enum::{IntoPrimitive, TryFromPrimitive};

// This trait implies a AffineCurve capable of crossing boundary
pub trait CrossAffine: AffineCurve + CrossBoundary
where
    <Self as AffineCurve>::Projective: CrossProjective,
{
}

// This trait implies a ProjectiveCurve capable of crossing boundary
pub trait CrossProjective: ProjectiveCurve + CrossBoundary
where
    <Self as ProjectiveCurve>::Affine: CrossAffine,
{
}

pub enum CurveType {
    SW,
    ED,
}
// This trait adds a associate type which will be used to match curves types
pub trait CurveParameters {
    type SWParameters: SWModelParameters;
    type TEParameters: TEModelParameters;
    const TYPE: CurveType;
}

// Default Implementations
//

impl<T: AffineCurve + CrossBoundary> CrossAffine for T where
    <T as AffineCurve>::Projective: CrossBoundary
{
}
impl<T: ProjectiveCurve + CrossBoundary> CrossProjective for T where
    <T as ProjectiveCurve>::Affine: CrossBoundary
{
}

impl<P: SWModelParameters> CurveParameters for SWAffine<P> {
    type SWParameters = P;
    // Just a placeholder
    type TEParameters = crate::curves::ed_on_bls12_377::EdwardsParameters;
    const TYPE: CurveType = CurveType::SW;
}

impl<P: SWModelParameters> CurveParameters for SWProjective<P> {
    type SWParameters = P;
    // Just a placeholder
    type TEParameters = crate::curves::ed_on_bls12_377::EdwardsParameters;
    const TYPE: CurveType = CurveType::SW;
}

impl<P: TEModelParameters> CurveParameters for EDAffine<P> {
    type TEParameters = P;
    // Just a placeholder
    type SWParameters = crate::curves::pallas::PallasParameters;
    const TYPE: CurveType = CurveType::ED;
}

impl<P: TEModelParameters> CurveParameters for EDProjective<P> {
    type TEParameters = P;
    // Just a placeholder
    type SWParameters = crate::curves::pallas::PallasParameters;
    const TYPE: CurveType = CurveType::ED;
}

// CrossBoundary - expose methods to set/get native boundary for a type
//
impl<T: NonCanonicalDeserialize + NonCanonicalSerialize + CurveParameters> CrossBoundary for T {}

pub trait CrossBoundary: NonCanonicalDeserialize + NonCanonicalSerialize + CurveParameters {
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

pub trait NativeCallHandler {
    fn handle_call(
        &self,
        id: CallId,
        args: Option<Vec<&[u8]>>,
        cp: Vec<u8>,
    ) -> Result<Option<Vec<Vec<u8>>>, &'static str> {
        // match call id
        match id {
            CallId::VBMul => {
                let vb_args = args.unwrap();
                // match curve type
                match cp[0].try_into().unwrap() {
                    BoundaryCurves::Pallas => Ok(Some(vec![self
                        .handle_vb_multi_scalar_mul::<pallas::Affine>(vb_args[0], vb_args[1])
                        .unwrap()])),
                    BoundaryCurves::MNT4_298G1 => Ok(Some(vec![self
                        .handle_vb_multi_scalar_mul::<mnt4_298::g1::G1Affine>(
                            vb_args[0], vb_args[1],
                        )
                        .unwrap()])),
                    BoundaryCurves::MNT4_298G2 => Ok(Some(vec![self
                        .handle_vb_multi_scalar_mul::<mnt4_298::g2::G2Affine>(
                            vb_args[0], vb_args[1],
                        )
                        .unwrap()])),
                    BoundaryCurves::EdBls12_377 => Ok(Some(vec![self
                        .handle_vb_multi_scalar_mul::<ed_on_bls12_377::EdwardsAffine>(
                            vb_args[0], vb_args[1],
                        )
                        .unwrap()])),
                }
            }
            CallId::ProjBN => {
                let args = args.unwrap()[0];
                // match curve type
                match cp[0].try_into().unwrap() {
                    BoundaryCurves::Pallas => Ok(Some(vec![self
                        .handle_batch_normalization::<pallas::Projective>(args)
                        .unwrap()])),
                    BoundaryCurves::MNT4_298G1 => Ok(Some(vec![self
                        .handle_batch_normalization::<mnt4_298::g1::G1Projective>(args)
                        .unwrap()])),
                    BoundaryCurves::MNT4_298G2 => Ok(Some(vec![self
                        .handle_batch_normalization::<mnt4_298::g2::G2Projective>(args)
                        .unwrap()])),
                    BoundaryCurves::EdBls12_377 => Ok(Some(vec![self
                        .handle_batch_normalization::<ed_on_bls12_377::EdwardsProjective>(args)
                        .unwrap()])),
                }
            }
            _ => panic!(),
        }
    }

    #[must_use]
    fn handle_vb_multi_scalar_mul<G: CrossAffine>(
        &self,
        sbases: &[u8],
        sscalars: &[u8],
    ) -> Result<Vec<u8>, ()>
    where
        G::Projective: CrossProjective;

    #[must_use]
    fn handle_batch_normalization<G: CrossProjective>(&self, v: &[u8]) -> Result<Vec<u8>, ()>
    where
        G::Affine: CrossAffine;
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
    use ark_pallas::Affine;

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
        Affine::set_native_boundary(Some(&NB));
        Affine::get_native_boundary().unwrap();

        Affine::set_native_fallback(true);
        assert_eq!(Affine::get_native_fallback(), true);
    }
}
