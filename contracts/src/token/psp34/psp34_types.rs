// SPDX-License-Identifier: MIT
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum Id {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Bytes(Vec<u8>),
}

impl Default for Id {
    fn default() -> Self {
        Self::U8(0)
    }
}
