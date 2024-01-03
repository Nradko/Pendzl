// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(Vesting)]
#[ink::contract]
pub mod vester {
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Vester {
        #[storage_field]
        vesting: VestingData,
    }
    #[ink::event]
    #[derive(Debug)]
    pub struct TransferMock {
        /// The account from which the tokens are transferred.
        #[ink(topic)]
        pub from: AccountId,
        /// The account to which the tokens are transferred.
        #[ink(topic)]
        pub to: AccountId,
        /// The amount of tokens transferred.
        pub amount: Balance,
        /// The asset transferred. None for native token.
        pub asset: Option<AccountId>,
    }

    impl Vester {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        #[ink(message)]
        pub fn get_schedule_by_id(
            &self,
            of: AccountId,
            asset: Option<AccountId>,
            id: u32,
        ) -> Option<VestingSchedule> {
            self.vesting.get_schedule_by_id(of, asset, id)
        }
    }
}

#[cfg(all(test, feature = "e2e-tests"))]
pub mod tests {
    use crate::vester::{VesterRef, VestingError, *};
    use ink::{
        codegen::Env,
        env::{test::EmittedEvent, DefaultEnvironment},
        scale::Decode as _,
        ToAccountId,
    };
    use ink_e2e::{events::ContractEmitted, ChainBackend, ContractsBackend};
    use my_psp22_mintable::my_psp22_mintable::{
        Contract as PSP22MintableContract, ContractRef as PSP22Ref, *,
    };
    use pendzl::{
        contracts::token::psp22::{PSP22Error, Transfer, PSP22},
        traits::{AccountId, Balance, Timestamp},
    };
    use test_helpers::{assert_eq_msg, assert_lt, keypair_to_account};

    pub const ONE_HOUR: u64 = 60 * 60 * 1000;
    pub const ONE_DAY: u64 = 24 * ONE_HOUR;

    struct CreateVestingScheduleArgs {
        vest_to: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        vesting_start: Timestamp,
        vesting_end: Timestamp,
    }

    fn create_duration_as_amount_schedule_args(
        vest_to: AccountId,
        asset: Option<AccountId>,
        vesting_start: Timestamp,
        vesting_end: Timestamp,
    ) -> CreateVestingScheduleArgs {
        let duration = vesting_end - vesting_start;
        CreateVestingScheduleArgs {
            vest_to,
            asset,
            amount: duration.into(), //1 per timestamp unit, for simplicity
            vesting_start,
            vesting_end,
        }
    }

    fn assert_token_released_event(
        event: &EmittedEvent,
        expected_asset: Option<AccountId>,
        expected_to: AccountId,
        expected_amount: Balance,
    ) {
        let TokenReleased { asset, to, amount } = <TokenReleased>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");

        assert_eq_msg!("Assets", asset, expected_asset);
        assert_eq_msg!("To", to, expected_to);
        assert_eq_msg!("Amounts", amount, expected_amount);
    }

    fn assert_vesting_scheduled_event(
        event: &ContractEmitted<DefaultEnvironment>,
        expected_asset: Option<AccountId>,
        expected_to: AccountId,
        expected_amount: Balance,
        expected_vesting_start: Timestamp,
        expected_vesting_end: Timestamp,
    ) {
        let VestingScheduled {
            asset,
            to,
            amount,
            vesting_start,
            vesting_end,
        } = <VestingScheduled>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        assert_eq_msg!("Assets", asset, expected_asset);
        assert_eq_msg!("To", to, expected_to);
        assert_eq_msg!("Amounts", amount, expected_amount);
        assert_eq_msg!("Vesting start", vesting_start, expected_vesting_start);
        assert_eq_msg!("Vesting end", vesting_end, expected_vesting_end);
    }

    fn assert_psp22_transfer_event<E: ink::env::Environment<AccountId = AccountId>>(
        event: &ContractEmitted<E>,
        expected_from: AccountId,
        expected_to: AccountId,
        expected_value: Balance,
        expected_asset: AccountId,
    ) {
        let decoded_event = <Transfer>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");

        let Transfer { from, to, value } = decoded_event;

        assert_eq_msg!("Transfer.from", from, Some(expected_from));
        assert_eq_msg!("Transfer.to", to, Some(expected_to));
        assert_eq_msg!("Transfer.value", value, expected_value);
        assert_eq_msg!("Transfer.asset", event.contract, expected_asset);
    }
    fn set_next_caller(caller: AccountId) {
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(caller);
    }

    fn set_account_balance(account: AccountId, balance: Balance) {
        ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(account, balance);
    }

    fn get_account_balance(account: AccountId) -> Balance {
        ink::env::test::get_account_balance::<DefaultEnvironment>(account)
            .expect("Cannot get account balance")
    }

    fn set_value_transferred(value: Balance) {
        ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(value);
    }

    fn set_block_timestamp(timestamp: Timestamp) {
        ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(timestamp);
    }

    type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;
    #[ink_e2e::test]
    async fn create_vesting_schedule_psp22(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let vester_creator = ink_e2e::alice();
        let vester_submitter = client
            .create_and_fund_account(&ink_e2e::alice(), 10_000_000_000_000)
            .await;
        let psp22_mintable_creator = ink_e2e::bob();
        let mut psp22_constructor = PSP22Ref::new(1_000_000);
        let mut psp22 = client
            .instantiate(
                "my_psp22_mintable",
                &psp22_mintable_creator,
                &mut psp22_constructor,
            )
            .submit()
            .await
            .expect("instantiate psp22 failed")
            .call::<Contract>();

        // create_vest args
        let create_vest_args = CreateVestingScheduleArgs {
            vest_to: keypair_to_account(&ink_e2e::charlie()),
            asset: Some(psp22.to_account_id()),
            amount: 100,
            vesting_start: 100,
            vesting_end: 101,
        };

        let mut vester_constructor = VesterRef::new();
        let mut vester = client
            .instantiate("vester", &vester_creator, &mut vester_constructor)
            .submit()
            .await
            .expect("instantiate vester failed")
            .call::<Vester>();

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.vesting_start,
                    create_vest_args.vesting_end,
                ),
            )
            .dry_run()
            .await
            .expect("create vest failed")
            .return_value();
        assert_eq!(
            create_vest_res,
            Err(VestingError::PSP22Error(PSP22Error::InsufficientAllowance))
        );

        let _ = client
            .call(
                &vester_submitter,
                &psp22.increase_allowance(vester.to_account_id(), create_vest_args.amount),
            )
            .submit()
            .await
            .expect("give allowance failed")
            .return_value();

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.vesting_start,
                    create_vest_args.vesting_end,
                ),
            )
            .dry_run()
            .await
            .expect("create vest failed")
            .return_value();
        assert_eq!(
            create_vest_res,
            Err(VestingError::PSP22Error(PSP22Error::InsufficientBalance))
        );

        let _ = client
            .call(
                &vester_creator,
                &psp22.mint(
                    keypair_to_account(&vester_submitter),
                    create_vest_args.amount,
                ),
            )
            .submit()
            .await
            .expect("mint failed");

        let balance_of_vester = client
            .call(&ink_e2e::alice(), &psp22.balance_of(vester.to_account_id()))
            .dry_run()
            .await?
            .return_value();

        let balance_of_vester_submitter = client
            .call(
                &ink_e2e::alice(),
                &psp22.balance_of(keypair_to_account(&vester_submitter)),
            )
            .dry_run()
            .await?
            .return_value();

        assert_eq!(balance_of_vester, 0);
        assert_eq!(balance_of_vester_submitter, create_vest_args.amount);

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.vesting_start,
                    create_vest_args.vesting_end,
                ),
            )
            .submit()
            .await
            .expect("create vest failed");

        let contract_emitted_events = create_vest_res.contract_emitted_events()?;
        let vester_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vester.to_account_id())
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == psp22.to_account_id())
            .collect();
        assert!(matches!(create_vest_res.return_value(), Ok(())));

        assert_psp22_transfer_event(
            &psp22_events[1].event,
            keypair_to_account(&vester_submitter),
            vester.to_account_id(),
            create_vest_args.amount,
            psp22.to_account_id(),
        );
        assert_vesting_scheduled_event(
            &vester_events[0].event,
            Some(psp22.to_account_id()),
            create_vest_args.vest_to,
            create_vest_args.amount,
            create_vest_args.vesting_start,
            create_vest_args.vesting_end,
        );

        let balance_of_vester = client
            .call(&ink_e2e::alice(), &psp22.balance_of(vester.to_account_id()))
            .dry_run()
            .await?
            .return_value();
        let balance_of_vester_submitter = client
            .call(
                &ink_e2e::alice(),
                &psp22.balance_of(keypair_to_account(&vester_submitter)),
            )
            .dry_run()
            .await?
            .return_value();

        assert_eq!(balance_of_vester, create_vest_args.amount);
        assert_eq!(balance_of_vester_submitter, 0);

        Ok(())
    }

    #[ink_e2e::test]
    async fn create_vesting_schedule_native(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let vester_creator = ink_e2e::alice();
        let vester_submitter = client
            .create_and_fund_account(&ink_e2e::alice(), 10_000_000_000_000)
            .await;

        // create_vest args
        let create_vest_args = CreateVestingScheduleArgs {
            vest_to: keypair_to_account(&ink_e2e::charlie()),
            asset: None,
            amount: 10_000_000,
            vesting_start: 100,
            vesting_end: 101,
        };

        let mut vester_constructor = VesterRef::new();
        let mut vester = client
            .instantiate("vester", &vester_creator, &mut vester_constructor)
            .submit()
            .await
            .expect("instantiate vester failed")
            .call::<Vester>();

        let balance_of_vester_before = client
            .free_balance(vester.to_account_id())
            .await
            .expect("free balance failed");
        let balance_of_vester_submitter_before = client
            .free_balance(keypair_to_account(&vester_submitter))
            .await
            .expect("free balance failed");

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.vesting_start,
                    create_vest_args.vesting_end,
                ),
            )
            .value(create_vest_args.amount - 1)
            .dry_run()
            .await
            .expect("create vest failed")
            .return_value();
        assert_eq!(create_vest_res, Err(VestingError::InvalidAmountPaid));

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.vesting_start,
                    create_vest_args.vesting_end,
                ),
            )
            .value(create_vest_args.amount + 1)
            .dry_run()
            .await
            .expect("create vest failed")
            .return_value();
        assert_eq!(create_vest_res, Err(VestingError::InvalidAmountPaid));

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.vesting_start,
                    create_vest_args.vesting_end,
                ),
            )
            .value(create_vest_args.amount)
            .submit()
            .await
            .expect("create vest failed");

        let contract_emitted_events = create_vest_res.contract_emitted_events()?;
        let vester_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vester.to_account_id())
            .collect();
        let pallet_balances_events: Vec<_> = create_vest_res
            .events
            .iter()
            .filter(|event| {
                let metadata = &event.as_ref().expect("expected event").event_metadata();
                return metadata.pallet.name() == "Balances" && metadata.variant.name == "Transfer";
            })
            .map(|e| e.unwrap())
            .collect();
        assert!(matches!(create_vest_res.return_value(), Ok(())));

        assert_eq!(pallet_balances_events.len(), 1);
        assert_vesting_scheduled_event(
            &vester_events[0].event,
            None,
            create_vest_args.vest_to,
            create_vest_args.amount,
            create_vest_args.vesting_start,
            create_vest_args.vesting_end,
        );

        let balance_of_vester_after = client
            .free_balance(vester.to_account_id())
            .await
            .expect("free balance failed");
        let balance_of_vester_submitter_after = client
            .free_balance(keypair_to_account(&vester_submitter))
            .await
            .expect("free balance failed");

        assert_eq!(
            balance_of_vester_after,
            balance_of_vester_before + create_vest_args.amount
        );
        assert_lt(
            balance_of_vester_submitter_after,
            balance_of_vester_submitter_before - create_vest_args.amount,
        );
        Ok(())
    }

    #[ink::test]
    fn release_works() {
        let accounts = ink::env::test::default_accounts::<DefaultEnvironment>();
        let vest_to = accounts.charlie;
        let vester_submitter = accounts.bob;
        let mut vester = Vester::new();

        set_next_caller(vester_submitter);
        let init_timestamp = vester.env().block_timestamp();

        let create_vest_args = create_duration_as_amount_schedule_args(
            vest_to,
            None,
            init_timestamp + ONE_DAY * 3,
            init_timestamp + ONE_DAY * 9,
        );
        let starting_balance = 100_000;
        set_account_balance(vest_to, starting_balance);
        set_account_balance(
            vester.env().account_id(),
            create_vest_args.amount + starting_balance,
        ); // why transfer does not work?
        let vest_to_balance_pre = get_account_balance(vest_to);
        let vester_balance_pre = get_account_balance(vester.env().account_id());

        set_value_transferred(create_vest_args.amount);
        let res = Vesting::create_vest(
            &mut vester,
            create_vest_args.vest_to,
            create_vest_args.asset,
            create_vest_args.amount,
            create_vest_args.vesting_start,
            create_vest_args.vesting_end,
        );
        set_block_timestamp(create_vest_args.vesting_start - 1);
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        //try release succeeds & does not release anything
        set_next_caller(create_vest_args.vest_to);
        let res = Vesting::release(&mut vester, create_vest_args.asset);
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_token_released_event(
            &emitted_events[emitted_events.len() - 1],
            create_vest_args.asset,
            create_vest_args.vest_to,
            0,
        );
        // try release succeeds & does release adequate amount of tokens eq 1
        set_block_timestamp(create_vest_args.vesting_start + 2);
        let res = Vesting::release(&mut vester, create_vest_args.asset);
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_token_released_event(
            &emitted_events[emitted_events.len() - 1],
            create_vest_args.asset,
            create_vest_args.vest_to,
            1, // accounting for rounding down
        );
        //verify storage
        let vesting_schedule =
            vester.get_schedule_by_id(create_vest_args.vest_to, create_vest_args.asset, 0);
        assert!(vesting_schedule.is_some());
        let vesting_schedule = vesting_schedule.unwrap();
        assert_eq!(vesting_schedule.released, 1);
        assert_eq!(vesting_schedule.amount, create_vest_args.amount);
        assert_eq!(vesting_schedule.end, create_vest_args.vesting_end);
        assert_eq!(vesting_schedule.start, create_vest_args.vesting_start);

        // try release succeeds & does release adequate amount of tokens
        set_block_timestamp(create_vest_args.vesting_start + ONE_DAY);
        let res = Vesting::release(&mut vester, create_vest_args.asset);
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_token_released_event(
            &emitted_events[emitted_events.len() - 1],
            create_vest_args.asset,
            create_vest_args.vest_to,
            (ONE_DAY - 1 - 1).into(), //1 already released + accounting for rounding down
        );
        //verify storage
        let vesting_schedule =
            vester.get_schedule_by_id(create_vest_args.vest_to, create_vest_args.asset, 0);
        assert!(vesting_schedule.is_some());
        let vesting_schedule = vesting_schedule.unwrap();
        assert_eq!(vesting_schedule.released, (ONE_DAY - 1).into());
        assert_eq!(vesting_schedule.amount, create_vest_args.amount);
        assert_eq!(vesting_schedule.end, create_vest_args.vesting_end);
        assert_eq!(vesting_schedule.start, create_vest_args.vesting_start);

        // try release succeeds & does release the rest of tokens
        set_block_timestamp(create_vest_args.vesting_end + 1);
        let res = Vesting::release(&mut vester, create_vest_args.asset);
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_token_released_event(
            &emitted_events[emitted_events.len() - 1],
            create_vest_args.asset,
            create_vest_args.vest_to,
            create_vest_args.amount - u128::from(ONE_DAY - 1), // ONE_DAY + 1 already released
        );
        let next_id = vester.next_id_vest_of(create_vest_args.vest_to, create_vest_args.asset);
        assert_eq!(next_id, 0);
        let vesting_schedule =
            vester.get_schedule_by_id(create_vest_args.vest_to, create_vest_args.asset, 0);
        assert!(vesting_schedule.is_none());

        let vest_to_balance_post = get_account_balance(vest_to);
        let vester_balance_post = get_account_balance(vester.env().account_id());
        assert_eq!(
            vest_to_balance_post,
            vest_to_balance_pre + create_vest_args.amount
        );
        assert_eq!(
            vester_balance_post,
            vester_balance_pre - create_vest_args.amount
        );
    }

    #[ink::test]
    fn release_when_multiple_schedules_created() {
        let accounts = ink::env::test::default_accounts::<DefaultEnvironment>();
        let vest_to = accounts.charlie;
        let vester_submitter = accounts.bob;

        let mut vester = Vester::new();

        set_next_caller(vester_submitter);
        let init_timestamp = vester.env().block_timestamp() + ONE_DAY * 365;
        let create_vest_args_vec: Vec<CreateVestingScheduleArgs> = vec![
            create_duration_as_amount_schedule_args(
                vest_to,
                None,
                init_timestamp - ONE_DAY * 9,
                init_timestamp - ONE_DAY * 3,
            ), //overdue
            create_duration_as_amount_schedule_args(
                vest_to,
                None,
                init_timestamp - ONE_DAY * 6,
                init_timestamp + ONE_DAY * 3,
            ), //started
            create_duration_as_amount_schedule_args(
                vest_to,
                None,
                init_timestamp + ONE_DAY * 1,
                init_timestamp + ONE_DAY * 6,
            ), //not started
            create_duration_as_amount_schedule_args(
                vest_to,
                None,
                init_timestamp + ONE_DAY * 3,
                init_timestamp + ONE_DAY * 9,
            ), //not started
            create_duration_as_amount_schedule_args(
                vest_to,
                None,
                init_timestamp + ONE_DAY * 18,
                init_timestamp + ONE_DAY * 64,
            ), //not started
        ];
        let starting_balance = 100_000;
        set_account_balance(vest_to, starting_balance);

        for create_vest_args in create_vest_args_vec.iter() {
            let vester_balance = get_account_balance(vester.env().account_id());
            set_account_balance(
                vester.env().account_id(),
                create_vest_args.amount + vester_balance,
            ); // why transfer does not work?
            set_value_transferred(create_vest_args.amount);
            let res = Vesting::create_vest(
                &mut vester,
                create_vest_args.vest_to,
                create_vest_args.asset,
                create_vest_args.amount,
                create_vest_args.vesting_start,
                create_vest_args.vesting_end,
            );
            assert!(res.is_ok(), "release failed. res: {:?}", res);
        }
        let vest_to_balance_pre = get_account_balance(vest_to);
        let vester_balance_pre = get_account_balance(vester.env().account_id());

        set_block_timestamp(init_timestamp);
        // pre action
        let event_count_before = ink::env::test::recorded_events().collect::<Vec<_>>().len();
        assert_eq!(vester.next_id_vest_of(vest_to, None), 5);
        for i in 0..3 {
            assert!(vester
                .get_schedule_by_id(vest_to, None, i.try_into().unwrap())
                .is_some());
        }
        set_next_caller(vest_to);
        let res = Vesting::release(&mut vester, None);
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_token_released_event(
            &emitted_events[emitted_events.len() - 1],
            None,
            vest_to,
            (ONE_DAY * 12 - 1).into(),
        );
        assert_eq!(emitted_events.len() - event_count_before, 1);
        assert_eq!(vester.next_id_vest_of(vest_to, None), 4);
        assert!(vester.get_schedule_by_id(vest_to, None, 4).is_none());
        for i in 0..3 {
            assert!(vester
                .get_schedule_by_id(vest_to, None, i.try_into().unwrap())
                .is_some());
        }

        // move time past 1st schedule end
        set_block_timestamp(create_vest_args_vec[1].vesting_end + ONE_DAY);
        let event_count_before = ink::env::test::recorded_events().collect::<Vec<_>>().len();
        let res = Vesting::release(&mut vester, None);
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_token_released_event(
            &emitted_events[emitted_events.len() - 1],
            None,
            vest_to,
            (ONE_DAY * 7 - 1).into(),
        );
        assert_eq!(emitted_events.len() - event_count_before, 1);
        assert_eq!(vester.next_id_vest_of(vest_to, None), 3); // 1st & 2nd schedule removed
        assert!(vester.get_schedule_by_id(vest_to, None, 3).is_none());
        for i in 0..2 {
            assert!(vester
                .get_schedule_by_id(vest_to, None, i.try_into().unwrap())
                .is_some());
        }

        // move time past last schedule end
        set_block_timestamp(create_vest_args_vec[create_vest_args_vec.len() - 1].vesting_end + 1);
        let res = Vesting::release(&mut vester, None);
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_token_released_event(
            &emitted_events[emitted_events.len() - 1],
            None,
            vest_to,
            (ONE_DAY * 53 + 1 + 1).into(),
        );
        let next_id = vester.next_id_vest_of(vest_to, None);
        assert_eq!(next_id, 0);
        for i in 0..5 {
            assert!(vester
                .get_schedule_by_id(vest_to, None, i.try_into().unwrap())
                .is_none());
        }

        let vest_to_balance_post = get_account_balance(vest_to);
        let vester_balance_post = get_account_balance(vester.env().account_id());
        let total_amount = create_vest_args_vec.iter().fold(0, |acc, x| acc + x.amount);
        assert_eq!(vest_to_balance_post, vest_to_balance_pre + total_amount);
        assert_eq!(vester_balance_post, vester_balance_pre - total_amount);
    }
}
