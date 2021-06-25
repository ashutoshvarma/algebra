use ark_ec::{models::mnt4::MNT4Parameters, CurveParameters};
use ark_ed_on_bls12_377::EdwardsParameters as EdBls12_377_Parameters;
use ark_mnt4_298::Parameters as MNT4_298_Parameters;
use ark_pallas::PallasParameters;
use ark_std::any::TypeId;

macro_rules! try_from_u8 {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl ark_std::convert::TryFrom<u8> for $name {
            type Error = ();

            fn try_from(v: u8) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as u8 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

try_from_u8! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum BoundaryCurves {
        // ark_pallas
        Pallas = 1,
        // ark_ed_on_bls12_377
        EdBls12_377 = 2,
        // ark_mnt4_298
        MNT4_298G1 = 3,
        MNT4_298G2 = 4,
    }
}

impl BoundaryCurves {
    pub fn try_from_wrapped<T: CurveParameters>() -> Result<Self, ()> {
        let id = TypeId::of::<T::Parameters>();
        // Pallas
        if id == TypeId::of::<PallasParameters>() {
            Ok(BoundaryCurves::Pallas)
        // EdBls12_377
        } else if id == TypeId::of::<EdBls12_377_Parameters>() {
            Ok(BoundaryCurves::EdBls12_377)
        // MNT4_298
        } else if id == TypeId::of::<<MNT4_298_Parameters as MNT4Parameters>::G1Parameters>() {
            Ok(BoundaryCurves::MNT4_298G1)
        } else if id == TypeId::of::<<MNT4_298_Parameters as MNT4Parameters>::G2Parameters>() {
            Ok(BoundaryCurves::MNT4_298G2)
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use ark_mnt4_298::{G1Affine as M298_G1Affine, G2Projective as M298_G2Projective};
    use ark_pallas::{Affine as PAffine, Projective as PProjective};
    use ark_std::convert::TryInto;

    fn assert_u8_try_from(p: BoundaryCurves) {
        let back = (p.clone() as u8).try_into().unwrap();
        assert_eq!(p, back);
    }
    #[test]
    fn test_from_u8() {
        assert_u8_try_from(BoundaryCurves::Pallas);
        assert_u8_try_from(BoundaryCurves::EdBls12_377);
        assert_u8_try_from(BoundaryCurves::MNT4_298G1);
        assert_u8_try_from(BoundaryCurves::MNT4_298G2);
    }

    fn assert_from_wrapped<C: CurveParameters>(p: BoundaryCurves) {
        let param = BoundaryCurves::try_from_wrapped::<C>().unwrap();
        assert_eq!(param, p);
    }

    #[test]
    fn test_from_wrapped() {
        assert_from_wrapped::<PAffine>(BoundaryCurves::Pallas);
        assert_from_wrapped::<PProjective>(BoundaryCurves::Pallas);
        // mnt4_298
        assert_from_wrapped::<M298_G1Affine>(BoundaryCurves::MNT4_298G1);
        assert_from_wrapped::<M298_G2Projective>(BoundaryCurves::MNT4_298G2);
    }
}
