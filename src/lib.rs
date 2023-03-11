use core::array::TryFromSliceError;

pub use binary_proc::*;

#[derive(Debug)]
pub struct Le<T> {
    pub raw: T,
}
#[derive(Debug)]
pub struct Be<T> {
    pub raw: T,
}
macro_rules! frome_le {
    ($($ty:ty),+) => {
        $(
            impl TryFrom<&[u8]> for Le<$ty> {
                type Error = TryFromSliceError;
                fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                    let bytes = value.try_into()?;
                    Ok(Self {
                        raw: <$ty>::from_le_bytes(bytes),
                    })
                }
            }
        )+
    };
}
macro_rules! frome_be {
    ($($ty:ty),+) => {
        $(
            impl TryFrom<&[u8]> for Be<$ty> {
                type Error = TryFromSliceError;
                fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                    let bytes = value.try_into()?;
                    Ok(Self {
                        raw: <$ty>::from_be_bytes(bytes),
                    })
                }
            }
        )+
    };
}

frome_le!(u8, u16, u32, u64);
frome_be!(u8, u16, u32, u64);

#[cfg(test)]
mod test {
    use crate::Le;

    #[test]
    fn test_try_from() {
        let input = [0, 1, 2, 3];
        let le32 = Le::<u32>::try_from(input.as_ref());
        println!("{:#?}", le32);
    }
}
