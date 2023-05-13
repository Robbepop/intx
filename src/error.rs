/// Error that may occur for fallible conversions between integers.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromIntError(pub(crate) ());

impl From<core::num::TryFromIntError> for TryFromIntError {
    #[inline]
    fn from(_: core::num::TryFromIntError) -> Self {
        Self(())
    }
}
