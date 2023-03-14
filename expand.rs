#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub use binary_proc::*;
pub mod endian {
    use core::array::TryFromSliceError;
    /// 小端法编码的 T （字节序）
    /// 小端法只会在 to_bytes 和 from_bytes 类似的操作时才会体现
    /// Le::0 始终为 native endian
    pub struct Le<T>(T);
    #[automatically_derived]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Le<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Le", &&self.0)
        }
    }
    /// 大端法编码的 T （字节序）
    /// 大端法只会在 to_bytes 和 from_bytes 类似的操作时才会体现
    /// Be::0 始终为 native endian
    pub struct Be<T>(T);
    #[automatically_derived]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Be<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Be", &&self.0)
        }
    }
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
    impl Endian for u8 {
        type Error = TryFromSliceError;
        fn try_from_le_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<u8>::from_le_bytes(bytes))
        }
        fn try_from_be_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<u8>::from_be_bytes(bytes))
        }
    }
    impl Endian for u32 {
        type Error = TryFromSliceError;
        fn try_from_le_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<u32>::from_le_bytes(bytes))
        }
        fn try_from_be_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<u32>::from_be_bytes(bytes))
        }
    }
    impl Endian for u64 {
        type Error = TryFromSliceError;
        fn try_from_le_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<u64>::from_le_bytes(bytes))
        }
        fn try_from_be_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<u64>::from_be_bytes(bytes))
        }
    }
    impl Endian for u128 {
        type Error = TryFromSliceError;
        fn try_from_le_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<u128>::from_le_bytes(bytes))
        }
        fn try_from_be_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<u128>::from_be_bytes(bytes))
        }
    }
    impl Endian for i8 {
        type Error = TryFromSliceError;
        fn try_from_le_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<i8>::from_le_bytes(bytes))
        }
        fn try_from_be_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<i8>::from_be_bytes(bytes))
        }
    }
    impl Endian for i32 {
        type Error = TryFromSliceError;
        fn try_from_le_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<i32>::from_le_bytes(bytes))
        }
        fn try_from_be_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<i32>::from_be_bytes(bytes))
        }
    }
    impl Endian for i64 {
        type Error = TryFromSliceError;
        fn try_from_le_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<i64>::from_le_bytes(bytes))
        }
        fn try_from_be_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<i64>::from_be_bytes(bytes))
        }
    }
    impl Endian for i128 {
        type Error = TryFromSliceError;
        fn try_from_le_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<i128>::from_le_bytes(bytes))
        }
        fn try_from_be_bytes(value: &[u8]) -> Result<Self, TryFromSliceError> {
            let bytes = value.try_into()?;
            Ok(<i128>::from_be_bytes(bytes))
        }
    }
}
