macro_rules! nonstandard_uint {
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
                    0x00_u8
                }
            }

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
            }

            $crate::utils::impl_commons!($name as $repr);
        )*
    };
}
nonstandard_uint! {
    /// 24-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u32)]
    pub struct U24([u8; 3]);

    /// 40-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u64)]
    pub struct U40([u8; 5]);

    /// 48-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u64)]
    pub struct U48([u8; 6]);

    /// 56-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u64)]
    pub struct U56([u8; 7]);

    /// 72-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128)]
    pub struct U72([u8; 9]);

    /// 80-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128)]
    pub struct U80([u8; 10]);

    /// 88-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128)]
    pub struct U88([u8; 11]);

    /// 96-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128)]
    pub struct U96([u8; 12]);

    /// 104-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128)]
    pub struct U104([u8; 13]);

    /// 112-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128)]
    pub struct U112([u8; 14]);

    /// 120-bit unsigned integer with alignment of 1.
    @[repr(::core::primitive::u128)]
    pub struct U120([u8; 15]);
}
