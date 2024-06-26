// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// Contract Summary:
/// A PSP34 contract with metadata module.
// ########################################################
// inject PSP34 trait's default implementation (PSP34DefaultImpl & PSP34InternalDefaultImpl)
// and PSP34Metadata trait's default implementation (PSP34MetadataDefaultImpl)
// which reduces the amount of boilerplate code required to implement trait messages drastically
#[pendzl::implementation(PSP34, PSP34Metadata)]
#[ink::contract]
pub mod my_psp34_metadata {
    use ink::prelude::string::*;
    use pendzl::contracts::psp34::*;
    #[ink(storage)]
    // derive explained below
    #[derive(Default, StorageFieldGetter)]
    pub struct Contract {
        // apply the storage_field attribute so it's accessible via `self.data::<PSP34>()` (provided by StorageFieldGetter derive)
        #[storage_field]
        // PSP34Data is a struct that implements PSP34Storage - required by PSP34InternalDefaultImpl trait
        // note it's not strictly required by PSP34 trait - just the default implementation
        // name of the field is arbitrary
        psp34: PSP34Data,
        // apply the storage_field attribute so it's accessible via `self.data::<PSP34Metadata>()` (provided by StorageFieldGetter derive)
        #[storage_field]
        // PSP34MetadataData is a struct that implements PSP34MetadataStorage - required by PSP34MetadataInternalDefaultImpl trait
        // note it's not strictly required by PSP34Metadata trait - just the default implementation
        // name of the field is arbitrary
        metadata: PSP34MetadataData,
    }

    impl Contract {
        /// A constructor which mints the first token to the owner
        #[ink(constructor)]
        pub fn new(id: Id, name: String, symbol: String) -> Self {
            let mut instance = Self::default();

            let name_key = String::from("name");
            let symbol_key = String::from("symbol");
            // use _set_attribute from PSP34MetadataInternal (implemented by PSP34MetadataDefaultImpl)
            instance._set_attribute(&id.clone(), &name_key, &name);
            instance._set_attribute(&id, &symbol_key, &symbol);

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::ContractsBackend;
        use pendzl::contracts::psp34::Id;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn metadata_works(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let id = Id::U8(0);
            let name = String::from("My PSP34");
            let symbol = String::from("MPS34");

            let mut constructor =
                ContractRef::new(id.clone(), name.clone(), symbol.clone());
            let contract = client
                .instantiate(
                    "my_psp34_metadata",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            let result_name = client
                .call(
                    &ink_e2e::alice(),
                    &contract.get_attribute(id.clone(), String::from("name")),
                )
                .dry_run()
                .await?
                .return_value();

            let result_symbol = client
                .call(
                    &ink_e2e::alice(),
                    &contract.get_attribute(id.clone(), String::from("symbol")),
                )
                .dry_run()
                .await?
                .return_value();

            assert_eq!(result_name, Some(name));
            assert_eq!(result_symbol, Some(symbol));

            Ok(())
        }
    }
}
