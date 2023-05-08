/// Returns the sign extension byte for signed integers.
///
/// Those are the bytes with which the integer is extended upon conversion
/// from smaller integer types such as `i16` to `i24` conversion.
#[inline]
pub const fn sign_ext_byte(is_positive: bool) -> u8 {
    match is_positive {
        true => 0x00,
        false => 0xFF,
    }
}

/// Copies bytes from smaller `src` to larger `dst` array and respects endianess.
///
/// # Note
///
/// The `dst` array is untouched for areas that have no respective `src` values.
#[inline]
pub fn extend_bytes<const N: usize, const M: usize>(dst: &mut [u8; N], src: &[u8; M]) {
    debug_assert!(N > M);
    let offset = cfg!(target_endian = "big")
        .then(|| usize::abs_diff(N, M))
        .unwrap_or(0);
    dst[offset..][..M].copy_from_slice(src);
}

/// Copies bytes from larger `src` to smaller `dst` array and respects endianess.
///
/// # Note
///
/// Only copies over elements from `src` to `dst` within bounds.
#[inline]
pub fn truncate_bytes<const N: usize, const M: usize>(dst: &mut [u8; N], src: &[u8; M]) {
    debug_assert!(N < M);
    let offset = cfg!(target_endian = "big")
        .then(|| usize::abs_diff(N, M))
        .unwrap_or(0);
    dst[..].copy_from_slice(&src[offset..][..N]);
}

/// Returns the array with reversed order of values.
#[inline]
pub fn reverse_bytes<const N: usize>(array: [u8; N]) -> [u8; N] {
    let mut array = array;
    array.reverse();
    array
}

/// Converts the byte array from little-endian to native-endian if necessary.
#[inline]
pub fn le_bytes_to_ne<const N: usize>(array: [u8; N]) -> [u8; N] {
    match cfg!(target_endian = "little") {
        true => array,
        false => reverse_bytes(array),
    }
}

/// Converts the byte array from native-endian to little-endian if necessary.
#[inline]
pub fn ne_bytes_to_le<const N: usize>(array: [u8; N]) -> [u8; N] {
    match cfg!(target_endian = "little") {
        true => array,
        false => reverse_bytes(array),
    }
}

/// Converts the byte array from big-endian to native-endian if necessary.
#[inline]
pub fn be_bytes_to_ne<const N: usize>(array: [u8; N]) -> [u8; N] {
    match cfg!(target_endian = "big") {
        true => array,
        false => reverse_bytes(array),
    }
}

/// Converts the byte array from native-endian to big-endian if necessary.
#[inline]
pub fn ne_bytes_to_be<const N: usize>(array: [u8; N]) -> [u8; N] {
    match cfg!(target_endian = "big") {
        true => array,
        false => reverse_bytes(array),
    }
}

/// Implements some Rust standard library traits for `$ty` as if it was a `$prim` type.
// #[macro_export]
macro_rules! impl_std_traits {
    ( $ty:ty as $prim:ty ) => {
        impl ::core::cmp::PartialOrd for $ty {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                <$prim as ::core::cmp::PartialOrd>::partial_cmp(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    &<$prim as ::core::convert::From<$ty>>::from(*other),
                )
            }

            #[inline]
            fn lt(&self, other: &Self) -> ::core::primitive::bool {
                <$prim as ::core::cmp::PartialOrd>::lt(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    &<$prim as ::core::convert::From<$ty>>::from(*other),
                )
            }

            #[inline]
            fn le(&self, other: &Self) -> ::core::primitive::bool {
                <$prim as ::core::cmp::PartialOrd>::le(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    &<$prim as ::core::convert::From<$ty>>::from(*other),
                )
            }

            #[inline]
            fn gt(&self, other: &Self) -> ::core::primitive::bool {
                <$prim as ::core::cmp::PartialOrd>::gt(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    &<$prim as ::core::convert::From<$ty>>::from(*other),
                )
            }

            #[inline]
            fn ge(&self, other: &Self) -> ::core::primitive::bool {
                <$prim as ::core::cmp::PartialOrd>::ge(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    &<$prim as ::core::convert::From<$ty>>::from(*other),
                )
            }
        }

        impl ::core::cmp::Ord for $ty {
            #[inline]
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                <$prim as ::core::cmp::Ord>::cmp(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    &<$prim as ::core::convert::From<$ty>>::from(*other),
                )
            }
        }

        impl ::core::hash::Hash for $ty {
            #[inline]
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                <$prim as ::core::hash::Hash>::hash(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    state,
                )
            }
        }

        impl ::core::fmt::Debug for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <$prim as ::core::fmt::Debug>::fmt(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    f,
                )
            }
        }

        impl ::core::fmt::Display for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <$prim as ::core::fmt::Display>::fmt(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    f,
                )
            }
        }

        impl ::core::fmt::Binary for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <$prim as ::core::fmt::Binary>::fmt(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    f,
                )
            }
        }

        impl ::core::fmt::Octal for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <$prim as ::core::fmt::Octal>::fmt(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    f,
                )
            }
        }

        impl ::core::fmt::LowerHex for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <$prim as ::core::fmt::LowerHex>::fmt(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    f,
                )
            }
        }

        impl ::core::fmt::UpperHex for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <$prim as ::core::fmt::UpperHex>::fmt(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    f,
                )
            }
        }

        impl ::core::fmt::LowerExp for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <$prim as ::core::fmt::LowerExp>::fmt(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    f,
                )
            }
        }

        impl ::core::fmt::UpperExp for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <$prim as ::core::fmt::UpperExp>::fmt(
                    &<$prim as ::core::convert::From<$ty>>::from(*self),
                    f,
                )
            }
        }
    };
}

#[doc(inline)]
pub(crate) use impl_std_traits;
