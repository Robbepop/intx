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
                pub(crate) const fn msb_pos() -> ::core::primitive::usize {
                    if ::core::cfg!(target_endian = "big") {
                        0_usize
                    } else {
                        $num_bytes - 1_usize
                    }
                }
            }

            $crate::utils::impl_commons!($name as $repr);
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
    /// 24-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u32, unsigned)]
    pub struct U24([u8; 3]);

    /// 24-bit signed integer with alignment of 1.
    @[repr(::core::primitive::i32, signed)]
    pub struct I24([u8; 3]);

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
}
