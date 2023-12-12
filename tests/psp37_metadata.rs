// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#[cfg(feature = "psp37")]
#[pendzl::implementation(PSP37, PSP37Metadata)]
#[ink::contract]
mod psp37_metadata {
    use ink::codegen::{EmitEvent, Env};
    use pendzl::traits::{Storage, String};

    #[ink(event)]
    pub struct AttributeSet {
        id: Id,
        key: String,
        data: String,
    }

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct PSP37Struct {
        #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    #[overrider(metadata::Internal)]
    fn _emit_attribute_set_event(&self, id: Id, key: String, data: String) {
        self.env().emit_event(AttributeSet {
            id: id.clone(),
            key: key.to_string(),
            data: data.to_string(),
        });
    }

    impl PSP37Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn set_attribute(
            &mut self,
            id: Id,
            key: String,
            data: String,
        ) -> Result<(), PSP37Error> {
            metadata::Internal::_set_attribute(self, id, &key, &data)
        }
    }

    #[ink::test]
    fn metadata_works() {
        let mut nft = PSP37Struct::new();

        assert!(nft
            .set_attribute(Id::U128(1), String::from("name"), String::from("TKN"))
            .is_ok());

        assert_eq!(
            PSP37Metadata::get_attribute(&nft, Id::U128(1), String::from("name")),
            Some(String::from("TKN"))
        );
    }
}
