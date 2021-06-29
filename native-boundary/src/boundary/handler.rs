use crate::{
    boundary::{CrossAffine, CrossProjective, NativeCallHandler},
    serialize::NonCanonicalSerialize,
};
use ark_ec::{msm::VariableBaseMSM, ProjectiveCurve};
use ark_ff::{PrimeField, Zero};
use ark_serialize::CanonicalDeserialize;
use ark_std::{io::Cursor, vec::Vec};

pub struct SimpleNativeCallHandler;
impl NativeCallHandler for SimpleNativeCallHandler {
    fn handle_vb_multi_scalar_mul<G: CrossAffine>(
        &self,
        sbases: &[u8],
        sscalars: &[u8],
    ) -> Result<Vec<u8>, ()>
    where
        G::Projective: CrossProjective,
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

        Ok(result.into_inner())
    }

    fn handle_batch_normalization<G: CrossProjective>(&self, v: &[u8]) -> Result<Vec<u8>, ()>
    where
        G::Affine: CrossAffine,
    {
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

        Ok(result.into_inner())
    }
}
