pub use nonfungible_position_manager::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod nonfungible_position_manager {
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
    #[doc = "NonfungiblePositionManager was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static NONFUNGIBLEPOSITIONMANAGER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_factory\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_WETH9\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_tokenDescriptor_\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"approved\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"Approval\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\"}],\"name\":\"ApprovalForAll\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\"}],\"name\":\"Collect\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint128\",\"name\":\"liquidity\",\"type\":\"uint128\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\"}],\"name\":\"DecreaseLiquidity\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint128\",\"name\":\"liquidity\",\"type\":\"uint128\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\"}],\"name\":\"IncreaseLiquidity\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"Transfer\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"PERMIT_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"WETH9\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"approve\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"baseURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"burn\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint128\",\"name\":\"amount0Max\",\"type\":\"uint128\"},{\"internalType\":\"uint128\",\"name\":\"amount1Max\",\"type\":\"uint128\"}],\"internalType\":\"struct INonfungiblePositionManager.CollectParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"collect\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token1\",\"type\":\"address\"},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\"}],\"name\":\"createAndInitializePoolIfNecessary\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint128\",\"name\":\"liquidity\",\"type\":\"uint128\"},{\"internalType\":\"uint256\",\"name\":\"amount0Min\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount1Min\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"}],\"internalType\":\"struct INonfungiblePositionManager.DecreaseLiquidityParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"decreaseLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"getApproved\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount0Desired\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount1Desired\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount0Min\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount1Min\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"}],\"internalType\":\"struct INonfungiblePositionManager.IncreaseLiquidityParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"increaseLiquidity\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"liquidity\",\"type\":\"uint128\"},{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"}],\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"token0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token1\",\"type\":\"address\"},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\"},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\"},{\"internalType\":\"uint256\",\"name\":\"amount0Desired\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount1Desired\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount0Min\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount1Min\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"}],\"internalType\":\"struct INonfungiblePositionManager.MintParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"mint\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint128\",\"name\":\"liquidity\",\"type\":\"uint128\"},{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"data\",\"type\":\"bytes[]\"}],\"name\":\"multicall\",\"outputs\":[{\"internalType\":\"bytes[]\",\"name\":\"results\",\"type\":\"bytes[]\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"ownerOf\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"permit\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"positions\",\"outputs\":[{\"internalType\":\"uint96\",\"name\":\"nonce\",\"type\":\"uint96\"},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token1\",\"type\":\"address\"},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\"},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\"},{\"internalType\":\"uint128\",\"name\":\"liquidity\",\"type\":\"uint128\"},{\"internalType\":\"uint256\",\"name\":\"feeGrowthInside0LastX128\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"feeGrowthInside1LastX128\",\"type\":\"uint256\"},{\"internalType\":\"uint128\",\"name\":\"tokensOwed0\",\"type\":\"uint128\"},{\"internalType\":\"uint128\",\"name\":\"tokensOwed1\",\"type\":\"uint128\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"refundETH\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"safeTransferFrom\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\"}],\"name\":\"safeTransferFrom\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"selfPermit\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"selfPermitAllowed\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"selfPermitAllowedIfNecessary\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"selfPermitIfNecessary\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\"}],\"name\":\"setApprovalForAll\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\"}],\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"}],\"name\":\"sweepToken\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"}],\"name\":\"tokenByIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"}],\"name\":\"tokenOfOwnerByIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"tokenURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"transferFrom\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount0Owed\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount1Owed\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"uniswapV3MintCallback\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"}],\"name\":\"unwrapWETH9\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"stateMutability\":\"payable\",\"type\":\"receive\"}]") . expect ("invalid abi")
        });
    pub struct NonfungiblePositionManager<M>(ethers::contract::Contract<M>);
    impl<M> Clone for NonfungiblePositionManager<M> {
        fn clone(&self) -> Self {
            NonfungiblePositionManager(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for NonfungiblePositionManager<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for NonfungiblePositionManager<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(NonfungiblePositionManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> NonfungiblePositionManager<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                NONFUNGIBLEPOSITIONMANAGER_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function"]
        pub fn permit_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `WETH9` (0x4aa4a4fc) function"]
        pub fn weth9(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([74, 164, 164, 252], ())
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
        #[doc = "Calls the contract's `baseURI` (0x6c0360eb) function"]
        pub fn base_uri(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([108, 3, 96, 235], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x42966c68) function"]
        pub fn burn(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collect` (0xfc6f7865) function"]
        pub fn collect(
            &self,
            params: CollectParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([252, 111, 120, 101], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createAndInitializePoolIfNecessary` (0x13ead562) function"]
        pub fn create_and_initialize_pool_if_necessary(
            &self,
            token_0: ethers::core::types::Address,
            token_1: ethers::core::types::Address,
            fee: u32,
            sqrt_price_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([19, 234, 213, 98], (token_0, token_1, fee, sqrt_price_x96))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseLiquidity` (0x0c49ccbe) function"]
        pub fn decrease_liquidity(
            &self,
            params: DecreaseLiquidityParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([12, 73, 204, 190], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
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
        #[doc = "Calls the contract's `increaseLiquidity` (0x219f5d17) function"]
        pub fn increase_liquidity(
            &self,
            params: IncreaseLiquidityParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (u128, ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([33, 159, 93, 23], (params,))
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
        #[doc = "Calls the contract's `mint` (0x88316456) function"]
        pub fn mint(
            &self,
            params: MintParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                u128,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([136, 49, 100, 86], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `multicall` (0xac9650d8) function"]
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::Bytes>>
        {
            self.0
                .method_hash([172, 150, 80, 216], data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        #[doc = "Calls the contract's `permit` (0x7ac2ff7b) function"]
        pub fn permit(
            &self,
            spender: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 194, 255, 123], (spender, token_id, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `positions` (0x99fbab88) function"]
        pub fn positions(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::Address,
                u32,
                i32,
                i32,
                u128,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
                u128,
            ),
        > {
            self.0
                .method_hash([153, 251, 171, 136], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `refundETH` (0x12210e8a) function"]
        pub fn refund_eth(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 33, 14, 138], ())
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
        #[doc = "Calls the contract's `selfPermit` (0xf3995c67) function"]
        pub fn self_permit(
            &self,
            token: ethers::core::types::Address,
            value: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 153, 92, 103], (token, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `selfPermitAllowed` (0x4659a494) function"]
        pub fn self_permit_allowed(
            &self,
            token: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
            expiry: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 89, 164, 148], (token, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `selfPermitAllowedIfNecessary` (0xa4a78f0c) function"]
        pub fn self_permit_allowed_if_necessary(
            &self,
            token: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
            expiry: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 167, 143, 12], (token, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `selfPermitIfNecessary` (0xc2e3140a) function"]
        pub fn self_permit_if_necessary(
            &self,
            token: ethers::core::types::Address,
            value: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 227, 20, 10], (token, value, deadline, v, r, s))
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
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sweepToken` (0xdf2ab5bb) function"]
        pub fn sweep_token(
            &self,
            token: ethers::core::types::Address,
            amount_minimum: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 42, 181, 187], (token, amount_minimum, recipient))
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
        #[doc = "Calls the contract's `uniswapV3MintCallback` (0xd3487997) function"]
        pub fn uniswap_v3_mint_callback(
            &self,
            amount_0_owed: ethers::core::types::U256,
            amount_1_owed: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 72, 121, 151], (amount_0_owed, amount_1_owed, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapWETH9` (0x49404b7c) function"]
        pub fn unwrap_weth9(
            &self,
            amount_minimum: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 64, 75, 124], (amount_minimum, recipient))
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
        #[doc = "Gets the contract's `Collect` event"]
        pub fn collect_filter(&self) -> ethers::contract::builders::Event<M, CollectFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DecreaseLiquidity` event"]
        pub fn decrease_liquidity_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DecreaseLiquidityFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IncreaseLiquidity` event"]
        pub fn increase_liquidity_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, IncreaseLiquidityFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, NonfungiblePositionManagerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for NonfungiblePositionManager<M>
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
    #[ethevent(name = "Collect", abi = "Collect(uint256,address,uint256,uint256)")]
    pub struct CollectFilter {
        #[ethevent(indexed)]
        pub token_id: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
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
        name = "DecreaseLiquidity",
        abi = "DecreaseLiquidity(uint256,uint128,uint256,uint256)"
    )]
    pub struct DecreaseLiquidityFilter {
        #[ethevent(indexed)]
        pub token_id: ethers::core::types::U256,
        pub liquidity: u128,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
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
        name = "IncreaseLiquidity",
        abi = "IncreaseLiquidity(uint256,uint128,uint256,uint256)"
    )]
    pub struct IncreaseLiquidityFilter {
        #[ethevent(indexed)]
        pub token_id: ethers::core::types::U256,
        pub liquidity: u128,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
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
    pub enum NonfungiblePositionManagerEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        CollectFilter(CollectFilter),
        DecreaseLiquidityFilter(DecreaseLiquidityFilter),
        IncreaseLiquidityFilter(IncreaseLiquidityFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for NonfungiblePositionManagerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(NonfungiblePositionManagerEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(NonfungiblePositionManagerEvents::ApprovalForAllFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = CollectFilter::decode_log(log) {
                return Ok(NonfungiblePositionManagerEvents::CollectFilter(decoded));
            }
            if let Ok(decoded) = DecreaseLiquidityFilter::decode_log(log) {
                return Ok(NonfungiblePositionManagerEvents::DecreaseLiquidityFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = IncreaseLiquidityFilter::decode_log(log) {
                return Ok(NonfungiblePositionManagerEvents::IncreaseLiquidityFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(NonfungiblePositionManagerEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for NonfungiblePositionManagerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                NonfungiblePositionManagerEvents::ApprovalFilter(element) => element.fmt(f),
                NonfungiblePositionManagerEvents::ApprovalForAllFilter(element) => element.fmt(f),
                NonfungiblePositionManagerEvents::CollectFilter(element) => element.fmt(f),
                NonfungiblePositionManagerEvents::DecreaseLiquidityFilter(element) => {
                    element.fmt(f)
                }
                NonfungiblePositionManagerEvents::IncreaseLiquidityFilter(element) => {
                    element.fmt(f)
                }
                NonfungiblePositionManagerEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    #[doc = "Container type for all input parameters for the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `[48, 173, 248, 31]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "PERMIT_TYPEHASH", abi = "PERMIT_TYPEHASH()")]
    pub struct PermitTypehashCall;
    #[doc = "Container type for all input parameters for the `WETH9` function with signature `WETH9()` and selector `[74, 164, 164, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "WETH9", abi = "WETH9()")]
    pub struct Weth9Call;
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
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
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
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `baseURI` function with signature `baseURI()` and selector `[108, 3, 96, 235]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "baseURI", abi = "baseURI()")]
    pub struct BaseURICall;
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(uint256)` and selector `[66, 150, 108, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `collect` function with signature `collect((uint256,address,uint128,uint128))` and selector `[252, 111, 120, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "collect", abi = "collect((uint256,address,uint128,uint128))")]
    pub struct CollectCall {
        pub params: CollectParams,
    }
    #[doc = "Container type for all input parameters for the `createAndInitializePoolIfNecessary` function with signature `createAndInitializePoolIfNecessary(address,address,uint24,uint160)` and selector `[19, 234, 213, 98]`"]
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
        name = "createAndInitializePoolIfNecessary",
        abi = "createAndInitializePoolIfNecessary(address,address,uint24,uint160)"
    )]
    pub struct CreateAndInitializePoolIfNecessaryCall {
        pub token_0: ethers::core::types::Address,
        pub token_1: ethers::core::types::Address,
        pub fee: u32,
        pub sqrt_price_x96: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `decreaseLiquidity` function with signature `decreaseLiquidity((uint256,uint128,uint256,uint256,uint256))` and selector `[12, 73, 204, 190]`"]
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
        name = "decreaseLiquidity",
        abi = "decreaseLiquidity((uint256,uint128,uint256,uint256,uint256))"
    )]
    pub struct DecreaseLiquidityCall {
        pub params: DecreaseLiquidityParams,
    }
    #[doc = "Container type for all input parameters for the `factory` function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    #[doc = "Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `[8, 24, 18, 252]`"]
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
    #[doc = "Container type for all input parameters for the `increaseLiquidity` function with signature `increaseLiquidity((uint256,uint256,uint256,uint256,uint256,uint256))` and selector `[33, 159, 93, 23]`"]
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
        name = "increaseLiquidity",
        abi = "increaseLiquidity((uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct IncreaseLiquidityCall {
        pub params: IncreaseLiquidityParams,
    }
    #[doc = "Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
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
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint((address,address,uint24,int24,int24,uint256,uint256,uint256,uint256,address,uint256))` and selector `[136, 49, 100, 86]`"]
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
        name = "mint",
        abi = "mint((address,address,uint24,int24,int24,uint256,uint256,uint256,uint256,address,uint256))"
    )]
    pub struct MintCall {
        pub params: MintParams,
    }
    #[doc = "Container type for all input parameters for the `multicall` function with signature `multicall(bytes[])` and selector `[172, 150, 80, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<ethers::core::types::Bytes>,
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
    #[doc = "Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `[99, 82, 33, 30]`"]
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
    #[doc = "Container type for all input parameters for the `permit` function with signature `permit(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[122, 194, 255, 123]`"]
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
        abi = "permit(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub spender: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `positions` function with signature `positions(uint256)` and selector `[153, 251, 171, 136]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "positions", abi = "positions(uint256)")]
    pub struct PositionsCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `refundETH` function with signature `refundETH()` and selector `[18, 33, 14, 138]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "refundETH", abi = "refundETH()")]
    pub struct RefundETHCall;
    #[doc = "Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `[66, 132, 46, 14]`"]
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
    #[doc = "Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `[184, 141, 79, 222]`"]
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
    #[doc = "Container type for all input parameters for the `selfPermit` function with signature `selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[243, 153, 92, 103]`"]
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
        name = "selfPermit",
        abi = "selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitCall {
        pub token: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `selfPermitAllowed` function with signature `selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[70, 89, 164, 148]`"]
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
        name = "selfPermitAllowed",
        abi = "selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitAllowedCall {
        pub token: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
        pub expiry: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `selfPermitAllowedIfNecessary` function with signature `selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[164, 167, 143, 12]`"]
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
        name = "selfPermitAllowedIfNecessary",
        abi = "selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitAllowedIfNecessaryCall {
        pub token: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
        pub expiry: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `selfPermitIfNecessary` function with signature `selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[194, 227, 20, 10]`"]
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
        name = "selfPermitIfNecessary",
        abi = "selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitIfNecessaryCall {
        pub token: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `[162, 44, 180, 101]`"]
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
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
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
    #[doc = "Container type for all input parameters for the `sweepToken` function with signature `sweepToken(address,uint256,address)` and selector `[223, 42, 181, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "sweepToken", abi = "sweepToken(address,uint256,address)")]
    pub struct SweepTokenCall {
        pub token: ethers::core::types::Address,
        pub amount_minimum: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `tokenByIndex` function with signature `tokenByIndex(uint256)` and selector `[79, 108, 204, 231]`"]
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
    #[doc = "Container type for all input parameters for the `tokenOfOwnerByIndex` function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `[47, 116, 92, 89]`"]
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
    #[doc = "Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `[200, 123, 86, 221]`"]
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
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `uniswapV3MintCallback` function with signature `uniswapV3MintCallback(uint256,uint256,bytes)` and selector `[211, 72, 121, 151]`"]
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
        name = "uniswapV3MintCallback",
        abi = "uniswapV3MintCallback(uint256,uint256,bytes)"
    )]
    pub struct UniswapV3MintCallbackCall {
        pub amount_0_owed: ethers::core::types::U256,
        pub amount_1_owed: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `unwrapWETH9` function with signature `unwrapWETH9(uint256,address)` and selector `[73, 64, 75, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "unwrapWETH9", abi = "unwrapWETH9(uint256,address)")]
    pub struct UnwrapWETH9Call {
        pub amount_minimum: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum NonfungiblePositionManagerCalls {
        DomainSeparator(DomainSeparatorCall),
        PermitTypehash(PermitTypehashCall),
        Weth9(Weth9Call),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        BaseURI(BaseURICall),
        Burn(BurnCall),
        Collect(CollectCall),
        CreateAndInitializePoolIfNecessary(CreateAndInitializePoolIfNecessaryCall),
        DecreaseLiquidity(DecreaseLiquidityCall),
        Factory(FactoryCall),
        GetApproved(GetApprovedCall),
        IncreaseLiquidity(IncreaseLiquidityCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Mint(MintCall),
        Multicall(MulticallCall),
        Name(NameCall),
        OwnerOf(OwnerOfCall),
        Permit(PermitCall),
        Positions(PositionsCall),
        RefundETH(RefundETHCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithData(SafeTransferFromWithDataCall),
        SelfPermit(SelfPermitCall),
        SelfPermitAllowed(SelfPermitAllowedCall),
        SelfPermitAllowedIfNecessary(SelfPermitAllowedIfNecessaryCall),
        SelfPermitIfNecessary(SelfPermitIfNecessaryCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        SweepToken(SweepTokenCall),
        Symbol(SymbolCall),
        TokenByIndex(TokenByIndexCall),
        TokenOfOwnerByIndex(TokenOfOwnerByIndexCall),
        TokenURI(TokenURICall),
        TotalSupply(TotalSupplyCall),
        TransferFrom(TransferFromCall),
        UniswapV3MintCallback(UniswapV3MintCallbackCall),
        UnwrapWETH9(UnwrapWETH9Call),
    }
    impl ethers::core::abi::AbiDecode for NonfungiblePositionManagerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <PermitTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::PermitTypehash(decoded));
            }
            if let Ok(decoded) = <Weth9Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::Weth9(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BaseURICall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::BaseURI(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(NonfungiblePositionManagerCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <CollectCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::Collect(decoded));
            }
            if let Ok(decoded) =
                <CreateAndInitializePoolIfNecessaryCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    NonfungiblePositionManagerCalls::CreateAndInitializePoolIfNecessary(decoded),
                );
            }
            if let Ok(decoded) =
                <DecreaseLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::DecreaseLiquidity(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <GetApprovedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::GetApproved(decoded));
            }
            if let Ok(decoded) =
                <IncreaseLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::IncreaseLiquidity(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(NonfungiblePositionManagerCalls::Mint(decoded));
            }
            if let Ok(decoded) =
                <MulticallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::Multicall(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(NonfungiblePositionManagerCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <OwnerOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::OwnerOf(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::Permit(decoded));
            }
            if let Ok(decoded) =
                <PositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::Positions(decoded));
            }
            if let Ok(decoded) =
                <RefundETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::RefundETH(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromWithDataCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(NonfungiblePositionManagerCalls::SafeTransferFromWithData(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SelfPermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::SelfPermit(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::SelfPermitAllowed(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitAllowedIfNecessaryCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(NonfungiblePositionManagerCalls::SelfPermitAllowedIfNecessary(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitIfNecessaryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::SelfPermitIfNecessary(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <SweepTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::SweepToken(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TokenByIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::TokenByIndex(decoded));
            }
            if let Ok(decoded) =
                <TokenOfOwnerByIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::TokenOfOwnerByIndex(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TokenURICall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::TokenURI(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UniswapV3MintCallbackCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::UniswapV3MintCallback(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UnwrapWETH9Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonfungiblePositionManagerCalls::UnwrapWETH9(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for NonfungiblePositionManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                NonfungiblePositionManagerCalls::DomainSeparator(element) => element.encode(),
                NonfungiblePositionManagerCalls::PermitTypehash(element) => element.encode(),
                NonfungiblePositionManagerCalls::Weth9(element) => element.encode(),
                NonfungiblePositionManagerCalls::Approve(element) => element.encode(),
                NonfungiblePositionManagerCalls::BalanceOf(element) => element.encode(),
                NonfungiblePositionManagerCalls::BaseURI(element) => element.encode(),
                NonfungiblePositionManagerCalls::Burn(element) => element.encode(),
                NonfungiblePositionManagerCalls::Collect(element) => element.encode(),
                NonfungiblePositionManagerCalls::CreateAndInitializePoolIfNecessary(element) => {
                    element.encode()
                }
                NonfungiblePositionManagerCalls::DecreaseLiquidity(element) => element.encode(),
                NonfungiblePositionManagerCalls::Factory(element) => element.encode(),
                NonfungiblePositionManagerCalls::GetApproved(element) => element.encode(),
                NonfungiblePositionManagerCalls::IncreaseLiquidity(element) => element.encode(),
                NonfungiblePositionManagerCalls::IsApprovedForAll(element) => element.encode(),
                NonfungiblePositionManagerCalls::Mint(element) => element.encode(),
                NonfungiblePositionManagerCalls::Multicall(element) => element.encode(),
                NonfungiblePositionManagerCalls::Name(element) => element.encode(),
                NonfungiblePositionManagerCalls::OwnerOf(element) => element.encode(),
                NonfungiblePositionManagerCalls::Permit(element) => element.encode(),
                NonfungiblePositionManagerCalls::Positions(element) => element.encode(),
                NonfungiblePositionManagerCalls::RefundETH(element) => element.encode(),
                NonfungiblePositionManagerCalls::SafeTransferFrom(element) => element.encode(),
                NonfungiblePositionManagerCalls::SafeTransferFromWithData(element) => {
                    element.encode()
                }
                NonfungiblePositionManagerCalls::SelfPermit(element) => element.encode(),
                NonfungiblePositionManagerCalls::SelfPermitAllowed(element) => element.encode(),
                NonfungiblePositionManagerCalls::SelfPermitAllowedIfNecessary(element) => {
                    element.encode()
                }
                NonfungiblePositionManagerCalls::SelfPermitIfNecessary(element) => element.encode(),
                NonfungiblePositionManagerCalls::SetApprovalForAll(element) => element.encode(),
                NonfungiblePositionManagerCalls::SupportsInterface(element) => element.encode(),
                NonfungiblePositionManagerCalls::SweepToken(element) => element.encode(),
                NonfungiblePositionManagerCalls::Symbol(element) => element.encode(),
                NonfungiblePositionManagerCalls::TokenByIndex(element) => element.encode(),
                NonfungiblePositionManagerCalls::TokenOfOwnerByIndex(element) => element.encode(),
                NonfungiblePositionManagerCalls::TokenURI(element) => element.encode(),
                NonfungiblePositionManagerCalls::TotalSupply(element) => element.encode(),
                NonfungiblePositionManagerCalls::TransferFrom(element) => element.encode(),
                NonfungiblePositionManagerCalls::UniswapV3MintCallback(element) => element.encode(),
                NonfungiblePositionManagerCalls::UnwrapWETH9(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for NonfungiblePositionManagerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                NonfungiblePositionManagerCalls::DomainSeparator(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::PermitTypehash(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::Weth9(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::Approve(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::BalanceOf(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::BaseURI(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::Burn(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::Collect(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::CreateAndInitializePoolIfNecessary(element) => {
                    element.fmt(f)
                }
                NonfungiblePositionManagerCalls::DecreaseLiquidity(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::Factory(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::GetApproved(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::IncreaseLiquidity(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::IsApprovedForAll(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::Mint(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::Multicall(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::Name(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::OwnerOf(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::Permit(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::Positions(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::RefundETH(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::SafeTransferFrom(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::SafeTransferFromWithData(element) => {
                    element.fmt(f)
                }
                NonfungiblePositionManagerCalls::SelfPermit(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::SelfPermitAllowed(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::SelfPermitAllowedIfNecessary(element) => {
                    element.fmt(f)
                }
                NonfungiblePositionManagerCalls::SelfPermitIfNecessary(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::SetApprovalForAll(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::SupportsInterface(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::SweepToken(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::Symbol(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::TokenByIndex(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::TokenOfOwnerByIndex(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::TokenURI(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::TotalSupply(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::TransferFrom(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::UniswapV3MintCallback(element) => element.fmt(f),
                NonfungiblePositionManagerCalls::UnwrapWETH9(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for NonfungiblePositionManagerCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            NonfungiblePositionManagerCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<PermitTypehashCall> for NonfungiblePositionManagerCalls {
        fn from(var: PermitTypehashCall) -> Self {
            NonfungiblePositionManagerCalls::PermitTypehash(var)
        }
    }
    impl ::std::convert::From<Weth9Call> for NonfungiblePositionManagerCalls {
        fn from(var: Weth9Call) -> Self {
            NonfungiblePositionManagerCalls::Weth9(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for NonfungiblePositionManagerCalls {
        fn from(var: ApproveCall) -> Self {
            NonfungiblePositionManagerCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for NonfungiblePositionManagerCalls {
        fn from(var: BalanceOfCall) -> Self {
            NonfungiblePositionManagerCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BaseURICall> for NonfungiblePositionManagerCalls {
        fn from(var: BaseURICall) -> Self {
            NonfungiblePositionManagerCalls::BaseURI(var)
        }
    }
    impl ::std::convert::From<BurnCall> for NonfungiblePositionManagerCalls {
        fn from(var: BurnCall) -> Self {
            NonfungiblePositionManagerCalls::Burn(var)
        }
    }
    impl ::std::convert::From<CollectCall> for NonfungiblePositionManagerCalls {
        fn from(var: CollectCall) -> Self {
            NonfungiblePositionManagerCalls::Collect(var)
        }
    }
    impl ::std::convert::From<CreateAndInitializePoolIfNecessaryCall>
        for NonfungiblePositionManagerCalls
    {
        fn from(var: CreateAndInitializePoolIfNecessaryCall) -> Self {
            NonfungiblePositionManagerCalls::CreateAndInitializePoolIfNecessary(var)
        }
    }
    impl ::std::convert::From<DecreaseLiquidityCall> for NonfungiblePositionManagerCalls {
        fn from(var: DecreaseLiquidityCall) -> Self {
            NonfungiblePositionManagerCalls::DecreaseLiquidity(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for NonfungiblePositionManagerCalls {
        fn from(var: FactoryCall) -> Self {
            NonfungiblePositionManagerCalls::Factory(var)
        }
    }
    impl ::std::convert::From<GetApprovedCall> for NonfungiblePositionManagerCalls {
        fn from(var: GetApprovedCall) -> Self {
            NonfungiblePositionManagerCalls::GetApproved(var)
        }
    }
    impl ::std::convert::From<IncreaseLiquidityCall> for NonfungiblePositionManagerCalls {
        fn from(var: IncreaseLiquidityCall) -> Self {
            NonfungiblePositionManagerCalls::IncreaseLiquidity(var)
        }
    }
    impl ::std::convert::From<IsApprovedForAllCall> for NonfungiblePositionManagerCalls {
        fn from(var: IsApprovedForAllCall) -> Self {
            NonfungiblePositionManagerCalls::IsApprovedForAll(var)
        }
    }
    impl ::std::convert::From<MintCall> for NonfungiblePositionManagerCalls {
        fn from(var: MintCall) -> Self {
            NonfungiblePositionManagerCalls::Mint(var)
        }
    }
    impl ::std::convert::From<MulticallCall> for NonfungiblePositionManagerCalls {
        fn from(var: MulticallCall) -> Self {
            NonfungiblePositionManagerCalls::Multicall(var)
        }
    }
    impl ::std::convert::From<NameCall> for NonfungiblePositionManagerCalls {
        fn from(var: NameCall) -> Self {
            NonfungiblePositionManagerCalls::Name(var)
        }
    }
    impl ::std::convert::From<OwnerOfCall> for NonfungiblePositionManagerCalls {
        fn from(var: OwnerOfCall) -> Self {
            NonfungiblePositionManagerCalls::OwnerOf(var)
        }
    }
    impl ::std::convert::From<PermitCall> for NonfungiblePositionManagerCalls {
        fn from(var: PermitCall) -> Self {
            NonfungiblePositionManagerCalls::Permit(var)
        }
    }
    impl ::std::convert::From<PositionsCall> for NonfungiblePositionManagerCalls {
        fn from(var: PositionsCall) -> Self {
            NonfungiblePositionManagerCalls::Positions(var)
        }
    }
    impl ::std::convert::From<RefundETHCall> for NonfungiblePositionManagerCalls {
        fn from(var: RefundETHCall) -> Self {
            NonfungiblePositionManagerCalls::RefundETH(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromCall> for NonfungiblePositionManagerCalls {
        fn from(var: SafeTransferFromCall) -> Self {
            NonfungiblePositionManagerCalls::SafeTransferFrom(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromWithDataCall> for NonfungiblePositionManagerCalls {
        fn from(var: SafeTransferFromWithDataCall) -> Self {
            NonfungiblePositionManagerCalls::SafeTransferFromWithData(var)
        }
    }
    impl ::std::convert::From<SelfPermitCall> for NonfungiblePositionManagerCalls {
        fn from(var: SelfPermitCall) -> Self {
            NonfungiblePositionManagerCalls::SelfPermit(var)
        }
    }
    impl ::std::convert::From<SelfPermitAllowedCall> for NonfungiblePositionManagerCalls {
        fn from(var: SelfPermitAllowedCall) -> Self {
            NonfungiblePositionManagerCalls::SelfPermitAllowed(var)
        }
    }
    impl ::std::convert::From<SelfPermitAllowedIfNecessaryCall> for NonfungiblePositionManagerCalls {
        fn from(var: SelfPermitAllowedIfNecessaryCall) -> Self {
            NonfungiblePositionManagerCalls::SelfPermitAllowedIfNecessary(var)
        }
    }
    impl ::std::convert::From<SelfPermitIfNecessaryCall> for NonfungiblePositionManagerCalls {
        fn from(var: SelfPermitIfNecessaryCall) -> Self {
            NonfungiblePositionManagerCalls::SelfPermitIfNecessary(var)
        }
    }
    impl ::std::convert::From<SetApprovalForAllCall> for NonfungiblePositionManagerCalls {
        fn from(var: SetApprovalForAllCall) -> Self {
            NonfungiblePositionManagerCalls::SetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for NonfungiblePositionManagerCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            NonfungiblePositionManagerCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<SweepTokenCall> for NonfungiblePositionManagerCalls {
        fn from(var: SweepTokenCall) -> Self {
            NonfungiblePositionManagerCalls::SweepToken(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for NonfungiblePositionManagerCalls {
        fn from(var: SymbolCall) -> Self {
            NonfungiblePositionManagerCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TokenByIndexCall> for NonfungiblePositionManagerCalls {
        fn from(var: TokenByIndexCall) -> Self {
            NonfungiblePositionManagerCalls::TokenByIndex(var)
        }
    }
    impl ::std::convert::From<TokenOfOwnerByIndexCall> for NonfungiblePositionManagerCalls {
        fn from(var: TokenOfOwnerByIndexCall) -> Self {
            NonfungiblePositionManagerCalls::TokenOfOwnerByIndex(var)
        }
    }
    impl ::std::convert::From<TokenURICall> for NonfungiblePositionManagerCalls {
        fn from(var: TokenURICall) -> Self {
            NonfungiblePositionManagerCalls::TokenURI(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for NonfungiblePositionManagerCalls {
        fn from(var: TotalSupplyCall) -> Self {
            NonfungiblePositionManagerCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for NonfungiblePositionManagerCalls {
        fn from(var: TransferFromCall) -> Self {
            NonfungiblePositionManagerCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UniswapV3MintCallbackCall> for NonfungiblePositionManagerCalls {
        fn from(var: UniswapV3MintCallbackCall) -> Self {
            NonfungiblePositionManagerCalls::UniswapV3MintCallback(var)
        }
    }
    impl ::std::convert::From<UnwrapWETH9Call> for NonfungiblePositionManagerCalls {
        fn from(var: UnwrapWETH9Call) -> Self {
            NonfungiblePositionManagerCalls::UnwrapWETH9(var)
        }
    }
    #[doc = "Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
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
    #[doc = "Container type for all return fields from the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `[48, 173, 248, 31]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PermitTypehashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `WETH9` function with signature `WETH9()` and selector `[74, 164, 164, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Weth9Return(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `baseURI` function with signature `baseURI()` and selector `[108, 3, 96, 235]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BaseURIReturn(pub String);
    #[doc = "Container type for all return fields from the `collect` function with signature `collect((uint256,address,uint128,uint128))` and selector `[252, 111, 120, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CollectReturn {
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `createAndInitializePoolIfNecessary` function with signature `createAndInitializePoolIfNecessary(address,address,uint24,uint160)` and selector `[19, 234, 213, 98]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CreateAndInitializePoolIfNecessaryReturn {
        pub pool: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `decreaseLiquidity` function with signature `decreaseLiquidity((uint256,uint128,uint256,uint256,uint256))` and selector `[12, 73, 204, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DecreaseLiquidityReturn {
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `factory` function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FactoryReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `[8, 24, 18, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetApprovedReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `increaseLiquidity` function with signature `increaseLiquidity((uint256,uint256,uint256,uint256,uint256,uint256))` and selector `[33, 159, 93, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IncreaseLiquidityReturn {
        pub liquidity: u128,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsApprovedForAllReturn(pub bool);
    #[doc = "Container type for all return fields from the `mint` function with signature `mint((address,address,uint24,int24,int24,uint256,uint256,uint256,uint256,address,uint256))` and selector `[136, 49, 100, 86]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MintReturn {
        pub token_id: ethers::core::types::U256,
        pub liquidity: u128,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `multicall` function with signature `multicall(bytes[])` and selector `[172, 150, 80, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
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
    #[doc = "Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `[99, 82, 33, 30]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OwnerOfReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `positions` function with signature `positions(uint256)` and selector `[153, 251, 171, 136]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PositionsReturn {
        pub nonce: u128,
        pub operator: ethers::core::types::Address,
        pub token_0: ethers::core::types::Address,
        pub token_1: ethers::core::types::Address,
        pub fee: u32,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub liquidity: u128,
        pub fee_growth_inside_0_last_x128: ethers::core::types::U256,
        pub fee_growth_inside_1_last_x128: ethers::core::types::U256,
        pub tokens_owed_0: u128,
        pub tokens_owed_1: u128,
    }
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
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
    #[doc = "Container type for all return fields from the `tokenByIndex` function with signature `tokenByIndex(uint256)` and selector `[79, 108, 204, 231]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TokenByIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `tokenOfOwnerByIndex` function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `[47, 116, 92, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TokenOfOwnerByIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `[200, 123, 86, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TokenURIReturn(pub String);
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
    #[doc = "`CollectParams(uint256,address,uint128,uint128)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CollectParams {
        pub token_id: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub amount_0_max: u128,
        pub amount_1_max: u128,
    }
    #[doc = "`DecreaseLiquidityParams(uint256,uint128,uint256,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DecreaseLiquidityParams {
        pub token_id: ethers::core::types::U256,
        pub liquidity: u128,
        pub amount_0_min: ethers::core::types::U256,
        pub amount_1_min: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "`IncreaseLiquidityParams(uint256,uint256,uint256,uint256,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IncreaseLiquidityParams {
        pub token_id: ethers::core::types::U256,
        pub amount_0_desired: ethers::core::types::U256,
        pub amount_1_desired: ethers::core::types::U256,
        pub amount_0_min: ethers::core::types::U256,
        pub amount_1_min: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "`MintParams(address,address,uint24,int24,int24,uint256,uint256,uint256,uint256,address,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MintParams {
        pub token_0: ethers::core::types::Address,
        pub token_1: ethers::core::types::Address,
        pub fee: u32,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount_0_desired: ethers::core::types::U256,
        pub amount_1_desired: ethers::core::types::U256,
        pub amount_0_min: ethers::core::types::U256,
        pub amount_1_min: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
}
