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
