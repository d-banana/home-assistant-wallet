pub use rocket_minipool_manager::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod rocket_minipool_manager {
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
    #[doc = "RocketMinipoolManager was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ROCKETMINIPOOLMANAGER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract RocketStorageInterface\",\"name\":\"_rocketStorageAddress\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"minipool\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"node\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"time\",\"type\":\"uint256\"}],\"name\":\"MinipoolCreated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"minipool\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"node\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"time\",\"type\":\"uint256\"}],\"name\":\"MinipoolDestroyed\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"},{\"internalType\":\"enum MinipoolDeposit\",\"name\":\"_depositType\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"_salt\",\"type\":\"uint256\"}],\"name\":\"createMinipool\",\"outputs\":[{\"internalType\":\"contract RocketMinipoolInterface\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"decrementNodeStakingMinipoolCount\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"destroyMinipool\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getActiveMinipoolCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getFinalisedMinipoolCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_index\",\"type\":\"uint256\"}],\"name\":\"getMinipoolAt\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_pubkey\",\"type\":\"bytes\"}],\"name\":\"getMinipoolByPubkey\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getMinipoolBytecode\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getMinipoolCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"offset\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"limit\",\"type\":\"uint256\"}],\"name\":\"getMinipoolCountPerStatus\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"initialisedCount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"prelaunchCount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"stakingCount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"withdrawableCount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"dissolvedCount\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_minipoolAddress\",\"type\":\"address\"}],\"name\":\"getMinipoolDestroyed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_minipoolAddress\",\"type\":\"address\"}],\"name\":\"getMinipoolExists\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_minipoolAddress\",\"type\":\"address\"}],\"name\":\"getMinipoolPubkey\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_minipoolAddress\",\"type\":\"address\"}],\"name\":\"getMinipoolWithdrawalCredentials\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getNodeActiveMinipoolCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getNodeFinalisedMinipoolCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_index\",\"type\":\"uint256\"}],\"name\":\"getNodeMinipoolAt\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getNodeMinipoolCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getNodeStakingMinipoolCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_index\",\"type\":\"uint256\"}],\"name\":\"getNodeValidatingMinipoolAt\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getNodeValidatingMinipoolCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"offset\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"limit\",\"type\":\"uint256\"}],\"name\":\"getPrelaunchMinipools\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getStakingMinipoolCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"incrementNodeFinalisedMinipoolCount\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"incrementNodeStakingMinipoolCount\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_pubkey\",\"type\":\"bytes\"}],\"name\":\"setMinipoolPubkey\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]") . expect ("invalid abi")
        });
    pub struct RocketMinipoolManager<M>(ethers::contract::Contract<M>);
    impl<M> Clone for RocketMinipoolManager<M> {
        fn clone(&self) -> Self {
            RocketMinipoolManager(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for RocketMinipoolManager<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for RocketMinipoolManager<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(RocketMinipoolManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> RocketMinipoolManager<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                ROCKETMINIPOOLMANAGER_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `createMinipool` (0x518e703c) function"]
        pub fn create_minipool(
            &self,
            node_address: ethers::core::types::Address,
            deposit_type: u8,
            salt: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([81, 142, 112, 60], (node_address, deposit_type, salt))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decrementNodeStakingMinipoolCount` (0x75b59c7f) function"]
        pub fn decrement_node_staking_minipool_count(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([117, 181, 156, 127], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `destroyMinipool` (0x7bb40aaf) function"]
        pub fn destroy_minipool(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 180, 10, 175], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getActiveMinipoolCount` (0xce9b79ad) function"]
        pub fn get_active_minipool_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([206, 155, 121, 173], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFinalisedMinipoolCount` (0xd1ea6ce0) function"]
        pub fn get_finalised_minipool_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([209, 234, 108, 224], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinipoolAt` (0xeff7319f) function"]
        pub fn get_minipool_at(
            &self,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([239, 247, 49, 159], index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinipoolByPubkey` (0xcf6a4763) function"]
        pub fn get_minipool_by_pubkey(
            &self,
            pubkey: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([207, 106, 71, 99], pubkey)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinipoolBytecode` (0xf85b6943) function"]
        pub fn get_minipool_bytecode(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([248, 91, 105, 67], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinipoolCount` (0xae4d0bed) function"]
        pub fn get_minipool_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([174, 77, 11, 237], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinipoolCountPerStatus` (0x3b5ecefa) function"]
        pub fn get_minipool_count_per_status(
            &self,
            offset: ethers::core::types::U256,
            limit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([59, 94, 206, 250], (offset, limit))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinipoolDestroyed` (0xa757987a) function"]
        pub fn get_minipool_destroyed(
            &self,
            minipool_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([167, 87, 152, 122], minipool_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinipoolExists` (0x606bb62e) function"]
        pub fn get_minipool_exists(
            &self,
            minipool_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([96, 107, 182, 46], minipool_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinipoolPubkey` (0x3eb535e9) function"]
        pub fn get_minipool_pubkey(
            &self,
            minipool_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([62, 181, 53, 233], minipool_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinipoolWithdrawalCredentials` (0x2cb76c37) function"]
        pub fn get_minipool_withdrawal_credentials(
            &self,
            minipool_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([44, 183, 108, 55], minipool_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeActiveMinipoolCount` (0x1844ec01) function"]
        pub fn get_node_active_minipool_count(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 68, 236, 1], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeFinalisedMinipoolCount` (0xb88a89f7) function"]
        pub fn get_node_finalised_minipool_count(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([184, 138, 137, 247], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeMinipoolAt` (0x8b300029) function"]
        pub fn get_node_minipool_at(
            &self,
            node_address: ethers::core::types::Address,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([139, 48, 0, 41], (node_address, index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeMinipoolCount` (0x1ce9ec33) function"]
        pub fn get_node_minipool_count(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([28, 233, 236, 51], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeStakingMinipoolCount` (0x57b4ef6b) function"]
        pub fn get_node_staking_minipool_count(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([87, 180, 239, 107], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeValidatingMinipoolAt` (0x9da0700f) function"]
        pub fn get_node_validating_minipool_at(
            &self,
            node_address: ethers::core::types::Address,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([157, 160, 112, 15], (node_address, index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeValidatingMinipoolCount` (0xf90267c4) function"]
        pub fn get_node_validating_minipool_count(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([249, 2, 103, 196], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPrelaunchMinipools` (0x5dfef965) function"]
        pub fn get_prelaunch_minipools(
            &self,
            offset: ethers::core::types::U256,
            limit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([93, 254, 249, 101], (offset, limit))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getStakingMinipoolCount` (0x67bca235) function"]
        pub fn get_staking_minipool_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([103, 188, 162, 53], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `incrementNodeFinalisedMinipoolCount` (0xb04e8868) function"]
        pub fn increment_node_finalised_minipool_count(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 78, 136, 104], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `incrementNodeStakingMinipoolCount` (0x9907288c) function"]
        pub fn increment_node_staking_minipool_count(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 7, 40, 140], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMinipoolPubkey` (0x2c7f64d4) function"]
        pub fn set_minipool_pubkey(
            &self,
            pubkey: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([44, 127, 100, 212], pubkey)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `MinipoolCreated` event"]
        pub fn minipool_created_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MinipoolCreatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MinipoolDestroyed` event"]
        pub fn minipool_destroyed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MinipoolDestroyedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, RocketMinipoolManagerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for RocketMinipoolManager<M>
    {
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
    #[ethevent(
        name = "MinipoolCreated",
        abi = "MinipoolCreated(address,address,uint256)"
    )]
    pub struct MinipoolCreatedFilter {
        #[ethevent(indexed)]
        pub minipool: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub node: ethers::core::types::Address,
        pub time: ethers::core::types::U256,
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
        name = "MinipoolDestroyed",
        abi = "MinipoolDestroyed(address,address,uint256)"
    )]
    pub struct MinipoolDestroyedFilter {
        #[ethevent(indexed)]
        pub minipool: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub node: ethers::core::types::Address,
        pub time: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum RocketMinipoolManagerEvents {
        MinipoolCreatedFilter(MinipoolCreatedFilter),
        MinipoolDestroyedFilter(MinipoolDestroyedFilter),
    }
    impl ethers::contract::EthLogDecode for RocketMinipoolManagerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = MinipoolCreatedFilter::decode_log(log) {
                return Ok(RocketMinipoolManagerEvents::MinipoolCreatedFilter(decoded));
            }
            if let Ok(decoded) = MinipoolDestroyedFilter::decode_log(log) {
                return Ok(RocketMinipoolManagerEvents::MinipoolDestroyedFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for RocketMinipoolManagerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                RocketMinipoolManagerEvents::MinipoolCreatedFilter(element) => element.fmt(f),
                RocketMinipoolManagerEvents::MinipoolDestroyedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `createMinipool` function with signature `createMinipool(address,uint8,uint256)` and selector `[81, 142, 112, 60]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "createMinipool", abi = "createMinipool(address,uint8,uint256)")]
    pub struct CreateMinipoolCall {
        pub node_address: ethers::core::types::Address,
        pub deposit_type: u8,
        pub salt: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `decrementNodeStakingMinipoolCount` function with signature `decrementNodeStakingMinipoolCount(address)` and selector `[117, 181, 156, 127]`"]
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
        name = "decrementNodeStakingMinipoolCount",
        abi = "decrementNodeStakingMinipoolCount(address)"
    )]
    pub struct DecrementNodeStakingMinipoolCountCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `destroyMinipool` function with signature `destroyMinipool()` and selector `[123, 180, 10, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "destroyMinipool", abi = "destroyMinipool()")]
    pub struct DestroyMinipoolCall;
    #[doc = "Container type for all input parameters for the `getActiveMinipoolCount` function with signature `getActiveMinipoolCount()` and selector `[206, 155, 121, 173]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getActiveMinipoolCount", abi = "getActiveMinipoolCount()")]
    pub struct GetActiveMinipoolCountCall;
    #[doc = "Container type for all input parameters for the `getFinalisedMinipoolCount` function with signature `getFinalisedMinipoolCount()` and selector `[209, 234, 108, 224]`"]
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
        name = "getFinalisedMinipoolCount",
        abi = "getFinalisedMinipoolCount()"
    )]
    pub struct GetFinalisedMinipoolCountCall;
    #[doc = "Container type for all input parameters for the `getMinipoolAt` function with signature `getMinipoolAt(uint256)` and selector `[239, 247, 49, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getMinipoolAt", abi = "getMinipoolAt(uint256)")]
    pub struct GetMinipoolAtCall {
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getMinipoolByPubkey` function with signature `getMinipoolByPubkey(bytes)` and selector `[207, 106, 71, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getMinipoolByPubkey", abi = "getMinipoolByPubkey(bytes)")]
    pub struct GetMinipoolByPubkeyCall {
        pub pubkey: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `getMinipoolBytecode` function with signature `getMinipoolBytecode()` and selector `[248, 91, 105, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getMinipoolBytecode", abi = "getMinipoolBytecode()")]
    pub struct GetMinipoolBytecodeCall;
    #[doc = "Container type for all input parameters for the `getMinipoolCount` function with signature `getMinipoolCount()` and selector `[174, 77, 11, 237]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getMinipoolCount", abi = "getMinipoolCount()")]
    pub struct GetMinipoolCountCall;
    #[doc = "Container type for all input parameters for the `getMinipoolCountPerStatus` function with signature `getMinipoolCountPerStatus(uint256,uint256)` and selector `[59, 94, 206, 250]`"]
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
        name = "getMinipoolCountPerStatus",
        abi = "getMinipoolCountPerStatus(uint256,uint256)"
    )]
    pub struct GetMinipoolCountPerStatusCall {
        pub offset: ethers::core::types::U256,
        pub limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getMinipoolDestroyed` function with signature `getMinipoolDestroyed(address)` and selector `[167, 87, 152, 122]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getMinipoolDestroyed", abi = "getMinipoolDestroyed(address)")]
    pub struct GetMinipoolDestroyedCall {
        pub minipool_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getMinipoolExists` function with signature `getMinipoolExists(address)` and selector `[96, 107, 182, 46]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getMinipoolExists", abi = "getMinipoolExists(address)")]
    pub struct GetMinipoolExistsCall {
        pub minipool_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getMinipoolPubkey` function with signature `getMinipoolPubkey(address)` and selector `[62, 181, 53, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getMinipoolPubkey", abi = "getMinipoolPubkey(address)")]
    pub struct GetMinipoolPubkeyCall {
        pub minipool_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getMinipoolWithdrawalCredentials` function with signature `getMinipoolWithdrawalCredentials(address)` and selector `[44, 183, 108, 55]`"]
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
        name = "getMinipoolWithdrawalCredentials",
        abi = "getMinipoolWithdrawalCredentials(address)"
    )]
    pub struct GetMinipoolWithdrawalCredentialsCall {
        pub minipool_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getNodeActiveMinipoolCount` function with signature `getNodeActiveMinipoolCount(address)` and selector `[24, 68, 236, 1]`"]
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
        name = "getNodeActiveMinipoolCount",
        abi = "getNodeActiveMinipoolCount(address)"
    )]
    pub struct GetNodeActiveMinipoolCountCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getNodeFinalisedMinipoolCount` function with signature `getNodeFinalisedMinipoolCount(address)` and selector `[184, 138, 137, 247]`"]
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
        name = "getNodeFinalisedMinipoolCount",
        abi = "getNodeFinalisedMinipoolCount(address)"
    )]
    pub struct GetNodeFinalisedMinipoolCountCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getNodeMinipoolAt` function with signature `getNodeMinipoolAt(address,uint256)` and selector `[139, 48, 0, 41]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNodeMinipoolAt", abi = "getNodeMinipoolAt(address,uint256)")]
    pub struct GetNodeMinipoolAtCall {
        pub node_address: ethers::core::types::Address,
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getNodeMinipoolCount` function with signature `getNodeMinipoolCount(address)` and selector `[28, 233, 236, 51]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNodeMinipoolCount", abi = "getNodeMinipoolCount(address)")]
    pub struct GetNodeMinipoolCountCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getNodeStakingMinipoolCount` function with signature `getNodeStakingMinipoolCount(address)` and selector `[87, 180, 239, 107]`"]
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
        name = "getNodeStakingMinipoolCount",
        abi = "getNodeStakingMinipoolCount(address)"
    )]
    pub struct GetNodeStakingMinipoolCountCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getNodeValidatingMinipoolAt` function with signature `getNodeValidatingMinipoolAt(address,uint256)` and selector `[157, 160, 112, 15]`"]
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
        name = "getNodeValidatingMinipoolAt",
        abi = "getNodeValidatingMinipoolAt(address,uint256)"
    )]
    pub struct GetNodeValidatingMinipoolAtCall {
        pub node_address: ethers::core::types::Address,
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getNodeValidatingMinipoolCount` function with signature `getNodeValidatingMinipoolCount(address)` and selector `[249, 2, 103, 196]`"]
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
        name = "getNodeValidatingMinipoolCount",
        abi = "getNodeValidatingMinipoolCount(address)"
    )]
    pub struct GetNodeValidatingMinipoolCountCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getPrelaunchMinipools` function with signature `getPrelaunchMinipools(uint256,uint256)` and selector `[93, 254, 249, 101]`"]
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
        name = "getPrelaunchMinipools",
        abi = "getPrelaunchMinipools(uint256,uint256)"
    )]
    pub struct GetPrelaunchMinipoolsCall {
        pub offset: ethers::core::types::U256,
        pub limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getStakingMinipoolCount` function with signature `getStakingMinipoolCount()` and selector `[103, 188, 162, 53]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getStakingMinipoolCount", abi = "getStakingMinipoolCount()")]
    pub struct GetStakingMinipoolCountCall;
    #[doc = "Container type for all input parameters for the `incrementNodeFinalisedMinipoolCount` function with signature `incrementNodeFinalisedMinipoolCount(address)` and selector `[176, 78, 136, 104]`"]
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
        name = "incrementNodeFinalisedMinipoolCount",
        abi = "incrementNodeFinalisedMinipoolCount(address)"
    )]
    pub struct IncrementNodeFinalisedMinipoolCountCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `incrementNodeStakingMinipoolCount` function with signature `incrementNodeStakingMinipoolCount(address)` and selector `[153, 7, 40, 140]`"]
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
        name = "incrementNodeStakingMinipoolCount",
        abi = "incrementNodeStakingMinipoolCount(address)"
    )]
    pub struct IncrementNodeStakingMinipoolCountCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setMinipoolPubkey` function with signature `setMinipoolPubkey(bytes)` and selector `[44, 127, 100, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setMinipoolPubkey", abi = "setMinipoolPubkey(bytes)")]
    pub struct SetMinipoolPubkeyCall {
        pub pubkey: ethers::core::types::Bytes,
    }
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
    pub enum RocketMinipoolManagerCalls {
        CreateMinipool(CreateMinipoolCall),
        DecrementNodeStakingMinipoolCount(DecrementNodeStakingMinipoolCountCall),
        DestroyMinipool(DestroyMinipoolCall),
        GetActiveMinipoolCount(GetActiveMinipoolCountCall),
        GetFinalisedMinipoolCount(GetFinalisedMinipoolCountCall),
        GetMinipoolAt(GetMinipoolAtCall),
        GetMinipoolByPubkey(GetMinipoolByPubkeyCall),
        GetMinipoolBytecode(GetMinipoolBytecodeCall),
        GetMinipoolCount(GetMinipoolCountCall),
        GetMinipoolCountPerStatus(GetMinipoolCountPerStatusCall),
        GetMinipoolDestroyed(GetMinipoolDestroyedCall),
        GetMinipoolExists(GetMinipoolExistsCall),
        GetMinipoolPubkey(GetMinipoolPubkeyCall),
        GetMinipoolWithdrawalCredentials(GetMinipoolWithdrawalCredentialsCall),
        GetNodeActiveMinipoolCount(GetNodeActiveMinipoolCountCall),
        GetNodeFinalisedMinipoolCount(GetNodeFinalisedMinipoolCountCall),
        GetNodeMinipoolAt(GetNodeMinipoolAtCall),
        GetNodeMinipoolCount(GetNodeMinipoolCountCall),
        GetNodeStakingMinipoolCount(GetNodeStakingMinipoolCountCall),
        GetNodeValidatingMinipoolAt(GetNodeValidatingMinipoolAtCall),
        GetNodeValidatingMinipoolCount(GetNodeValidatingMinipoolCountCall),
        GetPrelaunchMinipools(GetPrelaunchMinipoolsCall),
        GetStakingMinipoolCount(GetStakingMinipoolCountCall),
        IncrementNodeFinalisedMinipoolCount(IncrementNodeFinalisedMinipoolCountCall),
        IncrementNodeStakingMinipoolCount(IncrementNodeStakingMinipoolCountCall),
        SetMinipoolPubkey(SetMinipoolPubkeyCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for RocketMinipoolManagerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CreateMinipoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::CreateMinipool(decoded));
            }
            if let Ok(decoded) =
                <DecrementNodeStakingMinipoolCountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketMinipoolManagerCalls::DecrementNodeStakingMinipoolCount(decoded));
            }
            if let Ok(decoded) =
                <DestroyMinipoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::DestroyMinipool(decoded));
            }
            if let Ok(decoded) =
                <GetActiveMinipoolCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::GetActiveMinipoolCount(decoded));
            }
            if let Ok(decoded) =
                <GetFinalisedMinipoolCountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketMinipoolManagerCalls::GetFinalisedMinipoolCount(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetMinipoolAtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::GetMinipoolAt(decoded));
            }
            if let Ok(decoded) =
                <GetMinipoolByPubkeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::GetMinipoolByPubkey(decoded));
            }
            if let Ok(decoded) =
                <GetMinipoolBytecodeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::GetMinipoolBytecode(decoded));
            }
            if let Ok(decoded) =
                <GetMinipoolCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::GetMinipoolCount(decoded));
            }
            if let Ok(decoded) =
                <GetMinipoolCountPerStatusCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketMinipoolManagerCalls::GetMinipoolCountPerStatus(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetMinipoolDestroyedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::GetMinipoolDestroyed(decoded));
            }
            if let Ok(decoded) =
                <GetMinipoolExistsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::GetMinipoolExists(decoded));
            }
            if let Ok(decoded) =
                <GetMinipoolPubkeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::GetMinipoolPubkey(decoded));
            }
            if let Ok(decoded) =
                <GetMinipoolWithdrawalCredentialsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketMinipoolManagerCalls::GetMinipoolWithdrawalCredentials(decoded));
            }
            if let Ok(decoded) =
                <GetNodeActiveMinipoolCountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketMinipoolManagerCalls::GetNodeActiveMinipoolCount(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetNodeFinalisedMinipoolCountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketMinipoolManagerCalls::GetNodeFinalisedMinipoolCount(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetNodeMinipoolAtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::GetNodeMinipoolAt(decoded));
            }
            if let Ok(decoded) =
                <GetNodeMinipoolCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::GetNodeMinipoolCount(decoded));
            }
            if let Ok(decoded) =
                <GetNodeStakingMinipoolCountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketMinipoolManagerCalls::GetNodeStakingMinipoolCount(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetNodeValidatingMinipoolAtCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketMinipoolManagerCalls::GetNodeValidatingMinipoolAt(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetNodeValidatingMinipoolCountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketMinipoolManagerCalls::GetNodeValidatingMinipoolCount(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetPrelaunchMinipoolsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::GetPrelaunchMinipools(decoded));
            }
            if let Ok(decoded) =
                <GetStakingMinipoolCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::GetStakingMinipoolCount(decoded));
            }
            if let Ok(decoded) =
                <IncrementNodeFinalisedMinipoolCountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketMinipoolManagerCalls::IncrementNodeFinalisedMinipoolCount(decoded));
            }
            if let Ok(decoded) =
                <IncrementNodeStakingMinipoolCountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketMinipoolManagerCalls::IncrementNodeStakingMinipoolCount(decoded));
            }
            if let Ok(decoded) =
                <SetMinipoolPubkeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::SetMinipoolPubkey(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketMinipoolManagerCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for RocketMinipoolManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                RocketMinipoolManagerCalls::CreateMinipool(element) => element.encode(),
                RocketMinipoolManagerCalls::DecrementNodeStakingMinipoolCount(element) => {
                    element.encode()
                }
                RocketMinipoolManagerCalls::DestroyMinipool(element) => element.encode(),
                RocketMinipoolManagerCalls::GetActiveMinipoolCount(element) => element.encode(),
                RocketMinipoolManagerCalls::GetFinalisedMinipoolCount(element) => element.encode(),
                RocketMinipoolManagerCalls::GetMinipoolAt(element) => element.encode(),
                RocketMinipoolManagerCalls::GetMinipoolByPubkey(element) => element.encode(),
                RocketMinipoolManagerCalls::GetMinipoolBytecode(element) => element.encode(),
                RocketMinipoolManagerCalls::GetMinipoolCount(element) => element.encode(),
                RocketMinipoolManagerCalls::GetMinipoolCountPerStatus(element) => element.encode(),
                RocketMinipoolManagerCalls::GetMinipoolDestroyed(element) => element.encode(),
                RocketMinipoolManagerCalls::GetMinipoolExists(element) => element.encode(),
                RocketMinipoolManagerCalls::GetMinipoolPubkey(element) => element.encode(),
                RocketMinipoolManagerCalls::GetMinipoolWithdrawalCredentials(element) => {
                    element.encode()
                }
                RocketMinipoolManagerCalls::GetNodeActiveMinipoolCount(element) => element.encode(),
                RocketMinipoolManagerCalls::GetNodeFinalisedMinipoolCount(element) => {
                    element.encode()
                }
                RocketMinipoolManagerCalls::GetNodeMinipoolAt(element) => element.encode(),
                RocketMinipoolManagerCalls::GetNodeMinipoolCount(element) => element.encode(),
                RocketMinipoolManagerCalls::GetNodeStakingMinipoolCount(element) => {
                    element.encode()
                }
                RocketMinipoolManagerCalls::GetNodeValidatingMinipoolAt(element) => {
                    element.encode()
                }
                RocketMinipoolManagerCalls::GetNodeValidatingMinipoolCount(element) => {
                    element.encode()
                }
                RocketMinipoolManagerCalls::GetPrelaunchMinipools(element) => element.encode(),
                RocketMinipoolManagerCalls::GetStakingMinipoolCount(element) => element.encode(),
                RocketMinipoolManagerCalls::IncrementNodeFinalisedMinipoolCount(element) => {
                    element.encode()
                }
                RocketMinipoolManagerCalls::IncrementNodeStakingMinipoolCount(element) => {
                    element.encode()
                }
                RocketMinipoolManagerCalls::SetMinipoolPubkey(element) => element.encode(),
                RocketMinipoolManagerCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for RocketMinipoolManagerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                RocketMinipoolManagerCalls::CreateMinipool(element) => element.fmt(f),
                RocketMinipoolManagerCalls::DecrementNodeStakingMinipoolCount(element) => {
                    element.fmt(f)
                }
                RocketMinipoolManagerCalls::DestroyMinipool(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetActiveMinipoolCount(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetFinalisedMinipoolCount(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetMinipoolAt(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetMinipoolByPubkey(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetMinipoolBytecode(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetMinipoolCount(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetMinipoolCountPerStatus(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetMinipoolDestroyed(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetMinipoolExists(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetMinipoolPubkey(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetMinipoolWithdrawalCredentials(element) => {
                    element.fmt(f)
                }
                RocketMinipoolManagerCalls::GetNodeActiveMinipoolCount(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetNodeFinalisedMinipoolCount(element) => {
                    element.fmt(f)
                }
                RocketMinipoolManagerCalls::GetNodeMinipoolAt(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetNodeMinipoolCount(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetNodeStakingMinipoolCount(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetNodeValidatingMinipoolAt(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetNodeValidatingMinipoolCount(element) => {
                    element.fmt(f)
                }
                RocketMinipoolManagerCalls::GetPrelaunchMinipools(element) => element.fmt(f),
                RocketMinipoolManagerCalls::GetStakingMinipoolCount(element) => element.fmt(f),
                RocketMinipoolManagerCalls::IncrementNodeFinalisedMinipoolCount(element) => {
                    element.fmt(f)
                }
                RocketMinipoolManagerCalls::IncrementNodeStakingMinipoolCount(element) => {
                    element.fmt(f)
                }
                RocketMinipoolManagerCalls::SetMinipoolPubkey(element) => element.fmt(f),
                RocketMinipoolManagerCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CreateMinipoolCall> for RocketMinipoolManagerCalls {
        fn from(var: CreateMinipoolCall) -> Self {
            RocketMinipoolManagerCalls::CreateMinipool(var)
        }
    }
    impl ::std::convert::From<DecrementNodeStakingMinipoolCountCall> for RocketMinipoolManagerCalls {
        fn from(var: DecrementNodeStakingMinipoolCountCall) -> Self {
            RocketMinipoolManagerCalls::DecrementNodeStakingMinipoolCount(var)
        }
    }
    impl ::std::convert::From<DestroyMinipoolCall> for RocketMinipoolManagerCalls {
        fn from(var: DestroyMinipoolCall) -> Self {
            RocketMinipoolManagerCalls::DestroyMinipool(var)
        }
    }
    impl ::std::convert::From<GetActiveMinipoolCountCall> for RocketMinipoolManagerCalls {
        fn from(var: GetActiveMinipoolCountCall) -> Self {
            RocketMinipoolManagerCalls::GetActiveMinipoolCount(var)
        }
    }
    impl ::std::convert::From<GetFinalisedMinipoolCountCall> for RocketMinipoolManagerCalls {
        fn from(var: GetFinalisedMinipoolCountCall) -> Self {
            RocketMinipoolManagerCalls::GetFinalisedMinipoolCount(var)
        }
    }
    impl ::std::convert::From<GetMinipoolAtCall> for RocketMinipoolManagerCalls {
        fn from(var: GetMinipoolAtCall) -> Self {
            RocketMinipoolManagerCalls::GetMinipoolAt(var)
        }
    }
    impl ::std::convert::From<GetMinipoolByPubkeyCall> for RocketMinipoolManagerCalls {
        fn from(var: GetMinipoolByPubkeyCall) -> Self {
            RocketMinipoolManagerCalls::GetMinipoolByPubkey(var)
        }
    }
    impl ::std::convert::From<GetMinipoolBytecodeCall> for RocketMinipoolManagerCalls {
        fn from(var: GetMinipoolBytecodeCall) -> Self {
            RocketMinipoolManagerCalls::GetMinipoolBytecode(var)
        }
    }
    impl ::std::convert::From<GetMinipoolCountCall> for RocketMinipoolManagerCalls {
        fn from(var: GetMinipoolCountCall) -> Self {
            RocketMinipoolManagerCalls::GetMinipoolCount(var)
        }
    }
    impl ::std::convert::From<GetMinipoolCountPerStatusCall> for RocketMinipoolManagerCalls {
        fn from(var: GetMinipoolCountPerStatusCall) -> Self {
            RocketMinipoolManagerCalls::GetMinipoolCountPerStatus(var)
        }
    }
    impl ::std::convert::From<GetMinipoolDestroyedCall> for RocketMinipoolManagerCalls {
        fn from(var: GetMinipoolDestroyedCall) -> Self {
            RocketMinipoolManagerCalls::GetMinipoolDestroyed(var)
        }
    }
    impl ::std::convert::From<GetMinipoolExistsCall> for RocketMinipoolManagerCalls {
        fn from(var: GetMinipoolExistsCall) -> Self {
            RocketMinipoolManagerCalls::GetMinipoolExists(var)
        }
    }
    impl ::std::convert::From<GetMinipoolPubkeyCall> for RocketMinipoolManagerCalls {
        fn from(var: GetMinipoolPubkeyCall) -> Self {
            RocketMinipoolManagerCalls::GetMinipoolPubkey(var)
        }
    }
    impl ::std::convert::From<GetMinipoolWithdrawalCredentialsCall> for RocketMinipoolManagerCalls {
        fn from(var: GetMinipoolWithdrawalCredentialsCall) -> Self {
            RocketMinipoolManagerCalls::GetMinipoolWithdrawalCredentials(var)
        }
    }
    impl ::std::convert::From<GetNodeActiveMinipoolCountCall> for RocketMinipoolManagerCalls {
        fn from(var: GetNodeActiveMinipoolCountCall) -> Self {
            RocketMinipoolManagerCalls::GetNodeActiveMinipoolCount(var)
        }
    }
    impl ::std::convert::From<GetNodeFinalisedMinipoolCountCall> for RocketMinipoolManagerCalls {
        fn from(var: GetNodeFinalisedMinipoolCountCall) -> Self {
            RocketMinipoolManagerCalls::GetNodeFinalisedMinipoolCount(var)
        }
    }
    impl ::std::convert::From<GetNodeMinipoolAtCall> for RocketMinipoolManagerCalls {
        fn from(var: GetNodeMinipoolAtCall) -> Self {
            RocketMinipoolManagerCalls::GetNodeMinipoolAt(var)
        }
    }
    impl ::std::convert::From<GetNodeMinipoolCountCall> for RocketMinipoolManagerCalls {
        fn from(var: GetNodeMinipoolCountCall) -> Self {
            RocketMinipoolManagerCalls::GetNodeMinipoolCount(var)
        }
    }
    impl ::std::convert::From<GetNodeStakingMinipoolCountCall> for RocketMinipoolManagerCalls {
        fn from(var: GetNodeStakingMinipoolCountCall) -> Self {
            RocketMinipoolManagerCalls::GetNodeStakingMinipoolCount(var)
        }
    }
    impl ::std::convert::From<GetNodeValidatingMinipoolAtCall> for RocketMinipoolManagerCalls {
        fn from(var: GetNodeValidatingMinipoolAtCall) -> Self {
            RocketMinipoolManagerCalls::GetNodeValidatingMinipoolAt(var)
        }
    }
    impl ::std::convert::From<GetNodeValidatingMinipoolCountCall> for RocketMinipoolManagerCalls {
        fn from(var: GetNodeValidatingMinipoolCountCall) -> Self {
            RocketMinipoolManagerCalls::GetNodeValidatingMinipoolCount(var)
        }
    }
    impl ::std::convert::From<GetPrelaunchMinipoolsCall> for RocketMinipoolManagerCalls {
        fn from(var: GetPrelaunchMinipoolsCall) -> Self {
            RocketMinipoolManagerCalls::GetPrelaunchMinipools(var)
        }
    }
    impl ::std::convert::From<GetStakingMinipoolCountCall> for RocketMinipoolManagerCalls {
        fn from(var: GetStakingMinipoolCountCall) -> Self {
            RocketMinipoolManagerCalls::GetStakingMinipoolCount(var)
        }
    }
    impl ::std::convert::From<IncrementNodeFinalisedMinipoolCountCall> for RocketMinipoolManagerCalls {
        fn from(var: IncrementNodeFinalisedMinipoolCountCall) -> Self {
            RocketMinipoolManagerCalls::IncrementNodeFinalisedMinipoolCount(var)
        }
    }
    impl ::std::convert::From<IncrementNodeStakingMinipoolCountCall> for RocketMinipoolManagerCalls {
        fn from(var: IncrementNodeStakingMinipoolCountCall) -> Self {
            RocketMinipoolManagerCalls::IncrementNodeStakingMinipoolCount(var)
        }
    }
    impl ::std::convert::From<SetMinipoolPubkeyCall> for RocketMinipoolManagerCalls {
        fn from(var: SetMinipoolPubkeyCall) -> Self {
            RocketMinipoolManagerCalls::SetMinipoolPubkey(var)
        }
    }
    impl ::std::convert::From<VersionCall> for RocketMinipoolManagerCalls {
        fn from(var: VersionCall) -> Self {
            RocketMinipoolManagerCalls::Version(var)
        }
    }
    #[doc = "Container type for all return fields from the `createMinipool` function with signature `createMinipool(address,uint8,uint256)` and selector `[81, 142, 112, 60]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CreateMinipoolReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getActiveMinipoolCount` function with signature `getActiveMinipoolCount()` and selector `[206, 155, 121, 173]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetActiveMinipoolCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getFinalisedMinipoolCount` function with signature `getFinalisedMinipoolCount()` and selector `[209, 234, 108, 224]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetFinalisedMinipoolCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getMinipoolAt` function with signature `getMinipoolAt(uint256)` and selector `[239, 247, 49, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetMinipoolAtReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getMinipoolByPubkey` function with signature `getMinipoolByPubkey(bytes)` and selector `[207, 106, 71, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetMinipoolByPubkeyReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getMinipoolBytecode` function with signature `getMinipoolBytecode()` and selector `[248, 91, 105, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetMinipoolBytecodeReturn(pub ethers::core::types::Bytes);
    #[doc = "Container type for all return fields from the `getMinipoolCount` function with signature `getMinipoolCount()` and selector `[174, 77, 11, 237]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetMinipoolCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getMinipoolCountPerStatus` function with signature `getMinipoolCountPerStatus(uint256,uint256)` and selector `[59, 94, 206, 250]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetMinipoolCountPerStatusReturn {
        pub initialised_count: ethers::core::types::U256,
        pub prelaunch_count: ethers::core::types::U256,
        pub staking_count: ethers::core::types::U256,
        pub withdrawable_count: ethers::core::types::U256,
        pub dissolved_count: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getMinipoolDestroyed` function with signature `getMinipoolDestroyed(address)` and selector `[167, 87, 152, 122]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetMinipoolDestroyedReturn(pub bool);
    #[doc = "Container type for all return fields from the `getMinipoolExists` function with signature `getMinipoolExists(address)` and selector `[96, 107, 182, 46]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetMinipoolExistsReturn(pub bool);
    #[doc = "Container type for all return fields from the `getMinipoolPubkey` function with signature `getMinipoolPubkey(address)` and selector `[62, 181, 53, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetMinipoolPubkeyReturn(pub ethers::core::types::Bytes);
    #[doc = "Container type for all return fields from the `getMinipoolWithdrawalCredentials` function with signature `getMinipoolWithdrawalCredentials(address)` and selector `[44, 183, 108, 55]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetMinipoolWithdrawalCredentialsReturn(pub ethers::core::types::Bytes);
    #[doc = "Container type for all return fields from the `getNodeActiveMinipoolCount` function with signature `getNodeActiveMinipoolCount(address)` and selector `[24, 68, 236, 1]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeActiveMinipoolCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getNodeFinalisedMinipoolCount` function with signature `getNodeFinalisedMinipoolCount(address)` and selector `[184, 138, 137, 247]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeFinalisedMinipoolCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getNodeMinipoolAt` function with signature `getNodeMinipoolAt(address,uint256)` and selector `[139, 48, 0, 41]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeMinipoolAtReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getNodeMinipoolCount` function with signature `getNodeMinipoolCount(address)` and selector `[28, 233, 236, 51]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeMinipoolCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getNodeStakingMinipoolCount` function with signature `getNodeStakingMinipoolCount(address)` and selector `[87, 180, 239, 107]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeStakingMinipoolCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getNodeValidatingMinipoolAt` function with signature `getNodeValidatingMinipoolAt(address,uint256)` and selector `[157, 160, 112, 15]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeValidatingMinipoolAtReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getNodeValidatingMinipoolCount` function with signature `getNodeValidatingMinipoolCount(address)` and selector `[249, 2, 103, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeValidatingMinipoolCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getPrelaunchMinipools` function with signature `getPrelaunchMinipools(uint256,uint256)` and selector `[93, 254, 249, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetPrelaunchMinipoolsReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
    #[doc = "Container type for all return fields from the `getStakingMinipoolCount` function with signature `getStakingMinipoolCount()` and selector `[103, 188, 162, 53]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetStakingMinipoolCountReturn(pub ethers::core::types::U256);
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
    pub struct VersionReturn(pub u8);
}
