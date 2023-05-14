//! Definitions of unaligned standard bitwidth integers.

macro_rules! unaligned_int {
    (
        $(
            $( #[$docs:meta] )*
            @[repr($repr:ty, $signed:ident)]
            $vis:vis struct $name:ident;
        )*
    ) => {
        $(
            $( #[$docs] )*
            #[derive(
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::core::cmp::PartialEq,
                ::core::cmp::Eq,
            )]
            $vis struct $name([::core::primitive::u8; ::core::mem::size_of::<$repr>()]);

            unaligned_int!(
                @impl
                $( #[$docs] )*
                @[repr($repr, $signed)]
                $vis struct $name;
            );

            impl $name {
                /// The amount of bits required by this integer type.
                pub const BITS: ::core::primitive::u32 = <$repr>::BITS;

                /// The smallest value that can be represented by this integer type.
                pub const MIN: Self = Self::from_ne_bytes(<$repr>::MIN.to_ne_bytes());

                /// The largest value that can be represented by this integer type.
                pub const MAX: Self = Self::from_ne_bytes(<$repr>::MAX.to_ne_bytes());

                /// Returns the integer value as a byte array in native-endian order.
                #[inline]
                pub const fn to_ne_bytes(self) -> [::core::primitive::u8; ::core::mem::size_of::<$repr>()] {
                    self.0
                }

                /// Returns the integer value as a byte array in little-endian order.
                #[inline]
                pub fn to_le_bytes(self) -> [::core::primitive::u8; ::core::mem::size_of::<$repr>()] {
                    $crate::utils::ne_bytes_to_le(self.to_ne_bytes())
                }

                /// Returns the integer value as a byte array in big-endian order.
                #[inline]
                pub fn to_be_bytes(self) -> [::core::primitive::u8; ::core::mem::size_of::<$repr>()] {
                    $crate::utils::ne_bytes_to_be(self.to_ne_bytes())
                }

                /// Creates an unaligned signed integer from the given bytes in native-endian order.
                #[inline]
                pub const fn from_ne_bytes(bytes: [::core::primitive::u8; ::core::mem::size_of::<$repr>()]) -> Self {
                    Self(bytes)
                }

                /// Creates an unaligned signed integer from the given bytes in little-endian order.
                #[inline]
                pub fn from_le_bytes(bytes: [::core::primitive::u8; ::core::mem::size_of::<$repr>()]) -> Self {
                    Self::from_ne_bytes($crate::utils::le_bytes_to_ne(bytes))
                }

                /// Creates an unaligned signed integer from the given bytes in big-endian order.
                #[inline]
                pub fn from_be_bytes(bytes: [::core::primitive::u8; ::core::mem::size_of::<$repr>()]) -> Self {
                    Self::from_ne_bytes($crate::utils::be_bytes_to_ne(bytes))
                }
            }

            impl ::core::convert::From<$repr> for $name {
                #[inline]
                fn from(value: $repr) -> Self {
                    Self::from_ne_bytes(value.to_ne_bytes())
                }
            }

            impl ::core::convert::From<$name> for $repr {
                #[inline]
                fn from(value: $name) -> Self {
                    <$repr>::from_ne_bytes(value.to_ne_bytes())
                }
            }

            $crate::utils::impl_std_traits!($name as $repr);
        )*
    };
    (
        @impl
        $( #[$docs:meta] )*
        @[repr($repr:ty, unsigned)]
        $vis:vis struct $name:ident;
    ) => {
        impl $crate::UnalignedInteger for $name {
            type Repr = $repr;

            #[inline]
            fn sign_ext_byte(self) -> ::core::primitive::u8 {
                0x00_u8
            }
        }
    };
    (
        @impl
        $( #[$docs:meta] )*
        @[repr($repr:ty, signed)]
        $vis:vis struct $name:ident;
    ) => {
        impl $crate::UnalignedInteger for $name {
            type Repr = $repr;

            fn sign_ext_byte(self) -> ::core::primitive::u8 {
                $crate::utils::sign_ext_byte(self.is_positive())
            }
        }

        impl $name {
            /// Returns the index position of the most significant byte.
            #[inline]
            const fn msb_pos() -> ::core::primitive::usize {
                if ::core::cfg!(target_endian = "big") {
                    0_usize
                } else {
                    ::core::mem::size_of::<$repr>() - 1_usize
                }
            }

            /// Returns `true` if `self` is positive.
            #[inline]
            pub(crate) const fn is_positive(self) -> ::core::primitive::bool {
                (self.0[Self::msb_pos()] & 0x80_u8) == 0x00_u8
            }
        }
    };
}
unaligned_int! {
    /// 16-bit unsigned integer similar to `u16` with alignment of 1.
    @[repr(::core::primitive::u16, unsigned)]
    pub struct U16;

    /// 16-bit signed integer similar to `i16` with alignment of 1.
    @[repr(::core::primitive::i16, signed)]
    pub struct I16;

    /// 32-bit unsigned integer similar to `u32` with alignment of 1.
    @[repr(::core::primitive::u32, unsigned)]
    pub struct U32;

    /// 32-bit signed integer similar to `i32` with alignment of 1.
    @[repr(::core::primitive::i32, signed)]
    pub struct I32;

    /// 64-bit unsigned integer similar to `u64` with alignment of 1.
    @[repr(::core::primitive::u64, unsigned)]
    pub struct U64;

    /// 64-bit signed integer similar to `i64` with alignment of 1.
    @[repr(::core::primitive::i64, signed)]
    pub struct I64;

    /// 128-bit unsigned integer similar to `u128` with alignment of 1.
    @[repr(::core::primitive::u128, unsigned)]
    pub struct U128;

    /// 128-bit signed integer similar to `i128` with alignment of 1.
    @[repr(::core::primitive::i128, signed)]
    pub struct I128;
}
