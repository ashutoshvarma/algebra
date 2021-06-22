use crate::wrapped::Wrapped;
use ark_ec::models::short_weierstrass_jacobian::GroupProjective as GroupProjectiveSW;
use ark_ec::models::twisted_edwards_extended::GroupProjective as GroupProjectiveED;
use ark_ec::{models::TEModelParameters, AffineCurve, ProjectiveCurve, SWModelParameters};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};

use crate::wrapped::GroupProjective;

pub use ark_std::{
    boxed::Box,
    io::{Read, Write},
};

pub trait NonCanonicalSerialize {
    fn noncanonical_serialize_uncompressed_unchecked<W: Write>(
        &self,
        writer: W,
    ) -> Result<(), SerializationError>;
    fn noncanonical_serialized_size(&self) -> usize;
}

pub trait NonCanonicalDeserialize {
    fn noncanonical_deserialize_uncompressed_unchecked<R: Read>(
        reader: R,
    ) -> Result<Self, SerializationError>
    where
        Self: Sized;
}

impl<P: TEModelParameters> NonCanonicalSerialize for GroupProjective<GroupProjectiveED<P>> {
    fn noncanonical_serialize_uncompressed_unchecked<W: Write>(
        &self,
        mut writer: W,
    ) -> Result<(), SerializationError> {
        self.wrapped().x.serialize_uncompressed(&mut writer)?;
        self.wrapped().y.serialize_uncompressed(&mut writer)?;
        self.wrapped().t.serialize_uncompressed(&mut writer)?;
        self.wrapped().z.serialize_uncompressed(&mut writer)?;
        Ok(())
    }

    fn noncanonical_serialized_size(&self) -> usize {
        self.wrapped().x.uncompressed_size()
            + self.wrapped().y.uncompressed_size()
            + self.wrapped().t.uncompressed_size()
            + self.wrapped().z.uncompressed_size()
    }
}

impl<P: SWModelParameters> NonCanonicalSerialize for GroupProjective<GroupProjectiveSW<P>> {
    fn noncanonical_serialize_uncompressed_unchecked<W: Write>(
        &self,
        mut writer: W,
    ) -> Result<(), SerializationError> {
        self.wrapped().x.serialize_uncompressed(&mut writer)?;
        self.wrapped().y.serialize_uncompressed(&mut writer)?;
        self.wrapped().z.serialize_uncompressed(&mut writer)?;
        Ok(())
    }

    fn noncanonical_serialized_size(&self) -> usize {
        self.wrapped().x.uncompressed_size()
            + self.wrapped().y.uncompressed_size()
            + self.wrapped().z.uncompressed_size()
    }
}

impl<P: TEModelParameters> NonCanonicalDeserialize for GroupProjective<GroupProjectiveED<P>> {
    fn noncanonical_deserialize_uncompressed_unchecked<R: Read>(
        mut reader: R,
    ) -> Result<Self, SerializationError> {
        let x: P::BaseField = CanonicalDeserialize::deserialize(&mut reader)?;
        let y: P::BaseField = CanonicalDeserialize::deserialize(&mut reader)?;
        let t: P::BaseField = CanonicalDeserialize::deserialize(&mut reader)?;
        let z: P::BaseField = CanonicalDeserialize::deserialize(&mut reader)?;

        let p = GroupProjective(GroupProjectiveED::<P>::new(x, y, t, z));
        Ok(p)
    }
}

impl<P: SWModelParameters> NonCanonicalDeserialize for GroupProjective<GroupProjectiveSW<P>> {
    fn noncanonical_deserialize_uncompressed_unchecked<R: Read>(
        mut reader: R,
    ) -> Result<Self, SerializationError> {
        let x: P::BaseField = CanonicalDeserialize::deserialize(&mut reader)?;
        let y: P::BaseField = CanonicalDeserialize::deserialize(&mut reader)?;
        let z: P::BaseField = CanonicalDeserialize::deserialize(&mut reader)?;

        let p = GroupProjective(GroupProjectiveSW::<P>::new(x, y, z));
        Ok(p)
    }
}

//
// GroupAffine
//

// impl<P: TEModelParameters> NonCanonicalSerialize for GroupAffine<GroupAffineED<P>> {
//     fn noncanonical_serialize_uncompressed_unchecked<W: Write>(
//         &self,
//         mut writer: W,
//     ) -> Result<(), SerializationError> {
//         self.wrapped().serialize_uncompressed(writer)
//     }

//     fn noncanonical_serialized_size(&self) -> usize {
//         self.wrapped().uncompressed_size()
//     }
// }
// impl<P: SWModelParameters> NonCanonicalSerialize for GroupAffine<GroupAffineSW<P>> {
//     fn noncanonical_serialize_uncompressed_unchecked<W: Write>(
//         &self,
//         mut writer: W,
//     ) -> Result<(), SerializationError> {
//         self.wrapped().serialize_uncompressed(writer)
//     }

//     fn noncanonical_serialized_size(&self) -> usize {
//         self.wrapped().uncompressed_size()
//     }
// }

// impl<P: TEModelParameters> NonCanonicalDeserialize for GroupAffine<GroupAffineED<P>> {
//     fn noncanonical_deserialize_uncompressed_unchecked<R: Read>(
//         mut reader: R,
//     ) -> Result<Self, SerializationError> {
//         <Self as Wrapped>::WrapTarget::deserialize_uncompressed(reader).map(|v| Self(v))
//     }
// }

// impl<P: SWModelParameters> NonCanonicalDeserialize for GroupAffine<GroupAffineSW<P>> {
//     fn noncanonical_deserialize_uncompressed_unchecked<R: Read>(
//         mut reader: R,
//     ) -> Result<Self, SerializationError> {
//         <Self as Wrapped>::WrapTarget::deserialize_uncompressed(reader).map(|v| Self(v))
//     }
// }

// impl<C: AffineCurve> NonCanonicalDeserialize for C {
//     fn noncanonical_deserialize_uncompressed_unchecked<R: Read>(
//         reader: R,
//     ) -> Result<Self, SerializationError> {
//         C::deserialize_uncompressed(reader)
//     }
// }

// impl<C: AffineCurve> NonCanonicalSerialize for C {
//     fn noncanonical_serialize_uncompressed_unchecked<W: Write>(
//         &self,
//         writer: W,
//     ) -> Result<(), SerializationError> {
//         self.serialize_uncompressed(writer)
//     }

//     fn noncanonical_serialized_size(&self) -> usize {
//         self.uncompressed_size()
//     }
// }

#[cfg(test)]
pub mod tests {
    use super::*;
    use ark_std::{io::Cursor, test_rng};

    pub const ITERATIONS: usize = 10;
   
    pub fn test_serialize_projective<
        G: ProjectiveCurve + NonCanonicalDeserialize + NonCanonicalSerialize,
    >() {
        let mut rng = test_rng();
        let a = G::rand(&mut rng);
        for _ in 0..ITERATIONS {
            {
                let mut serialized = vec![0; a.noncanonical_serialized_size()];
                let mut cursor = Cursor::new(&mut serialized[..]);
                a.noncanonical_serialize_uncompressed_unchecked(&mut cursor)
                    .unwrap();
                let mut cursor = Cursor::new(&serialized[..]);
                let b = G::noncanonical_deserialize_uncompressed_unchecked(&mut cursor).unwrap();
                assert_eq!(a, b);
            }
            {
                let a = G::zero();
                let mut serialized = vec![0; a.noncanonical_serialized_size()];
                let mut cursor = Cursor::new(&mut serialized[..]);
                a.noncanonical_serialize_uncompressed_unchecked(&mut cursor)
                    .unwrap();
                let mut cursor = Cursor::new(&serialized[..]);
                let b = G::noncanonical_deserialize_uncompressed_unchecked(&mut cursor).unwrap();
                assert_eq!(a, b);
            }
        }
    }
}
