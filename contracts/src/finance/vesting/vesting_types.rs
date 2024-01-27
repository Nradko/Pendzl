// SPDX-License-Identifier: MIT

use ink::env::DefaultEnvironment;
use pendzl::traits::Timestamp;
use scale::{Decode, Encode};

pub type SelectorBytes = [u8; 4];

#[derive(Debug, Encode, PartialEq, Eq, Decode, Clone)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum VestingTimeConstraint {
    Default(Timestamp),
    External(AccountId, SelectorBytes),
}

#[derive(Debug, Encode, Decode, Clone)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct VestingSchedule {
    pub start: VestingTimeConstraint,
    pub end: VestingTimeConstraint,
    pub amount: Balance,
    pub released: Balance,
}

#[derive(Debug, Encode, Decode, Clone)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct VestingTimeConstraintData {
    pub current_value: Timestamp,
    pub fetch_value_data: VestingTimeConstraint,
}

#[derive(Debug, Encode, Decode, Clone)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct VestingScheduleData {
    pub start: VestingTimeConstraintData,
    pub end: VestingTimeConstraintData,
    pub amount: Balance,
    pub released: Balance,
}
