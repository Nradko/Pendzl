// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(Vesting)]
#[ink::contract]
pub mod vester {
    use ink::codegen::Env;
    use pendzl::contracts::finance::uvester::implementation::VestingInternalDefaultImpl;
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Vester {
        #[storage_field]
        vesting: VestingData,
        mock_transfers: bool,
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
            Vester {
                mock_transfers: false,
                ..Default::default()
            }
        }

        #[ink(message)]
        pub fn set_mock_transfers(&mut self, should_skip: bool) {
            self.mock_transfers = should_skip;
        }
    }

    #[overrider(VestingInternal)]
    fn _handle_transfer_in(
        &mut self,
        asset: Option<AccountId>,
        from: AccountId,
        amount: Balance,
    ) -> Result<(), VestingError> {
        if self.mock_transfers {
            self.env().emit_event(TransferMock {
                from,
                to: self.env().account_id(),
                amount,
                asset,
            });
            return Ok(());
        }
        self._handle_transfer_in_default_impl(asset, from, amount)
    }

    #[overrider(VestingInternal)]
    fn _handle_transfer_out(
        &mut self,
        asset: Option<AccountId>,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), VestingError> {
        if self.mock_transfers {
            self.env().emit_event(TransferMock {
                from: self.env().account_id(),
                to,
                amount,
                asset,
            });
            return Ok(());
        }
        self._handle_transfer_out_default_impl(asset, to, amount)
    }
}

#[cfg(all(test, feature = "e2e-tests"))]
pub mod tests {
    #[rustfmt::skip]
    use crate::vester::{VesterRef, VestingError, *};
    use my_psp22_mintable::my_psp22_mintable::{
        Contract as PSP22MintableContract, ContractRef as PSP22Ref, *,
    };
    use pendzl::contracts::token::psp22::{Approval, Transfer};
    #[rustfmt::skip]
    // #[rustfmt::skip]
    // use ink_e2e::build_message;
    use ink::ToAccountId;
    use ink::codegen::Env;
    use ink::env::DefaultEnvironment;
    use ink::scale::Decode as _;
    use ink_e2e::events::ContractEmitted;
    use ink_e2e::{ChainBackend, ContractsBackend};
    use pendzl::{
        contracts::token::psp22::{PSP22Error, PSP22},
        traits::{AccountId, Balance, Timestamp},
    };
    use test_helpers::keypair_to_account;
    // use ink_e2e::CallBuilder;
    // use ink_e2e::Client;
    // use ink_e2e::PolkadotConfig;
    // use pendzl::traits::AccountId;
    // use scale::Decode;
    // use scale::Encode;

    // async fn instantiate_vester<'a, E, Args, RetType, B>(
    //     mut client: Client<PolkadotConfig, DefaultEnvironment>,
    //     vester_creator: &Keypair,
    // ) -> AccountId {
    //     let mut vester_constructor = VesterRef::new();
    //     let mut vester = client
    //         .instantiate("vester", vester_creator, &mut vester_constructor)
    //         .submit()
    //         .await
    //         .expect("instantiate vester failed")
    //         .call::<Vester>();

    //     vester.to_account_id()
    // }

    struct CreateVestingScheduleArgs {
        vest_to: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        vesting_start: Timestamp,
        vesting_end: Timestamp,
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
        assert_eq!(
            asset, expected_asset,
            "Assets were not equal: encountered role {:?}, expected role {:?}",
            asset, expected_asset
        );
        assert_eq!(
            to, expected_to,
            "To were not equal: encountered role {:?}, expected role {:?}",
            to, expected_to
        );
        assert_eq!(
            amount, expected_amount,
            "Amounts were not equal: encountered role {:?}, expected role {:?}",
            amount, expected_amount
        );
        assert_eq!(
            vesting_start, expected_vesting_start,
            "Vesting start were not equal: encountered role {:?}, expected role {:?}",
            vesting_start, expected_vesting_start
        );
        assert_eq!(
            vesting_end, expected_vesting_end,
            "Vesting end were not equal: encountered role {:?}, expected role {:?}",
            vesting_end, expected_vesting_end
        );
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

        assert_eq!(
            from,
            Some(expected_from),
            "encountered invalid Transfer.from"
        );
        assert_eq!(to, Some(expected_to), "encountered invalid Transfer.to");
        assert_eq!(value, expected_value, "encountered invalid Trasfer.value");
        assert_eq!(
            event.contract, expected_asset,
            "encountered invalid Trasfer.asset"
        );
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

        assert!(matches!(balance_of_vester, 0));
        assert!(matches!(balance_of_vester_submitter, amount));

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

        let vesting_scheduled_event =
            <VestingScheduled as scale::Decode>::decode(&mut &vester_events[0].event.data[..])
                .expect("invalid data");
        let transfer_event =
            <Transfer as scale::Decode>::decode(&mut &psp22_events[1].event.data[..])
                .expect("invalid data");

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

        assert!(matches!(balance_of_vester, amount));
        assert!(matches!(balance_of_vester_submitter, 0));

        Ok(())
    }

    #[ink_e2e::test]
    async fn create_vesting_schedule_native(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let vester_creator = ink_e2e::alice();
        let amount = 10_000_000;
        let vester_submitter = client
            .create_and_fund_account(&ink_e2e::alice(), 10_000_000_000_000)
            .await;

        // create_vest args
        let create_vest_args = CreateVestingScheduleArgs {
            vest_to: keypair_to_account(&ink_e2e::charlie()),
            asset: None,
            amount,
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
            .value(amount - 1)
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
            .value(amount + 1)
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
            .value(amount)
            .dry_run()
            .await
            .expect("create vest failed")
            .return_value();
        assert!(matches!(create_vest_res, Ok(())));

        let balance_of_vester_after = client
            .free_balance(vester.to_account_id())
            .await
            .expect("free balance failed");
        let balance_of_vester_submitter_after = client
            .free_balance(keypair_to_account(&vester_submitter))
            .await
            .expect("free balance failed");

        assert!(matches!(
            balance_of_vester_before + amount,
            balance_of_vester_after
        ));
        assert!(matches!(
            balance_of_vester_submitter_before - amount,
            balance_of_vester_submitter_after
        ));
        Ok(())
    }
    #[ink::test]
    fn emit_events_works() {
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        let vest_to = accounts.charlie;
        let create_vest_args: Vec<CreateVestingScheduleArgs> = vec![
            CreateVestingScheduleArgs {
                vest_to,
                asset: None,
                amount: 100,
                vesting_start: 100,
                vesting_end: 101,
            },
            CreateVestingScheduleArgs {
                vest_to,
                asset: None,
                amount: 100,
                vesting_start: 100,
                vesting_end: 101,
            },
            CreateVestingScheduleArgs {
                vest_to,
                asset: None,
                amount: 100,
                vesting_start: 100,
                vesting_end: 101,
            },
        ];

        let mut vester = Vester::new();
        Vester::set_mock_transfers(&mut vester, true);
        let mut psp22 = PSP22MintableContract::new(1_000_000);

        let vester_creator = ink_e2e::alice();
        let amount = 10_000_000;
        let vester_submitter = accounts.bob;

        // create_vest args
        let vesting_start = 100;
        let vesting_end = 101;

        assert!(PSP22Mintable::mint(&mut psp22, vester_submitter, amount).is_ok());
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();

        let event_count_before = emitted_events.len();
        let res = Vesting::create_vest(
            &mut vester,
            vest_to,
            None,
            amount,
            vesting_start,
            vesting_end,
        );
        assert!(res.is_ok());
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        vester.env().caller();
        println!("caller: {:?}", vester.env().caller());
        println!("vester account_id: {:?}", vester.env().account_id());
        println!("alice account_id: {:?}", accounts.alice);
        println!("bob account_id: {:?}", accounts.bob);
        assert_eq!(2, emitted_events.len() - event_count_before);
        // assert_transfer_mock_event(
        //     &emitted_events[emitted_events.len() - 2],
        //     vester.env().caller(),
        //     vester.env().account_id(),
        //     amount,
        //     None,
        // );
        // assert_vesting_scheduled_event(
        //     &emitted_events[emitted_events.len() - 1],
        //     None,
        //     vest_to,
        //     amount,
        //     vesting_start,
        //     vesting_end,
        // );
    }

    /*
    e2e:
    - emit vesting events (in/out) (if possible test in ink:test)
    - balance increase/decrease (psp22/native)
    - create schedule fails if has insufficient balance (psp22/native)
    ink:test:
    - test release/claim logic
        - correctly creates vesting schedule
        - test against vesting schedules that:
            - have not started (release nothing)
            - are overdue (release all)
            - are partially vested (release partial)
            - scenario:
                - create schedule => advance time 1/3 & release => advance time 1/3 & release => advance time 1/3 + 1 & release
            - seed several schedules of various states => release all => verify schedules map have been updated/shrinked correctly
     */
}
