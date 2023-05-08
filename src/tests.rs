use crate::*;

#[test]
fn try_from_i32_for_i24() {
    // Testing for very small positive i32 values that fit into a single byte.
    for byte in 0..=u8::MAX {
        assert_eq!(
            I24::try_from(byte as i32),
            Ok(I24::from_ne_bytes([byte, 0x00, 0x00]))
        );
    }
    // Testing for very small negative i32 values that fit into a single byte.
    for byte in 1..=i8::MAX {
        assert_eq!(
            I24::try_from((-byte) as i32),
            Ok(I24::from_ne_bytes([
                (!byte as u8).wrapping_add(1),
                0xFF,
                0xFF
            ]))
        );
    }
    // Testing a few remaining edge cases in the following.
    assert_eq!(
        I24::try_from((i8::MIN) as i32),
        Ok(I24::from_ne_bytes([0x80, 0xFF, 0xFF]))
    );
    assert_eq!(
        I24::try_from((i8::MAX) as i32),
        Ok(I24::from_ne_bytes([0x7F, 0x00, 0x00]))
    );
    assert_eq!(
        I24::try_from((i16::MIN) as i32),
        Ok(I24::from_ne_bytes([0x00, 0x80, 0xFF]))
    );
    assert_eq!(
        I24::try_from((i16::MAX) as i32),
        Ok(I24::from_ne_bytes([0xFF, 0x7F, 0x00]))
    );
    assert_eq!(I24::try_from(8388607), Ok(I24::MAX));
    assert_eq!(I24::try_from(-8388608), Ok(I24::MIN));
    assert_eq!(
        I24::try_from(i32::from_le_bytes([0x00, 0x00, 0x80, 0xFF])),
        Ok(I24::MIN)
    );
    assert_eq!(
        I24::try_from(i32::from_le_bytes([0x00, 0x00, 0x80, 0x00])),
        Err(TryFromIntError(()))
    );
    assert_eq!(
        I24::try_from(i32::from_le_bytes([0xFF, 0xFF, 0x7F, 0x00])),
        Ok(I24::MAX)
    );
    assert_eq!(
        I24::try_from(i32::from_le_bytes([0xFF, 0xFF, 0x7F, 0xFF])),
        Err(TryFromIntError(()))
    );
}

#[test]
fn it_works() {
    assert_eq!(0x0123_4567_i32.to_le_bytes(), [0x67, 0x45, 0x23, 0x01]);
    assert_eq!(0x0123_4567_i32.to_be_bytes(), [0x01, 0x23, 0x45, 0x67]);
    assert_eq!(0x0123_4567_i32.to_ne_bytes(), [0x67, 0x45, 0x23, 0x01]); // le

    assert_eq!(1_i32.to_le_bytes(), [0x01, 0x00, 0x00, 0x00]);
    assert_eq!(1_i32.to_be_bytes(), [0x00, 0x00, 0x00, 0x01]);
    assert_eq!((-(1_i32)).to_le_bytes(), [0xFF, 0xFF, 0xFF, 0xFF]);
    assert_eq!((-(1_i32)).to_be_bytes(), [0xFF, 0xFF, 0xFF, 0xFF]);
    assert_eq!(8388607_i32.to_le_bytes(), [0xFF, 0xFF, 0x7F, 0x00]);
    assert_eq!(8388607_i32.to_be_bytes(), [0x00, 0x7F, 0xFF, 0xFF]);
    assert_eq!((-(8388608_i32)).to_le_bytes(), [0x00, 0x00, 0x80, 0xFF]);
    assert_eq!((-(8388608_i32)).to_be_bytes(), [0xFF, 0x80, 0x00, 0x00]);
    assert_eq!((-(42_i32)).to_be_bytes(), [0xFF, 0xFF, 0xFF, 214]);
}

mod size_and_align_of {
    macro_rules! test_size_of {
        ( $( size_of($ty:ident) == $num_bytes:literal );* $(;)? ) => {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn $ty() {
                    ::core::assert_eq!(::core::mem::align_of::<$crate::$ty>(), 1);
                    ::core::assert_eq!(::core::mem::size_of::<$crate::$ty>(), $num_bytes);
                }
            )*
        };
    }
    test_size_of!(
        size_of(I16) == 2;
        size_of(U16) == 2;
        size_of(I24) == 3;
        size_of(U24) == 3;
        size_of(I32) == 4;
        size_of(U32) == 4;
        size_of(I40) == 5;
        size_of(U40) == 5;
        size_of(I48) == 6;
        size_of(U48) == 6;
        size_of(I56) == 7;
        size_of(U56) == 7;
        size_of(I64) == 8;
        size_of(U64) == 8;
        size_of(I72) == 9;
        size_of(U72) == 9;
        size_of(I80) == 10;
        size_of(U80) == 10;
        size_of(I88) == 11;
        size_of(U88) == 11;
        size_of(U96) == 12;
        size_of(I96) == 12;
        size_of(U104) == 13;
        size_of(I104) == 13;
        size_of(U112) == 14;
        size_of(I112) == 14;
        size_of(I120) == 15;
        size_of(U120) == 15;
        size_of(I128) == 16;
        size_of(U128) == 16;
    );
}
