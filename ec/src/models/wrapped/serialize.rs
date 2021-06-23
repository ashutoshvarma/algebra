use ark_serialize::SerializationError;
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

#[cfg(test)]
pub mod tests {
    use crate::ProjectiveCurve;
    use ark_std::{io::Cursor, test_rng};

    pub const ITERATIONS: usize = 10;
    pub fn test_serialize_projective<G: ProjectiveCurve>() {
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
