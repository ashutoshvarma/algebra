#![macro_use]

macro_rules! impl_additive_ops_from_wrapped_cross_projective {
    ($type: ident) => {
        #[allow(unused_qualifications)]
        impl<P: CrossProjective> core::ops::Add<Self> for $type<P>
        where
            P::Affine: CrossAffine,
        {
            type Output = Self;

            #[inline]
            fn add(self, other: Self) -> Self {
                let mut result = self;
                result.add_assign(&other);
                result
            }
        }

        #[allow(unused_qualifications)]
        impl<'a, P: CrossProjective> core::ops::Add<&'a mut Self> for $type<P>
        where
            P::Affine: CrossAffine,
        {
            type Output = Self;

            #[inline]
            fn add(self, other: &'a mut Self) -> Self {
                let mut result = self;
                result.add_assign(&*other);
                result
            }
        }

        #[allow(unused_qualifications)]
        impl<P: CrossProjective> core::ops::Sub<Self> for $type<P>
        where
            P::Affine: CrossAffine,
        {
            type Output = Self;

            #[inline]
            fn sub(self, other: Self) -> Self {
                let mut result = self;
                result.sub_assign(&other);
                result
            }
        }

        #[allow(unused_qualifications)]
        impl<'a, P: CrossProjective> core::ops::Sub<&'a mut Self> for $type<P>
        where
            P::Affine: CrossAffine,
        {
            type Output = Self;

            #[inline]
            fn sub(self, other: &'a mut Self) -> Self {
                let mut result = self;
                result.sub_assign(&*other);
                result
            }
        }

        #[allow(unused_qualifications)]
        impl<P: CrossProjective> core::iter::Sum<Self> for $type<P>
        where
            P::Affine: CrossAffine,
        {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::zero(), core::ops::Add::add)
            }
        }

        #[allow(unused_qualifications)]
        impl<'a, P: CrossProjective> core::iter::Sum<&'a Self> for $type<P>
        where
            P::Affine: CrossAffine,
        {
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::zero(), core::ops::Add::add)
            }
        }

        #[allow(unused_qualifications)]
        impl<P: CrossProjective> core::ops::AddAssign<Self> for $type<P>
        where
            P::Affine: CrossAffine,
        {
            fn add_assign(&mut self, other: Self) {
                self.add_assign(&other)
            }
        }

        #[allow(unused_qualifications)]
        impl<P: CrossProjective> core::ops::SubAssign<Self> for $type<P>
        where
            P::Affine: CrossAffine,
        {
            fn sub_assign(&mut self, other: Self) {
                self.sub_assign(&other)
            }
        }

        #[allow(unused_qualifications)]
        impl<'a, P: CrossProjective> core::ops::AddAssign<&'a mut Self> for $type<P>
        where
            P::Affine: CrossAffine,
        {
            fn add_assign(&mut self, other: &'a mut Self) {
                self.add_assign(&*other)
            }
        }

        #[allow(unused_qualifications)]
        impl<'a, P: CrossProjective> core::ops::SubAssign<&'a mut Self> for $type<P>
        where
            P::Affine: CrossAffine,
        {
            fn sub_assign(&mut self, other: &'a mut Self) {
                self.sub_assign(&*other)
            }
        }
    };
}
