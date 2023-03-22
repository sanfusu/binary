use core::array::TryFromSliceError;
use core::mem::size_of;
pub enum Endian<T> {
    Le(T),
    Be(T),
    Ne(T),
}

macro_rules! impl_try_from_endian_slice {
    ($($ty:ty),+) => {
        $(
            impl TryFrom<Endian<&[u8]>> for $ty {
                type Error = TryFromSliceError;

                fn try_from(value: Endian<&[u8]>) -> Result<Self, Self::Error> {
                    match value {
                        Endian::Le(x) => {
                            Ok(<$ty>::from_le_bytes(x.try_into()?))
                        },
                        Endian::Be(x) => {
                            Ok(<$ty>::from_be_bytes(x.try_into()?))
                        },
                        Endian::Ne(x) => {
                            Ok(<$ty>::from_ne_bytes(x.try_into()?))
                        },
                    }
                }
            }
        )+
    };
}
impl_try_from_endian_slice!(u8, u16, u32, u64, u128);

pub struct IntgerEndianIter<T: Sized> {
    data: T, // 此时必须是对于的 endian 布局
    idx: usize,
}

impl<T> Iterator for IntgerEndianIter<T> {
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
            impl IntoIterator for Endian<$ty> {
                type Item = u8;

                type IntoIter = IntgerEndianIter<$ty>;

                fn into_iter(self) -> Self::IntoIter {
                    match self {
                     Endian::Le(x) => IntgerEndianIter {
                            data: x.to_le(),
                            idx: 0,
                        },
                     Endian::Be(x) => IntgerEndianIter {
                            data: x.to_be(),
                            idx: 0,
                        },
                     Endian::Ne(x) => IntgerEndianIter {
                            data: x,
                            idx: 0,
                        },
                    }
                }
            }
        )+
    };
}

impl_into_iterator!(u8, u16, u32, u64, u128);
