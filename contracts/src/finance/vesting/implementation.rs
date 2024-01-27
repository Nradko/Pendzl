// SPDX-License-Identifier: MIT
use ink::{
    env::{
        call::{build_call, ExecutionInput},
        DefaultEnvironment,
    },
    prelude::vec,
    prelude::vec::*,
    storage::Mapping,
};
use pendzl::{
    math::errors::MathError::Overflow,
    traits::{AccountId, Balance, DefaultEnv, Storage, Timestamp},
};

use crate::token::psp22::{PSP22Ref, PSP22};

use super::{
    TokenReleased, VestingError, VestingInternal, VestingScheduleData, VestingScheduled,
    VestingStorage, VestingTimeConstraint, VestingTimeConstraintData,
};

impl VestingScheduleData {
    pub fn collect_releasable_rdown(&mut self) -> Result<Balance, VestingError> {
        let amount_releaseable = self.amount_releaseable_rdown()?;
        self.released += amount_releaseable;
        Ok(amount_releaseable)
    }
    pub fn amount_releaseable_rdown(&self) -> Result<Balance, VestingError> {
        let now: Timestamp = Self::env().block_timestamp();
        let start_time = self._extract_timestamp_from_constraint(&self.start)?;
        let end_time = self._extract_timestamp_from_constraint(&self.end)?;
        let is_overdue = self.is_overdue()?;
        if is_overdue {
            return Ok(self.amount - self.released);
        }
        if now < start_time || start_time == end_time {
            return Ok(0);
        }
        let total_to_release = self.amount * u128::try_from(now - start_time).unwrap()
            / u128::try_from(end_time - start_time).unwrap()
            - 1; //TODO ??
        let amount_releaseable = total_to_release - self.released;
        Ok(amount_releaseable)
    }
    pub fn is_overdue(&self) -> Result<bool, VestingError> {
        let now: Timestamp = Self::env().block_timestamp();
        let end_time = self._extract_timestamp_from_constraint(&self.end)?;
        Ok(now >= end_time)
    }
    fn _extract_timestamp_from_constraint(
        &self,
        constraint: &VestingTimeConstraintData,
    ) -> Result<Timestamp, VestingError> {
        match constraint.fetch_value_data {
            VestingTimeConstraint::Default(timestamp) => Ok(timestamp),
            VestingTimeConstraint::External(account_id, selector_bytes) => {
                let call_result = build_call::<DefaultEnvironment>()
                    .call(account_id)
                    .exec_input(ExecutionInput::new(ink::env::call::Selector::new(
                        selector_bytes,
                    )))
                    .returns::<Timestamp>()
                    .try_invoke();

                match call_result {
                    Ok(timestamp) => match timestamp {
                        Ok(timestamp) => Ok(timestamp),
                        Err(_) => Ok(constraint.current_value),
                    },
                    Err(_) => Ok(constraint.current_value),
                }
            }
        }
    }
}

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct VestingData {
    schedules: Mapping<(AccountId, Option<AccountId>, u32), VestingScheduleData>,
    next_id: Mapping<(AccountId, Option<AccountId>), u32>,
}

impl VestingStorage for VestingData {
    fn create(
        &mut self,
        to: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        vesting_start: VestingTimeConstraint,
        vesting_end: VestingTimeConstraint,
        _data: &Vec<u8>,
    ) -> Result<(), VestingError> {
        let id = self.next_id.get((to, asset)).unwrap_or(0);
        let current_start = match vesting_start {
            VestingTimeConstraint::Default(timestamp) => Ok(timestamp),
            VestingTimeConstraint::External(account_id, selector_bytes) => {
                let call_result = build_call::<DefaultEnvironment>()
                    .call(account_id)
                    .exec_input(ExecutionInput::new(ink::env::call::Selector::new(
                        selector_bytes,
                    )))
                    .returns::<Timestamp>()
                    .try_invoke();

                match call_result {
                    Ok(timestamp) => match timestamp {
                        Ok(timestamp) => Ok(timestamp),
                        Err(_) => Err(VestingError::CouldNotResolveTimeConstraint),
                    },
                    Err(_) => Err(VestingError::CouldNotResolveTimeConstraint),
                }
            }
        }?;
        let current_end = match vesting_end {
            VestingTimeConstraint::Default(timestamp) => Ok(timestamp),
            VestingTimeConstraint::External(account_id, selector_bytes) => {
                let call_result = build_call::<DefaultEnvironment>()
                    .call(account_id)
                    .exec_input(ExecutionInput::new(ink::env::call::Selector::new(
                        selector_bytes,
                    )))
                    .returns::<Timestamp>()
                    .try_invoke();

                match call_result {
                    Ok(timestamp) => match timestamp {
                        Ok(timestamp) => Ok(timestamp),
                        Err(_) => Err(VestingError::CouldNotResolveTimeConstraint),
                    },
                    Err(_) => Err(VestingError::CouldNotResolveTimeConstraint),
                }
            }
        }?;

        self.schedules.insert(
            (to, asset, id),
            &VestingScheduleData {
                start: VestingTimeConstraintData {
                    current_value: current_start,
                    fetch_value_data: vesting_start.clone(),
                },
                end: VestingTimeConstraintData {
                    current_value: current_end,
                    fetch_value_data: vesting_end.clone(),
                },
                amount,
                released: 0,
            },
        );

        self.next_id.insert(
            (to, asset),
            &(id.checked_add(1).ok_or(VestingError::MathError(Overflow))?),
        );
        Ok(())
    }

    fn release(
        &mut self,
        to: AccountId,
        asset: Option<AccountId>,
        data: &Vec<u8>,
    ) -> Result<Balance, VestingError> {
        let next_id = self.next_id.get((to, asset)).unwrap_or(0);
        let mut tail_id = next_id - 1;
        let mut current_id = 0;
        let mut total_amount = 0;
        while current_id <= tail_id {
            let (was_swapped_for_tail, amount_released) =
                self.release_by_vest_id(to, asset, current_id, &data)?;
            total_amount += amount_released;
            if was_swapped_for_tail {
                tail_id = tail_id.saturating_sub(1);
                continue;
            }
            current_id = current_id
                .checked_add(1)
                .ok_or(VestingError::MathError(Overflow))?;
        }
        Ok(total_amount)
    }

    fn release_by_vest_id(
        &mut self,
        to: AccountId,
        asset: Option<AccountId>,
        id: u32,
        _data: &Vec<u8>,
    ) -> Result<(bool, Balance), VestingError> {
        let mut data = match self.schedules.get(&(to, asset, id)) {
            Some(data) => data,
            None => return Ok((false, 0)),
        };
        let amount_released = data.collect_releasable_rdown()?;
        if data.is_overdue()? {
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
        _data: &Vec<u8>,
    ) -> Option<VestingScheduleData> {
        self.schedules.get(&(to, asset, id))
    }
}

pub trait VestingDefaultImpl: VestingInternal + Sized {
    fn create_vest_default_impl(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        vesting_start: VestingTimeConstraint,
        vesting_end: VestingTimeConstraint,
        data: Vec<u8>,
    ) -> Result<(), VestingError> {
        self._create_vest(receiver, asset, amount, vesting_start, vesting_end, &data)
    }

    fn release_default_impl(
        &mut self,
        receiver: Option<AccountId>,
        asset: Option<AccountId>,
        data: Vec<u8>,
    ) -> Result<(), VestingError> {
        self._release(receiver, asset, &data)
    }
    fn release_by_vest_id_default_impl(
        &mut self,
        receiver: Option<AccountId>,
        asset: Option<AccountId>,
        id: u32,
        data: Vec<u8>,
    ) -> Result<(), VestingError> {
        self._release_by_vest_id(receiver, asset, id, &data)
    }

    fn vesting_schedule_of_default_impl(
        &self,
        of: AccountId,
        asset: Option<AccountId>,
        id: u32,
        data: Vec<u8>,
    ) -> Option<VestingScheduleData> {
        self._vesting_schedule_of(of, asset, id, &data)
    }
    fn next_id_vest_of_default_impl(
        &self,
        of: AccountId,
        asset: Option<AccountId>,
        data: Vec<u8>,
    ) -> u32 {
        self._next_id_vest_of(of, asset, &data)
    }
}

pub trait VestingInternalDefaultImpl: Storage<VestingData> + VestingInternal
where
    VestingData: VestingStorage,
{
    fn _create_vest_default_impl(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        vesting_start: VestingTimeConstraint,
        vesting_end: VestingTimeConstraint,
        data: &Vec<u8>,
    ) -> Result<(), VestingError> {
        let creator = Self::env().caller();
        self._handle_transfer_in(asset, creator, amount, data)?;
        self.data().create(
            receiver,
            asset,
            amount,
            vesting_start.clone(),
            vesting_end.clone(),
            data,
        )?;
        Self::env().emit_event(VestingScheduled {
            creator,
            receiver,
            asset,
            amount,
            vesting_start: vesting_start.clone(),
            vesting_end: vesting_end.clone(),
        });
        Ok(())
    }

    fn _release_default_impl(
        &mut self,
        receiver: Option<AccountId>,
        asset: Option<AccountId>,
        data: &Vec<u8>,
    ) -> Result<(), VestingError> {
        let caller = Self::env().caller();
        let receiver = receiver.unwrap_or(caller);
        let amount_released = self.data().release(receiver, asset, &data)?;
        self._handle_transfer_out(asset, receiver, amount_released, &data)?;
        Self::env().emit_event(TokenReleased {
            caller,
            asset,
            to: receiver,
            amount: amount_released,
        });
        Ok(())
    }
    fn _release_by_vest_id_default_impl(
        &mut self,
        receiver: Option<AccountId>,
        asset: Option<AccountId>,
        id: u32,
        data: &Vec<u8>,
    ) -> Result<(), VestingError> {
        let caller = Self::env().caller();
        let receiver = receiver.unwrap_or(caller);
        let (_, amount_released) = self.data().release_by_vest_id(receiver, asset, id, &data)?;
        self._handle_transfer_out(asset, receiver, amount_released, &data)?;
        Self::env().emit_event(TokenReleased {
            caller,
            asset,
            to: receiver,
            amount: amount_released,
        });
        Ok(())
    }

    fn _handle_transfer_out_default_impl(
        &mut self,
        asset: Option<AccountId>,
        to: AccountId,
        amount: Balance,
        _data: &Vec<u8>,
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
        _data: &Vec<u8>,
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

    fn _vesting_schedule_of_default_impl(
        &self,
        of: AccountId,
        asset: Option<AccountId>,
        id: u32,
        _data: &Vec<u8>,
    ) -> Option<VestingScheduleData> {
        self.data().schedules.get(&(of, asset, id))
    }

    fn _next_id_vest_of_default_impl(
        &self,
        of: AccountId,
        asset: Option<AccountId>,
        _data: &Vec<u8>,
    ) -> u32 {
        self.data().next_id.get((of, asset)).unwrap_or(0)
    }
}
