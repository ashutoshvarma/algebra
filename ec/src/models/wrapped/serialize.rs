use crate::models::short_weierstrass_jacobian::GroupProjective as GroupProjectiveSW;
use crate::models::twisted_edwards_extended::GroupProjective as GroupProjectiveED;
use crate::models::wrapped::Wrapped;
use crate::{models::TEModelParameters, ProjectiveCurve, SWModelParameters};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};

use crate::wrapped::twisted_edwards_extended::GroupProjective;

pub use ark_std::io::{Read, Write};

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
        self.wrapped().x.serialized_size()
            + self.wrapped().y.serialized_size()
            + self.wrapped().t.serialized_size()
            + self.wrapped().z.serialized_size()
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
        self.wrapped().x.serialized_size()
            + self.wrapped().y.serialized_size()
            + self.wrapped().z.serialized_size()
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
