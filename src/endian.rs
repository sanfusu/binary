use core::array::TryFromSliceError;
use core::mem::size_of;

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

pub trait IntoLeIter {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_leiter(self) -> Self::IntoIter;
}
pub trait IntoBeIter {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_beiter(self) -> Self::IntoIter;
}

impl<T: IntoLeIter> IntoIterator for Le<T> {
    type Item = T::Item;

    type IntoIter = T::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_leiter()
    }
}

impl<T: IntoBeIter> IntoIterator for Be<T> {
    type Item = T::Item;

    type IntoIter = T::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_beiter()
    }
}

pub struct IntegerEndianIter<T: Sized> {
    data: T, // 此时必须是对于的 endian 布局
    idx: usize,
}

impl<T> Iterator for IntegerEndianIter<T> {
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
            impl IntoLeIter for $ty {
                type Item = u8;

                type IntoIter = IntegerEndianIter<$ty>;

                fn into_leiter(self) -> Self::IntoIter {
                    IntegerEndianIter {
                        data: self.to_le(),
                        idx: 0,
                    }
                }
            }
            impl IntoBeIter for $ty {
                type Item = u8;

                type IntoIter = IntegerEndianIter<$ty>;

                fn into_beiter(self) -> Self::IntoIter {
                    IntegerEndianIter {
                        data: self.to_be(),
                        idx: 0,
                    }
                }
            }
        )+
    };
}

impl_into_iterator!(u8, u16, u32, u64, u128);
