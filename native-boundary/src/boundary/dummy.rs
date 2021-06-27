use crate::boundary::{CallId, NativeBoundary, NativeCallHandler, SimpleNativeCallHandler};
use ark_std::vec::Vec;
use core::convert::TryFrom;

// Dummy boundary interface for testing (de)serialization, delegation calls and
// dynamic dispatch of curve types
#[derive(Clone)]
pub struct DummyBoundary;

// dummy native host exported function call
fn dummy_host_export_call(
    id: u8,
    args: Option<Vec<&[u8]>>,
    cp: Vec<u8>,
) -> Result<Option<Vec<Vec<u8>>>, &'static str> {
    SimpleNativeCallHandler.handle_call(CallId::try_from(id).unwrap(), args, cp)
}

impl NativeBoundary for DummyBoundary {
    fn call(
        &self,
        id: CallId,
        args: Option<Vec<&[u8]>>,
        cp: Vec<u8>,
    ) -> Result<Option<Vec<Vec<u8>>>, &'static str> {
        dummy_host_export_call(id.into(), args, cp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::CrossBoundary;
    use crate::wrapped::{G1Projective, G2Projective, GroupProjective};
    use ark_ec::ProjectiveCurve;
    use ark_std::rand::distributions::{Distribution, Standard};
    use ark_std::UniformRand;

    pub fn test_batch_normalization_helper<G: ProjectiveCurve>()
    where
        Standard: Distribution<G>,
    {
        // set DummyBoundary and disable fallback
        GroupProjective::<G>::set_native_boundary(Some(&DummyBoundary));
        GroupProjective::<G>::set_native_fallback(false);
        G::set_native_boundary(Some(&DummyBoundary));
        G::set_native_fallback(false);

        const SAMPLES: usize = 1 << 10;
        let mut rng = ark_std::test_rng();
        let mut g = (0..SAMPLES).map(|_| G::rand(&mut rng)).collect::<Vec<_>>();
        let mut wg = (0..SAMPLES)
            .map(|_| GroupProjective::<G>::rand(&mut rng))
            .collect::<Vec<_>>();
        let g = G::batch_normalization(&mut g);
        let wg = GroupProjective::<G>::batch_normalization(&mut wg);
        assert_eq!(g, wg);
    }

    #[test]
    fn test_batch_normalization() {
        test_batch_normalization_helper::<ark_pallas::Projective>();
        test_batch_normalization_helper::<ark_ed_on_bls12_377::EdwardsProjective>();
        test_batch_normalization_helper::<G1Projective<ark_mnt4_298::MNT4_298>>();
        test_batch_normalization_helper::<G2Projective<ark_mnt4_298::MNT4_298>>();
    }
}
