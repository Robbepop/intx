macro_rules! nonstandard_uint {
    (
        $(
            $( #[$docs:meta] )*
            $vis:vis struct $name:ident(bytes = $num_bytes:literal, lower = $lower:ty, upper = $upper:ty)
        );* $(;)?
    ) => {
        $(
            $( #[$docs] )*
            #[derive(
                ::core::marker::Copy,
                ::core::clone::Clone,
                ::core::cmp::PartialEq,
                ::core::cmp::Eq,
            )]
            $vis struct $name([::core::primitive::u8; $num_bytes]);

            impl $name {
                /// The amount of bits required by this integer type.
                pub const BITS: ::core::primitive::u32 = $num_bytes * 8_u32;

                /// The smallest value that can be represented by this integer type.
                pub const MIN: Self = Self::from_ne_bytes([0x00_u8; $num_bytes]);

                /// The largest value that can be represented by this integer type.
                pub const MAX: Self = Self::from_ne_bytes([0xFF_u8; $num_bytes]);

                /// Returns the index position of the most significant byte.
                #[inline]
                pub(crate) const fn msb_pos() -> ::core::primitive::usize {
                    if ::core::cfg!(target_endian = "big") {
                        0_usize
                    } else {
                        $num_bytes - 1_usize
                    }
                }

                /// Returns the integer value as a byte array in native-endian order.
                #[inline]
                pub const fn to_ne_bytes(self) -> [::core::primitive::u8; $num_bytes] {
                    self.0
                }

                /// Returns the integer value as a byte array in little-endian order.
                #[inline]
                pub fn to_le_bytes(self) -> [::core::primitive::u8; $num_bytes] {
                    $crate::utils::ne_bytes_to_le(self.to_ne_bytes())
                }

                /// Returns the integer value as a byte array in big-endian order.
                #[inline]
                pub fn to_be_bytes(self) -> [::core::primitive::u8; $num_bytes] {
                    $crate::utils::ne_bytes_to_be(self.to_ne_bytes())
                }

                /// Creates an unaligned signed integer from the given bytes in native-endian order.
                #[inline]
                pub const fn from_ne_bytes(bytes: [::core::primitive::u8; $num_bytes]) -> Self {
                    Self(bytes)
                }

                /// Creates an unaligned signed integer from the given bytes in little-endian order.
                #[inline]
                pub fn from_le_bytes(bytes: [::core::primitive::u8; $num_bytes]) -> Self {
                    Self::from_ne_bytes($crate::utils::le_bytes_to_ne(bytes))
                }

                /// Creates an unaligned signed integer from the given bytes in big-endian order.
                #[inline]
                pub fn from_be_bytes(bytes: [::core::primitive::u8; $num_bytes]) -> Self {
                    Self::from_ne_bytes($crate::utils::be_bytes_to_ne(bytes))
                }
            }

            impl $crate::UnalignedInteger for $name {
                type LowerPrimitive = $lower;
                type UpperPrimitive = $upper;

                #[inline]
                fn sign_ext_byte(self) -> ::core::primitive::u8 {
                    0x00_u8
                }
            }

            impl ::core::default::Default for $name {
                #[inline]
                fn default() -> Self {
                    Self([0x00_u8; $num_bytes])
                }
            }

            impl ::core::convert::TryFrom<$upper> for $name {
                type Error = $crate::TryFromIntError;

                #[inline]
                fn try_from(value: $upper) -> ::core::result::Result<Self, Self::Error> {
                    if value >= (1 << Self::BITS) {
                        return ::core::result::Result::Err($crate::TryFromIntError(()))
                    }
                    let mut dst = [0x00_u8; $num_bytes];
                    let src = value.to_ne_bytes();
                    $crate::utils::truncate_bytes(&mut dst, &src);
                    ::core::result::Result::Ok(Self::from_ne_bytes(dst))
                }
            }

            $crate::utils::impl_std_traits!($name as $upper);
        )*
    };
}
nonstandard_uint! {
    /// 24-bit unsigned integer with alignment of 1.
    pub struct U24(bytes = 3, lower = ::core::primitive::u16, upper = ::core::primitive::u32);

    /// 40-bit unsigned integer with alignment of 1.
    pub struct U40(bytes = 5, lower = ::core::primitive::u32, upper = ::core::primitive::u64);

    /// 48-bit unsigned integer with alignment of 1.
    pub struct U48(bytes = 6, lower = ::core::primitive::u32, upper = ::core::primitive::u64);

    /// 56-bit unsigned integer with alignment of 1.
    pub struct U56(bytes = 7, lower = ::core::primitive::u32, upper = ::core::primitive::u64);

    /// 72-bit unsigned integer with alignment of 1.
    pub struct U72(bytes = 9, lower = ::core::primitive::u64, upper = ::core::primitive::u128);

    /// 80-bit unsigned integer with alignment of 1.
    pub struct U80(bytes = 10, lower = ::core::primitive::u64, upper = ::core::primitive::u128);

    /// 88-bit unsigned integer with alignment of 1.
    pub struct U88(bytes = 11, lower = ::core::primitive::u64, upper = ::core::primitive::u128);

    /// 96-bit unsigned integer with alignment of 1.
    pub struct U96(bytes = 12, lower = ::core::primitive::u64, upper = ::core::primitive::u128);

    /// 104-bit unsigned integer with alignment of 1.
    pub struct U104(bytes = 13, lower = ::core::primitive::u64, upper = ::core::primitive::u128);

    /// 112-bit unsigned integer with alignment of 1.
    pub struct U112(bytes = 14, lower = ::core::primitive::u64, upper = ::core::primitive::u128);

    /// 120-bit unsigned integer with alignment of 1.
    pub struct U120(bytes = 15, lower = ::core::primitive::u64, upper = ::core::primitive::u128);
}
