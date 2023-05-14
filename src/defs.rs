macro_rules! unaligned_int {
    (
        $(
            $( #[$docs:meta] )*
            @[repr($repr:ty, $signedness:ident)]
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

            unaligned_int!(
                @impl
                $( #[$docs] )*
                @[repr($repr, $signedness)]
                $vis struct $name([u8; $num_bytes])
            );

            impl $name {
                /// The amount of bits required by this integer type.
                pub const BITS: ::core::primitive::u32 = $num_bytes * 8_u32;

                /// Returns the index position of the most significant byte.
                #[inline]
                #[allow(dead_code)] // Note: not used by power-of-two sized ints atm
                pub(crate) const fn msb_pos() -> ::core::primitive::usize {
                    if ::core::cfg!(target_endian = "big") {
                        0_usize
                    } else {
                        $num_bytes - 1_usize
                    }
                }
            }

            impl $name {
                /// Returns the integer value as a byte array in native-endian order.
                #[inline]
                pub const fn to_ne_bytes(
                    self,
                ) -> [::core::primitive::u8; ::core::mem::size_of::<Self>()] {
                    self.0
                }

                /// Returns the integer value as a byte array in little-endian order.
                #[inline]
                pub fn to_le_bytes(self) -> [::core::primitive::u8; ::core::mem::size_of::<Self>()] {
                    $crate::utils::ne_bytes_to_le(self.to_ne_bytes())
                }

                /// Returns the integer value as a byte array in big-endian order.
                #[inline]
                pub fn to_be_bytes(self) -> [::core::primitive::u8; ::core::mem::size_of::<Self>()] {
                    $crate::utils::ne_bytes_to_be(self.to_ne_bytes())
                }

                /// Creates an unaligned signed integer from the given bytes in native-endian order.
                #[inline]
                pub const fn from_ne_bytes(
                    bytes: [::core::primitive::u8; ::core::mem::size_of::<Self>()],
                ) -> Self {
                    Self(bytes)
                }

                /// Creates an unaligned signed integer from the given bytes in little-endian order.
                #[inline]
                pub fn from_le_bytes(
                    bytes: [::core::primitive::u8; ::core::mem::size_of::<Self>()],
                ) -> Self {
                    Self::from_ne_bytes($crate::utils::le_bytes_to_ne(bytes))
                }

                /// Creates an unaligned signed integer from the given bytes in big-endian order.
                #[inline]
                pub fn from_be_bytes(
                    bytes: [::core::primitive::u8; ::core::mem::size_of::<Self>()],
                ) -> Self {
                    Self::from_ne_bytes($crate::utils::be_bytes_to_ne(bytes))
                }
            }

            impl ::core::default::Default for $name {
                #[inline]
                fn default() -> Self {
                    Self([0x00_u8; ::core::mem::size_of::<Self>()])
                }
            }

            impl ::core::cmp::PartialOrd for $name {
                #[inline]
                fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                    <$repr as ::core::cmp::PartialOrd>::partial_cmp(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        &<$repr as ::core::convert::From<Self>>::from(*other),
                    )
                }

                #[inline]
                fn lt(&self, other: &Self) -> ::core::primitive::bool {
                    <$repr as ::core::cmp::PartialOrd>::lt(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        &<$repr as ::core::convert::From<Self>>::from(*other),
                    )
                }

                #[inline]
                fn le(&self, other: &Self) -> ::core::primitive::bool {
                    <$repr as ::core::cmp::PartialOrd>::le(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        &<$repr as ::core::convert::From<Self>>::from(*other),
                    )
                }

                #[inline]
                fn gt(&self, other: &Self) -> ::core::primitive::bool {
                    <$repr as ::core::cmp::PartialOrd>::gt(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        &<$repr as ::core::convert::From<Self>>::from(*other),
                    )
                }

                #[inline]
                fn ge(&self, other: &Self) -> ::core::primitive::bool {
                    <$repr as ::core::cmp::PartialOrd>::ge(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        &<$repr as ::core::convert::From<Self>>::from(*other),
                    )
                }
            }

            impl ::core::cmp::Ord for $name {
                #[inline]
                fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                    <$repr as ::core::cmp::Ord>::cmp(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        &<$repr as ::core::convert::From<Self>>::from(*other),
                    )
                }
            }

            impl ::core::hash::Hash for $name {
                #[inline]
                fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                    <$repr as ::core::hash::Hash>::hash(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        state,
                    )
                }
            }

            impl ::core::fmt::Debug for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    <$repr as ::core::fmt::Debug>::fmt(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        f,
                    )
                }
            }

            impl ::core::fmt::Display for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    <$repr as ::core::fmt::Display>::fmt(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        f,
                    )
                }
            }

            impl ::core::fmt::Binary for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    <$repr as ::core::fmt::Binary>::fmt(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        f,
                    )
                }
            }

            impl ::core::fmt::Octal for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    <$repr as ::core::fmt::Octal>::fmt(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        f,
                    )
                }
            }

            impl ::core::fmt::LowerHex for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    <$repr as ::core::fmt::LowerHex>::fmt(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        f,
                    )
                }
            }

            impl ::core::fmt::UpperHex for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    <$repr as ::core::fmt::UpperHex>::fmt(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        f,
                    )
                }
            }

            impl ::core::fmt::LowerExp for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    <$repr as ::core::fmt::LowerExp>::fmt(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        f,
                    )
                }
            }

            impl ::core::fmt::UpperExp for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    <$repr as ::core::fmt::UpperExp>::fmt(
                        &<$repr as ::core::convert::From<Self>>::from(*self),
                        f,
                    )
                }
            }
        )*
    };
    (
        @impl
        $( #[$docs:meta] )*
        @[repr($repr:ty, unsigned)]
        $vis:vis struct $name:ident([u8; $num_bytes:literal])
    ) => {
        impl $name {
            /// The smallest value that can be represented by this integer type.
            pub const MIN: Self = Self::from_ne_bytes([0x00_u8; $num_bytes]);

            /// The largest value that can be represented by this integer type.
            pub const MAX: Self = Self::from_ne_bytes([0xFF_u8; $num_bytes]);
        }

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
        $vis:vis struct $name:ident([u8; $num_bytes:literal])
    ) => {
        impl $name {
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

            /// Returns `true` if `self` is positive.
            #[inline]
            pub(crate) const fn is_positive(self) -> ::core::primitive::bool {
                (self.0[Self::msb_pos()] & 0x80_u8) == 0x00_u8
            }
        }

        impl $crate::UnalignedInteger for $name {
            type Repr = $repr;

            #[inline]
            fn sign_ext_byte(self) -> ::core::primitive::u8 {
                $crate::utils::sign_ext_byte(self.is_positive())
            }
        }
    }
}
unaligned_int! {
    /// 16-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u16, unsigned)]
    pub struct U16([u8; 2]);

    /// 16-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i16, signed)]
    pub struct I16([u8; 2]);

    /// 24-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u32, unsigned)]
    pub struct U24([u8; 3]);

    /// 24-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i32, signed)]
    pub struct I24([u8; 3]);

    /// 32-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u32, unsigned)]
    pub struct U32([u8; 4]);

    /// 32-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i32, signed)]
    pub struct I32([u8; 4]);

    /// 40-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u64, unsigned)]
    pub struct U40([u8; 5]);

    /// 40-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i64, signed)]
    pub struct I40([u8; 5]);

    /// 48-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u64, unsigned)]
    pub struct U48([u8; 6]);

    /// 48-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i64, signed)]
    pub struct I48([u8; 6]);

    /// 56-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u64, unsigned)]
    pub struct U56([u8; 7]);

    /// 56-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i64, signed)]
    pub struct I56([u8; 7]);

    /// 64-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u64, unsigned)]
    pub struct U64([u8; 8]);

    /// 64-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i64, signed)]
    pub struct I64([u8; 8]);

    /// 72-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128, unsigned)]
    pub struct U72([u8; 9]);

    /// 72-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128, signed)]
    pub struct I72([u8; 9]);

    /// 80-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128, unsigned)]
    pub struct U80([u8; 10]);

    /// 80-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128, signed)]
    pub struct I80([u8; 10]);

    /// 88-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128, unsigned)]
    pub struct U88([u8; 11]);

    /// 88-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128, signed)]
    pub struct I88([u8; 11]);

    /// 96-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128, unsigned)]
    pub struct U96([u8; 12]);

    /// 96-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128, signed)]
    pub struct I96([u8; 12]);

    /// 104-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128, unsigned)]
    pub struct U104([u8; 13]);

    /// 104-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128, signed)]
    pub struct I104([u8; 13]);

    /// 112-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128, unsigned)]
    pub struct U112([u8; 14]);

    /// 112-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128, signed)]
    pub struct I112([u8; 14]);

    /// 120-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128, unsigned)]
    pub struct U120([u8; 15]);

    /// 120-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128, signed)]
    pub struct I120([u8; 15]);

    /// 128-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128, unsigned)]
    pub struct U128([u8; 16]);

    /// 128-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i128, signed)]
    pub struct I128([u8; 16]);
}
