pub use pendzl::traits::Balance;

use ink::primitives::AccountId;
use pendzl::traits::Timestamp;

#[ink::trait_definition]
pub trait Vesting {
    #[ink(message, payable)]
    fn create_vest(
        &mut self,
        to: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        vesting_start: Timestamp,
        vesting_end: Timestamp,
    ) -> Result<(), VestingError>;
    #[ink(message)]
    fn release(&mut self, asset: Option<AccountId>) -> Result<(), VestingError>;
    #[ink(message)]
    fn release_by_vest_id(&mut self, asset: Option<AccountId>, id: u32)
        -> Result<(), VestingError>;
    // #[ink(message)]
    // fn vest_of(
    //     &mut self,
    //     of: AccountId,
    //     asset: Option<AccountId>,
    //     id: u32,
    // ) -> Option<VestingSchedule>;
    #[ink(message)]
    fn next_id_vest_of(&self, of: AccountId, asset: Option<AccountId>) -> u32;
}

pub trait VestingInternal {
    fn _create_vest(
        &mut self,
        to: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        vesting_start: Timestamp,
        vesting_end: Timestamp,
    ) -> Result<(), VestingError>;

    fn _release(&mut self, asset: Option<AccountId>) -> Result<(), VestingError>;

    fn _release_by_vest_id(
        &mut self,
        asset: Option<AccountId>,
        id: u32,
    ) -> Result<(), VestingError>;
    fn _handle_transfer_in(
        &mut self,
        asset: Option<AccountId>,
        from: AccountId,
        amount: Balance,
    ) -> Result<(), VestingError>;
    fn _handle_transfer_out(
        &mut self,
        asset: Option<AccountId>,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), VestingError>;

    // fn _vest_of(
    //     &mut self,
    //     of: AccountId,
    //     asset: Option<AccountId>,
    //     id: u32,
    // ) -> Option<VestingSchedule>;
    fn _next_id_vest_of(&self, of: AccountId, asset: Option<AccountId>) -> u32;
}
/// supports adding element and identifying it by some id/ iterating over all elements, removing element by id
pub trait VestingStorage {
    fn create(
        &mut self,
        to: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        vesting_start: Timestamp,
        vesting_end: Timestamp,
    ) -> Result<(), VestingError>;

    fn release(&mut self, to: AccountId, asset: Option<AccountId>)
        -> Result<Balance, VestingError>;

    fn release_by_vest_id(
        &mut self,
        to: AccountId,
        asset: Option<AccountId>,
        id: u32,
    ) -> Result<(bool, Balance), VestingError>;
}
