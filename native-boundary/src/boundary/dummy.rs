use crate::{
    wrapped::serialize::{NonCanonicalDeserialize, NonCanonicalSerialize},
    {curves::CurveParameters, GroupAffine},
};
use ark_ec::{msm::VariableBaseMSM, AffineCurve};
use ark_ff::PrimeField;
use ark_ff::Zero;
use ark_serialize::CanonicalDeserialize;
use ark_std::{io::Cursor, vec::Vec};
use std::convert::TryInto;

use crate::boundary::{CallId, NativeBoundary};

#[derive(Clone)]
pub struct DummyBoundary;

impl NativeBoundary for DummyBoundary {
    fn call(
        &self,
        id: CallId,
        args: Option<Vec<&[u8]>>,
        cp: Vec<u8>,
    ) -> Result<Option<Vec<Vec<u8>>>, &'static str> {
        match id {
            CallId::VBMul => {
                let vb_args = args.unwrap();
                match cp[0].try_into().unwrap() {
                    CurveParameters::Pallas => {
                        Ok(Some(vec![DummyBoundary::pass_vb_multi_scalar_mul::<
                            GroupAffine<ark_pallas::Affine>,
                        >(vb_args[0], vb_args[1])]))
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
}

impl DummyBoundary {
    fn pass_vb_multi_scalar_mul<G: AffineCurve + NonCanonicalDeserialize + NonCanonicalSerialize>(
        sbases: &[u8],
        sscalars: &[u8],
    ) -> Vec<u8>
    where
        G::Projective: NonCanonicalSerialize,
    {
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
