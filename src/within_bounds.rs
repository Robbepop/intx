pub use crate::{
    I104, I112, I120, I128, I16, I24, I32, I40, I48, I56, I64, I72, I80, I88, I96, U104, U112,
    U120, U128, U16, U24, U32, U40, U48, U56, U64, U72, U80, U88, U96,
};

/// Convenience trait implemented by primitive integers to streamline
/// `TryFrom` implementations for non-standard bitwidth integers like `I24`
/// or `U48`.
pub(crate) trait IsWithinBoundsOf<T> {
    /// Returns `true` if the value of `self` is within bounds for type `T`.
    #[allow(clippy::wrong_self_convention)] // Note: we only implement this for Rust built-in integer types.
    fn is_within_bounds(self) -> bool;
}

macro_rules! impl_is_within_bounds_of_for {
    ( $( impl IsWithinBoundsOf<$of:ty> for $for:ty as $mode:ident );* $(;)? ) => {
        $(
            impl_is_within_bounds_of_for!(
                @impl IsWithinBoundsOf<$of> for $for as $mode
            );
        )*
    };
    ( @impl IsWithinBoundsOf<$of:ty> for $for:ty as u2u ) => {
        impl $crate::within_bounds::IsWithinBoundsOf<$of> for $for {
            #[inline]
            fn is_within_bounds(self) -> ::core::primitive::bool {
                self < (1 << <$of>::BITS)
            }
        }
    };
    ( @impl IsWithinBoundsOf<$of:ty> for $for:ty as i2u ) => {
        impl $crate::within_bounds::IsWithinBoundsOf<$of> for $for {
            #[inline]
            fn is_within_bounds(self) -> ::core::primitive::bool {
                self < (1 << (<$of>::BITS - 1))
            }
        }
    };
    ( @impl IsWithinBoundsOf<$of:ty> for $for:ty as u2i ) => {
        impl $crate::within_bounds::IsWithinBoundsOf<$of> for $for {
            #[inline]
            fn is_within_bounds(self) -> ::core::primitive::bool {
                !self.is_negative() && self < (1 << <$of>::BITS)
            }
        }
    };
    ( @impl IsWithinBoundsOf<$of:ty> for $for:ty as isNonNegative ) => {
        // Case: Needed when converting from a smaller signed integer
        // primitive to a larger unsigned unaligned integer.
        //
        // These conversions are valid as long as the source value
        // is non-negative.
        //
        // # Example
        //
        // - `impl TryFrom<i8> for U24`
        impl $crate::within_bounds::IsWithinBoundsOf<$of> for $for {
            #[inline]
            fn is_within_bounds(self) -> ::core::primitive::bool {
                !self.is_negative()
            }
        }
    };
    ( @impl IsWithinBoundsOf<$of:ty> for $for:ty as i2i ) => {
        impl $crate::within_bounds::IsWithinBoundsOf<$of> for $for {
            #[inline]
            fn is_within_bounds(self) -> ::core::primitive::bool {
                let upper_bound = 1 << (<$of>::BITS - 1);
                let lower_bound = -upper_bound;
                lower_bound <= self && self < upper_bound
            }
        }
    };
}
impl_is_within_bounds_of_for! {
    impl IsWithinBoundsOf<U16> for i8 as isNonNegative;
    impl IsWithinBoundsOf<U24> for i8 as isNonNegative;
    impl IsWithinBoundsOf<U32> for i8 as isNonNegative;
    impl IsWithinBoundsOf<U40> for i8 as isNonNegative;
    impl IsWithinBoundsOf<U48> for i8 as isNonNegative;
    impl IsWithinBoundsOf<U56> for i8 as isNonNegative;
    impl IsWithinBoundsOf<U64> for i8 as isNonNegative;
    impl IsWithinBoundsOf<U72> for i8 as isNonNegative;
    impl IsWithinBoundsOf<U80> for i8 as isNonNegative;
    impl IsWithinBoundsOf<U88> for i8 as isNonNegative;
    impl IsWithinBoundsOf<U96> for i8 as isNonNegative;
    impl IsWithinBoundsOf<U104> for i8 as isNonNegative;
    impl IsWithinBoundsOf<U112> for i8 as isNonNegative;
    impl IsWithinBoundsOf<U120> for i8 as isNonNegative;
    impl IsWithinBoundsOf<U128> for i8 as isNonNegative;

    impl IsWithinBoundsOf<U16> for i16 as isNonNegative;
    impl IsWithinBoundsOf<U24> for i16 as isNonNegative;
    impl IsWithinBoundsOf<U32> for i16 as isNonNegative;
    impl IsWithinBoundsOf<U40> for i16 as isNonNegative;
    impl IsWithinBoundsOf<U48> for i16 as isNonNegative;
    impl IsWithinBoundsOf<U56> for i16 as isNonNegative;
    impl IsWithinBoundsOf<U64> for i16 as isNonNegative;
    impl IsWithinBoundsOf<U72> for i16 as isNonNegative;
    impl IsWithinBoundsOf<U80> for i16 as isNonNegative;
    impl IsWithinBoundsOf<U88> for i16 as isNonNegative;
    impl IsWithinBoundsOf<U96> for i16 as isNonNegative;
    impl IsWithinBoundsOf<U104> for i16 as isNonNegative;
    impl IsWithinBoundsOf<U112> for i16 as isNonNegative;
    impl IsWithinBoundsOf<U120> for i16 as isNonNegative;
    impl IsWithinBoundsOf<U128> for i16 as isNonNegative;

    impl IsWithinBoundsOf<U24> for u32 as u2u;
    impl IsWithinBoundsOf<I24> for u32 as i2u;

    impl IsWithinBoundsOf<U16> for i32 as u2i;
    impl IsWithinBoundsOf<U24> for i32 as u2i;
    impl IsWithinBoundsOf<I24> for i32 as i2i;
    impl IsWithinBoundsOf<U32> for i32 as isNonNegative;
    impl IsWithinBoundsOf<U40> for i32 as isNonNegative;
    impl IsWithinBoundsOf<U48> for i32 as isNonNegative;
    impl IsWithinBoundsOf<U56> for i32 as isNonNegative;
    impl IsWithinBoundsOf<U64> for i32 as isNonNegative;
    impl IsWithinBoundsOf<U72> for i32 as isNonNegative;
    impl IsWithinBoundsOf<U80> for i32 as isNonNegative;
    impl IsWithinBoundsOf<U88> for i32 as isNonNegative;
    impl IsWithinBoundsOf<U96> for i32 as isNonNegative;
    impl IsWithinBoundsOf<U104> for i32 as isNonNegative;
    impl IsWithinBoundsOf<U112> for i32 as isNonNegative;
    impl IsWithinBoundsOf<U120> for i32 as isNonNegative;
    impl IsWithinBoundsOf<U128> for i32 as isNonNegative;

    impl IsWithinBoundsOf<U24> for u64 as u2u;
    impl IsWithinBoundsOf<I24> for u64 as i2u;
    impl IsWithinBoundsOf<U40> for u64 as u2u;
    impl IsWithinBoundsOf<I40> for u64 as i2u;
    impl IsWithinBoundsOf<U48> for u64 as u2u;
    impl IsWithinBoundsOf<I48> for u64 as i2u;
    impl IsWithinBoundsOf<U56> for u64 as u2u;
    impl IsWithinBoundsOf<I56> for u64 as i2u;

    impl IsWithinBoundsOf<U16> for i64 as u2i;
    impl IsWithinBoundsOf<U24> for i64 as u2i;
    impl IsWithinBoundsOf<I24> for i64 as i2i;
    impl IsWithinBoundsOf<U32> for i64 as u2i;
    impl IsWithinBoundsOf<U40> for i64 as u2i;
    impl IsWithinBoundsOf<I40> for i64 as i2i;
    impl IsWithinBoundsOf<U48> for i64 as u2i;
    impl IsWithinBoundsOf<I48> for i64 as i2i;
    impl IsWithinBoundsOf<U56> for i64 as u2i;
    impl IsWithinBoundsOf<I56> for i64 as i2i;
    impl IsWithinBoundsOf<U64> for i64 as isNonNegative;
    impl IsWithinBoundsOf<U72> for i64 as isNonNegative;
    impl IsWithinBoundsOf<U80> for i64 as isNonNegative;
    impl IsWithinBoundsOf<U88> for i64 as isNonNegative;
    impl IsWithinBoundsOf<U96> for i64 as isNonNegative;
    impl IsWithinBoundsOf<U104> for i64 as isNonNegative;
    impl IsWithinBoundsOf<U112> for i64 as isNonNegative;
    impl IsWithinBoundsOf<U120> for i64 as isNonNegative;
    impl IsWithinBoundsOf<U128> for i64 as isNonNegative;

    impl IsWithinBoundsOf<U24> for u128 as u2u;
    impl IsWithinBoundsOf<I24> for u128 as i2u;
    impl IsWithinBoundsOf<U40> for u128 as u2u;
    impl IsWithinBoundsOf<I40> for u128 as i2u;
    impl IsWithinBoundsOf<U48> for u128 as u2u;
    impl IsWithinBoundsOf<I48> for u128 as i2u;
    impl IsWithinBoundsOf<U56> for u128 as u2u;
    impl IsWithinBoundsOf<I56> for u128 as i2u;
    impl IsWithinBoundsOf<U72> for u128 as u2u;
    impl IsWithinBoundsOf<I72> for u128 as i2u;
    impl IsWithinBoundsOf<U80> for u128 as u2u;
    impl IsWithinBoundsOf<I80> for u128 as i2u;
    impl IsWithinBoundsOf<U88> for u128 as u2u;
    impl IsWithinBoundsOf<I88> for u128 as i2u;
    impl IsWithinBoundsOf<U96> for u128 as u2u;
    impl IsWithinBoundsOf<I96> for u128 as i2u;
    impl IsWithinBoundsOf<U104> for u128 as u2u;
    impl IsWithinBoundsOf<I104> for u128 as i2u;
    impl IsWithinBoundsOf<U112> for u128 as u2u;
    impl IsWithinBoundsOf<I112> for u128 as i2u;
    impl IsWithinBoundsOf<U120> for u128 as u2u;
    impl IsWithinBoundsOf<I120> for u128 as i2u;

    impl IsWithinBoundsOf<U24> for i128 as u2i;
    impl IsWithinBoundsOf<I24> for i128 as i2i;
    impl IsWithinBoundsOf<U40> for i128 as u2i;
    impl IsWithinBoundsOf<I40> for i128 as i2i;
    impl IsWithinBoundsOf<U48> for i128 as u2i;
    impl IsWithinBoundsOf<I48> for i128 as i2i;
    impl IsWithinBoundsOf<U56> for i128 as u2i;
    impl IsWithinBoundsOf<I56> for i128 as i2i;
    impl IsWithinBoundsOf<U72> for i128 as u2i;
    impl IsWithinBoundsOf<I72> for i128 as i2i;
    impl IsWithinBoundsOf<U80> for i128 as u2i;
    impl IsWithinBoundsOf<I80> for i128 as i2i;
    impl IsWithinBoundsOf<U88> for i128 as u2i;
    impl IsWithinBoundsOf<I88> for i128 as i2i;
    impl IsWithinBoundsOf<U96> for i128 as u2i;
    impl IsWithinBoundsOf<I96> for i128 as i2i;
    impl IsWithinBoundsOf<U104> for i128 as u2i;
    impl IsWithinBoundsOf<I104> for i128 as i2i;
    impl IsWithinBoundsOf<U112> for i128 as u2i;
    impl IsWithinBoundsOf<I112> for i128 as i2i;
    impl IsWithinBoundsOf<U120> for i128 as u2i;
    impl IsWithinBoundsOf<U128> for i128 as isNonNegative;
    impl IsWithinBoundsOf<I120> for i128 as i2i;
}
