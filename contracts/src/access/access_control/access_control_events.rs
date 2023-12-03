// SPDX-License-Identifier: MIT
#[ink::event]
pub struct RoleAdminChanged {
    #[ink(topic)]
    role: RoleType,
    previous: RoleType,
    new: RoleType,
}

#[ink::event]
pub struct RoleGranted {
    #[ink(topic)]
    role: RoleType,
    #[ink(topic)]
    grantee: Option<AccountId>,
    #[ink(topic)]
    grantor: Option<AccountId>,
}

#[ink::event]
pub struct RoleRevoked {
    #[ink(topic)]
    role: RoleType,
    #[ink(topic)]
    account: Option<AccountId>,
    #[ink(topic)]
    sender: AccountId,
}
