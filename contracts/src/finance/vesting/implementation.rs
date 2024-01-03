use ink::{prelude::vec, storage::Mapping};
use pendzl::traits::{AccountId, Balance, Storage, Timestamp};

use crate::{
    finance::vesting::VestingSchedule,
    token::psp22::{PSP22Ref, PSP22},
};

use super::{TokenReleased, VestingError, VestingInternal, VestingScheduled, VestingStorage};

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct Data {
    schedules: Mapping<(AccountId, Option<AccountId>, u32), VestingSchedule>,
    next_id: Mapping<(AccountId, Option<AccountId>), u32>,
}

impl VestingStorage for Data {
    fn create(
        &mut self,
        to: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        vesting_start: Timestamp,
        vesting_end: Timestamp,
    ) -> Result<(), VestingError> {
        let id = self.next_id.get((to, asset)).unwrap_or(0);
        self.schedules.insert(
            (to, asset, id),
            &VestingSchedule {
                start: vesting_start,
                end: vesting_end,
                amount,
                released: 0,
            },
        );
        self.next_id.insert((to, asset), &(id + 1));
        Ok(())
    }

    fn release(
        &mut self,
        to: AccountId,
        asset: Option<AccountId>,
    ) -> Result<Balance, VestingError> {
        let next_id = self.next_id.get((to, asset)).unwrap_or(0);
        let mut tail_id = next_id - 1;
        let mut current_id = 0;
        let mut total_amount = 0;
        while current_id <= tail_id {
            let (was_swapped_for_tail, amount_released) =
                self.release_by_vest_id(to, asset, current_id)?;
            total_amount += amount_released;
            if was_swapped_for_tail {
                tail_id = tail_id.saturating_sub(1);
                continue;
            }
            current_id += 1;
        }
        Ok(total_amount)
    }

    fn release_by_vest_id(
        &mut self,
        to: AccountId,
        asset: Option<AccountId>,
        id: u32,
    ) -> Result<(bool, Balance), VestingError> {
        let mut data = match self.schedules.get(&(to, asset, id)) {
            Some(data) => data,
            None => return Ok((false, 0)),
        };
        let amount_released = data.collect_releasable_rdown();
        if data.is_overdue() {
            let leftover = data.amount - data.released;
            let next_id = self.next_id.get((to, asset)).unwrap(); // data is some => next_id must exist and be > 0
            let tail_id = next_id - 1;
            let tail = self
                .schedules
                .get(&(to, asset, tail_id))
                .ok_or(VestingError::InvalidScheduleKey)?;
            self.schedules.remove(&(to, asset, tail_id));
            if tail_id != id {
                self.schedules.insert((to, asset, id), &tail);
            }
            self.next_id.insert((to, asset), &(tail_id));
            return Ok((true, amount_released + leftover));
        }
        self.schedules.insert((to, asset, id), &data);
        Ok((false, amount_released))
    }

    fn get_schedule_by_id(
        &self,
        to: AccountId,
        asset: Option<AccountId>,
        id: u32,
    ) -> Option<VestingSchedule> {
        self.schedules.get(&(to, asset, id))
    }
}

pub trait VestingDefaultImpl: VestingInternal + Sized {
    fn create_vest_default_impl(
        &mut self,
        to: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        vesting_start: Timestamp,
        vesting_end: Timestamp,
    ) -> Result<(), VestingError> {
        self._create_vest(to, asset, amount, vesting_start, vesting_end)
    }

    fn release_default_impl(&mut self, asset: Option<AccountId>) -> Result<(), VestingError> {
        self._release(asset)
    }
    fn release_by_vest_id_default_impl(
        &mut self,
        asset: Option<AccountId>,
        id: u32,
    ) -> Result<(), VestingError> {
        self._release_by_vest_id(asset, id)
    }

    // fn vest_of(
    //     &mut self,
    //     of: AccountId,
    //     asset: Option<AccountId>,
    //     id: u32,
    // ) -> Option<VestingSchedule> {
    //     self._vest_of(of, asset, id)
    // }
    fn next_id_vest_of_default_impl(&self, of: AccountId, asset: Option<AccountId>) -> u32 {
        self._next_id_vest_of(of, asset)
    }
}

pub trait VestingInternalDefaultImpl: Storage<Data> + VestingInternal
where
    Data: VestingStorage,
{
    fn _create_vest_default_impl(
        &mut self,
        to: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        vesting_start: Timestamp,
        vesting_end: Timestamp,
    ) -> Result<(), VestingError> {
        let from = Self::env().caller();
        self._handle_transfer_in(asset, from, amount)?;
        self.data()
            .create(to, asset, amount, vesting_start, vesting_end)?;
        Self::env().emit_event(VestingScheduled {
            to,
            asset,
            amount,
            vesting_start,
            vesting_end,
        });
        Ok(())
    }

    fn _release_default_impl(&mut self, asset: Option<AccountId>) -> Result<(), VestingError> {
        let to = Self::env().caller();
        let amount_released = self.data().release(to, asset)?;
        self._handle_transfer_out(asset, to, amount_released)?;
        Self::env().emit_event(TokenReleased {
            asset,
            to,
            amount: amount_released,
        });
        Ok(())
    }
    fn _release_by_vest_id_default_impl(
        &mut self,
        asset: Option<AccountId>,
        id: u32,
    ) -> Result<(), VestingError> {
        let to = Self::env().caller();
        let (_, amount_released) = self.data().release_by_vest_id(to, asset, id)?;
        self._handle_transfer_out(asset, to, amount_released)?;
        Self::env().emit_event(TokenReleased {
            asset,
            to,
            amount: amount_released,
        });
        Ok(())
    }

    fn _handle_transfer_out_default_impl(
        &mut self,
        asset: Option<AccountId>,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), VestingError> {
        match asset {
            Some(asset) => {
                let mut psp22: PSP22Ref = asset.into();
                psp22.transfer(to, amount as Balance, vec![])?
            }
            None => match Self::env().transfer(to, amount) {
                Ok(_) => {}
                Err(_) => return Err(VestingError::NativeTransferFailed),
            },
        }
        Ok(())
    }
    fn _handle_transfer_in_default_impl(
        &mut self,
        asset: Option<AccountId>,
        from: AccountId,
        amount: Balance,
    ) -> Result<(), VestingError> {
        match asset {
            Some(asset) => {
                let mut psp22: PSP22Ref = asset.into();
                let to = Self::env().account_id();
                psp22.transfer_from(from, to, amount as Balance, vec![])?
            }
            None => {
                if Self::env().transferred_value() != amount {
                    return Err(VestingError::InvalidAmountPaid);
                }
            }
        }
        Ok(())
    }

    // fn _vest_of_default_impl(
    //     &self,
    //     of: AccountId,
    //     asset: Option<AccountId>,
    //     id: u32,
    // ) -> Option<VestingSchedule> {
    //     self.data().schedules.get(&(of, asset, id))
    // }

    fn _next_id_vest_of_default_impl(&self, of: AccountId, asset: Option<AccountId>) -> u32 {
        self.data().next_id.get((of, asset)).unwrap_or(0)
    }
}
