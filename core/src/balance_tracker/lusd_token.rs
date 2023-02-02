pub use lusd_token::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod lusd_token {
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
    #[doc = "LUSDToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static LUSDTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_troveManagerAddress\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_stabilityPoolAddress\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_borrowerOperationsAddress\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Approval\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"_newBorrowerOperationsAddress\",\"type\":\"address\"}],\"name\":\"BorrowerOperationsAddressChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"LUSDTokenBalanceUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"_newStabilityPoolAddress\",\"type\":\"address\"}],\"name\":\"StabilityPoolAddressChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Transfer\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"_troveManagerAddress\",\"type\":\"address\"}],\"name\":\"TroveManagerAddressChanged\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"}],\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"borrowerOperationsAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"burn\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"subtractedValue\",\"type\":\"uint256\"}],\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"domainSeparator\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"addedValue\",\"type\":\"uint256\"}],\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"mint\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"permit\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"permitTypeHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_poolAddress\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_receiver\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"returnFromPool\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_sender\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_poolAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"sendToPool\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"stabilityPoolAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"troveManagerAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"version\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]") . expect ("invalid abi")
        });
    pub struct LUSDToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for LUSDToken<M> {
        fn clone(&self) -> Self {
            LUSDToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for LUSDToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for LUSDToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(LUSDToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> LUSDToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), LUSDTOKEN_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowerOperationsAddress` (0xb7f8cf9b) function"]
        pub fn borrower_operations_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([183, 248, 207, 155], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x9dc29fac) function"]
        pub fn burn(
            &self,
            account: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 194, 159, 172], (account, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(
            &self,
            spender: ethers::core::types::Address,
            subtracted_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `domainSeparator` (0xf698da25) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([246, 152, 218, 37], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(
            &self,
            spender: ethers::core::types::Address,
            added_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x40c10f19) function"]
        pub fn mint(
            &self,
            account: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (account, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit` (0xd505accf) function"]
        pub fn permit(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, amount, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permitTypeHash` (0x10ce43bd) function"]
        pub fn permit_type_hash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([16, 206, 67, 189], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `returnFromPool` (0x20c582be) function"]
        pub fn return_from_pool(
            &self,
            pool_address: ethers::core::types::Address,
            receiver: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 197, 130, 190], (pool_address, receiver, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sendToPool` (0xbb997bac) function"]
        pub fn send_to_pool(
            &self,
            sender: ethers::core::types::Address,
            pool_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 153, 123, 172], (sender, pool_address, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stabilityPoolAddress` (0x0b622ab2) function"]
        pub fn stability_pool_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([11, 98, 42, 178], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
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
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            sender: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `troveManagerAddress` (0x5a4d28bb) function"]
        pub fn trove_manager_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([90, 77, 40, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BorrowerOperationsAddressChanged` event"]
        pub fn borrower_operations_address_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BorrowerOperationsAddressChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LUSDTokenBalanceUpdated` event"]
        pub fn lusd_token_balance_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LusdtokenBalanceUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `StabilityPoolAddressChanged` event"]
        pub fn stability_pool_address_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, StabilityPoolAddressChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TroveManagerAddressChanged` event"]
        pub fn trove_manager_address_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TroveManagerAddressChangedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, LUSDTokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for LUSDToken<M> {
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
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
        name = "BorrowerOperationsAddressChanged",
        abi = "BorrowerOperationsAddressChanged(address)"
    )]
    pub struct BorrowerOperationsAddressChangedFilter {
        pub new_borrower_operations_address: ethers::core::types::Address,
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
        name = "LUSDTokenBalanceUpdated",
        abi = "LUSDTokenBalanceUpdated(address,uint256)"
    )]
    pub struct LusdtokenBalanceUpdatedFilter {
        pub user: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
        name = "StabilityPoolAddressChanged",
        abi = "StabilityPoolAddressChanged(address)"
    )]
    pub struct StabilityPoolAddressChangedFilter {
        pub new_stability_pool_address: ethers::core::types::Address,
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
        pub value: ethers::core::types::U256,
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
        name = "TroveManagerAddressChanged",
        abi = "TroveManagerAddressChanged(address)"
    )]
    pub struct TroveManagerAddressChangedFilter {
        pub trove_manager_address: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LUSDTokenEvents {
        ApprovalFilter(ApprovalFilter),
        BorrowerOperationsAddressChangedFilter(BorrowerOperationsAddressChangedFilter),
        LusdtokenBalanceUpdatedFilter(LusdtokenBalanceUpdatedFilter),
        StabilityPoolAddressChangedFilter(StabilityPoolAddressChangedFilter),
        TransferFilter(TransferFilter),
        TroveManagerAddressChangedFilter(TroveManagerAddressChangedFilter),
    }
    impl ethers::contract::EthLogDecode for LUSDTokenEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(LUSDTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BorrowerOperationsAddressChangedFilter::decode_log(log) {
                return Ok(LUSDTokenEvents::BorrowerOperationsAddressChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LusdtokenBalanceUpdatedFilter::decode_log(log) {
                return Ok(LUSDTokenEvents::LusdtokenBalanceUpdatedFilter(decoded));
            }
            if let Ok(decoded) = StabilityPoolAddressChangedFilter::decode_log(log) {
                return Ok(LUSDTokenEvents::StabilityPoolAddressChangedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(LUSDTokenEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = TroveManagerAddressChangedFilter::decode_log(log) {
                return Ok(LUSDTokenEvents::TroveManagerAddressChangedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for LUSDTokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LUSDTokenEvents::ApprovalFilter(element) => element.fmt(f),
                LUSDTokenEvents::BorrowerOperationsAddressChangedFilter(element) => element.fmt(f),
                LUSDTokenEvents::LusdtokenBalanceUpdatedFilter(element) => element.fmt(f),
                LUSDTokenEvents::StabilityPoolAddressChangedFilter(element) => element.fmt(f),
                LUSDTokenEvents::TransferFilter(element) => element.fmt(f),
                LUSDTokenEvents::TroveManagerAddressChangedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
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
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
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
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `borrowerOperationsAddress` function with signature `borrowerOperationsAddress()` and selector `[183, 248, 207, 155]`"]
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
        name = "borrowerOperationsAddress",
        abi = "borrowerOperationsAddress()"
    )]
    pub struct BorrowerOperationsAddressCall;
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(address,uint256)` and selector `[157, 194, 159, 172]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "burn", abi = "burn(address,uint256)")]
    pub struct BurnCall {
        pub account: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub subtracted_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `domainSeparator` function with signature `domainSeparator()` and selector `[246, 152, 218, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "domainSeparator", abi = "domainSeparator()")]
    pub struct DomainSeparatorCall;
    #[doc = "Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub added_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `[64, 193, 15, 25]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub account: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
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
    #[doc = "Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[213, 5, 172, 207]`"]
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `permitTypeHash` function with signature `permitTypeHash()` and selector `[16, 206, 67, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "permitTypeHash", abi = "permitTypeHash()")]
    pub struct PermitTypeHashCall;
    #[doc = "Container type for all input parameters for the `returnFromPool` function with signature `returnFromPool(address,address,uint256)` and selector `[32, 197, 130, 190]`"]
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
        name = "returnFromPool",
        abi = "returnFromPool(address,address,uint256)"
    )]
    pub struct ReturnFromPoolCall {
        pub pool_address: ethers::core::types::Address,
        pub receiver: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `sendToPool` function with signature `sendToPool(address,address,uint256)` and selector `[187, 153, 123, 172]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "sendToPool", abi = "sendToPool(address,address,uint256)")]
    pub struct SendToPoolCall {
        pub sender: ethers::core::types::Address,
        pub pool_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `stabilityPoolAddress` function with signature `stabilityPoolAddress()` and selector `[11, 98, 42, 178]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "stabilityPoolAddress", abi = "stabilityPoolAddress()")]
    pub struct StabilityPoolAddressCall;
    #[doc = "Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
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
    #[doc = "Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
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
    #[doc = "Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
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
        pub sender: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `troveManagerAddress` function with signature `troveManagerAddress()` and selector `[90, 77, 40, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "troveManagerAddress", abi = "troveManagerAddress()")]
    pub struct TroveManagerAddressCall;
    #[doc = "Container type for all input parameters for the `version` function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LUSDTokenCalls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        BorrowerOperationsAddress(BorrowerOperationsAddressCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        DomainSeparator(DomainSeparatorCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        PermitTypeHash(PermitTypeHashCall),
        ReturnFromPool(ReturnFromPoolCall),
        SendToPool(SendToPoolCall),
        StabilityPoolAddress(StabilityPoolAddressCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TroveManagerAddress(TroveManagerAddressCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for LUSDTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BorrowerOperationsAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(LUSDTokenCalls::BorrowerOperationsAddress(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LUSDTokenCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LUSDTokenCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LUSDTokenCalls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::Nonces(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::Permit(decoded));
            }
            if let Ok(decoded) =
                <PermitTypeHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::PermitTypeHash(decoded));
            }
            if let Ok(decoded) =
                <ReturnFromPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::ReturnFromPool(decoded));
            }
            if let Ok(decoded) =
                <SendToPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::SendToPool(decoded));
            }
            if let Ok(decoded) =
                <StabilityPoolAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::StabilityPoolAddress(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TroveManagerAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::TroveManagerAddress(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LUSDTokenCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for LUSDTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                LUSDTokenCalls::Allowance(element) => element.encode(),
                LUSDTokenCalls::Approve(element) => element.encode(),
                LUSDTokenCalls::BalanceOf(element) => element.encode(),
                LUSDTokenCalls::BorrowerOperationsAddress(element) => element.encode(),
                LUSDTokenCalls::Burn(element) => element.encode(),
                LUSDTokenCalls::Decimals(element) => element.encode(),
                LUSDTokenCalls::DecreaseAllowance(element) => element.encode(),
                LUSDTokenCalls::DomainSeparator(element) => element.encode(),
                LUSDTokenCalls::IncreaseAllowance(element) => element.encode(),
                LUSDTokenCalls::Mint(element) => element.encode(),
                LUSDTokenCalls::Name(element) => element.encode(),
                LUSDTokenCalls::Nonces(element) => element.encode(),
                LUSDTokenCalls::Permit(element) => element.encode(),
                LUSDTokenCalls::PermitTypeHash(element) => element.encode(),
                LUSDTokenCalls::ReturnFromPool(element) => element.encode(),
                LUSDTokenCalls::SendToPool(element) => element.encode(),
                LUSDTokenCalls::StabilityPoolAddress(element) => element.encode(),
                LUSDTokenCalls::Symbol(element) => element.encode(),
                LUSDTokenCalls::TotalSupply(element) => element.encode(),
                LUSDTokenCalls::Transfer(element) => element.encode(),
                LUSDTokenCalls::TransferFrom(element) => element.encode(),
                LUSDTokenCalls::TroveManagerAddress(element) => element.encode(),
                LUSDTokenCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for LUSDTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LUSDTokenCalls::Allowance(element) => element.fmt(f),
                LUSDTokenCalls::Approve(element) => element.fmt(f),
                LUSDTokenCalls::BalanceOf(element) => element.fmt(f),
                LUSDTokenCalls::BorrowerOperationsAddress(element) => element.fmt(f),
                LUSDTokenCalls::Burn(element) => element.fmt(f),
                LUSDTokenCalls::Decimals(element) => element.fmt(f),
                LUSDTokenCalls::DecreaseAllowance(element) => element.fmt(f),
                LUSDTokenCalls::DomainSeparator(element) => element.fmt(f),
                LUSDTokenCalls::IncreaseAllowance(element) => element.fmt(f),
                LUSDTokenCalls::Mint(element) => element.fmt(f),
                LUSDTokenCalls::Name(element) => element.fmt(f),
                LUSDTokenCalls::Nonces(element) => element.fmt(f),
                LUSDTokenCalls::Permit(element) => element.fmt(f),
                LUSDTokenCalls::PermitTypeHash(element) => element.fmt(f),
                LUSDTokenCalls::ReturnFromPool(element) => element.fmt(f),
                LUSDTokenCalls::SendToPool(element) => element.fmt(f),
                LUSDTokenCalls::StabilityPoolAddress(element) => element.fmt(f),
                LUSDTokenCalls::Symbol(element) => element.fmt(f),
                LUSDTokenCalls::TotalSupply(element) => element.fmt(f),
                LUSDTokenCalls::Transfer(element) => element.fmt(f),
                LUSDTokenCalls::TransferFrom(element) => element.fmt(f),
                LUSDTokenCalls::TroveManagerAddress(element) => element.fmt(f),
                LUSDTokenCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AllowanceCall> for LUSDTokenCalls {
        fn from(var: AllowanceCall) -> Self {
            LUSDTokenCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for LUSDTokenCalls {
        fn from(var: ApproveCall) -> Self {
            LUSDTokenCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for LUSDTokenCalls {
        fn from(var: BalanceOfCall) -> Self {
            LUSDTokenCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BorrowerOperationsAddressCall> for LUSDTokenCalls {
        fn from(var: BorrowerOperationsAddressCall) -> Self {
            LUSDTokenCalls::BorrowerOperationsAddress(var)
        }
    }
    impl ::std::convert::From<BurnCall> for LUSDTokenCalls {
        fn from(var: BurnCall) -> Self {
            LUSDTokenCalls::Burn(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for LUSDTokenCalls {
        fn from(var: DecimalsCall) -> Self {
            LUSDTokenCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for LUSDTokenCalls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            LUSDTokenCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for LUSDTokenCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            LUSDTokenCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for LUSDTokenCalls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            LUSDTokenCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<MintCall> for LUSDTokenCalls {
        fn from(var: MintCall) -> Self {
            LUSDTokenCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for LUSDTokenCalls {
        fn from(var: NameCall) -> Self {
            LUSDTokenCalls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for LUSDTokenCalls {
        fn from(var: NoncesCall) -> Self {
            LUSDTokenCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<PermitCall> for LUSDTokenCalls {
        fn from(var: PermitCall) -> Self {
            LUSDTokenCalls::Permit(var)
        }
    }
    impl ::std::convert::From<PermitTypeHashCall> for LUSDTokenCalls {
        fn from(var: PermitTypeHashCall) -> Self {
            LUSDTokenCalls::PermitTypeHash(var)
        }
    }
    impl ::std::convert::From<ReturnFromPoolCall> for LUSDTokenCalls {
        fn from(var: ReturnFromPoolCall) -> Self {
            LUSDTokenCalls::ReturnFromPool(var)
        }
    }
    impl ::std::convert::From<SendToPoolCall> for LUSDTokenCalls {
        fn from(var: SendToPoolCall) -> Self {
            LUSDTokenCalls::SendToPool(var)
        }
    }
    impl ::std::convert::From<StabilityPoolAddressCall> for LUSDTokenCalls {
        fn from(var: StabilityPoolAddressCall) -> Self {
            LUSDTokenCalls::StabilityPoolAddress(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for LUSDTokenCalls {
        fn from(var: SymbolCall) -> Self {
            LUSDTokenCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for LUSDTokenCalls {
        fn from(var: TotalSupplyCall) -> Self {
            LUSDTokenCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for LUSDTokenCalls {
        fn from(var: TransferCall) -> Self {
            LUSDTokenCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for LUSDTokenCalls {
        fn from(var: TransferFromCall) -> Self {
            LUSDTokenCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TroveManagerAddressCall> for LUSDTokenCalls {
        fn from(var: TroveManagerAddressCall) -> Self {
            LUSDTokenCalls::TroveManagerAddress(var)
        }
    }
    impl ::std::convert::From<VersionCall> for LUSDTokenCalls {
        fn from(var: VersionCall) -> Self {
            LUSDTokenCalls::Version(var)
        }
    }
    #[doc = "Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AllowanceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ApproveReturn(pub bool);
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `borrowerOperationsAddress` function with signature `borrowerOperationsAddress()` and selector `[183, 248, 207, 155]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BorrowerOperationsAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DecimalsReturn(pub u8);
    #[doc = "Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DecreaseAllowanceReturn(pub bool);
    #[doc = "Container type for all return fields from the `domainSeparator` function with signature `domainSeparator()` and selector `[246, 152, 218, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IncreaseAllowanceReturn(pub bool);
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NameReturn(pub String);
    #[doc = "Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NoncesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `permitTypeHash` function with signature `permitTypeHash()` and selector `[16, 206, 67, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PermitTypeHashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `stabilityPoolAddress` function with signature `stabilityPoolAddress()` and selector `[11, 98, 42, 178]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct StabilityPoolAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SymbolReturn(pub String);
    #[doc = "Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TotalSupplyReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TransferReturn(pub bool);
    #[doc = "Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TransferFromReturn(pub bool);
    #[doc = "Container type for all return fields from the `troveManagerAddress` function with signature `troveManagerAddress()` and selector `[90, 77, 40, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TroveManagerAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `version` function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct VersionReturn(pub String);
}
