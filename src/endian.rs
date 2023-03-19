use core::array::TryFromSliceError;
use std::mem::size_of;

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

pub struct EndianIter<T: Sized> {
    data: T, // 此时必须是对于的 endian 布局
    idx: usize,
}

impl<T> Iterator for EndianIter<T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= size_of::<T>() {
            return None;
        } else {
            let ptr = (&self.data) as *const T as *const u8;
            let byte = unsafe { Some(ptr.offset(self.idx as isize).read()) };
            self.idx += 1;
            return byte;
        }
    }
}

macro_rules! impl_into_iterator {
    ($($ty:ty),+) => {
        $(
            impl IntoIterator for Le<$ty> {
                type Item = u8;
                type IntoIter = EndianIter<$ty>;
                fn into_iter(self)->Self::IntoIter{
                    EndianIter {
                        data: self.0.to_le(),
                        idx:0,
                    }
                }
            }
            impl IntoIterator for Be<$ty> {
                type Item = u8;
                type IntoIter = EndianIter<$ty>;
                fn into_iter(self)->Self::IntoIter{
                    EndianIter {
                        data: self.0.to_be(),
                        idx:0,
                    }
                }
            }
        )+
    };
}

impl_into_iterator!(u8, u16, u32, u64, u128);

macro_rules! impl_try_from_endian_slice {
    ($($ty:ty),+) => {
        $(
            impl TryFrom<Le<&[u8]>> for $ty {
                type Error = TryFromSliceError;

                fn try_from(value: Le<&[u8]>) -> Result<Self, Self::Error> {
                    Ok(<$ty>::from_le_bytes(value.0.try_into()?))
                }
            }
            impl TryFrom<Be<&[u8]>> for $ty {
                type Error = TryFromSliceError;

                fn try_from(value: Be<&[u8]>) -> Result<Self, Self::Error> {
                    Ok(<$ty>::from_be_bytes(value.0.try_into()?))
                }
            }
        )+
    };
}
impl_try_from_endian_slice!(u8, u16, u32, u64, u128);
