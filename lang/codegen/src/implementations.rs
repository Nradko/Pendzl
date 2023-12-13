use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::Block;

pub type IsDefault = bool;
pub type OverridenFnMap =
    HashMap<String, Vec<(String, (Box<Block>, Vec<syn::Attribute>, IsDefault))>>;

pub struct ImplArgs<'a> {
    pub map: &'a OverridenFnMap,
    pub items: &'a mut Vec<syn::Item>,
    pub imports: &'a mut HashMap<&'a str, syn::ItemUse>,
    pub overriden_traits: &'a mut HashMap<&'a str, syn::Item>,
    pub storage_struct_name: String,
}

impl<'a> ImplArgs<'a> {
    pub fn new(
        map: &'a OverridenFnMap,
        items: &'a mut Vec<syn::Item>,
        imports: &'a mut HashMap<&'a str, syn::ItemUse>,
        overriden_traits: &'a mut HashMap<&'a str, syn::Item>,
        storage_struct_name: String,
    ) -> Self {
        Self {
            map,
            items,
            imports,
            overriden_traits,
            storage_struct_name,
        }
    }

    fn contract_name(&self) -> proc_macro2::Ident {
        format_ident!("{}", self.storage_struct_name)
    }

    fn vec_import(&mut self) {
        let vec_import = syn::parse2::<syn::ItemUse>(quote!(
            use ink::prelude::vec::Vec;
        ))
        .expect("Should parse");
        self.imports.insert("vec", vec_import);
    }

    // fn signature_import(&mut self) {
    //     let sig_import = syn::parse2::<syn::ItemUse>(quote!(
    //       pub use pendzl::utils::crypto::Signature;
    //     ))
    //     .expect("Should parse");
    //     self.imports.insert("Signature", sig_import);
    // }
}

pub(crate) fn impl_psp22(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp22::implementation::PSP22InternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl  pendzl::contracts::token::psp22::PSP22Internal for #storage_struct_name {
            fn _total_supply(&self) -> Balance {
                pendzl::contracts::token::psp22::implementation::PSP22InternalDefaultImpl::_total_supply_default_impl(self)
            }

            fn _balance_of(&self, owner: &AccountId) -> Balance {
                pendzl::contracts::token::psp22::implementation::PSP22InternalDefaultImpl::_balance_of_default_impl(self, owner)
            }

            fn _allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance {
                pendzl::contracts::token::psp22::implementation::PSP22InternalDefaultImpl::_allowance_default_impl(self, owner, spender)
            }

            fn _update(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22InternalDefaultImpl::_update_default_impl(self, from, to, amount)
            }

            fn _transfer(
                &mut self,
                from: &AccountId,
                to: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::PSP22Internal::_update(self, Some(from), Some(to), amount)
            }

            fn _mint_to(
                &mut self,
                to: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::PSP22Internal::_update(self, None, Some(to), amount)
            }
            
            fn _burn_from(
                &mut self,
                from: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::PSP22Internal::_update(self, Some(from), None, amount)
            }

            fn _approve(
                &mut self,
                owner: &AccountId,
                spender: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22InternalDefaultImpl::_approve_default_impl(self, owner, spender, amount)
            }

            fn _decrease_allowance_from_to(
                &mut self,
                owner: &AccountId,
                spender: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22InternalDefaultImpl::_decrease_allowance_from_to_default_impl(self, owner, spender, amount)
            }

            fn _increase_allowance_from_to(
                &mut self,
                owner: &AccountId,
                spender: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error>{
                pendzl::contracts::token::psp22::implementation::PSP22InternalDefaultImpl::_increase_allowance_from_to_default_impl(self, owner, spender, amount)

            }
        }
    ))
    .expect("Should parse");

    let psp22_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp22::implementation::PSP22DefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp22 = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp22::PSP22 for #storage_struct_name {
            #[ink(message)]
            fn total_supply(&self) -> Balance {
                pendzl::contracts::token::psp22::implementation::PSP22DefaultImpl::total_supply_default_impl(self)
            }

            #[ink(message)]
            fn balance_of(&self, owner: AccountId) -> Balance {
                pendzl::contracts::token::psp22::implementation::PSP22DefaultImpl::balance_of_default_impl(self, owner)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
                pendzl::contracts::token::psp22::implementation::PSP22DefaultImpl::allowance_default_impl(self, owner, spender)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22DefaultImpl::transfer_default_impl(self, to, value, data)
            }

            #[ink(message)]
            fn transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                value: Balance,
                data: Vec<u8>,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22DefaultImpl::transfer_from_default_impl(self, from, to, value, data)
            }

            #[ink(message)]
            fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22DefaultImpl::approve_default_impl(self, spender, value)
            }

            #[ink(message)]
            fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22DefaultImpl::increase_allowance_default_impl(self, spender, delta_value)
            }

            #[ink(message)]
            fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22DefaultImpl::decrease_allowance_default_impl(self, spender, delta_value)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::token::psp22::*;
    ))
    .expect("Should parse");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::token::psp22::implementation::Data as PSP22Data;
    ))
    .expect("Should parse import");

    impl_args.imports.insert("PSP22", import);
    impl_args.imports.insert("PSP22Data", import_data);
    impl_args.vec_import();

    override_functions(
        "PSP22Internal",
        &mut internal,
        impl_args.map,
    );
    override_functions("PSP22", &mut psp22, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(psp22_default_impl));
    impl_args.items.push(syn::Item::Impl(psp22));
}

pub(crate) fn impl_psp22_burnable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let burnable_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp22::extensions::burnable::implementation::PSP22BurnableDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp22::extensions::burnable::PSP22Burnable for #storage_struct_name {
            #[ink(message)]
            fn burn(&mut self, from: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::extensions::burnable::implementation::PSP22BurnableDefaultImpl::burn_default_impl(self,from,amount)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::token::psp22::extensions::burnable::*;
    ))
    .expect("Should parse");

    impl_args.imports.insert("PSP22Burnable", import);
    impl_args.vec_import();

    override_functions("PSP22Burnable", &mut burnable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(burnable_default_impl));
    impl_args.items.push(syn::Item::Impl(burnable));
}


pub(crate) fn impl_psp22_mintable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let mintable_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp22::extensions::mintable::implementation::PSP22MintableDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut mintable = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp22::extensions::mintable::PSP22Mintable for #storage_struct_name {
            #[ink(message)]
            fn mint(&mut self, from: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::extensions::mintable::implementation::PSP22MintableDefaultImpl::mint_default_impl(self,from,amount)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::token::psp22::extensions::mintable::*;
    ))
    .expect("Should parse");

    impl_args.imports.insert("PSP22Mintable", import);
    impl_args.vec_import();

    override_functions("PSP22Mintable", &mut mintable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(mintable_default_impl));
    impl_args.items.push(syn::Item::Impl(mintable));
}

pub(crate) fn impl_psp22_metadata(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let metadata_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp22::extensions::metadata::implementation::PSP22MetadataDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut metadata = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp22::extensions::metadata::PSP22Metadata for #storage_struct_name {
            #[ink(message)]
            fn token_name(&self) -> Option<String> {
                pendzl::contracts::token::psp22::extensions::metadata::implementation::PSP22MetadataDefaultImpl::token_name_default_impl(self)
            }

            #[ink(message)]
            fn token_symbol(&self) -> Option<String> {
                pendzl::contracts::token::psp22::extensions::metadata::implementation::PSP22MetadataDefaultImpl::token_symbol_default_impl(self)
            }

            #[ink(message)]
            fn token_decimals(&self) -> u8 {
                pendzl::contracts::token::psp22::extensions::metadata::implementation::PSP22MetadataDefaultImpl::token_decimals_default_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::token::psp22::extensions::metadata::*;
    ))
    .expect("Should parse");
    let import_data = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::token::psp22::extensions::metadata::implementation::Data as PSP22MetadataData;
    ))
    .expect("Should parse");

    impl_args.imports.insert("PSP22Metadata", import);
    impl_args.imports.insert("PSP22MetadataData", import_data);
    impl_args.vec_import();

    override_functions("PSP22Metadata", &mut metadata, impl_args.map);

    impl_args.items.push(syn::Item::Impl(metadata_default_impl));
    impl_args.items.push(syn::Item::Impl(metadata));
}


pub(crate) fn impl_psp34(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp34::implementation::PSP34InternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp34::PSP34Internal for #storage_struct_name {
            fn _balance_of(&self, owner: &AccountId) -> u32 {
                pendzl::contracts::token::psp34::implementation::PSP34InternalDefaultImpl::_balance_of_default_impl(self, owner)
            }

            fn _total_supply(&self) -> u64 {
                pendzl::contracts::token::psp34::implementation::PSP34InternalDefaultImpl::_total_supply_default_impl(self)
            }

            fn _owner_of(&self, id: &Id) -> Option<AccountId> {
                pendzl::contracts::token::psp34::implementation::PSP34InternalDefaultImpl::_owner_of_default_impl(self,id)
            }
            
            fn _allowance(&self, owner: &AccountId, operator: &AccountId, id: &Option<Id>) -> bool {
                pendzl::contracts::token::psp34::implementation::PSP34InternalDefaultImpl::_allowance_default_impl(self, owner, operator, id)
            }
            
            fn _approve(&mut self, owner: &AccountId,operator: &AccountId, id: &Option<Id>, approved: &bool) -> Result<(), PSP34Error> {
                pendzl::contracts::token::psp34::implementation::PSP34InternalDefaultImpl::_approve_default_impl(self, owner, operator, id, approved)
            }

            fn _update(
                &mut self,
                from: &Option<&AccountId>,
                to: &Option<&AccountId>,
                id: &Id,
            ) -> Result<(), PSP34Error>{
                pendzl::contracts::token::psp34::implementation::PSP34InternalDefaultImpl::_update_default_impl(self, from, to, id)

            }
        
            fn _transfer(&mut self, from: &AccountId, to: &AccountId, id: &Id, data: &Vec<u8>) -> Result<(), PSP34Error> {
                pendzl::contracts::token::psp34::PSP34Internal::_update(self, &Some(from), &Some(to), id)
            }

            fn _mint_to(&mut self, to: &AccountId, id: &Id) -> Result<(), PSP34Error> {
                pendzl::contracts::token::psp34::PSP34Internal::_update(self, &None, &Some(to), id)
            }

            fn _burn_from(&mut self, from: &AccountId, id: &Id) -> Result<(), PSP34Error> {
                pendzl::contracts::token::psp34::PSP34Internal::_update(self, &Some(from), &None, id)
            }

        }
    ))
    .expect("Should parse");

    let psp34_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp34::implementation::PSP34DefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp34 = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp34::PSP34 for #storage_struct_name {
            #[ink(message)]
            fn collection_id(&self) -> Id {
                pendzl::contracts::token::psp34::implementation::PSP34DefaultImpl::collection_id_default_impl(self)
            }

            #[ink(message)]
            fn balance_of(&self, owner: AccountId) -> u32 {
                pendzl::contracts::token::psp34::implementation::PSP34DefaultImpl::balance_of_default_impl(self, owner)
            }

            #[ink(message)]
            fn owner_of(&self, id: Id) -> Option<AccountId> {
                pendzl::contracts::token::psp34::implementation::PSP34DefaultImpl::owner_of_default_impl(self, id)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
                pendzl::contracts::token::psp34::implementation::PSP34DefaultImpl::allowance_default_impl(self, owner, operator, id)
            }

            #[ink(message)]
            fn approve(&mut self, operator: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
                pendzl::contracts::token::psp34::implementation::PSP34DefaultImpl::approve_default_impl(self, operator, id, approved)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
                pendzl::contracts::token::psp34::implementation::PSP34DefaultImpl::transfer_default_impl(self, to, id, data)
            }

            #[ink(message)]
            fn total_supply(&self) -> u64 {
                pendzl::contracts::token::psp34::implementation::PSP34DefaultImpl::total_supply_default_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::token::psp34::*;
    ))
    .expect("Should parse");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::token::psp34::implementation::Data as PSP34Data;
    ))
    .expect("Should parse import");

    impl_args.imports.insert("PSP34", import);
    impl_args.imports.insert("PSP34Data", import_data);
    impl_args.vec_import();

    override_functions("PSP34Internal", &mut internal, impl_args.map);
    override_functions("PSP34", &mut psp34, impl_args.map);

    // only insert this if it is not present
    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(psp34_default_impl));
    impl_args.items.push(syn::Item::Impl(psp34));
}


pub(crate) fn impl_psp34_burnable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let burnable_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp34::extensions::burnable::implementation::PSP34BurnableDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp34::extensions::burnable::PSP34Burnable for #storage_struct_name {
            #[ink(message)]
            fn burn(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error> {
                pendzl::contracts::token::psp34::extensions::burnable::implementation::PSP34BurnableDefaultImpl::burn_default_impl(self,from,id)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::token::psp34::extensions::burnable::*;
    ))
    .expect("Should parse");

    impl_args.imports.insert("PSP34Burnable", import);
    impl_args.vec_import();

    override_functions("PSP34Burnable", &mut burnable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(burnable_default_impl));
    impl_args.items.push(syn::Item::Impl(burnable));
}


pub(crate) fn impl_psp34_mintable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let mintable_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp34::extensions::mintable::implementation::PSP34MintableDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut mintable = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp34::extensions::mintable::PSP34Mintable for #storage_struct_name {
            #[ink(message)]
            fn mint(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error> {
                pendzl::contracts::token::psp34::extensions::mintable::implementation::PSP34MintableDefaultImpl::mint_default_impl(self,from,id)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::token::psp34::extensions::mintable::*;
    ))
    .expect("Should parse");

    impl_args.imports.insert("PSP34Mintable", import);
    impl_args.vec_import();

    override_functions("PSP34Mintable", &mut mintable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(mintable_default_impl));
    impl_args.items.push(syn::Item::Impl(mintable));
}

pub(crate) fn impl_psp34_metadata(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp34::extensions::metadata::implementation::PSP34MetadataInternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp34::extensions::metadata::PSP34MetadataInternal for #storage_struct_name {

            fn _set_attribute(&mut self, id: &Id, key: &String, value: &String) {
                pendzl::contracts::token::psp34::extensions::metadata::implementation::PSP34MetadataInternalDefaultImpl::_set_attribute_default_impl(self, id, key, value)
            }
        }
    ))
    .expect("Should parse");

    let metadata_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp34::extensions::metadata::implementation::PSP34MetadataDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut metadata = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp34::extensions::metadata::PSP34Metadata for #storage_struct_name {
            #[ink(message)]
            fn get_attribute(&self, id: Id, key: String) -> Option<String> {
                pendzl::contracts::token::psp34::extensions::metadata::implementation::PSP34MetadataDefaultImpl::get_attribute_default_impl(self, id, key)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::token::psp34::extensions::metadata::*;
    ))
    .expect("Should parse");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::token::psp34::extensions::metadata::implementation::Data as PSP34MetadataData;
    ))
    .expect("Should parse import");

    impl_args.imports.insert("PSP34Metadata", import);
    impl_args.imports.insert("PSP34MetadataData", import_data);

    impl_args.vec_import();

    override_functions("PSP34MetadataInternal", &mut internal, impl_args.map);
    override_functions("PSP34Metadata", &mut metadata, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(metadata_default_impl));
    impl_args.items.push(syn::Item::Impl(metadata));
}

pub(crate) fn impl_ownable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::ownable::implementation::OwnableInternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::ownable::OwnableInternal for #storage_struct_name {
            fn _owner(&self) -> Option<AccountId>{
                pendzl::contracts::access::ownable::implementation::OwnableInternalDefaultImpl::_owner_default_impl(self)
            }
            fn _update_owner(&mut self, owner: &Option<AccountId>){
                pendzl::contracts::access::ownable::implementation::OwnableInternalDefaultImpl::_update_owner_default_impl(self, owner);

            }
            fn _only_owner(&self) -> Result<(), OwnableError> {
                pendzl::contracts::access::ownable::implementation::OwnableInternalDefaultImpl::_only_owner_default_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let ownable_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::ownable::implementation::OwnableDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut ownable = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::ownable::Ownable for #storage_struct_name {
            #[ink(message)]
            fn owner(&self) -> Option<AccountId> {
                pendzl::contracts::access::ownable::implementation::OwnableDefaultImpl::owner_default_impl(self)
            }

            #[ink(message)]
            fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
                pendzl::contracts::access::ownable::implementation::OwnableDefaultImpl::renounce_ownership_default_impl(self)
            }

            #[ink(message)]
            fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError> {
                pendzl::contracts::access::ownable::implementation::OwnableDefaultImpl::transfer_ownership_default_impl(self, new_owner)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::access::ownable::*;
    ))
    .expect("Should parse");
    
    let import_data = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::access::ownable::implementation::Data as OwnableData;
    ))
    .expect("Should parse import");

    impl_args.imports.insert("Ownable", import);
    impl_args.imports.insert("OwnableData", import_data);

    override_functions("OwnableInternal", &mut internal, impl_args.map);
    override_functions("Ownable", &mut ownable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(ownable_default_impl));
    impl_args.items.push(syn::Item::Impl(ownable));
}

pub(crate) fn impl_access_control(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::access_control::implementation::AccessControlInternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::access_control::AccessControlInternal for #storage_struct_name {
            fn _default_admin() -> RoleType {
                <Self as pendzl::contracts::access::access_control::implementation::AccessControlInternalDefaultImpl>::_default_admin_default_impl()
            }

            fn _has_role(&self, role: RoleType, account: Option<AccountId>) -> bool{
                pendzl::contracts::access::access_control::implementation::AccessControlInternalDefaultImpl::_has_role_default_impl(self, role, account)
            }

            fn _grant_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access::access_control::implementation::AccessControlInternalDefaultImpl::_grant_role_default_impl(self, role, account)
            }

            fn _do_revoke_role(&mut self, role: RoleType, account: Option<AccountId>)  -> Result<(), AccessControlError>  {
                pendzl::contracts::access::access_control::implementation::AccessControlInternalDefaultImpl::_do_revoke_role_default_impl(self, role, account)
            }
            
            fn _get_role_admin(&self, role: RoleType) -> RoleType {
                pendzl::contracts::access::access_control::implementation::AccessControlInternalDefaultImpl::_get_role_admin_default_impl(self, role)
            }

            fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
                pendzl::contracts::access::access_control::implementation::AccessControlInternalDefaultImpl::_set_role_admin_default_impl(self, role, new_admin);
            }

            fn _ensure_has_role(&self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access::access_control::implementation::AccessControlInternalDefaultImpl::_ensure_has_role_default_impl(self, role, account)
            }

        }
    ))
    .expect("Should parse");

    let access_control_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::access_control::implementation::AccessControlDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut access_control = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::access_control::AccessControl for #storage_struct_name {
            #[ink(message)]
            fn has_role(&self, role: RoleType, address: Option<AccountId>) -> bool {
                pendzl::contracts::access::access_control::implementation::AccessControlDefaultImpl::has_role_default_impl(self, role, address)
            }

            #[ink(message)]
            fn get_role_admin(&self, role: RoleType) -> RoleType {
                pendzl::contracts::access::access_control::implementation::AccessControlDefaultImpl::get_role_admin_default_impl(self, role)
            }

            #[ink(message)]
            fn grant_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access::access_control::implementation::AccessControlDefaultImpl::grant_role_default_impl(self, role, account)
            }

            #[ink(message)]
            fn revoke_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access::access_control::implementation::AccessControlDefaultImpl::revoke_role_default_impl(self, role, account)
            }

            #[ink(message)]
            fn renounce_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access::access_control::implementation::AccessControlDefaultImpl::renounce_role_default_impl(self, role, account)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::access::access_control::*;
    ))
    .expect("Should parse");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::access::access_control::implementation::Data as AccessControlData;
    ))
    .expect("Should parse import");
        
    impl_args.imports.insert("AccessControl", import);
    impl_args.imports.insert("AccessControlData", import_data);

    override_functions("AccessControlInternal", &mut internal, impl_args.map);
    override_functions("AccessControl", &mut access_control, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(access_control_default_impl));
    impl_args.items.push(syn::Item::Impl(access_control));
}

pub(crate) fn impl_pausable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::security::pausable::implementation::PausableInternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::security::pausable::PausableInternal for #storage_struct_name {
            fn _paused(&self) -> bool {
                pendzl::contracts::security::pausable::implementation::PausableInternalDefaultImpl::_paused_default_impl(self)
            }

            fn _pause(&mut self) -> Result<(), PausableError> {
                pendzl::contracts::security::pausable::implementation::PausableInternalDefaultImpl::_pause_default_impl(self)
            }

            fn _unpause(&mut self) -> Result<(), PausableError> {
                pendzl::contracts::security::pausable::implementation::PausableInternalDefaultImpl::_unpause_default_impl(self)
            }

            fn _ensure_paused(&self) -> Result<(), PausableError> {
                pendzl::contracts::security::pausable::implementation::PausableInternalDefaultImpl::_ensure_paused_default_impl(self)
            }

            fn _ensure_not_paused(&self) -> Result<(), PausableError> {
                pendzl::contracts::security::pausable::implementation::PausableInternalDefaultImpl::_ensure_not_paused_default_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let pausable_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl  pendzl::contracts::security::pausable::implementation::PausableDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut pausable = syn::parse2::<syn::ItemImpl>(quote!(
        impl  pendzl::contracts::security::pausable::Pausable for #storage_struct_name {
            #[ink(message)]
            fn paused(&self) -> bool {
                pendzl::contracts::security::pausable::implementation::PausableDefaultImpl::paused_default_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::security::pausable::*;
    ))
    .expect("Should parse import");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
      pub use pendzl::contracts::security::pausable::implementation::Data as PausableData;
    ))
    .expect("Should parse import");
    impl_args.imports.insert("Pausable", import);
    impl_args.imports.insert("PausableData", import_data);

    override_functions("PausableInternal", &mut internal, impl_args.map);
    override_functions("Pausable", &mut pausable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(pausable_default_impl));
    impl_args.items.push(syn::Item::Impl(pausable));
}

fn override_functions(trait_name: &str, implementation: &mut syn::ItemImpl, map: &OverridenFnMap) {
    if let Some(overrides) = map.get(trait_name) {
        // we will find which fns we wanna override
        for (fn_name, (fn_code, attributes, is_default)) in overrides {
            for item in implementation.items.iter_mut() {
                if let syn::ImplItem::Method(method) = item {
                    if &method.sig.ident.to_string() == fn_name {
                        if !is_default {
                            method.block = *fn_code.clone();
                        }
                        method.attrs.append(&mut attributes.to_vec());
                    }
                }
            }
        }
    }
}
