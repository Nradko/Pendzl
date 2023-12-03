// SPDX-License-Identifier: MIT
#[ink::event]
pub struct OwnershipTransferred {
    #[ink(topic)]
    new: Option<AccountId>,
}
