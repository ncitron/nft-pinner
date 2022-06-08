pub use erc721_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod erc721_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "ERC721 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ERC721_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"approved\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"Approval\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\"}],\"name\":\"ApprovalForAll\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnershipTransferred\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"Transfer\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"MAX_PUBLIC_MINT\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"MAX_SUPPLY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"PRICE_PER_TOKEN\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"PROVENANCE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"approve\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"getApproved\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"isAllowListActive\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"}],\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numberOfTokens\",\"type\":\"uint256\"}],\"name\":\"mint\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"numberOfTokens\",\"type\":\"uint8\"}],\"name\":\"mintAllowList\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\"}],\"name\":\"numAvailableToMint\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"ownerOf\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"renounceOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"n\",\"type\":\"uint256\"}],\"name\":\"reserve\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"safeTransferFrom\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\"}],\"name\":\"safeTransferFrom\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"saleIsActive\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"addresses\",\"type\":\"address[]\"},{\"internalType\":\"uint8\",\"name\":\"numAllowedToMint\",\"type\":\"uint8\"}],\"name\":\"setAllowList\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\"}],\"name\":\"setApprovalForAll\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"baseURI_\",\"type\":\"string\"}],\"name\":\"setBaseURI\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_isAllowListActive\",\"type\":\"bool\"}],\"name\":\"setIsAllowListActive\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"provenance\",\"type\":\"string\"}],\"name\":\"setProvenance\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"newState\",\"type\":\"bool\"}],\"name\":\"setSaleState\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\"}],\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"}],\"name\":\"tokenByIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"}],\"name\":\"tokenOfOwnerByIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"tokenURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"transferFrom\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"withdraw\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]\n") . expect ("invalid abi")
        });
    pub struct ERC721<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ERC721<M> {
        fn clone(&self) -> Self {
            ERC721(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ERC721<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ERC721<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ERC721))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ERC721<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ERC721_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `MAX_PUBLIC_MINT` (0x65f13097) function"]
        pub fn max_public_mint(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([101, 241, 48, 151], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_SUPPLY` (0x32cb6b0c) function"]
        pub fn max_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([50, 203, 107, 12], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PRICE_PER_TOKEN` (0x833b9499) function"]
        pub fn price_per_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([131, 59, 148, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PROVENANCE` (0x6373a6b1) function"]
        pub fn provenance(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([99, 115, 166, 177], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getApproved` (0x081812fc) function"]
        pub fn get_approved(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isAllowListActive` (0x29fc6bae) function"]
        pub fn is_allow_list_active(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([41, 252, 107, 174], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isApprovedForAll` (0xe985e9c5) function"]
        pub fn is_approved_for_all(
            &self,
            owner: ethers::core::types::Address,
            operator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0xa0712d68) function"]
        pub fn mint(
            &self,
            number_of_tokens: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 113, 45, 104], number_of_tokens)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintAllowList` (0xddff5b1c) function"]
        pub fn mint_allow_list(
            &self,
            number_of_tokens: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 255, 91, 28], number_of_tokens)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `numAvailableToMint` (0xc04a2836) function"]
        pub fn num_available_to_mint(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([192, 74, 40, 54], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ownerOf` (0x6352211e) function"]
        pub fn owner_of(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `reserve` (0x819b25ba) function"]
        pub fn reserve(
            &self,
            n: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 155, 37, 186], n)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0x42842e0e) function"]
        pub fn safe_transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0xb88d4fde) function"]
        pub fn safe_transfer_from_with_data(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `saleIsActive` (0xeb8d2444) function"]
        pub fn sale_is_active(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([235, 141, 36, 68], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAllowList` (0x8295784d) function"]
        pub fn set_allow_list(
            &self,
            addresses: ::std::vec::Vec<ethers::core::types::Address>,
            num_allowed_to_mint: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 149, 120, 77], (addresses, num_allowed_to_mint))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setApprovalForAll` (0xa22cb465) function"]
        pub fn set_approval_for_all(
            &self,
            operator: ethers::core::types::Address,
            approved: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBaseURI` (0x55f804b3) function"]
        pub fn set_base_uri(
            &self,
            base_uri: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 248, 4, 179], base_uri)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setIsAllowListActive` (0x718bc4af) function"]
        pub fn set_is_allow_list_active(
            &self,
            is_allow_list_active: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 139, 196, 175], is_allow_list_active)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setProvenance` (0xffe630b5) function"]
        pub fn set_provenance(
            &self,
            provenance: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 230, 48, 181], provenance)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSaleState` (0xc4e37095) function"]
        pub fn set_sale_state(
            &self,
            new_state: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 227, 112, 149], new_state)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenByIndex` (0x4f6ccce7) function"]
        pub fn token_by_index(
            &self,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([79, 108, 204, 231], index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenOfOwnerByIndex` (0x2f745c59) function"]
        pub fn token_of_owner_by_index(
            &self,
            owner: ethers::core::types::Address,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([47, 116, 92, 89], (owner, index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenURI` (0xc87b56dd) function"]
        pub fn token_uri(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x3ccfd60b) function"]
        pub fn withdraw(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 207, 214, 11], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ApprovalForAll` event"]
        pub fn approval_for_all_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ApprovalForAllFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ERC721Events> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ERC721<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub approved: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        pub approved: bool,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC721Events {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for ERC721Events {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC721Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ERC721Events::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ERC721Events::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ERC721Events::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ERC721Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC721Events::ApprovalFilter(element) => element.fmt(f),
                ERC721Events::ApprovalForAllFilter(element) => element.fmt(f),
                ERC721Events::OwnershipTransferredFilter(element) => element.fmt(f),
                ERC721Events::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `MAX_PUBLIC_MINT`function with signature `MAX_PUBLIC_MINT()` and selector `[101, 241, 48, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "MAX_PUBLIC_MINT", abi = "MAX_PUBLIC_MINT()")]
    pub struct MaxPublicMintCall;
    #[doc = "Container type for all input parameters for the `MAX_SUPPLY`function with signature `MAX_SUPPLY()` and selector `[50, 203, 107, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "MAX_SUPPLY", abi = "MAX_SUPPLY()")]
    pub struct MaxSupplyCall;
    #[doc = "Container type for all input parameters for the `PRICE_PER_TOKEN`function with signature `PRICE_PER_TOKEN()` and selector `[131, 59, 148, 153]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "PRICE_PER_TOKEN", abi = "PRICE_PER_TOKEN()")]
    pub struct PricePerTokenCall;
    #[doc = "Container type for all input parameters for the `PROVENANCE`function with signature `PROVENANCE()` and selector `[99, 115, 166, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "PROVENANCE", abi = "PROVENANCE()")]
    pub struct ProvenanceCall;
    #[doc = "Container type for all input parameters for the `approve`function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getApproved`function with signature `getApproved(uint256)` and selector `[8, 24, 18, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isAllowListActive`function with signature `isAllowListActive()` and selector `[41, 252, 107, 174]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isAllowListActive", abi = "isAllowListActive()")]
    pub struct IsAllowListActiveCall;
    #[doc = "Container type for all input parameters for the `isApprovedForAll`function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ethers::core::types::Address,
        pub operator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(uint256)` and selector `[160, 113, 45, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mint", abi = "mint(uint256)")]
    pub struct MintCall {
        pub number_of_tokens: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mintAllowList`function with signature `mintAllowList(uint8)` and selector `[221, 255, 91, 28]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mintAllowList", abi = "mintAllowList(uint8)")]
    pub struct MintAllowListCall {
        pub number_of_tokens: u8,
    }
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `numAvailableToMint`function with signature `numAvailableToMint(address)` and selector `[192, 74, 40, 54]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "numAvailableToMint", abi = "numAvailableToMint(address)")]
    pub struct NumAvailableToMintCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `owner`function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `ownerOf`function with signature `ownerOf(uint256)` and selector `[99, 82, 33, 30]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `renounceOwnership`function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `reserve`function with signature `reserve(uint256)` and selector `[129, 155, 37, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "reserve", abi = "reserve(uint256)")]
    pub struct ReserveCall {
        pub n: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `safeTransferFrom`function with signature `safeTransferFrom(address,address,uint256)` and selector `[66, 132, 46, 14]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `safeTransferFrom`function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `[184, 141, 79, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithDataCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `saleIsActive`function with signature `saleIsActive()` and selector `[235, 141, 36, 68]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "saleIsActive", abi = "saleIsActive()")]
    pub struct SaleIsActiveCall;
    #[doc = "Container type for all input parameters for the `setAllowList`function with signature `setAllowList(address[],uint8)` and selector `[130, 149, 120, 77]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setAllowList", abi = "setAllowList(address[],uint8)")]
    pub struct SetAllowListCall {
        pub addresses: ::std::vec::Vec<ethers::core::types::Address>,
        pub num_allowed_to_mint: u8,
    }
    #[doc = "Container type for all input parameters for the `setApprovalForAll`function with signature `setApprovalForAll(address,bool)` and selector `[162, 44, 180, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ethers::core::types::Address,
        pub approved: bool,
    }
    #[doc = "Container type for all input parameters for the `setBaseURI`function with signature `setBaseURI(string)` and selector `[85, 248, 4, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setBaseURI", abi = "setBaseURI(string)")]
    pub struct SetBaseURICall {
        pub base_uri: String,
    }
    #[doc = "Container type for all input parameters for the `setIsAllowListActive`function with signature `setIsAllowListActive(bool)` and selector `[113, 139, 196, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setIsAllowListActive", abi = "setIsAllowListActive(bool)")]
    pub struct SetIsAllowListActiveCall {
        pub is_allow_list_active: bool,
    }
    #[doc = "Container type for all input parameters for the `setProvenance`function with signature `setProvenance(string)` and selector `[255, 230, 48, 181]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setProvenance", abi = "setProvenance(string)")]
    pub struct SetProvenanceCall {
        pub provenance: String,
    }
    #[doc = "Container type for all input parameters for the `setSaleState`function with signature `setSaleState(bool)` and selector `[196, 227, 112, 149]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setSaleState", abi = "setSaleState(bool)")]
    pub struct SetSaleStateCall {
        pub new_state: bool,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface`function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `symbol`function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `tokenByIndex`function with signature `tokenByIndex(uint256)` and selector `[79, 108, 204, 231]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tokenByIndex", abi = "tokenByIndex(uint256)")]
    pub struct TokenByIndexCall {
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `tokenOfOwnerByIndex`function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `[47, 116, 92, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "tokenOfOwnerByIndex",
        abi = "tokenOfOwnerByIndex(address,uint256)"
    )]
    pub struct TokenOfOwnerByIndexCall {
        pub owner: ethers::core::types::Address,
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `tokenURI`function with signature `tokenURI(uint256)` and selector `[200, 123, 86, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transferFrom`function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership`function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw()` and selector `[60, 207, 214, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw()")]
    pub struct WithdrawCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC721Calls {
        MaxPublicMint(MaxPublicMintCall),
        MaxSupply(MaxSupplyCall),
        PricePerToken(PricePerTokenCall),
        Provenance(ProvenanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        GetApproved(GetApprovedCall),
        IsAllowListActive(IsAllowListActiveCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Mint(MintCall),
        MintAllowList(MintAllowListCall),
        Name(NameCall),
        NumAvailableToMint(NumAvailableToMintCall),
        Owner(OwnerCall),
        OwnerOf(OwnerOfCall),
        RenounceOwnership(RenounceOwnershipCall),
        Reserve(ReserveCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithData(SafeTransferFromWithDataCall),
        SaleIsActive(SaleIsActiveCall),
        SetAllowList(SetAllowListCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SetBaseURI(SetBaseURICall),
        SetIsAllowListActive(SetIsAllowListActiveCall),
        SetProvenance(SetProvenanceCall),
        SetSaleState(SetSaleStateCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenByIndex(TokenByIndexCall),
        TokenOfOwnerByIndex(TokenOfOwnerByIndexCall),
        TokenURI(TokenURICall),
        TotalSupply(TotalSupplyCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for ERC721Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <MaxPublicMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::MaxPublicMint(decoded));
            }
            if let Ok(decoded) =
                <MaxSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::MaxSupply(decoded));
            }
            if let Ok(decoded) =
                <PricePerTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::PricePerToken(decoded));
            }
            if let Ok(decoded) =
                <ProvenanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::Provenance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <GetApprovedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::GetApproved(decoded));
            }
            if let Ok(decoded) =
                <IsAllowListActiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::IsAllowListActive(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ERC721Calls::Mint(decoded));
            }
            if let Ok(decoded) =
                <MintAllowListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::MintAllowList(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ERC721Calls::Name(decoded));
            }
            if let Ok(decoded) =
                <NumAvailableToMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::NumAvailableToMint(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::Owner(decoded));
            }
            if let Ok(decoded) =
                <OwnerOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::OwnerOf(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <ReserveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::Reserve(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromWithDataCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ERC721Calls::SafeTransferFromWithData(decoded));
            }
            if let Ok(decoded) =
                <SaleIsActiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::SaleIsActive(decoded));
            }
            if let Ok(decoded) =
                <SetAllowListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::SetAllowList(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SetBaseURICall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::SetBaseURI(decoded));
            }
            if let Ok(decoded) =
                <SetIsAllowListActiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::SetIsAllowListActive(decoded));
            }
            if let Ok(decoded) =
                <SetProvenanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::SetProvenance(decoded));
            }
            if let Ok(decoded) =
                <SetSaleStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::SetSaleState(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TokenByIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::TokenByIndex(decoded));
            }
            if let Ok(decoded) =
                <TokenOfOwnerByIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::TokenOfOwnerByIndex(decoded));
            }
            if let Ok(decoded) =
                <TokenURICall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::TokenURI(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ERC721Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                ERC721Calls::MaxPublicMint(element) => element.encode(),
                ERC721Calls::MaxSupply(element) => element.encode(),
                ERC721Calls::PricePerToken(element) => element.encode(),
                ERC721Calls::Provenance(element) => element.encode(),
                ERC721Calls::Approve(element) => element.encode(),
                ERC721Calls::BalanceOf(element) => element.encode(),
                ERC721Calls::GetApproved(element) => element.encode(),
                ERC721Calls::IsAllowListActive(element) => element.encode(),
                ERC721Calls::IsApprovedForAll(element) => element.encode(),
                ERC721Calls::Mint(element) => element.encode(),
                ERC721Calls::MintAllowList(element) => element.encode(),
                ERC721Calls::Name(element) => element.encode(),
                ERC721Calls::NumAvailableToMint(element) => element.encode(),
                ERC721Calls::Owner(element) => element.encode(),
                ERC721Calls::OwnerOf(element) => element.encode(),
                ERC721Calls::RenounceOwnership(element) => element.encode(),
                ERC721Calls::Reserve(element) => element.encode(),
                ERC721Calls::SafeTransferFrom(element) => element.encode(),
                ERC721Calls::SafeTransferFromWithData(element) => element.encode(),
                ERC721Calls::SaleIsActive(element) => element.encode(),
                ERC721Calls::SetAllowList(element) => element.encode(),
                ERC721Calls::SetApprovalForAll(element) => element.encode(),
                ERC721Calls::SetBaseURI(element) => element.encode(),
                ERC721Calls::SetIsAllowListActive(element) => element.encode(),
                ERC721Calls::SetProvenance(element) => element.encode(),
                ERC721Calls::SetSaleState(element) => element.encode(),
                ERC721Calls::SupportsInterface(element) => element.encode(),
                ERC721Calls::Symbol(element) => element.encode(),
                ERC721Calls::TokenByIndex(element) => element.encode(),
                ERC721Calls::TokenOfOwnerByIndex(element) => element.encode(),
                ERC721Calls::TokenURI(element) => element.encode(),
                ERC721Calls::TotalSupply(element) => element.encode(),
                ERC721Calls::TransferFrom(element) => element.encode(),
                ERC721Calls::TransferOwnership(element) => element.encode(),
                ERC721Calls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ERC721Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC721Calls::MaxPublicMint(element) => element.fmt(f),
                ERC721Calls::MaxSupply(element) => element.fmt(f),
                ERC721Calls::PricePerToken(element) => element.fmt(f),
                ERC721Calls::Provenance(element) => element.fmt(f),
                ERC721Calls::Approve(element) => element.fmt(f),
                ERC721Calls::BalanceOf(element) => element.fmt(f),
                ERC721Calls::GetApproved(element) => element.fmt(f),
                ERC721Calls::IsAllowListActive(element) => element.fmt(f),
                ERC721Calls::IsApprovedForAll(element) => element.fmt(f),
                ERC721Calls::Mint(element) => element.fmt(f),
                ERC721Calls::MintAllowList(element) => element.fmt(f),
                ERC721Calls::Name(element) => element.fmt(f),
                ERC721Calls::NumAvailableToMint(element) => element.fmt(f),
                ERC721Calls::Owner(element) => element.fmt(f),
                ERC721Calls::OwnerOf(element) => element.fmt(f),
                ERC721Calls::RenounceOwnership(element) => element.fmt(f),
                ERC721Calls::Reserve(element) => element.fmt(f),
                ERC721Calls::SafeTransferFrom(element) => element.fmt(f),
                ERC721Calls::SafeTransferFromWithData(element) => element.fmt(f),
                ERC721Calls::SaleIsActive(element) => element.fmt(f),
                ERC721Calls::SetAllowList(element) => element.fmt(f),
                ERC721Calls::SetApprovalForAll(element) => element.fmt(f),
                ERC721Calls::SetBaseURI(element) => element.fmt(f),
                ERC721Calls::SetIsAllowListActive(element) => element.fmt(f),
                ERC721Calls::SetProvenance(element) => element.fmt(f),
                ERC721Calls::SetSaleState(element) => element.fmt(f),
                ERC721Calls::SupportsInterface(element) => element.fmt(f),
                ERC721Calls::Symbol(element) => element.fmt(f),
                ERC721Calls::TokenByIndex(element) => element.fmt(f),
                ERC721Calls::TokenOfOwnerByIndex(element) => element.fmt(f),
                ERC721Calls::TokenURI(element) => element.fmt(f),
                ERC721Calls::TotalSupply(element) => element.fmt(f),
                ERC721Calls::TransferFrom(element) => element.fmt(f),
                ERC721Calls::TransferOwnership(element) => element.fmt(f),
                ERC721Calls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<MaxPublicMintCall> for ERC721Calls {
        fn from(var: MaxPublicMintCall) -> Self {
            ERC721Calls::MaxPublicMint(var)
        }
    }
    impl ::std::convert::From<MaxSupplyCall> for ERC721Calls {
        fn from(var: MaxSupplyCall) -> Self {
            ERC721Calls::MaxSupply(var)
        }
    }
    impl ::std::convert::From<PricePerTokenCall> for ERC721Calls {
        fn from(var: PricePerTokenCall) -> Self {
            ERC721Calls::PricePerToken(var)
        }
    }
    impl ::std::convert::From<ProvenanceCall> for ERC721Calls {
        fn from(var: ProvenanceCall) -> Self {
            ERC721Calls::Provenance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for ERC721Calls {
        fn from(var: ApproveCall) -> Self {
            ERC721Calls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for ERC721Calls {
        fn from(var: BalanceOfCall) -> Self {
            ERC721Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<GetApprovedCall> for ERC721Calls {
        fn from(var: GetApprovedCall) -> Self {
            ERC721Calls::GetApproved(var)
        }
    }
    impl ::std::convert::From<IsAllowListActiveCall> for ERC721Calls {
        fn from(var: IsAllowListActiveCall) -> Self {
            ERC721Calls::IsAllowListActive(var)
        }
    }
    impl ::std::convert::From<IsApprovedForAllCall> for ERC721Calls {
        fn from(var: IsApprovedForAllCall) -> Self {
            ERC721Calls::IsApprovedForAll(var)
        }
    }
    impl ::std::convert::From<MintCall> for ERC721Calls {
        fn from(var: MintCall) -> Self {
            ERC721Calls::Mint(var)
        }
    }
    impl ::std::convert::From<MintAllowListCall> for ERC721Calls {
        fn from(var: MintAllowListCall) -> Self {
            ERC721Calls::MintAllowList(var)
        }
    }
    impl ::std::convert::From<NameCall> for ERC721Calls {
        fn from(var: NameCall) -> Self {
            ERC721Calls::Name(var)
        }
    }
    impl ::std::convert::From<NumAvailableToMintCall> for ERC721Calls {
        fn from(var: NumAvailableToMintCall) -> Self {
            ERC721Calls::NumAvailableToMint(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ERC721Calls {
        fn from(var: OwnerCall) -> Self {
            ERC721Calls::Owner(var)
        }
    }
    impl ::std::convert::From<OwnerOfCall> for ERC721Calls {
        fn from(var: OwnerOfCall) -> Self {
            ERC721Calls::OwnerOf(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for ERC721Calls {
        fn from(var: RenounceOwnershipCall) -> Self {
            ERC721Calls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<ReserveCall> for ERC721Calls {
        fn from(var: ReserveCall) -> Self {
            ERC721Calls::Reserve(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromCall> for ERC721Calls {
        fn from(var: SafeTransferFromCall) -> Self {
            ERC721Calls::SafeTransferFrom(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromWithDataCall> for ERC721Calls {
        fn from(var: SafeTransferFromWithDataCall) -> Self {
            ERC721Calls::SafeTransferFromWithData(var)
        }
    }
    impl ::std::convert::From<SaleIsActiveCall> for ERC721Calls {
        fn from(var: SaleIsActiveCall) -> Self {
            ERC721Calls::SaleIsActive(var)
        }
    }
    impl ::std::convert::From<SetAllowListCall> for ERC721Calls {
        fn from(var: SetAllowListCall) -> Self {
            ERC721Calls::SetAllowList(var)
        }
    }
    impl ::std::convert::From<SetApprovalForAllCall> for ERC721Calls {
        fn from(var: SetApprovalForAllCall) -> Self {
            ERC721Calls::SetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<SetBaseURICall> for ERC721Calls {
        fn from(var: SetBaseURICall) -> Self {
            ERC721Calls::SetBaseURI(var)
        }
    }
    impl ::std::convert::From<SetIsAllowListActiveCall> for ERC721Calls {
        fn from(var: SetIsAllowListActiveCall) -> Self {
            ERC721Calls::SetIsAllowListActive(var)
        }
    }
    impl ::std::convert::From<SetProvenanceCall> for ERC721Calls {
        fn from(var: SetProvenanceCall) -> Self {
            ERC721Calls::SetProvenance(var)
        }
    }
    impl ::std::convert::From<SetSaleStateCall> for ERC721Calls {
        fn from(var: SetSaleStateCall) -> Self {
            ERC721Calls::SetSaleState(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for ERC721Calls {
        fn from(var: SupportsInterfaceCall) -> Self {
            ERC721Calls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for ERC721Calls {
        fn from(var: SymbolCall) -> Self {
            ERC721Calls::Symbol(var)
        }
    }
    impl ::std::convert::From<TokenByIndexCall> for ERC721Calls {
        fn from(var: TokenByIndexCall) -> Self {
            ERC721Calls::TokenByIndex(var)
        }
    }
    impl ::std::convert::From<TokenOfOwnerByIndexCall> for ERC721Calls {
        fn from(var: TokenOfOwnerByIndexCall) -> Self {
            ERC721Calls::TokenOfOwnerByIndex(var)
        }
    }
    impl ::std::convert::From<TokenURICall> for ERC721Calls {
        fn from(var: TokenURICall) -> Self {
            ERC721Calls::TokenURI(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for ERC721Calls {
        fn from(var: TotalSupplyCall) -> Self {
            ERC721Calls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for ERC721Calls {
        fn from(var: TransferFromCall) -> Self {
            ERC721Calls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for ERC721Calls {
        fn from(var: TransferOwnershipCall) -> Self {
            ERC721Calls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for ERC721Calls {
        fn from(var: WithdrawCall) -> Self {
            ERC721Calls::Withdraw(var)
        }
    }
}
