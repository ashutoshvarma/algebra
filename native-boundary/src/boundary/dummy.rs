use crate::curves::BoundaryCurves;
use ark_ec::{msm::VariableBaseMSM, AffineCurve};
use ark_ff::PrimeField;
use ark_ff::Zero;
use ark_serialize::CanonicalDeserialize;
use ark_std::{io::Cursor, vec::Vec};
use std::convert::TryInto;

use ark_ec::models::wrapped::{
    boundary::{CallId, NativeBoundary},
    serialize::NonCanonicalSerialize,
    GroupAffine,
};

// Dummy boundary interface for testing (de)serialization, delegation calls and
// dynamic dispatch of curve types
#[derive(Clone)]
pub struct DummyBoundary;

impl NativeBoundary for DummyBoundary {
    fn call(
        &self,
        id: CallId,
        args: Option<Vec<&[u8]>>,
        cp: Vec<u8>,
        wrapped: bool,
    ) -> Result<Option<Vec<Vec<u8>>>, &'static str> {
        // match call id
        match id {
            CallId::VBMul => {
                let vb_args = args.unwrap();
                // match curve type
                match cp[0].try_into().unwrap() {
                    BoundaryCurves::Pallas => {
                        if wrapped {
                            Ok(Some(vec![DummyBoundary::handle_vb_multi_scalar_mul::<
                                GroupAffine<ark_pallas::Affine>,
                            >(
                                vb_args[0], vb_args[1]
                            )]))
                        } else {
                            Ok(Some(vec![DummyBoundary::handle_vb_multi_scalar_mul::<
                                ark_pallas::Affine,
                            >(
                                vb_args[0], vb_args[1]
                            )]))
                        }
                    }
                    BoundaryCurves::MNT4_298G1 => {
                        if wrapped {
                            Ok(Some(vec![DummyBoundary::handle_vb_multi_scalar_mul::<
                                GroupAffine<ark_mnt4_298::g1::G1Affine>,
                            >(
                                vb_args[0], vb_args[1]
                            )]))
                        } else {
                            Ok(Some(vec![DummyBoundary::handle_vb_multi_scalar_mul::<
                                ark_mnt4_298::g1::G1Affine,
                            >(
                                vb_args[0], vb_args[1]
                            )]))
                        }
                    }
                    BoundaryCurves::MNT4_298G2 => {
                        if wrapped {
                            Ok(Some(vec![DummyBoundary::handle_vb_multi_scalar_mul::<
                                GroupAffine<ark_mnt4_298::g2::G2Affine>,
                            >(
                                vb_args[0], vb_args[1]
                            )]))
                        } else {
                            Ok(Some(vec![DummyBoundary::handle_vb_multi_scalar_mul::<
                                ark_mnt4_298::g2::G2Affine,
                            >(
                                vb_args[0], vb_args[1]
                            )]))
                        }
                    }
                    BoundaryCurves::EdBls12_377 => {
                        if wrapped {
                            Ok(Some(vec![DummyBoundary::handle_vb_multi_scalar_mul::<
                                GroupAffine<ark_ed_on_bls12_377::EdwardsAffine>,
                            >(
                                vb_args[0], vb_args[1]
                            )]))
                        } else {
                            Ok(Some(vec![DummyBoundary::handle_vb_multi_scalar_mul::<
                                ark_ed_on_bls12_377::EdwardsAffine,
                            >(
                                vb_args[0], vb_args[1]
                            )]))
                        }
                    }
                }
            }
            _ => panic!(),
        }
    }
}

impl DummyBoundary {
    fn handle_vb_multi_scalar_mul<G: AffineCurve>(sbases: &[u8], sscalars: &[u8]) -> Vec<u8> {
        // TODO: remove this simple hack to get the serialised size
        let len = sbases.len() / G::noncanonical_serialized_size(&G::zero());
        let mut bases_buff = Cursor::new(sbases);
        let mut scalar_buff = Cursor::new(sscalars);

        let mut bases = vec![];
        let mut scalar = vec![];

        for _ in 0..len {
            bases
                .push(G::noncanonical_deserialize_uncompressed_unchecked(&mut bases_buff).unwrap());
            scalar.push(
                <G::ScalarField as PrimeField>::BigInt::deserialize_uncompressed(&mut scalar_buff)
                    .unwrap(),
            );
        }

        let mut result = Cursor::new(vec![
            0;
            G::Projective::noncanonical_serialized_size(
                &G::Projective::zero()
            )
        ]);

        VariableBaseMSM::multi_scalar_mul(&bases, &scalar)
            .noncanonical_serialize_uncompressed_unchecked(&mut result)
            .unwrap();

        result.into_inner()
    }
}
