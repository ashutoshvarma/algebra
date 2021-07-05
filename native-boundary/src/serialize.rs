use crate::ark_ec::models::{
    short_weierstrass_jacobian::{GroupAffine as SWAffine, GroupProjective as SWProjective},
    twisted_edwards_extended::{GroupAffine as EDAffine, GroupProjective as EDProjective},
    SWModelParameters, TEModelParameters,
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use ark_std::io::{Read, Write};

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

// Immplementations for SW & ED curves
//

// SW Curves
impl<P: SWModelParameters> NonCanonicalSerialize for SWProjective<P> {
    fn noncanonical_serialize_uncompressed_unchecked<W: Write>(
        &self,
        mut writer: W,
    ) -> Result<(), SerializationError> {
        self.x.serialize_uncompressed(&mut writer)?;
        self.y.serialize_uncompressed(&mut writer)?;
        self.z.serialize_uncompressed(&mut writer)?;
        Ok(())
    }

    fn noncanonical_serialized_size(&self) -> usize {
        self.x.uncompressed_size() + self.y.uncompressed_size() + self.z.uncompressed_size()
    }
}

impl<P: SWModelParameters> NonCanonicalDeserialize for SWProjective<P> {
    fn noncanonical_deserialize_uncompressed_unchecked<R: Read>(
        mut reader: R,
    ) -> Result<Self, SerializationError> {
        let x: P::BaseField = CanonicalDeserialize::deserialize_unchecked(&mut reader)?;
        let y: P::BaseField = CanonicalDeserialize::deserialize_unchecked(&mut reader)?;
        let z: P::BaseField = CanonicalDeserialize::deserialize_unchecked(&mut reader)?;

        let p = Self::new(x, y, z);
        Ok(p)
    }
}

impl<P: SWModelParameters> NonCanonicalSerialize for SWAffine<P> {
    fn noncanonical_serialize_uncompressed_unchecked<W: Write>(
        &self,
        writer: W,
    ) -> Result<(), SerializationError> {
        self.serialize_uncompressed(writer)
    }

    fn noncanonical_serialized_size(&self) -> usize {
        self.uncompressed_size()
    }
}

impl<P: SWModelParameters> NonCanonicalDeserialize for SWAffine<P> {
    fn noncanonical_deserialize_uncompressed_unchecked<R: Read>(
        reader: R,
    ) -> Result<Self, SerializationError> {
        Self::deserialize_uncompressed(reader)
    }
}

// ED Curves
impl<P: TEModelParameters> NonCanonicalSerialize for EDProjective<P> {
    fn noncanonical_serialize_uncompressed_unchecked<W: Write>(
        &self,
        mut writer: W,
    ) -> Result<(), SerializationError> {
        self.x.serialize_uncompressed(&mut writer)?;
        self.y.serialize_uncompressed(&mut writer)?;
        self.t.serialize_uncompressed(&mut writer)?;
        self.z.serialize_uncompressed(&mut writer)?;
        Ok(())
    }

    fn noncanonical_serialized_size(&self) -> usize {
        self.x.uncompressed_size()
            + self.y.uncompressed_size()
            + self.t.uncompressed_size()
            + self.z.uncompressed_size()
    }
}

impl<P: TEModelParameters> NonCanonicalDeserialize for EDProjective<P> {
    fn noncanonical_deserialize_uncompressed_unchecked<R: Read>(
        mut reader: R,
    ) -> Result<Self, SerializationError> {
        let x: P::BaseField = CanonicalDeserialize::deserialize_unchecked(&mut reader)?;
        let y: P::BaseField = CanonicalDeserialize::deserialize_unchecked(&mut reader)?;
        let t: P::BaseField = CanonicalDeserialize::deserialize_unchecked(&mut reader)?;
        let z: P::BaseField = CanonicalDeserialize::deserialize_unchecked(&mut reader)?;

        let p = Self::new(x, y, t, z);
        Ok(p)
    }
}

impl<P: TEModelParameters> NonCanonicalSerialize for EDAffine<P> {
    fn noncanonical_serialize_uncompressed_unchecked<W: Write>(
        &self,
        writer: W,
    ) -> Result<(), SerializationError> {
        self.serialize_uncompressed(writer)
    }

    fn noncanonical_serialized_size(&self) -> usize {
        self.uncompressed_size()
    }
}

impl<P: TEModelParameters> NonCanonicalDeserialize for EDAffine<P> {
    fn noncanonical_deserialize_uncompressed_unchecked<R: Read>(
        reader: R,
    ) -> Result<Self, SerializationError> {
        Self::deserialize_uncompressed(reader)
    }
}

pub mod tests {
    use super::*;
    use crate::ark_ec::ProjectiveCurve;
    use ark_std::{io::Cursor, test_rng};

    pub const ITERATIONS: usize = 10;
    pub fn test_serialize_projective<
        G: ProjectiveCurve + NonCanonicalSerialize + NonCanonicalDeserialize,
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
