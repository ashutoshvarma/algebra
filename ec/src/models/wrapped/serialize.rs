use crate::AffineCurve;
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


// impls for AffineCurve

impl<C: AffineCurve> NonCanonicalDeserialize for C {
    fn noncanonical_deserialize_uncompressed_unchecked<R: Read>(
        reader: R,
    ) -> Result<Self, SerializationError> {
        C::deserialize_uncompressed(reader)
    }
}

impl<C: AffineCurve> NonCanonicalSerialize for C {
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
