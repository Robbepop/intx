//! Module to implement all `From` implementations for
//! all unaligned integer types provided by this crate.

pub use crate::{
    I104, I112, I120, I128, I16, I24, I32, I40, I48, I56, I64, I72, I80, I88, I96, U104, U112,
    U120, U128, U16, U24, U32, U40, U48, U56, U64, U72, U80, U88, U96,
};

macro_rules! impl_for {
    ( $( impl From<$from:ty> for $to:ty $(as $modus:ident)? );* $(;)? ) => {
        $( impl_for!(@impl From<$from> for $to $(as $modus)? ); )*
    };
    ( @impl From<$from:ty> for $to:ty as std ) => {
        impl ::core::convert::From<$from> for $to {
            #[inline]
            fn from(value: $from) -> Self {
                <Self as ::core::convert::From<<Self as $crate::UnalignedInteger>::UpperPrimitive>>::from(
                    <<Self as $crate::UnalignedInteger>::UpperPrimitive as ::core::convert::From<$from>>::from(value)
                )
            }
        }
    };
    ( @impl From<$from:ty> for $to:ty as primitive ) => {
        impl ::core::convert::From<$from> for $to {
            #[inline]
            fn from(value: $from) -> Self {
                <Self as ::core::convert::From<<$from as $crate::UnalignedInteger>::UpperPrimitive>>::from(
                    <<$from as $crate::UnalignedInteger>::UpperPrimitive as ::core::convert::From<$from>>::from(value)
                )
            }
        }
    };
    ( @impl From<$from:ty> for $to:ty ) => {
        impl ::core::convert::From<$from> for $to {
            #[inline]
            fn from(value: $from) -> Self {
                let mut result = [
                    <$from as $crate::UnalignedInteger>::sign_ext_byte(value);
                    ::core::mem::size_of::<$to>()
                ];
                $crate::utils::extend_bytes(&mut result, &value.to_ne_bytes());
                Self::from_ne_bytes(result)
            }
        }
    };
}
// The following macro call contains lots of `From` impl declarations.
//
// This sheer mass of declarations are ordered from lowest to largest
// bitwidth of the implemented type.
// Unsigned integer types with appear before signed integer types and
// Rust primitive integer types appear before integer types introduced
// by this crate.
// Within the same type the implementations of `From<$from>` are ordered
// given the `$from` type using the same ordering semantics.
//
// # Example
//
// `u32` < `i32` < `U32` < `I32`
impl_for!(
    impl From<u8> for U16 as std;

    impl From<u8> for I16 as std;
    impl From<i8> for I16 as std;

    impl From<u8> for U24;
    impl From<u16> for U24;
    impl From<U16> for U24;

    impl From<u8> for I24;
    impl From<i8> for I24;
    impl From<u16> for I24;
    impl From<U16> for I24;
    impl From<i16> for I24;
    impl From<I16> for I24;

    impl From<U16> for u32 as primitive;
    impl From<U24> for u32;

    impl From<u8> for U32 as std;
    impl From<u16> for U32 as std;
    impl From<U16> for U32 as std;
    impl From<U24> for U32;

    impl From<u8> for I32 as std;
    impl From<i8> for I32 as std;
    impl From<u16> for I32 as std;
    impl From<i16> for I32 as std;
    impl From<U16> for I32 as std;
    impl From<I16> for I32 as std;
    impl From<U24> for I32;

    impl From<U16> for i32 as primitive;
    impl From<I16> for i32 as primitive;
    impl From<I24> for i32;

    impl From<u8> for U40;
    impl From<u16> for U40;
    impl From<U16> for U40;
    impl From<U24> for U40;
    impl From<u32> for U40;
    impl From<U32> for U40;

    impl From<u8> for I40;
    impl From<i8> for I40;
    impl From<u16> for I40;
    impl From<U16> for I40;
    impl From<i16> for I40;
    impl From<I16> for I40;
    impl From<U24> for I40;
    impl From<u32> for I40;
    impl From<U32> for I40;
    impl From<i32> for I40;
    impl From<I32> for I40;

    impl From<u8> for U48;
    impl From<u16> for U48;
    impl From<U16> for U48;
    impl From<U24> for U48;
    impl From<u32> for U48;
    impl From<U32> for U48;
    impl From<U40> for U48;

    impl From<u8> for I48;
    impl From<i8> for I48;
    impl From<u16> for I48;
    impl From<U16> for I48;
    impl From<i16> for I48;
    impl From<I16> for I48;
    impl From<U24> for I48;
    impl From<u32> for I48;
    impl From<U32> for I48;
    impl From<i32> for I48;
    impl From<I32> for I48;
    impl From<U40> for I48;
    impl From<I40> for I48;

    impl From<u8> for U56;
    impl From<u16> for U56;
    impl From<U16> for U56;
    impl From<U24> for U56;
    impl From<u32> for U56;
    impl From<U32> for U56;
    impl From<U40> for U56;
    impl From<U48> for U56;

    impl From<u8> for I56;
    impl From<i8> for I56;
    impl From<u16> for I56;
    impl From<U16> for I56;
    impl From<i16> for I56;
    impl From<I16> for I56;
    impl From<U24> for I56;
    impl From<u32> for I56;
    impl From<U32> for I56;
    impl From<i32> for I56;
    impl From<I32> for I56;
    impl From<U40> for I56;
    impl From<I40> for I56;
    impl From<U48> for I56;
    impl From<I48> for I56;

    impl From<U16> for u64 as primitive;
    impl From<U32> for u64 as primitive;
    impl From<U24> for u64;
    impl From<U40> for u64;
    impl From<U48> for u64;
    impl From<U56> for u64;

    impl From<u8> for U64 as std;
    impl From<u16> for U64 as std;
    impl From<U16> for U64 as std;
    impl From<U24> for U64;
    impl From<u32> for U64 as std;
    impl From<U32> for U64 as std;
    impl From<U40> for U64;
    impl From<U48> for U64;
    impl From<U56> for U64;

    impl From<U16> for i64 as primitive;
    impl From<I16> for i64 as primitive;
    impl From<U32> for i64 as primitive;
    impl From<I32> for i64 as primitive;
    impl From<I24> for i64;
    impl From<I40> for i64;
    impl From<I48> for i64;
    impl From<I56> for i64;

    impl From<u8> for I64 as std;
    impl From<i8> for I64 as std;
    impl From<u16> for I64 as std;
    impl From<U16> for I64 as std;
    impl From<i16> for I64 as std;
    impl From<I16> for I64 as std;
    impl From<U24> for I64;
    impl From<u32> for I64 as std;
    impl From<U32> for I64 as std;
    impl From<i32> for I64 as std;
    impl From<I32> for I64 as std;
    impl From<U40> for I64;
    impl From<U48> for I64;
    impl From<U56> for I64;

    impl From<u8> for U72;
    impl From<u16> for U72;
    impl From<U16> for U72;
    impl From<U24> for U72;
    impl From<u32> for U72;
    impl From<U32> for U72;
    impl From<U40> for U72;
    impl From<U48> for U72;
    impl From<U56> for U72;
    impl From<u64> for U72;
    impl From<U64> for U72;

    impl From<u8> for I72;
    impl From<i8> for I72;
    impl From<u16> for I72;
    impl From<U16> for I72;
    impl From<i16> for I72;
    impl From<I16> for I72;
    impl From<U24> for I72;
    impl From<u32> for I72;
    impl From<U32> for I72;
    impl From<i32> for I72;
    impl From<I32> for I72;
    impl From<U40> for I72;
    impl From<I40> for I72;
    impl From<U48> for I72;
    impl From<I48> for I72;
    impl From<U56> for I72;
    impl From<I56> for I72;
    impl From<u64> for I72;
    impl From<U64> for I72;
    impl From<i64> for I72;
    impl From<I64> for I72;

    impl From<u16> for U80;
    impl From<U16> for U80;
    impl From<U24> for U80;
    impl From<u32> for U80;
    impl From<U32> for U80;
    impl From<U40> for U80;
    impl From<U48> for U80;
    impl From<U56> for U80;
    impl From<u64> for U80;
    impl From<U64> for U80;
    impl From<U72> for U80;

    impl From<u8> for I80;
    impl From<i8> for I80;
    impl From<u16> for I80;
    impl From<U16> for I80;
    impl From<i16> for I80;
    impl From<I16> for I80;
    impl From<U24> for I80;
    impl From<u32> for I80;
    impl From<U32> for I80;
    impl From<i32> for I80;
    impl From<I32> for I80;
    impl From<U40> for I80;
    impl From<I40> for I80;
    impl From<U48> for I80;
    impl From<I48> for I80;
    impl From<U56> for I80;
    impl From<I56> for I80;
    impl From<u64> for I80;
    impl From<i64> for I80;
    impl From<U64> for I80;
    impl From<I64> for I80;
    impl From<U72> for I80;
    impl From<I72> for I80;

    impl From<u16> for U88;
    impl From<U16> for U88;
    impl From<U24> for U88;
    impl From<u32> for U88;
    impl From<U32> for U88;
    impl From<U40> for U88;
    impl From<U48> for U88;
    impl From<U56> for U88;
    impl From<u64> for U88;
    impl From<U64> for U88;
    impl From<U72> for U88;
    impl From<U80> for U88;

    impl From<u8> for I88;
    impl From<i8> for I88;
    impl From<u16> for I88;
    impl From<U16> for I88;
    impl From<i16> for I88;
    impl From<I16> for I88;
    impl From<U24> for I88;
    impl From<u32> for I88;
    impl From<U32> for I88;
    impl From<i32> for I88;
    impl From<I32> for I88;
    impl From<U40> for I88;
    impl From<I40> for I88;
    impl From<U48> for I88;
    impl From<I48> for I88;
    impl From<U56> for I88;
    impl From<I56> for I88;
    impl From<u64> for I88;
    impl From<i64> for I88;
    impl From<U64> for I88;
    impl From<I64> for I88;
    impl From<U72> for I88;
    impl From<I72> for I88;
    impl From<U80> for I88;
    impl From<I80> for I88;

    impl From<u16> for U96;
    impl From<U16> for U96;
    impl From<U24> for U96;
    impl From<u32> for U96;
    impl From<U32> for U96;
    impl From<U40> for U96;
    impl From<U48> for U96;
    impl From<U56> for U96;
    impl From<u64> for U96;
    impl From<U64> for U96;
    impl From<U72> for U96;
    impl From<U80> for U96;
    impl From<U88> for U96;

    impl From<u8> for I96;
    impl From<i8> for I96;
    impl From<u16> for I96;
    impl From<U16> for I96;
    impl From<i16> for I96;
    impl From<I16> for I96;
    impl From<U24> for I96;
    impl From<u32> for I96;
    impl From<U32> for I96;
    impl From<i32> for I96;
    impl From<I32> for I96;
    impl From<U40> for I96;
    impl From<I40> for I96;
    impl From<U48> for I96;
    impl From<I48> for I96;
    impl From<U56> for I96;
    impl From<I56> for I96;
    impl From<u64> for I96;
    impl From<U64> for I96;
    impl From<i64> for I96;
    impl From<I64> for I96;
    impl From<U72> for I96;
    impl From<I72> for I96;
    impl From<U80> for I96;
    impl From<I80> for I96;
    impl From<U88> for I96;
    impl From<I88> for I96;

    impl From<u16> for U104;
    impl From<U16> for U104;
    impl From<U24> for U104;
    impl From<u32> for U104;
    impl From<U32> for U104;
    impl From<U40> for U104;
    impl From<U48> for U104;
    impl From<U56> for U104;
    impl From<u64> for U104;
    impl From<U64> for U104;
    impl From<U72> for U104;
    impl From<U80> for U104;
    impl From<U88> for U104;
    impl From<U96> for U104;

    impl From<u8> for I104;
    impl From<i8> for I104;
    impl From<u16> for I104;
    impl From<U16> for I104;
    impl From<i16> for I104;
    impl From<I16> for I104;
    impl From<U24> for I104;
    impl From<u32> for I104;
    impl From<U32> for I104;
    impl From<i32> for I104;
    impl From<I32> for I104;
    impl From<U40> for I104;
    impl From<I40> for I104;
    impl From<U48> for I104;
    impl From<I48> for I104;
    impl From<U56> for I104;
    impl From<I56> for I104;
    impl From<u64> for I104;
    impl From<U64> for I104;
    impl From<i64> for I104;
    impl From<I64> for I104;
    impl From<U72> for I104;
    impl From<I72> for I104;
    impl From<U80> for I104;
    impl From<I80> for I104;
    impl From<U88> for I104;
    impl From<I88> for I104;
    impl From<U96> for I104;
    impl From<I96> for I104;

    impl From<u16> for U112;
    impl From<U16> for U112;
    impl From<U24> for U112;
    impl From<u32> for U112;
    impl From<U32> for U112;
    impl From<U40> for U112;
    impl From<U48> for U112;
    impl From<U56> for U112;
    impl From<u64> for U112;
    impl From<U64> for U112;
    impl From<U72> for U112;
    impl From<U80> for U112;
    impl From<U88> for U112;
    impl From<U96> for U112;
    impl From<U104> for U112;
    
    impl From<u8> for I112;
    impl From<i8> for I112;
    impl From<u16> for I112;
    impl From<U16> for I112;
    impl From<i16> for I112;
    impl From<I16> for I112;
    impl From<U24> for I112;
    impl From<u32> for I112;
    impl From<U32> for I112;
    impl From<i32> for I112;
    impl From<I32> for I112;
    impl From<U40> for I112;
    impl From<I40> for I112;
    impl From<U48> for I112;
    impl From<I48> for I112;
    impl From<U56> for I112;
    impl From<I56> for I112;
    impl From<u64> for I112;
    impl From<U64> for I112;
    impl From<i64> for I112;
    impl From<I64> for I112;
    impl From<U72> for I112;
    impl From<I72> for I112;
    impl From<U80> for I112;
    impl From<I80> for I112;
    impl From<U88> for I112;
    impl From<I88> for I112;
    impl From<U96> for I112;
    impl From<I96> for I112;
    impl From<U104> for I112;
    impl From<I104> for I112;

    impl From<u16> for U120;
    impl From<U16> for U120;
    impl From<U24> for U120;
    impl From<u32> for U120;
    impl From<U32> for U120;
    impl From<U40> for U120;
    impl From<U48> for U120;
    impl From<U56> for U120;
    impl From<u64> for U120;
    impl From<U64> for U120;
    impl From<U72> for U120;
    impl From<U80> for U120;
    impl From<U88> for U120;
    impl From<U96> for U120;
    impl From<U104> for U120;
    impl From<U112> for U120;

    impl From<u8> for I120;
    impl From<i8> for I120;
    impl From<u16> for I120;
    impl From<U16> for I120;
    impl From<i16> for I120;
    impl From<I16> for I120;
    impl From<U24> for I120;
    impl From<u32> for I120;
    impl From<U32> for I120;
    impl From<i32> for I120;
    impl From<I32> for I120;
    impl From<U40> for I120;
    impl From<U48> for I120;
    impl From<I40> for I120;
    impl From<I48> for I120;
    impl From<U56> for I120;
    impl From<I56> for I120;
    impl From<u64> for I120;
    impl From<U64> for I120;
    impl From<i64> for I120;
    impl From<I64> for I120;
    impl From<U72> for I120;
    impl From<I72> for I120;
    impl From<U80> for I120;
    impl From<I80> for I120;
    impl From<U88> for I120;
    impl From<I88> for I120;
    impl From<U96> for I120;
    impl From<I96> for I120;
    impl From<U104> for I120;
    impl From<I104> for I120;
    impl From<U112> for I120;
    impl From<I112> for I120;

    impl From<U16> for u128 as primitive;
    impl From<U24> for u128;
    impl From<U32> for u128 as primitive;
    impl From<U40> for u128;
    impl From<U48> for u128;
    impl From<U56> for u128;
    impl From<U64> for u128 as primitive;
    impl From<U72> for u128;
    impl From<U80> for u128;
    impl From<U88> for u128;
    impl From<U96> for u128;
    impl From<U104> for u128;
    impl From<U112> for u128;
    impl From<U120> for u128;

    impl From<u8> for U128 as std;
    impl From<u16> for U128 as std;
    impl From<U16> for U128 as std;
    impl From<U24> for U128;
    impl From<u32> for U128 as std;
    impl From<U32> for U128 as std;
    impl From<U40> for U128;
    impl From<U48> for U128;
    impl From<U56> for U128;
    impl From<u64> for U128 as std;
    impl From<U64> for U128 as std;
    impl From<U72> for U128;
    impl From<U80> for U128;
    impl From<U88> for U128;
    impl From<U96> for U128;
    impl From<U104> for U128;
    impl From<U112> for U128;
    impl From<U120> for U128;

    impl From<u8> for I128 as std;
    impl From<i8> for I128 as std;
    impl From<u16> for I128 as std;
    impl From<i16> for I128 as std;
    impl From<U16> for I128 as std;
    impl From<I16> for I128 as std;
    impl From<U24> for I128;
    impl From<U40> for I128;
    impl From<U48> for I128;
    impl From<U56> for I128;
    impl From<u32> for I128 as std;
    impl From<U32> for I128 as std;
    impl From<i32> for I128 as std;
    impl From<I32> for I128 as std;
    impl From<u64> for I128 as std;
    impl From<U64> for I128 as std;
    impl From<i64> for I128 as std;
    impl From<I64> for I128 as std;
    impl From<U72> for I128;
    impl From<U80> for I128;
    impl From<U88> for I128;
    impl From<U96> for I128;
    impl From<U104> for I128;
    impl From<U112> for I128;
    impl From<U120> for I128;

    impl From<U16> for i128 as primitive;
    impl From<I16> for i128 as primitive;
    impl From<I24> for i128;
    impl From<U32> for i128 as primitive;
    impl From<I32> for i128 as primitive;
    impl From<I40> for i128;
    impl From<I48> for i128;
    impl From<I56> for i128;
    impl From<U64> for i128 as primitive;
    impl From<I64> for i128 as primitive;
    impl From<I72> for i128;
    impl From<I80> for i128;
    impl From<I88> for i128;
    impl From<I96> for i128;
    impl From<I104> for i128;
    impl From<I112> for i128;
    impl From<I120> for i128;
);
