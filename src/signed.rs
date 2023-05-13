macro_rules! nonstandard_int {
    (
        $(
            $( #[$docs:meta] )*
            @[repr($repr:ty)]
            $vis:vis struct $name:ident([u8; $num_bytes:literal])
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

            impl $crate::UnalignedInteger for $name {
                type Repr = $repr;

                #[inline]
                fn sign_ext_byte(self) -> ::core::primitive::u8 {
                    $crate::utils::sign_ext_byte(self.is_positive())
                }
            }

            impl $name {
                /// The amount of bits required by this integer type.
                pub const BITS: ::core::primitive::u32 = $num_bytes * 8_u32;

                /// The smallest value that can be represented by this integer type.
                pub const MIN: Self = {
                    let mut bytes = [0x00_u8; $num_bytes];
                    bytes[Self::msb_pos()] = 0x80_u8;
                    Self(bytes)
                };

                /// The largest value that can be represented by this integer type.
                pub const MAX: Self = {
                    let mut bytes = [0xFF_u8; $num_bytes];
                    bytes[Self::msb_pos()] = 0x7F_u8;
                    Self(bytes)
                };

                /// Returns the index position of the most significant byte.
                #[inline]
                pub(crate) const fn msb_pos() -> ::core::primitive::usize {
                    if ::core::cfg!(target_endian = "big") {
                        0_usize
                    } else {
                        $num_bytes - 1_usize
                    }
                }

                /// Returns `true` if `self` is positive.
                #[inline]
                pub(crate) const fn is_positive(self) -> ::core::primitive::bool {
                    (self.0[Self::msb_pos()] & 0x80_u8) == 0x00_u8
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

            impl ::core::default::Default for $name {
                #[inline]
                fn default() -> Self {
                    Self([0x00_u8; $num_bytes])
                }
            }

            $crate::utils::impl_std_traits!($name as $repr);
        )*
    }
}
nonstandard_int! {
    /// 24-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i32)]
    pub struct I24([u8; 3]);

    /// 40-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i64)]
    pub struct I40([u8; 5]);

    /// 48-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i64)]
    pub struct I48([u8; 6]);

    /// 56-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i64)]
    pub struct I56([u8; 7]);

    /// 72-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128)]
    pub struct I72([u8; 9]);

    /// 80-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128)]
    pub struct I80([u8; 10]);

    /// 88-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128)]
    pub struct I88([u8; 11]);

    /// 96-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128)]
    pub struct I96([u8; 12]);

    /// 104-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128)]
    pub struct I104([u8; 13]);

    /// 112-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128)]
    pub struct I112([u8; 14]);

    /// 120-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128)]
    pub struct I120([u8; 15]);
}
