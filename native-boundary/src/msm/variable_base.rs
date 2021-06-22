use crate::{
    boundary::{CallId, CrossBoundary},
    curves::CurveParameters,
    wrapped::{
        serialize::{NonCanonicalDeserialize, NonCanonicalSerialize},
        WrappedCurve,
    },
};
use ark_ff::prelude::*;
use ark_serialize::CanonicalSerialize;
use ark_std::{io::Cursor, vec::Vec};

use ark_ec::AffineCurve;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub struct VariableBaseMSM;

impl VariableBaseMSM {
    // pub fn multi_scalar_mul<G: AffineCurve>(
    //     bases: &[G],
    //     scalars: &[<G::ScalarField as PrimeField>::BigInt],
    // ) -> G::Projective {
    pub fn multi_scalar_mul<G: AffineCurve + WrappedCurve + CrossBoundary>(
        bases: &[G],
        scalars: &[<G::ScalarField as PrimeField>::BigInt],
    ) -> G::Projective
    where
        G: NonCanonicalDeserialize + NonCanonicalSerialize,
        G::Projective: NonCanonicalDeserialize,
    {
        match G::get_boundary() {
            Some(nb) => {
                let cp = CurveParameters::try_from_wrapped::<G>().unwrap();

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
            // TODO: Handle None better
            None => panic!(),
        }
    }
}
