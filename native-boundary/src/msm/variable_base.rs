use crate::boundary::{CallId, CrossBoundary};
use crate::curves::BoundaryCurves;
use ark_ec::boundary::serialize::NonCanonicalDeserialize;
use ark_ff::prelude::*;
use ark_serialize::CanonicalSerialize;
use ark_std::{io::Cursor, vec::Vec};

use ark_ec::{msm, AffineCurve};

pub struct VariableBaseMSM;

impl VariableBaseMSM {
    pub fn multi_scalar_mul<G: AffineCurve>(
        bases: &[G],
        scalars: &[<G::ScalarField as PrimeField>::BigInt],
    ) -> G::Projective {
        match G::get_native_boundary() {
            Some(nb) => {
                let cp = BoundaryCurves::try_from::<G>().unwrap();

                let size = ark_std::cmp::min(bases.len(), scalars.len());
                let scalars = &scalars[..size];
                let bases = &bases[..size];
                let scalars_and_bases_iter = scalars.iter().zip(bases);

                // create temp buffers
                let buff1: Vec<u8> = vec![0; bases.len() * bases[0].noncanonical_serialized_size()];
                let buff2: Vec<u8> = vec![0; bases.len() * scalars[0].uncompressed_size()];

                // fill buffers with serialised args
                let mut bases_buff = Cursor::new(buff1);
                let mut scalars_buff = Cursor::new(buff2);
                for (s, b) in scalars_and_bases_iter {
                    b.noncanonical_serialize_uncompressed_unchecked(&mut bases_buff)
                        .unwrap();
                    s.serialize_uncompressed(&mut scalars_buff).unwrap();
                }

                let bases_buff = bases_buff.into_inner();
                let scalars_buff = scalars_buff.into_inner();
                // call boundary
                let result = nb
                    .call(
                        CallId::VBMul,
                        Some(vec![&bases_buff, &scalars_buff]),
                        vec![cp as u8],
                    )
                    .unwrap()
                    .unwrap();

                // deserialise and return
                let raw = Cursor::new(&result[0]);
                G::Projective::noncanonical_deserialize_uncompressed_unchecked(raw).unwrap()
            }
            // If no native boundary is set for `G`, check for fallback
            None => {
                if G::get_native_fallback() {
                    msm::VariableBaseMSM::multi_scalar_mul(bases, scalars)
                } else {
                    panic!("No native boundary set for given type!")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::CrossBoundary;
    use crate::boundary::DummyBoundary;
    use crate::wrapped::{G1Affine, G2Affine, GroupAffine};
    use ark_ec::msm;
    use ark_ec::ProjectiveCurve;

    pub fn test_var_base_msm<G: AffineCurve>() {
        const SAMPLES: usize = 1 << 10;
        let mut rng = ark_std::test_rng();
        let v = (0..SAMPLES - 1)
            .map(|_| G::ScalarField::rand(&mut rng).into_repr())
            .collect::<Vec<_>>();
        let g = (0..SAMPLES)
            .map(|_| G::Projective::rand(&mut rng))
            .collect::<Vec<_>>();
        let g = <G::Projective as ProjectiveCurve>::batch_normalization_into_affine(&g);

        // set DummyBoundary and disable fallback
        G::set_native_boundary(Some(&DummyBoundary));
        G::set_native_fallback(false);

        let wasm_call = msm::VariableBaseMSM::multi_scalar_mul(g.as_slice(), v.as_slice());
        let native_call = VariableBaseMSM::multi_scalar_mul(g.as_slice(), v.as_slice());

        assert_eq!(native_call.into_affine(), wasm_call.into_affine());
    }

    #[test]
    fn test_msm_vb() {
        // non-wrapped
        test_var_base_msm::<ark_pallas::Affine>();

        // non-pairing curves
        test_var_base_msm::<GroupAffine<ark_pallas::Affine>>();
        test_var_base_msm::<GroupAffine<ark_ed_on_bls12_377::EdwardsAffine>>();
        // pairing curves
        test_var_base_msm::<G1Affine<ark_mnt4_298::MNT4_298>>();
        test_var_base_msm::<G2Affine<ark_mnt4_298::MNT4_298>>();
    }
}
