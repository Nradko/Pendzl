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

    impl Vester {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}

#[cfg(all(test, feature = "e2e-tests"))]
pub mod tests {
    #[rustfmt::skip]
    use pendzl::contracts::token::psp22::PSP22;
    use crate::vester::{VesterRef, *};
    use my_psp22_mintable::my_psp22_mintable::{ContractRef as PSP22Ref, *};
    #[rustfmt::skip]
    use ink_e2e::ContractsBackend;
    use ink::ToAccountId;
    use ink_e2e::account_id;
    use ink_e2e::AccountKeyring::{Alice, Bob};
    use test_helpers::balance_of;

    type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn create_vesting_schedule(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let mut constructor = PSP22Ref::new(1000);
        let mut psp22 = client
            .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate psp22 failed")
            .call::<Contract>();

        let mint_tx = client
            .call(&ink_e2e::alice(), &psp22.mint(account_id(Alice), 1000))
            .submit()
            .await
            .expect("mint failed")
            .return_value();
        let mut vester_constructor = VesterRef::new();

        let mut vester = client
            .instantiate("vester", &ink_e2e::alice(), &mut vester_constructor)
            .submit()
            .await
            .expect("instantiate vester failed")
            .call::<Vester>();

        client
            .call(
                &ink_e2e::alice(),
                &psp22.increase_allowance(vester.to_account_id(), 100),
            )
            .submit()
            .await
            .expect("give allowance failed")
            .return_value();

        let create_vest_tx = client
            .call(
                &ink_e2e::alice(),
                &vester.create_vest(account_id(Bob), Some(psp22.to_account_id()), 100, 100, 101),
            )
            .submit()
            .await
            .expect("create vest failed")
            .return_value();
        println!("create vest tx: {:?}", create_vest_tx);
        Ok(())
    }

    #[ink::test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
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
