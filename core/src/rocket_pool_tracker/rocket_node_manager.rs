pub use rocket_node_manager::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod rocket_node_manager {
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
    #[doc = "RocketNodeManager was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ROCKETNODEMANAGER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract RocketStorageInterface\",\"name\":\"_rocketStorageAddress\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"node\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"time\",\"type\":\"uint256\"}],\"name\":\"NodeRegistered\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"node\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"network\",\"type\":\"uint256\"}],\"name\":\"NodeRewardNetworkChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"node\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\"}],\"name\":\"NodeSmoothingPoolStateChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"node\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"time\",\"type\":\"uint256\"}],\"name\":\"NodeTimezoneLocationSet\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getAverageNodeFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getFeeDistributorInitialised\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_offset\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_limit\",\"type\":\"uint256\"}],\"name\":\"getNodeAddresses\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_index\",\"type\":\"uint256\"}],\"name\":\"getNodeAt\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getNodeCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_offset\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_limit\",\"type\":\"uint256\"}],\"name\":\"getNodeCountPerTimezone\",\"outputs\":[{\"components\":[{\"internalType\":\"string\",\"name\":\"timezone\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"count\",\"type\":\"uint256\"}],\"internalType\":\"struct RocketNodeManagerInterface.TimezoneCount[]\",\"name\":\"\",\"type\":\"tuple[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getNodeDetails\",\"outputs\":[{\"components\":[{\"internalType\":\"bool\",\"name\":\"exists\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"registrationTime\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"timezoneLocation\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"feeDistributorInitialised\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"feeDistributorAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"rewardNetwork\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"rplStake\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"effectiveRPLStake\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"minimumRPLStake\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"maximumRPLStake\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"minipoolLimit\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"minipoolCount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"balanceETH\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"balanceRETH\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"balanceRPL\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"balanceOldRPL\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"withdrawalAddress\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"pendingWithdrawalAddress\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"smoothingPoolRegistrationState\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"smoothingPoolRegistrationChanged\",\"type\":\"uint256\"}],\"internalType\":\"struct NodeDetails\",\"name\":\"nodeDetails\",\"type\":\"tuple\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getNodeExists\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getNodePendingWithdrawalAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getNodeRegistrationTime\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getNodeTimezoneLocation\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getNodeWithdrawalAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getRewardNetwork\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_offset\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_limit\",\"type\":\"uint256\"}],\"name\":\"getSmoothingPoolRegisteredNodeCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getSmoothingPoolRegistrationChanged\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"}],\"name\":\"getSmoothingPoolRegistrationState\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"initialiseFeeDistributor\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_timezoneLocation\",\"type\":\"string\"}],\"name\":\"registerNode\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_nodeAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_network\",\"type\":\"uint256\"}],\"name\":\"setRewardNetwork\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_state\",\"type\":\"bool\"}],\"name\":\"setSmoothingPoolRegistrationState\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_timezoneLocation\",\"type\":\"string\"}],\"name\":\"setTimezoneLocation\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]") . expect ("invalid abi")
        });
    pub struct RocketNodeManager<M>(ethers::contract::Contract<M>);
    impl<M> Clone for RocketNodeManager<M> {
        fn clone(&self) -> Self {
            RocketNodeManager(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for RocketNodeManager<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for RocketNodeManager<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(RocketNodeManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> RocketNodeManager<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ROCKETNODEMANAGER_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `getAverageNodeFee` (0x414dd1d2) function"]
        pub fn get_average_node_fee(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([65, 77, 209, 210], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFeeDistributorInitialised` (0x927ece4f) function"]
        pub fn get_fee_distributor_initialised(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([146, 126, 206, 79], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeAddresses` (0x2d7f21d0) function"]
        pub fn get_node_addresses(
            &self,
            offset: ethers::core::types::U256,
            limit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([45, 127, 33, 208], (offset, limit))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeAt` (0xba75d806) function"]
        pub fn get_node_at(
            &self,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([186, 117, 216, 6], index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeCount` (0x39bf397e) function"]
        pub fn get_node_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([57, 191, 57, 126], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeCountPerTimezone` (0x29554540) function"]
        pub fn get_node_count_per_timezone(
            &self,
            offset: ethers::core::types::U256,
            limit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<TimezoneCount>> {
            self.0
                .method_hash([41, 85, 69, 64], (offset, limit))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeDetails` (0xbafb3581) function"]
        pub fn get_node_details(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, NodeDetails> {
            self.0
                .method_hash([186, 251, 53, 129], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeExists` (0x65d4176f) function"]
        pub fn get_node_exists(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([101, 212, 23, 111], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodePendingWithdrawalAddress` (0xfd412513) function"]
        pub fn get_node_pending_withdrawal_address(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([253, 65, 37, 19], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeRegistrationTime` (0x02d8a732) function"]
        pub fn get_node_registration_time(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([2, 216, 167, 50], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeTimezoneLocation` (0xb018f026) function"]
        pub fn get_node_timezone_location(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([176, 24, 240, 38], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNodeWithdrawalAddress` (0x5b49ff62) function"]
        pub fn get_node_withdrawal_address(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([91, 73, 255, 98], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRewardNetwork` (0x43f88981) function"]
        pub fn get_reward_network(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([67, 248, 137, 129], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSmoothingPoolRegisteredNodeCount` (0xb715a1aa) function"]
        pub fn get_smoothing_pool_registered_node_count(
            &self,
            offset: ethers::core::types::U256,
            limit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([183, 21, 161, 170], (offset, limit))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSmoothingPoolRegistrationChanged` (0x4d99f633) function"]
        pub fn get_smoothing_pool_registration_changed(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([77, 153, 246, 51], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSmoothingPoolRegistrationState` (0xa4cef9dd) function"]
        pub fn get_smoothing_pool_registration_state(
            &self,
            node_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 206, 249, 221], node_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialiseFeeDistributor` (0x64908a86) function"]
        pub fn initialise_fee_distributor(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 144, 138, 134], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerNode` (0x27c6f43e) function"]
        pub fn register_node(
            &self,
            timezone_location: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 198, 244, 62], timezone_location)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRewardNetwork` (0xd565f276) function"]
        pub fn set_reward_network(
            &self,
            node_address: ethers::core::types::Address,
            network: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 101, 242, 118], (node_address, network))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSmoothingPoolRegistrationState` (0x99283f8b) function"]
        pub fn set_smoothing_pool_registration_state(
            &self,
            state: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 40, 63, 139], state)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTimezoneLocation` (0xa7e6e8b3) function"]
        pub fn set_timezone_location(
            &self,
            timezone_location: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 230, 232, 179], timezone_location)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NodeRegistered` event"]
        pub fn node_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NodeRegisteredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NodeRewardNetworkChanged` event"]
        pub fn node_reward_network_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NodeRewardNetworkChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NodeSmoothingPoolStateChanged` event"]
        pub fn node_smoothing_pool_state_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NodeSmoothingPoolStateChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NodeTimezoneLocationSet` event"]
        pub fn node_timezone_location_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NodeTimezoneLocationSetFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, RocketNodeManagerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for RocketNodeManager<M>
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
    #[ethevent(name = "NodeRegistered", abi = "NodeRegistered(address,uint256)")]
    pub struct NodeRegisteredFilter {
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
        name = "NodeRewardNetworkChanged",
        abi = "NodeRewardNetworkChanged(address,uint256)"
    )]
    pub struct NodeRewardNetworkChangedFilter {
        #[ethevent(indexed)]
        pub node: ethers::core::types::Address,
        pub network: ethers::core::types::U256,
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
        name = "NodeSmoothingPoolStateChanged",
        abi = "NodeSmoothingPoolStateChanged(address,bool)"
    )]
    pub struct NodeSmoothingPoolStateChangedFilter {
        #[ethevent(indexed)]
        pub node: ethers::core::types::Address,
        pub state: bool,
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
        name = "NodeTimezoneLocationSet",
        abi = "NodeTimezoneLocationSet(address,uint256)"
    )]
    pub struct NodeTimezoneLocationSetFilter {
        #[ethevent(indexed)]
        pub node: ethers::core::types::Address,
        pub time: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum RocketNodeManagerEvents {
        NodeRegisteredFilter(NodeRegisteredFilter),
        NodeRewardNetworkChangedFilter(NodeRewardNetworkChangedFilter),
        NodeSmoothingPoolStateChangedFilter(NodeSmoothingPoolStateChangedFilter),
        NodeTimezoneLocationSetFilter(NodeTimezoneLocationSetFilter),
    }
    impl ethers::contract::EthLogDecode for RocketNodeManagerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = NodeRegisteredFilter::decode_log(log) {
                return Ok(RocketNodeManagerEvents::NodeRegisteredFilter(decoded));
            }
            if let Ok(decoded) = NodeRewardNetworkChangedFilter::decode_log(log) {
                return Ok(RocketNodeManagerEvents::NodeRewardNetworkChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NodeSmoothingPoolStateChangedFilter::decode_log(log) {
                return Ok(RocketNodeManagerEvents::NodeSmoothingPoolStateChangedFilter(decoded));
            }
            if let Ok(decoded) = NodeTimezoneLocationSetFilter::decode_log(log) {
                return Ok(RocketNodeManagerEvents::NodeTimezoneLocationSetFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for RocketNodeManagerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                RocketNodeManagerEvents::NodeRegisteredFilter(element) => element.fmt(f),
                RocketNodeManagerEvents::NodeRewardNetworkChangedFilter(element) => element.fmt(f),
                RocketNodeManagerEvents::NodeSmoothingPoolStateChangedFilter(element) => {
                    element.fmt(f)
                }
                RocketNodeManagerEvents::NodeTimezoneLocationSetFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `getAverageNodeFee` function with signature `getAverageNodeFee(address)` and selector `[65, 77, 209, 210]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAverageNodeFee", abi = "getAverageNodeFee(address)")]
    pub struct GetAverageNodeFeeCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getFeeDistributorInitialised` function with signature `getFeeDistributorInitialised(address)` and selector `[146, 126, 206, 79]`"]
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
        name = "getFeeDistributorInitialised",
        abi = "getFeeDistributorInitialised(address)"
    )]
    pub struct GetFeeDistributorInitialisedCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getNodeAddresses` function with signature `getNodeAddresses(uint256,uint256)` and selector `[45, 127, 33, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNodeAddresses", abi = "getNodeAddresses(uint256,uint256)")]
    pub struct GetNodeAddressesCall {
        pub offset: ethers::core::types::U256,
        pub limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getNodeAt` function with signature `getNodeAt(uint256)` and selector `[186, 117, 216, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNodeAt", abi = "getNodeAt(uint256)")]
    pub struct GetNodeAtCall {
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getNodeCount` function with signature `getNodeCount()` and selector `[57, 191, 57, 126]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNodeCount", abi = "getNodeCount()")]
    pub struct GetNodeCountCall;
    #[doc = "Container type for all input parameters for the `getNodeCountPerTimezone` function with signature `getNodeCountPerTimezone(uint256,uint256)` and selector `[41, 85, 69, 64]`"]
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
        name = "getNodeCountPerTimezone",
        abi = "getNodeCountPerTimezone(uint256,uint256)"
    )]
    pub struct GetNodeCountPerTimezoneCall {
        pub offset: ethers::core::types::U256,
        pub limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getNodeDetails` function with signature `getNodeDetails(address)` and selector `[186, 251, 53, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNodeDetails", abi = "getNodeDetails(address)")]
    pub struct GetNodeDetailsCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getNodeExists` function with signature `getNodeExists(address)` and selector `[101, 212, 23, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNodeExists", abi = "getNodeExists(address)")]
    pub struct GetNodeExistsCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getNodePendingWithdrawalAddress` function with signature `getNodePendingWithdrawalAddress(address)` and selector `[253, 65, 37, 19]`"]
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
        name = "getNodePendingWithdrawalAddress",
        abi = "getNodePendingWithdrawalAddress(address)"
    )]
    pub struct GetNodePendingWithdrawalAddressCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getNodeRegistrationTime` function with signature `getNodeRegistrationTime(address)` and selector `[2, 216, 167, 50]`"]
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
        name = "getNodeRegistrationTime",
        abi = "getNodeRegistrationTime(address)"
    )]
    pub struct GetNodeRegistrationTimeCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getNodeTimezoneLocation` function with signature `getNodeTimezoneLocation(address)` and selector `[176, 24, 240, 38]`"]
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
        name = "getNodeTimezoneLocation",
        abi = "getNodeTimezoneLocation(address)"
    )]
    pub struct GetNodeTimezoneLocationCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getNodeWithdrawalAddress` function with signature `getNodeWithdrawalAddress(address)` and selector `[91, 73, 255, 98]`"]
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
        name = "getNodeWithdrawalAddress",
        abi = "getNodeWithdrawalAddress(address)"
    )]
    pub struct GetNodeWithdrawalAddressCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getRewardNetwork` function with signature `getRewardNetwork(address)` and selector `[67, 248, 137, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getRewardNetwork", abi = "getRewardNetwork(address)")]
    pub struct GetRewardNetworkCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getSmoothingPoolRegisteredNodeCount` function with signature `getSmoothingPoolRegisteredNodeCount(uint256,uint256)` and selector `[183, 21, 161, 170]`"]
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
        name = "getSmoothingPoolRegisteredNodeCount",
        abi = "getSmoothingPoolRegisteredNodeCount(uint256,uint256)"
    )]
    pub struct GetSmoothingPoolRegisteredNodeCountCall {
        pub offset: ethers::core::types::U256,
        pub limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getSmoothingPoolRegistrationChanged` function with signature `getSmoothingPoolRegistrationChanged(address)` and selector `[77, 153, 246, 51]`"]
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
        name = "getSmoothingPoolRegistrationChanged",
        abi = "getSmoothingPoolRegistrationChanged(address)"
    )]
    pub struct GetSmoothingPoolRegistrationChangedCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getSmoothingPoolRegistrationState` function with signature `getSmoothingPoolRegistrationState(address)` and selector `[164, 206, 249, 221]`"]
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
        name = "getSmoothingPoolRegistrationState",
        abi = "getSmoothingPoolRegistrationState(address)"
    )]
    pub struct GetSmoothingPoolRegistrationStateCall {
        pub node_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `initialiseFeeDistributor` function with signature `initialiseFeeDistributor()` and selector `[100, 144, 138, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "initialiseFeeDistributor", abi = "initialiseFeeDistributor()")]
    pub struct InitialiseFeeDistributorCall;
    #[doc = "Container type for all input parameters for the `registerNode` function with signature `registerNode(string)` and selector `[39, 198, 244, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "registerNode", abi = "registerNode(string)")]
    pub struct RegisterNodeCall {
        pub timezone_location: String,
    }
    #[doc = "Container type for all input parameters for the `setRewardNetwork` function with signature `setRewardNetwork(address,uint256)` and selector `[213, 101, 242, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setRewardNetwork", abi = "setRewardNetwork(address,uint256)")]
    pub struct SetRewardNetworkCall {
        pub node_address: ethers::core::types::Address,
        pub network: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setSmoothingPoolRegistrationState` function with signature `setSmoothingPoolRegistrationState(bool)` and selector `[153, 40, 63, 139]`"]
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
        name = "setSmoothingPoolRegistrationState",
        abi = "setSmoothingPoolRegistrationState(bool)"
    )]
    pub struct SetSmoothingPoolRegistrationStateCall {
        pub state: bool,
    }
    #[doc = "Container type for all input parameters for the `setTimezoneLocation` function with signature `setTimezoneLocation(string)` and selector `[167, 230, 232, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setTimezoneLocation", abi = "setTimezoneLocation(string)")]
    pub struct SetTimezoneLocationCall {
        pub timezone_location: String,
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
    pub enum RocketNodeManagerCalls {
        GetAverageNodeFee(GetAverageNodeFeeCall),
        GetFeeDistributorInitialised(GetFeeDistributorInitialisedCall),
        GetNodeAddresses(GetNodeAddressesCall),
        GetNodeAt(GetNodeAtCall),
        GetNodeCount(GetNodeCountCall),
        GetNodeCountPerTimezone(GetNodeCountPerTimezoneCall),
        GetNodeDetails(GetNodeDetailsCall),
        GetNodeExists(GetNodeExistsCall),
        GetNodePendingWithdrawalAddress(GetNodePendingWithdrawalAddressCall),
        GetNodeRegistrationTime(GetNodeRegistrationTimeCall),
        GetNodeTimezoneLocation(GetNodeTimezoneLocationCall),
        GetNodeWithdrawalAddress(GetNodeWithdrawalAddressCall),
        GetRewardNetwork(GetRewardNetworkCall),
        GetSmoothingPoolRegisteredNodeCount(GetSmoothingPoolRegisteredNodeCountCall),
        GetSmoothingPoolRegistrationChanged(GetSmoothingPoolRegistrationChangedCall),
        GetSmoothingPoolRegistrationState(GetSmoothingPoolRegistrationStateCall),
        InitialiseFeeDistributor(InitialiseFeeDistributorCall),
        RegisterNode(RegisterNodeCall),
        SetRewardNetwork(SetRewardNetworkCall),
        SetSmoothingPoolRegistrationState(SetSmoothingPoolRegistrationStateCall),
        SetTimezoneLocation(SetTimezoneLocationCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for RocketNodeManagerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetAverageNodeFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketNodeManagerCalls::GetAverageNodeFee(decoded));
            }
            if let Ok(decoded) =
                <GetFeeDistributorInitialisedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketNodeManagerCalls::GetFeeDistributorInitialised(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetNodeAddressesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketNodeManagerCalls::GetNodeAddresses(decoded));
            }
            if let Ok(decoded) =
                <GetNodeAtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketNodeManagerCalls::GetNodeAt(decoded));
            }
            if let Ok(decoded) =
                <GetNodeCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketNodeManagerCalls::GetNodeCount(decoded));
            }
            if let Ok(decoded) =
                <GetNodeCountPerTimezoneCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketNodeManagerCalls::GetNodeCountPerTimezone(decoded));
            }
            if let Ok(decoded) =
                <GetNodeDetailsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketNodeManagerCalls::GetNodeDetails(decoded));
            }
            if let Ok(decoded) =
                <GetNodeExistsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketNodeManagerCalls::GetNodeExists(decoded));
            }
            if let Ok(decoded) =
                <GetNodePendingWithdrawalAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketNodeManagerCalls::GetNodePendingWithdrawalAddress(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetNodeRegistrationTimeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketNodeManagerCalls::GetNodeRegistrationTime(decoded));
            }
            if let Ok(decoded) =
                <GetNodeTimezoneLocationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketNodeManagerCalls::GetNodeTimezoneLocation(decoded));
            }
            if let Ok(decoded) =
                <GetNodeWithdrawalAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketNodeManagerCalls::GetNodeWithdrawalAddress(decoded));
            }
            if let Ok(decoded) =
                <GetRewardNetworkCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketNodeManagerCalls::GetRewardNetwork(decoded));
            }
            if let Ok(decoded) =
                <GetSmoothingPoolRegisteredNodeCountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketNodeManagerCalls::GetSmoothingPoolRegisteredNodeCount(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetSmoothingPoolRegistrationChangedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketNodeManagerCalls::GetSmoothingPoolRegistrationChanged(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetSmoothingPoolRegistrationStateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketNodeManagerCalls::GetSmoothingPoolRegistrationState(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InitialiseFeeDistributorCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketNodeManagerCalls::InitialiseFeeDistributor(decoded));
            }
            if let Ok(decoded) =
                <RegisterNodeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketNodeManagerCalls::RegisterNode(decoded));
            }
            if let Ok(decoded) =
                <SetRewardNetworkCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketNodeManagerCalls::SetRewardNetwork(decoded));
            }
            if let Ok(decoded) =
                <SetSmoothingPoolRegistrationStateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(RocketNodeManagerCalls::SetSmoothingPoolRegistrationState(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetTimezoneLocationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketNodeManagerCalls::SetTimezoneLocation(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RocketNodeManagerCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for RocketNodeManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                RocketNodeManagerCalls::GetAverageNodeFee(element) => element.encode(),
                RocketNodeManagerCalls::GetFeeDistributorInitialised(element) => element.encode(),
                RocketNodeManagerCalls::GetNodeAddresses(element) => element.encode(),
                RocketNodeManagerCalls::GetNodeAt(element) => element.encode(),
                RocketNodeManagerCalls::GetNodeCount(element) => element.encode(),
                RocketNodeManagerCalls::GetNodeCountPerTimezone(element) => element.encode(),
                RocketNodeManagerCalls::GetNodeDetails(element) => element.encode(),
                RocketNodeManagerCalls::GetNodeExists(element) => element.encode(),
                RocketNodeManagerCalls::GetNodePendingWithdrawalAddress(element) => {
                    element.encode()
                }
                RocketNodeManagerCalls::GetNodeRegistrationTime(element) => element.encode(),
                RocketNodeManagerCalls::GetNodeTimezoneLocation(element) => element.encode(),
                RocketNodeManagerCalls::GetNodeWithdrawalAddress(element) => element.encode(),
                RocketNodeManagerCalls::GetRewardNetwork(element) => element.encode(),
                RocketNodeManagerCalls::GetSmoothingPoolRegisteredNodeCount(element) => {
                    element.encode()
                }
                RocketNodeManagerCalls::GetSmoothingPoolRegistrationChanged(element) => {
                    element.encode()
                }
                RocketNodeManagerCalls::GetSmoothingPoolRegistrationState(element) => {
                    element.encode()
                }
                RocketNodeManagerCalls::InitialiseFeeDistributor(element) => element.encode(),
                RocketNodeManagerCalls::RegisterNode(element) => element.encode(),
                RocketNodeManagerCalls::SetRewardNetwork(element) => element.encode(),
                RocketNodeManagerCalls::SetSmoothingPoolRegistrationState(element) => {
                    element.encode()
                }
                RocketNodeManagerCalls::SetTimezoneLocation(element) => element.encode(),
                RocketNodeManagerCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for RocketNodeManagerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                RocketNodeManagerCalls::GetAverageNodeFee(element) => element.fmt(f),
                RocketNodeManagerCalls::GetFeeDistributorInitialised(element) => element.fmt(f),
                RocketNodeManagerCalls::GetNodeAddresses(element) => element.fmt(f),
                RocketNodeManagerCalls::GetNodeAt(element) => element.fmt(f),
                RocketNodeManagerCalls::GetNodeCount(element) => element.fmt(f),
                RocketNodeManagerCalls::GetNodeCountPerTimezone(element) => element.fmt(f),
                RocketNodeManagerCalls::GetNodeDetails(element) => element.fmt(f),
                RocketNodeManagerCalls::GetNodeExists(element) => element.fmt(f),
                RocketNodeManagerCalls::GetNodePendingWithdrawalAddress(element) => element.fmt(f),
                RocketNodeManagerCalls::GetNodeRegistrationTime(element) => element.fmt(f),
                RocketNodeManagerCalls::GetNodeTimezoneLocation(element) => element.fmt(f),
                RocketNodeManagerCalls::GetNodeWithdrawalAddress(element) => element.fmt(f),
                RocketNodeManagerCalls::GetRewardNetwork(element) => element.fmt(f),
                RocketNodeManagerCalls::GetSmoothingPoolRegisteredNodeCount(element) => {
                    element.fmt(f)
                }
                RocketNodeManagerCalls::GetSmoothingPoolRegistrationChanged(element) => {
                    element.fmt(f)
                }
                RocketNodeManagerCalls::GetSmoothingPoolRegistrationState(element) => {
                    element.fmt(f)
                }
                RocketNodeManagerCalls::InitialiseFeeDistributor(element) => element.fmt(f),
                RocketNodeManagerCalls::RegisterNode(element) => element.fmt(f),
                RocketNodeManagerCalls::SetRewardNetwork(element) => element.fmt(f),
                RocketNodeManagerCalls::SetSmoothingPoolRegistrationState(element) => {
                    element.fmt(f)
                }
                RocketNodeManagerCalls::SetTimezoneLocation(element) => element.fmt(f),
                RocketNodeManagerCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetAverageNodeFeeCall> for RocketNodeManagerCalls {
        fn from(var: GetAverageNodeFeeCall) -> Self {
            RocketNodeManagerCalls::GetAverageNodeFee(var)
        }
    }
    impl ::std::convert::From<GetFeeDistributorInitialisedCall> for RocketNodeManagerCalls {
        fn from(var: GetFeeDistributorInitialisedCall) -> Self {
            RocketNodeManagerCalls::GetFeeDistributorInitialised(var)
        }
    }
    impl ::std::convert::From<GetNodeAddressesCall> for RocketNodeManagerCalls {
        fn from(var: GetNodeAddressesCall) -> Self {
            RocketNodeManagerCalls::GetNodeAddresses(var)
        }
    }
    impl ::std::convert::From<GetNodeAtCall> for RocketNodeManagerCalls {
        fn from(var: GetNodeAtCall) -> Self {
            RocketNodeManagerCalls::GetNodeAt(var)
        }
    }
    impl ::std::convert::From<GetNodeCountCall> for RocketNodeManagerCalls {
        fn from(var: GetNodeCountCall) -> Self {
            RocketNodeManagerCalls::GetNodeCount(var)
        }
    }
    impl ::std::convert::From<GetNodeCountPerTimezoneCall> for RocketNodeManagerCalls {
        fn from(var: GetNodeCountPerTimezoneCall) -> Self {
            RocketNodeManagerCalls::GetNodeCountPerTimezone(var)
        }
    }
    impl ::std::convert::From<GetNodeDetailsCall> for RocketNodeManagerCalls {
        fn from(var: GetNodeDetailsCall) -> Self {
            RocketNodeManagerCalls::GetNodeDetails(var)
        }
    }
    impl ::std::convert::From<GetNodeExistsCall> for RocketNodeManagerCalls {
        fn from(var: GetNodeExistsCall) -> Self {
            RocketNodeManagerCalls::GetNodeExists(var)
        }
    }
    impl ::std::convert::From<GetNodePendingWithdrawalAddressCall> for RocketNodeManagerCalls {
        fn from(var: GetNodePendingWithdrawalAddressCall) -> Self {
            RocketNodeManagerCalls::GetNodePendingWithdrawalAddress(var)
        }
    }
    impl ::std::convert::From<GetNodeRegistrationTimeCall> for RocketNodeManagerCalls {
        fn from(var: GetNodeRegistrationTimeCall) -> Self {
            RocketNodeManagerCalls::GetNodeRegistrationTime(var)
        }
    }
    impl ::std::convert::From<GetNodeTimezoneLocationCall> for RocketNodeManagerCalls {
        fn from(var: GetNodeTimezoneLocationCall) -> Self {
            RocketNodeManagerCalls::GetNodeTimezoneLocation(var)
        }
    }
    impl ::std::convert::From<GetNodeWithdrawalAddressCall> for RocketNodeManagerCalls {
        fn from(var: GetNodeWithdrawalAddressCall) -> Self {
            RocketNodeManagerCalls::GetNodeWithdrawalAddress(var)
        }
    }
    impl ::std::convert::From<GetRewardNetworkCall> for RocketNodeManagerCalls {
        fn from(var: GetRewardNetworkCall) -> Self {
            RocketNodeManagerCalls::GetRewardNetwork(var)
        }
    }
    impl ::std::convert::From<GetSmoothingPoolRegisteredNodeCountCall> for RocketNodeManagerCalls {
        fn from(var: GetSmoothingPoolRegisteredNodeCountCall) -> Self {
            RocketNodeManagerCalls::GetSmoothingPoolRegisteredNodeCount(var)
        }
    }
    impl ::std::convert::From<GetSmoothingPoolRegistrationChangedCall> for RocketNodeManagerCalls {
        fn from(var: GetSmoothingPoolRegistrationChangedCall) -> Self {
            RocketNodeManagerCalls::GetSmoothingPoolRegistrationChanged(var)
        }
    }
    impl ::std::convert::From<GetSmoothingPoolRegistrationStateCall> for RocketNodeManagerCalls {
        fn from(var: GetSmoothingPoolRegistrationStateCall) -> Self {
            RocketNodeManagerCalls::GetSmoothingPoolRegistrationState(var)
        }
    }
    impl ::std::convert::From<InitialiseFeeDistributorCall> for RocketNodeManagerCalls {
        fn from(var: InitialiseFeeDistributorCall) -> Self {
            RocketNodeManagerCalls::InitialiseFeeDistributor(var)
        }
    }
    impl ::std::convert::From<RegisterNodeCall> for RocketNodeManagerCalls {
        fn from(var: RegisterNodeCall) -> Self {
            RocketNodeManagerCalls::RegisterNode(var)
        }
    }
    impl ::std::convert::From<SetRewardNetworkCall> for RocketNodeManagerCalls {
        fn from(var: SetRewardNetworkCall) -> Self {
            RocketNodeManagerCalls::SetRewardNetwork(var)
        }
    }
    impl ::std::convert::From<SetSmoothingPoolRegistrationStateCall> for RocketNodeManagerCalls {
        fn from(var: SetSmoothingPoolRegistrationStateCall) -> Self {
            RocketNodeManagerCalls::SetSmoothingPoolRegistrationState(var)
        }
    }
    impl ::std::convert::From<SetTimezoneLocationCall> for RocketNodeManagerCalls {
        fn from(var: SetTimezoneLocationCall) -> Self {
            RocketNodeManagerCalls::SetTimezoneLocation(var)
        }
    }
    impl ::std::convert::From<VersionCall> for RocketNodeManagerCalls {
        fn from(var: VersionCall) -> Self {
            RocketNodeManagerCalls::Version(var)
        }
    }
    #[doc = "Container type for all return fields from the `getAverageNodeFee` function with signature `getAverageNodeFee(address)` and selector `[65, 77, 209, 210]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAverageNodeFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getFeeDistributorInitialised` function with signature `getFeeDistributorInitialised(address)` and selector `[146, 126, 206, 79]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetFeeDistributorInitialisedReturn(pub bool);
    #[doc = "Container type for all return fields from the `getNodeAddresses` function with signature `getNodeAddresses(uint256,uint256)` and selector `[45, 127, 33, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeAddressesReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
    #[doc = "Container type for all return fields from the `getNodeAt` function with signature `getNodeAt(uint256)` and selector `[186, 117, 216, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeAtReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getNodeCount` function with signature `getNodeCount()` and selector `[57, 191, 57, 126]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getNodeCountPerTimezone` function with signature `getNodeCountPerTimezone(uint256,uint256)` and selector `[41, 85, 69, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeCountPerTimezoneReturn(
        pub ::std::vec::Vec<(String, ethers::core::types::U256)>,
    );
    #[doc = "Container type for all return fields from the `getNodeExists` function with signature `getNodeExists(address)` and selector `[101, 212, 23, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeExistsReturn(pub bool);
    #[doc = "Container type for all return fields from the `getNodePendingWithdrawalAddress` function with signature `getNodePendingWithdrawalAddress(address)` and selector `[253, 65, 37, 19]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodePendingWithdrawalAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getNodeRegistrationTime` function with signature `getNodeRegistrationTime(address)` and selector `[2, 216, 167, 50]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeRegistrationTimeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getNodeTimezoneLocation` function with signature `getNodeTimezoneLocation(address)` and selector `[176, 24, 240, 38]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeTimezoneLocationReturn(pub String);
    #[doc = "Container type for all return fields from the `getNodeWithdrawalAddress` function with signature `getNodeWithdrawalAddress(address)` and selector `[91, 73, 255, 98]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNodeWithdrawalAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getRewardNetwork` function with signature `getRewardNetwork(address)` and selector `[67, 248, 137, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetRewardNetworkReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getSmoothingPoolRegisteredNodeCount` function with signature `getSmoothingPoolRegisteredNodeCount(uint256,uint256)` and selector `[183, 21, 161, 170]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetSmoothingPoolRegisteredNodeCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getSmoothingPoolRegistrationChanged` function with signature `getSmoothingPoolRegistrationChanged(address)` and selector `[77, 153, 246, 51]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetSmoothingPoolRegistrationChangedReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getSmoothingPoolRegistrationState` function with signature `getSmoothingPoolRegistrationState(address)` and selector `[164, 206, 249, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetSmoothingPoolRegistrationStateReturn(pub bool);
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
    #[doc = "`NodeDetails(bool,uint256,string,bool,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,address,address,bool,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NodeDetails {
        pub exists: bool,
        pub registration_time: ethers::core::types::U256,
        pub timezone_location: String,
        pub fee_distributor_initialised: bool,
        pub fee_distributor_address: ethers::core::types::Address,
        pub reward_network: ethers::core::types::U256,
        pub rpl_stake: ethers::core::types::U256,
        pub effective_rpl_stake: ethers::core::types::U256,
        pub minimum_rpl_stake: ethers::core::types::U256,
        pub maximum_rpl_stake: ethers::core::types::U256,
        pub minipool_limit: ethers::core::types::U256,
        pub minipool_count: ethers::core::types::U256,
        pub balance_eth: ethers::core::types::U256,
        pub balance_reth: ethers::core::types::U256,
        pub balance_rpl: ethers::core::types::U256,
        pub balance_old_rpl: ethers::core::types::U256,
        pub withdrawal_address: ethers::core::types::Address,
        pub pending_withdrawal_address: ethers::core::types::Address,
        pub smoothing_pool_registration_state: bool,
        pub smoothing_pool_registration_changed: ethers::core::types::U256,
    }
    #[doc = "`TimezoneCount(string,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TimezoneCount {
        pub timezone: String,
        pub count: ethers::core::types::U256,
    }
}
