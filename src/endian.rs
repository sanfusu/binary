use core::array::TryFromSliceError;

/// 小端法编码的 T （字节序）
/// 小端法只会在 to_bytes 和 from_bytes 类似的操作时才会体现
/// Le::0 始终为 native endian
#[derive(Debug)]
#[repr(transparent)]
pub struct Le<T>(pub T);

/// 大端法编码的 T （字节序）
/// 大端法只会在 to_bytes 和 from_bytes 类似的操作时才会体现
/// Be::0 始终为 native endian
#[derive(Debug)]
#[repr(transparent)]
pub struct Be<T>(pub T);

pub trait Endian: Sized {
    type Error;
    fn try_from_le_bytes(bytes: &[u8]) -> Result<Self, Self::Error>;
    fn try_from_be_bytes(bytes: &[u8]) -> Result<Self, Self::Error>;
}

impl<T: Endian> TryFrom<&[u8]> for Le<T> {
    type Error = T::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Le(T::try_from_le_bytes(value)?))
    }
}
impl<T: Endian> TryFrom<&[u8]> for Be<T> {
    type Error = T::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Be(T::try_from_be_bytes(value)?))
    }
}

macro_rules! impl_endian {
    ($($ty:ty),+) => {
        $(
            impl Endian for $ty {
                type Error=TryFromSliceError;
                fn try_from_le_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
                    let bytes = value.try_into()?;
                    Ok(
                        <$ty>::from_le_bytes(bytes)
                    )
                }
                fn try_from_be_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
                    let bytes = value.try_into()?;
                    Ok(
                        <$ty>::from_be_bytes(bytes)
                    )
                }
            }
        )+
    };
}

impl_endian!(u8, u16, u32, u64, u128);
impl_endian!(i8, i16, i32, i64, i128);
