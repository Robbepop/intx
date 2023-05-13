use crate::{
    I104, I112, I120, I128, I16, I24, I32, I40, I48, I56, I64, I72, I80, I88, I96, U104, U112,
    U120, U128, U16, U24, U32, U40, U48, U56, U64, U72, U80, U88, U96,
};

macro_rules! impl_try_from_for {
    ( $( impl TryFrom<$from:ty> for $to:ty $(as $mode:ident)? );* $(;)? ) => {
        $( impl_try_from_for!(@impl TryFrom<$from> for $to $(as $mode)? ); )*
    };
    ( @impl TryFrom<$from:ty> for $to:ty as std ) => {
        // Case to be used when converting between unaligned integers
        // with a standard power-of-two bitwidth. This has superior
        // performance compared to more general cases.
        impl ::core::convert::TryFrom<$from> for $to {
            type Error = $crate::TryFromIntError;

            #[inline]
            fn try_from(value: $from) -> ::core::result::Result<Self, Self::Error> {
                let repr = <<$from as crate::UnalignedInteger>::Repr
                    as ::core::convert::From<$from>>::from(value);
                let lower = <<Self as crate::UnalignedInteger>::Repr
                    as ::core::convert::TryFrom<<$from
                    as crate::UnalignedInteger>::Repr>>::try_from(repr)?;
                let result = <Self as ::core::convert::From<<Self
                    as crate::UnalignedInteger>::Repr>>::from(lower);
                ::core::result::Result::Ok(result)
            }
        }
    };
    ( @impl TryFrom<$from:ty> for $to:ty as primitive ) => {
        // Case to be used when converting from a signed Rust primitive type
        // such as `i16` to an unaligned integer type provided by this crate.
        impl ::core::convert::TryFrom<$from> for $to {
            type Error = $crate::TryFromIntError;

            #[inline]
            fn try_from(value: $from) -> ::core::result::Result<Self, Self::Error> {
                if !<$from as $crate::IsWithinBoundsOf<$to>>::is_within_bounds(value) {
                    return ::core::result::Result::Err($crate::TryFromIntError(()));
                }
                let mut dst = [0x00_u8; ::core::mem::size_of::<Self>()];
                let src = value.to_ne_bytes();
                $crate::utils::truncate_bytes(&mut dst, &src);
                ::core::result::Result::Ok(Self::from_ne_bytes(dst))
            }
        }
    };
    ( @impl TryFrom<$from:ty> for $to:ty as eq_width ) => {
        // Case for converting between signed and unsigned integers
        // of non-standard but equal bitwidths.
        impl ::core::convert::TryFrom<$from> for $to {
            type Error = $crate::TryFromIntError;

            #[inline]
            fn try_from(value: $from) -> ::core::result::Result<Self, Self::Error> {
                let bytes = value.to_ne_bytes();
                if bytes[<$from>::msb_pos()] & 0x80_u8 != 0x00_u8 {
                    return ::core::result::Result::Err($crate::TryFromIntError(()))
                }
                ::core::result::Result::Ok(Self::from_ne_bytes(bytes))
            }
        }
    };
    ( @impl TryFrom<$from:ty> for $to:ty as base ) => {
        impl ::core::convert::TryFrom<$from> for $to {
            type Error = $crate::TryFromIntError;

            #[inline]
            fn try_from(value: $from) -> ::core::result::Result<Self, Self::Error> {
                <Self as ::core::convert::TryFrom<<$from as $crate::UnalignedInteger>::Repr>>::try_from(
                    <<$from as $crate::UnalignedInteger>::Repr as ::core::convert::From<$from>>::from(value)
                )
            }
        }
    };
}
impl_try_from_for! {
    impl TryFrom<I16> for u16 as std;
    impl TryFrom<U24> for u16 as std;
    impl TryFrom<I24> for u16 as std;
    impl TryFrom<U32> for u16 as std;
    impl TryFrom<I32> for u16 as std;
    impl TryFrom<U40> for u16 as std;
    impl TryFrom<I40> for u16 as std;
    impl TryFrom<U48> for u16 as std;
    impl TryFrom<I48> for u16 as std;
    impl TryFrom<U56> for u16 as std;
    impl TryFrom<I56> for u16 as std;
    impl TryFrom<U64> for u16 as std;
    impl TryFrom<I64> for u16 as std;
    impl TryFrom<U72> for u16 as std;
    impl TryFrom<I72> for u16 as std;
    impl TryFrom<U80> for u16 as std;
    impl TryFrom<I80> for u16 as std;
    impl TryFrom<U88> for u16 as std;
    impl TryFrom<I88> for u16 as std;
    impl TryFrom<U96> for u16 as std;
    impl TryFrom<I96> for u16 as std;
    impl TryFrom<U104> for u16 as std;
    impl TryFrom<I104> for u16 as std;
    impl TryFrom<U112> for u16 as std;
    impl TryFrom<I112> for u16 as std;
    impl TryFrom<U120> for u16 as std;
    impl TryFrom<I120> for u16 as std;
    impl TryFrom<U128> for u16 as std;
    impl TryFrom<I128> for u16 as std;

    impl TryFrom<i8> for U16 as std;
    impl TryFrom<i16> for U16 as std;
    impl TryFrom<I16> for U16 as std;
    impl TryFrom<U24> for U16 as std;
    impl TryFrom<I24> for U16 as std;
    impl TryFrom<u32> for U16 as std;
    impl TryFrom<i32> for U16 as std;
    impl TryFrom<U32> for U16 as std;
    impl TryFrom<I32> for U16 as std;
    impl TryFrom<U40> for U16 as std;
    impl TryFrom<I40> for U16 as std;
    impl TryFrom<U48> for U16 as std;
    impl TryFrom<I48> for U16 as std;
    impl TryFrom<U56> for U16 as std;
    impl TryFrom<I56> for U16 as std;
    impl TryFrom<u64> for U16 as std;
    impl TryFrom<i64> for U16 as std;
    impl TryFrom<U64> for U16 as std;
    impl TryFrom<I64> for U16 as std;
    impl TryFrom<U72> for U16 as std;
    impl TryFrom<I72> for U16 as std;
    impl TryFrom<U80> for U16 as std;
    impl TryFrom<I80> for U16 as std;
    impl TryFrom<U88> for U16 as std;
    impl TryFrom<I88> for U16 as std;
    impl TryFrom<U96> for U16 as std;
    impl TryFrom<I96> for U16 as std;
    impl TryFrom<U104> for U16 as std;
    impl TryFrom<I104> for U16 as std;
    impl TryFrom<U112> for U16 as std;
    impl TryFrom<I112> for U16 as std;
    impl TryFrom<U120> for U16 as std;
    impl TryFrom<I120> for U16 as std;
    impl TryFrom<u128> for U16 as std;
    impl TryFrom<i128> for U16 as std;
    impl TryFrom<U128> for U16 as std;
    impl TryFrom<I128> for U16 as std;

    impl TryFrom<U16> for i16 as std;
    impl TryFrom<U24> for i16 as std;
    impl TryFrom<I24> for i16 as std;
    impl TryFrom<U32> for i16 as std;
    impl TryFrom<I32> for i16 as std;
    impl TryFrom<U40> for i16 as std;
    impl TryFrom<I40> for i16 as std;
    impl TryFrom<U48> for i16 as std;
    impl TryFrom<I48> for i16 as std;
    impl TryFrom<U56> for i16 as std;
    impl TryFrom<I56> for i16 as std;
    impl TryFrom<U64> for i16 as std;
    impl TryFrom<I64> for i16 as std;
    impl TryFrom<U72> for i16 as std;
    impl TryFrom<I72> for i16 as std;
    impl TryFrom<U80> for i16 as std;
    impl TryFrom<I80> for i16 as std;
    impl TryFrom<U88> for i16 as std;
    impl TryFrom<I88> for i16 as std;
    impl TryFrom<U96> for i16 as std;
    impl TryFrom<I96> for i16 as std;
    impl TryFrom<U104> for i16 as std;
    impl TryFrom<I104> for i16 as std;
    impl TryFrom<U112> for i16 as std;
    impl TryFrom<I112> for i16 as std;
    impl TryFrom<U120> for i16 as std;
    impl TryFrom<I120> for i16 as std;
    impl TryFrom<U128> for i16 as std;
    impl TryFrom<I128> for i16 as std;

    impl TryFrom<u16> for I16 as std;
    impl TryFrom<U16> for I16 as std;
    impl TryFrom<U24> for I16 as std;
    impl TryFrom<I24> for I16 as std;
    impl TryFrom<u32> for I16 as std;
    impl TryFrom<i32> for I16 as std;
    impl TryFrom<U32> for I16 as std;
    impl TryFrom<I32> for I16 as std;
    impl TryFrom<U40> for I16 as std;
    impl TryFrom<I40> for I16 as std;
    impl TryFrom<U48> for I16 as std;
    impl TryFrom<I48> for I16 as std;
    impl TryFrom<U56> for I16 as std;
    impl TryFrom<I56> for I16 as std;
    impl TryFrom<u64> for I16 as std;
    impl TryFrom<i64> for I16 as std;
    impl TryFrom<U64> for I16 as std;
    impl TryFrom<I64> for I16 as std;
    impl TryFrom<U72> for I16 as std;
    impl TryFrom<I72> for I16 as std;
    impl TryFrom<U80> for I16 as std;
    impl TryFrom<I80> for I16 as std;
    impl TryFrom<U88> for I16 as std;
    impl TryFrom<I88> for I16 as std;
    impl TryFrom<U96> for I16 as std;
    impl TryFrom<I96> for I16 as std;
    impl TryFrom<U104> for I16 as std;
    impl TryFrom<I104> for I16 as std;
    impl TryFrom<U112> for I16 as std;
    impl TryFrom<I112> for I16 as std;
    impl TryFrom<U120> for I16 as std;
    impl TryFrom<I120> for I16 as std;
    impl TryFrom<u128> for I16 as std;
    impl TryFrom<i128> for I16 as std;
    impl TryFrom<U128> for I16 as std;
    impl TryFrom<I128> for I16 as std;

    impl TryFrom<I24> for U24 as eq_width;
    impl TryFrom<u32> for U24 as primitive;
    impl TryFrom<i32> for U24 as primitive;
    impl TryFrom<U32> for U24 as base;
    impl TryFrom<u64> for U24 as primitive;
    impl TryFrom<i64> for U24 as primitive;
    impl TryFrom<u128> for U24 as primitive;
    impl TryFrom<i128> for U24 as primitive;

    impl TryFrom<U24> for I24 as eq_width;
    impl TryFrom<u32> for I24 as primitive;
    impl TryFrom<i32> for I24 as primitive;
    impl TryFrom<I32> for I24 as base;
    impl TryFrom<I40> for I24 as base;
    impl TryFrom<I48> for I24 as base;
    impl TryFrom<I56> for I24 as base;
    impl TryFrom<u64> for I24 as primitive;
    impl TryFrom<i64> for I24 as primitive;
    impl TryFrom<I64> for I24 as base;
    impl TryFrom<I72> for I24 as base;
    impl TryFrom<I80> for I24 as base;
    impl TryFrom<I88> for I24 as base;
    impl TryFrom<I96> for I24 as base;
    impl TryFrom<I104> for I24 as base;
    impl TryFrom<I112> for I24 as base;
    impl TryFrom<I120> for I24 as base;
    impl TryFrom<u128> for I24 as primitive;
    impl TryFrom<i128> for I24 as primitive;
    impl TryFrom<I128> for I24 as base;

    impl TryFrom<I16> for u32 as std;
    impl TryFrom<I24> for u32 as std;
    impl TryFrom<I32> for u32 as std;
    impl TryFrom<U40> for u32 as std;
    impl TryFrom<I40> for u32 as std;
    impl TryFrom<U48> for u32 as std;
    impl TryFrom<I48> for u32 as std;
    impl TryFrom<U56> for u32 as std;
    impl TryFrom<I56> for u32 as std;
    impl TryFrom<U64> for u32 as std;
    impl TryFrom<I64> for u32 as std;
    impl TryFrom<U72> for u32 as std;
    impl TryFrom<I72> for u32 as std;
    impl TryFrom<U80> for u32 as std;
    impl TryFrom<I80> for u32 as std;
    impl TryFrom<U88> for u32 as std;
    impl TryFrom<I88> for u32 as std;
    impl TryFrom<U96> for u32 as std;
    impl TryFrom<I96> for u32 as std;
    impl TryFrom<U104> for u32 as std;
    impl TryFrom<I104> for u32 as std;
    impl TryFrom<U112> for u32 as std;
    impl TryFrom<I112> for u32 as std;
    impl TryFrom<U120> for u32 as std;
    impl TryFrom<I120> for u32 as std;
    impl TryFrom<U128> for u32 as std;
    impl TryFrom<I128> for u32 as std;

    impl TryFrom<i8> for U32 as std;
    impl TryFrom<i16> for U32 as std;
    impl TryFrom<I16> for U32 as std;
    impl TryFrom<I24> for U32 as std;
    impl TryFrom<i32> for U32 as std;
    impl TryFrom<I32> for U32 as std;
    impl TryFrom<U40> for U32 as std;
    impl TryFrom<I40> for U32 as std;
    impl TryFrom<U48> for U32 as std;
    impl TryFrom<I48> for U32 as std;
    impl TryFrom<U56> for U32 as std;
    impl TryFrom<I56> for U32 as std;
    impl TryFrom<u64> for U32 as std;
    impl TryFrom<i64> for U32 as std;
    impl TryFrom<U64> for U32 as std;
    impl TryFrom<I64> for U32 as std;
    impl TryFrom<U72> for U32 as std;
    impl TryFrom<I72> for U32 as std;
    impl TryFrom<U80> for U32 as std;
    impl TryFrom<I80> for U32 as std;
    impl TryFrom<U88> for U32 as std;
    impl TryFrom<I88> for U32 as std;
    impl TryFrom<U96> for U32 as std;
    impl TryFrom<I96> for U32 as std;
    impl TryFrom<U104> for U32 as std;
    impl TryFrom<I104> for U32 as std;
    impl TryFrom<U112> for U32 as std;
    impl TryFrom<I112> for U32 as std;
    impl TryFrom<U120> for U32 as std;
    impl TryFrom<I120> for U32 as std;
    impl TryFrom<u128> for U32 as std;
    impl TryFrom<i128> for U32 as std;
    impl TryFrom<U128> for U32 as std;
    impl TryFrom<I128> for U32 as std;

    impl TryFrom<U32> for i32 as std;
    impl TryFrom<U40> for i32 as std;
    impl TryFrom<I40> for i32 as std;
    impl TryFrom<U48> for i32 as std;
    impl TryFrom<I48> for i32 as std;
    impl TryFrom<U56> for i32 as std;
    impl TryFrom<I56> for i32 as std;
    impl TryFrom<U64> for i32 as std;
    impl TryFrom<I64> for i32 as std;
    impl TryFrom<U72> for i32 as std;
    impl TryFrom<I72> for i32 as std;
    impl TryFrom<U80> for i32 as std;
    impl TryFrom<I80> for i32 as std;
    impl TryFrom<U88> for i32 as std;
    impl TryFrom<I88> for i32 as std;
    impl TryFrom<U96> for i32 as std;
    impl TryFrom<I96> for i32 as std;
    impl TryFrom<U104> for i32 as std;
    impl TryFrom<I104> for i32 as std;
    impl TryFrom<U112> for i32 as std;
    impl TryFrom<I112> for i32 as std;
    impl TryFrom<U120> for i32 as std;
    impl TryFrom<I120> for i32 as std;
    impl TryFrom<U128> for i32 as std;
    impl TryFrom<I128> for i32 as std;

    impl TryFrom<u32> for I32 as std;
    impl TryFrom<U32> for I32 as std;
    impl TryFrom<U40> for I32 as std;
    impl TryFrom<I40> for I32 as std;
    impl TryFrom<U48> for I32 as std;
    impl TryFrom<I48> for I32 as std;
    impl TryFrom<U56> for I32 as std;
    impl TryFrom<I56> for I32 as std;
    impl TryFrom<u64> for I32 as std;
    impl TryFrom<i64> for I32 as std;
    impl TryFrom<U64> for I32 as std;
    impl TryFrom<I64> for I32 as std;
    impl TryFrom<U72> for I32 as std;
    impl TryFrom<I72> for I32 as std;
    impl TryFrom<U80> for I32 as std;
    impl TryFrom<I80> for I32 as std;
    impl TryFrom<U88> for I32 as std;
    impl TryFrom<I88> for I32 as std;
    impl TryFrom<U96> for I32 as std;
    impl TryFrom<I96> for I32 as std;
    impl TryFrom<U104> for I32 as std;
    impl TryFrom<I104> for I32 as std;
    impl TryFrom<U112> for I32 as std;
    impl TryFrom<I112> for I32 as std;
    impl TryFrom<U120> for I32 as std;
    impl TryFrom<I120> for I32 as std;
    impl TryFrom<u128> for I32 as std;
    impl TryFrom<i128> for I32 as std;
    impl TryFrom<U128> for I32 as std;
    impl TryFrom<I128> for I32 as std;

    impl TryFrom<I40> for U40 as eq_width;
    impl TryFrom<u64> for U40 as primitive;
    impl TryFrom<i64> for U40 as primitive;
    impl TryFrom<u128> for U40 as primitive;
    impl TryFrom<i128> for U40 as primitive;

    impl TryFrom<U40> for I40 as eq_width;
    impl TryFrom<I48> for I40 as base;
    impl TryFrom<I56> for I40 as base;
    impl TryFrom<u64> for I40 as primitive;
    impl TryFrom<i64> for I40 as primitive;
    impl TryFrom<I64> for I40 as base;
    impl TryFrom<I72> for I40 as base;
    impl TryFrom<I80> for I40 as base;
    impl TryFrom<I88> for I40 as base;
    impl TryFrom<I96> for I40 as base;
    impl TryFrom<I104> for I40 as base;
    impl TryFrom<I112> for I40 as base;
    impl TryFrom<I120> for I40 as base;
    impl TryFrom<u128> for I40 as primitive;
    impl TryFrom<i128> for I40 as primitive;
    impl TryFrom<I128> for I40 as base;

    impl TryFrom<I48> for U48 as eq_width;
    impl TryFrom<u64> for U48 as primitive;
    impl TryFrom<i64> for U48 as primitive;
    impl TryFrom<u128> for U48 as primitive;
    impl TryFrom<i128> for U48 as primitive;

    impl TryFrom<U48> for I48 as eq_width;
    impl TryFrom<I56> for I48 as base;
    impl TryFrom<u64> for I48 as primitive;
    impl TryFrom<i64> for I48 as primitive;
    impl TryFrom<I64> for I48 as base;
    impl TryFrom<I72> for I48 as base;
    impl TryFrom<I80> for I48 as base;
    impl TryFrom<I88> for I48 as base;
    impl TryFrom<I96> for I48 as base;
    impl TryFrom<I104> for I48 as base;
    impl TryFrom<I112> for I48 as base;
    impl TryFrom<I120> for I48 as base;
    impl TryFrom<u128> for I48 as primitive;
    impl TryFrom<i128> for I48 as primitive;
    impl TryFrom<I128> for I48 as base;

    impl TryFrom<I56> for U56 as eq_width;
    impl TryFrom<u64> for U56 as primitive;
    impl TryFrom<i64> for U56 as primitive;
    impl TryFrom<u128> for U56 as primitive;
    impl TryFrom<i128> for U56 as primitive;

    impl TryFrom<U56> for I56 as eq_width;
    impl TryFrom<u64> for I56 as primitive;
    impl TryFrom<i64> for I56 as primitive;
    impl TryFrom<I64> for I56 as base;
    impl TryFrom<I72> for I56 as base;
    impl TryFrom<I80> for I56 as base;
    impl TryFrom<I88> for I56 as base;
    impl TryFrom<I96> for I56 as base;
    impl TryFrom<I104> for I56 as base;
    impl TryFrom<I112> for I56 as base;
    impl TryFrom<I120> for I56 as base;
    impl TryFrom<u128> for I56 as primitive;
    impl TryFrom<i128> for I56 as primitive;
    impl TryFrom<I128> for I56 as base;

    impl TryFrom<I16> for u64 as std;
    impl TryFrom<I24> for u64 as std;
    impl TryFrom<I32> for u64 as std;
    impl TryFrom<I40> for u64 as std;
    impl TryFrom<I48> for u64 as std;
    impl TryFrom<I56> for u64 as std;
    impl TryFrom<I64> for u64 as std;
    impl TryFrom<U72> for u64 as std;
    impl TryFrom<I72> for u64 as std;
    impl TryFrom<U80> for u64 as std;
    impl TryFrom<I80> for u64 as std;
    impl TryFrom<U88> for u64 as std;
    impl TryFrom<I88> for u64 as std;
    impl TryFrom<U96> for u64 as std;
    impl TryFrom<I96> for u64 as std;
    impl TryFrom<U104> for u64 as std;
    impl TryFrom<I104> for u64 as std;
    impl TryFrom<U112> for u64 as std;
    impl TryFrom<I112> for u64 as std;
    impl TryFrom<U120> for u64 as std;
    impl TryFrom<I120> for u64 as std;
    impl TryFrom<U128> for u64 as std;
    impl TryFrom<I128> for u64 as std;

    impl TryFrom<i8> for U64 as std;
    impl TryFrom<i16> for U64 as std;
    impl TryFrom<I16> for U64 as std;
    impl TryFrom<I24> for U64 as std;
    impl TryFrom<i32> for U64 as std;
    impl TryFrom<I32> for U64 as std;
    impl TryFrom<I40> for U64 as std;
    impl TryFrom<I48> for U64 as std;
    impl TryFrom<I56> for U64 as std;
    impl TryFrom<i64> for U64 as std;
    impl TryFrom<I64> for U64 as std;
    impl TryFrom<U72> for U64 as std;
    impl TryFrom<I72> for U64 as std;
    impl TryFrom<U80> for U64 as std;
    impl TryFrom<I80> for U64 as std;
    impl TryFrom<U88> for U64 as std;
    impl TryFrom<I88> for U64 as std;
    impl TryFrom<U96> for U64 as std;
    impl TryFrom<I96> for U64 as std;
    impl TryFrom<U104> for U64 as std;
    impl TryFrom<I104> for U64 as std;
    impl TryFrom<U112> for U64 as std;
    impl TryFrom<I112> for U64 as std;
    impl TryFrom<U120> for U64 as std;
    impl TryFrom<I120> for U64 as std;
    impl TryFrom<u128> for U64 as std;
    impl TryFrom<i128> for U64 as std;
    impl TryFrom<U128> for U64 as std;
    impl TryFrom<I128> for U64 as std;

    impl TryFrom<U64> for i64 as std;
    impl TryFrom<U72> for i64 as std;
    impl TryFrom<I72> for i64 as std;
    impl TryFrom<U80> for i64 as std;
    impl TryFrom<I80> for i64 as std;
    impl TryFrom<U88> for i64 as std;
    impl TryFrom<I88> for i64 as std;
    impl TryFrom<U96> for i64 as std;
    impl TryFrom<I96> for i64 as std;
    impl TryFrom<U104> for i64 as std;
    impl TryFrom<I104> for i64 as std;
    impl TryFrom<U112> for i64 as std;
    impl TryFrom<I112> for i64 as std;
    impl TryFrom<U120> for i64 as std;
    impl TryFrom<I120> for i64 as std;
    impl TryFrom<U128> for i64 as std;
    impl TryFrom<I128> for i64 as std;

    impl TryFrom<u64> for I64 as std;
    impl TryFrom<U64> for I64 as std;
    impl TryFrom<U72> for I64 as std;
    impl TryFrom<I72> for I64 as std;
    impl TryFrom<U80> for I64 as std;
    impl TryFrom<I80> for I64 as std;
    impl TryFrom<U88> for I64 as std;
    impl TryFrom<I88> for I64 as std;
    impl TryFrom<U96> for I64 as std;
    impl TryFrom<I96> for I64 as std;
    impl TryFrom<U104> for I64 as std;
    impl TryFrom<I104> for I64 as std;
    impl TryFrom<U112> for I64 as std;
    impl TryFrom<I112> for I64 as std;
    impl TryFrom<U120> for I64 as std;
    impl TryFrom<I120> for I64 as std;
    impl TryFrom<u128> for I64 as std;
    impl TryFrom<i128> for I64 as std;
    impl TryFrom<U128> for I64 as std;
    impl TryFrom<I128> for I64 as std;

    impl TryFrom<I72> for U72 as eq_width;
    impl TryFrom<u128> for U72 as primitive;
    impl TryFrom<i128> for U72 as primitive;

    impl TryFrom<U72> for I72 as eq_width;
    impl TryFrom<I80> for I72 as base;
    impl TryFrom<I88> for I72 as base;
    impl TryFrom<I96> for I72 as base;
    impl TryFrom<I104> for I72 as base;
    impl TryFrom<I112> for I72 as base;
    impl TryFrom<I120> for I72 as base;
    impl TryFrom<u128> for I72 as primitive;
    impl TryFrom<i128> for I72 as primitive;
    impl TryFrom<I128> for I72 as base;

    impl TryFrom<I80> for U80 as eq_width;
    impl TryFrom<u128> for U80 as primitive;
    impl TryFrom<i128> for U80 as primitive;

    impl TryFrom<U80> for I80 as eq_width;
    impl TryFrom<I88> for I80 as base;
    impl TryFrom<I96> for I80 as base;
    impl TryFrom<I104> for I80 as base;
    impl TryFrom<I112> for I80 as base;
    impl TryFrom<I120> for I80 as base;
    impl TryFrom<u128> for I80 as primitive;
    impl TryFrom<i128> for I80 as primitive;
    impl TryFrom<I128> for I80 as base;

    impl TryFrom<I88> for U88 as eq_width;
    impl TryFrom<u128> for U88 as primitive;
    impl TryFrom<i128> for U88 as primitive;

    impl TryFrom<U88> for I88 as eq_width;
    impl TryFrom<I96> for I88 as base;
    impl TryFrom<I104> for I88 as base;
    impl TryFrom<I112> for I88 as base;
    impl TryFrom<I120> for I88 as base;
    impl TryFrom<u128> for I88 as primitive;
    impl TryFrom<i128> for I88 as primitive;
    impl TryFrom<I128> for I88 as base;

    impl TryFrom<I96> for U96 as eq_width;
    impl TryFrom<u128> for U96 as primitive;
    impl TryFrom<i128> for U96 as primitive;

    impl TryFrom<U96> for I96 as eq_width;
    impl TryFrom<I104> for I96 as base;
    impl TryFrom<I112> for I96 as base;
    impl TryFrom<I120> for I96 as base;
    impl TryFrom<u128> for I96 as primitive;
    impl TryFrom<i128> for I96 as primitive;
    impl TryFrom<I128> for I96 as base;

    impl TryFrom<I104> for U104 as eq_width;
    impl TryFrom<u128> for U104 as primitive;
    impl TryFrom<i128> for U104 as primitive;

    impl TryFrom<U104> for I104 as eq_width;
    impl TryFrom<I112> for I104 as base;
    impl TryFrom<I120> for I104 as base;
    impl TryFrom<u128> for I104 as primitive;
    impl TryFrom<i128> for I104 as primitive;
    impl TryFrom<I128> for I104 as base;

    impl TryFrom<I112> for U112 as eq_width;
    impl TryFrom<u128> for U112 as primitive;
    impl TryFrom<i128> for U112 as primitive;

    impl TryFrom<U112> for I112 as eq_width;
    impl TryFrom<I120> for I112 as base;
    impl TryFrom<u128> for I112 as primitive;
    impl TryFrom<i128> for I112 as primitive;
    impl TryFrom<I128> for I112 as base;

    impl TryFrom<I120> for U120 as eq_width;
    impl TryFrom<u128> for U120 as primitive;
    impl TryFrom<i128> for U120 as primitive;

    impl TryFrom<U120> for I120 as eq_width;
    impl TryFrom<u128> for I120 as primitive;
    impl TryFrom<i128> for I120 as primitive;
    impl TryFrom<I128> for I120 as base;

    impl TryFrom<I16> for u128 as std;
    impl TryFrom<I24> for u128 as std;
    impl TryFrom<I32> for u128 as std;
    impl TryFrom<I40> for u128 as std;
    impl TryFrom<I48> for u128 as std;
    impl TryFrom<I56> for u128 as std;
    impl TryFrom<I64> for u128 as std;
    impl TryFrom<I72> for u128 as std;
    impl TryFrom<I80> for u128 as std;
    impl TryFrom<I88> for u128 as std;
    impl TryFrom<I96> for u128 as std;
    impl TryFrom<I104> for u128 as std;
    impl TryFrom<I112> for u128 as std;
    impl TryFrom<I120> for u128 as std;
    impl TryFrom<I128> for u128 as std;

    impl TryFrom<i8> for U128 as std;
    impl TryFrom<i16> for U128 as std;
    impl TryFrom<I16> for U128 as std;
    impl TryFrom<I24> for U128 as std;
    impl TryFrom<i32> for U128 as std;
    impl TryFrom<I32> for U128 as std;
    impl TryFrom<I40> for U128 as std;
    impl TryFrom<I48> for U128 as std;
    impl TryFrom<I56> for U128 as std;
    impl TryFrom<i64> for U128 as std;
    impl TryFrom<I64> for U128 as std;
    impl TryFrom<I72> for U128 as std;
    impl TryFrom<I80> for U128 as std;
    impl TryFrom<I88> for U128 as std;
    impl TryFrom<I96> for U128 as std;
    impl TryFrom<I104> for U128 as std;
    impl TryFrom<I112> for U128 as std;
    impl TryFrom<I120> for U128 as std;
    impl TryFrom<i128> for U128 as std;
    impl TryFrom<I128> for U128 as std;

    impl TryFrom<U128> for i128 as std;

    impl TryFrom<u128> for I128 as std;
    impl TryFrom<U128> for I128 as std;
}

#[test]
fn test_conv() {
    assert_eq!(<U24>::try_from(I24::default()), Ok(<U24>::default()));
    assert_eq!(<I24>::try_from(U24::default()), Ok(<I24>::default()));
    assert!(<U24>::try_from(I24::try_from(-1i32).unwrap()).is_err());
    assert!(<U24>::try_from(I24::MIN).is_err());
    assert!(<U24>::try_from(I24::MAX).is_ok());
    assert_eq!(<I24>::try_from(U24::MIN), Ok(I24::default()));
    assert!(<I24>::try_from(U24::MAX).is_err());
}
