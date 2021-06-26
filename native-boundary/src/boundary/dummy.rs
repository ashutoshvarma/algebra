use crate::boundary::{CallId, NativeBoundary};
use crate::curves::BoundaryCurves;
use ark_ec::boundary::serialize::NonCanonicalSerialize;
use ark_ec::{msm::VariableBaseMSM, AffineCurve, ProjectiveCurve};
use ark_ff::PrimeField;
use ark_ff::Zero;
use ark_serialize::CanonicalDeserialize;
use ark_std::{convert::TryInto, io::Cursor, vec::Vec};

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
    ) -> Result<Option<Vec<Vec<u8>>>, &'static str> {
        // match call id
        match id {
            CallId::VBMul => {
                let vb_args = args.unwrap();
                // match curve type
                match cp[0].try_into().unwrap() {
                    BoundaryCurves::Pallas => {
                        Ok(Some(vec![DummyBoundary::handle_vb_multi_scalar_mul::<
                            ark_pallas::Affine,
                        >(vb_args[0], vb_args[1])]))
                    }
                    BoundaryCurves::MNT4_298G1 => {
                        Ok(Some(vec![DummyBoundary::handle_vb_multi_scalar_mul::<
                            ark_mnt4_298::g1::G1Affine,
                        >(vb_args[0], vb_args[1])]))
                    }
                    BoundaryCurves::MNT4_298G2 => {
                        Ok(Some(vec![DummyBoundary::handle_vb_multi_scalar_mul::<
                            ark_mnt4_298::g2::G2Affine,
                        >(vb_args[0], vb_args[1])]))
                    }
                    BoundaryCurves::EdBls12_377 => {
                        Ok(Some(vec![DummyBoundary::handle_vb_multi_scalar_mul::<
                            ark_ed_on_bls12_377::EdwardsAffine,
                        >(vb_args[0], vb_args[1])]))
                    }
                }
            }
            CallId::ProjBN => {
                let args = args.unwrap()[0];
                // match curve type
                match cp[0].try_into().unwrap() {
                    BoundaryCurves::Pallas => {
                        Ok(Some(vec![DummyBoundary::handle_batch_normalization::<
                            ark_pallas::Projective,
                        >(args)]))
                    }
                    BoundaryCurves::MNT4_298G1 => {
                        Ok(Some(vec![DummyBoundary::handle_batch_normalization::<
                            ark_mnt4_298::g1::G1Projective,
                        >(args)]))
                    }
                    BoundaryCurves::MNT4_298G2 => {
                        Ok(Some(vec![DummyBoundary::handle_batch_normalization::<
                            ark_mnt4_298::g2::G2Projective,
                        >(args)]))
                    }
                    BoundaryCurves::EdBls12_377 => {
                        Ok(Some(vec![DummyBoundary::handle_batch_normalization::<
                            ark_ed_on_bls12_377::EdwardsProjective,
                        >(args)]))
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

    fn handle_batch_normalization<G: ProjectiveCurve>(v: &[u8]) -> Vec<u8> {
        // TODO: remove this simple hack to get the serialised size
        let size = G::noncanonical_serialized_size(&G::zero());
        let len = v.len() / size;
        let mut buff = Cursor::new(v);

        let mut curves = vec![];
        for _ in 0..len {
            curves.push(G::noncanonical_deserialize_uncompressed_unchecked(&mut buff).unwrap());
        }

        ProjectiveCurve::batch_normalization(&mut curves);

        let mut result = Cursor::new(vec![0; v.len()]);
        for i in curves.iter() {
            i.noncanonical_serialize_uncompressed_unchecked(&mut result)
                .unwrap();
        }

        result.into_inner()
    }
}
