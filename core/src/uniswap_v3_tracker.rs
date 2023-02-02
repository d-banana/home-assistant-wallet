mod uniswap_v3_pool;
mod nonfungible_position_manager;
mod logic;

pub use ethers::{
    prelude::*,
    providers,
    providers::Provider,
};
use std::sync::Arc;

use nonfungible_position_manager::NonfungiblePositionManager;
use uniswap_v3_pool::UniswapV3Pool;
use logic::*;
use super::utils::Error;

pub use logic::from_u256_to_f32;

pub const NFT_POSITION_MANAGER_ADDRESS: &str = "0xc36442b4a4522e871399cd717abdd847ab11fe88";

pub struct Slot0{
    sqrt_price_x96: U256,
    tick: i32,
    _observation_index: u16,
    _observation_cardinality: u16,
    _observation_cardinality_next: u16,
    _fee_protocol: u8,
    _unlocked: bool,
}

pub struct Tick{
    _liquidity_gross: u128,
    _liquidity_net: i128,
    fee_growth_outside_0_x128: U256,
    fee_growth_outside_1_x128: U256,
    _tick_cumulative_outside: i64,
    _seconds_per_liquidity_outside_x128: U256,
    _seconds_outside: u32,
    _initialized: bool,
}

pub struct Position {
    _nounce: u128,
    _operator: Address,
    _token_0: Address,
    _token_1: Address,
    _fee: u32,
    tick_lower: i32,
    tick_upper: i32,
    liquidity: u128,
    fee_growth_inside_0_last_x128: U256,
    fee_growth_inside_1_last_x128: U256,
    token_owed_0: u128,
    token_owed_1: u128,
}

pub struct UniswapV3PositionTracker {
    owner_address: H160,
    uni_v3_pool: UniswapV3Pool<Provider<providers::Http>>,
    nft_position_manager: NonfungiblePositionManager<Provider<providers::Http>>
}

impl UniswapV3PositionTracker {
    pub fn new(
        client: &Arc<Provider<providers::Http>>,
        pool_address: &str,
        owner_address: &str,
    ) -> Result<UniswapV3PositionTracker, Error>{
        let pool_address = pool_address
            .parse::<Address>().map_err(|_| Error::ParseAddress)?;
        let nft_address = NFT_POSITION_MANAGER_ADDRESS
            .parse::<Address>().map_err(|_| Error::ParseAddress)?;
        let owner_address = owner_address
            .parse::<Address>().map_err(|_| Error::ParseAddress)?;

        Ok(UniswapV3PositionTracker {
            owner_address,
            uni_v3_pool: UniswapV3Pool::new(
                pool_address, Arc::clone(client)),
            nft_position_manager: NonfungiblePositionManager::new(
                nft_address, Arc::clone(client))
        })
    }

    pub async fn fetch_all_owned_token_id(&self) -> Vec<U256> {
        let mut i = U256::from(0);
        let mut token_id_list = vec![];

        while let Ok(token_id) = self.nft_position_manager
            .token_of_owner_by_index(self.owner_address, i)
            .call()
            .await {
            token_id_list.push(token_id);
            i = i.checked_add(U256::from(1)).unwrap();
        }
        token_id_list
    }

    pub async fn fetch_slot_0(&self) -> Result<Slot0, Error> {
        let (sqrt_price_x96,
            tick,
            _observation_index,
            _observation_cardinality,
            _observation_cardinality_next,
            _fee_protocol,
            _unlocked
        ) = self.uni_v3_pool.slot_0().call().await.map_err(|_| Error::ProviderRequestError)?;

        Ok(
            Slot0 {
                sqrt_price_x96,
                tick,
                _observation_index,
                _observation_cardinality,
                _observation_cardinality_next,
                _fee_protocol,
                _unlocked
            }
        )

    }

    pub async fn fetch_tick(&self, tick: i32) -> Result<Tick, Error> {
        let (_liquidity_gross,
            _liquidity_net,
            fee_growth_outside_0_x128,
            fee_growth_outside_1_x128,
            _tick_cumulative_outside,
            _seconds_per_liquidity_outside_x128,
            _seconds_outside,
            _initialized,
        ) = self.uni_v3_pool.ticks(tick).call().await.map_err(|_| Error::ProviderRequestError)?;

        Ok(
            Tick {
                _liquidity_gross,
                _liquidity_net,
                fee_growth_outside_0_x128,
                fee_growth_outside_1_x128,
                _tick_cumulative_outside,
                _seconds_per_liquidity_outside_x128,
                _seconds_outside,
                _initialized,
            }
        )

    }

    pub async fn fetch_position(&self, token_id: U256) -> Result<Position, Error> {
        let (_nounce,
            _operator,
            _token_0,
            _token_1,
            _fee,
            tick_lower,
            tick_upper,
            liquidity,
            fee_growth_inside_0_last_x128,
            fee_growth_inside_1_last_x128,
            token_owed_0,
            token_owed_1,
        ) = self.nft_position_manager.positions(token_id).call().await.map_err(|_| Error::ProviderRequestError)?;

        Ok(
            Position {
                _nounce,
                _operator,
                _token_0,
                _token_1,
                _fee,
                tick_lower,
                tick_upper,
                liquidity,
                fee_growth_inside_0_last_x128,
                fee_growth_inside_1_last_x128,
                token_owed_0,
                token_owed_1,
            }
        )

    }

    pub async fn fetch_fee_growth_global(&self) -> Result<(U256, U256), Error> {
        let fee_growth_global_0_x128 = self.uni_v3_pool
            .fee_growth_global_0x128().call().await.map_err(|_| Error::ProviderRequestError)?;
        let fee_growth_global_1_x128 = self.uni_v3_pool
            .fee_growth_global_1x128().call().await.map_err(|_| Error::ProviderRequestError)?;

        Ok((fee_growth_global_0_x128, fee_growth_global_1_x128))

    }
    
    pub async fn compute_liquidity_amount_position(&self, token_id: U256) -> Result<(U256, U256), Error> {
        let Position {
            tick_lower,
            tick_upper,
            liquidity,
            ..} = self.fetch_position(token_id).await?;
        let Slot0 { sqrt_price_x96, ..} = self.fetch_slot_0().await?;

        compute_liquidity_amount(
            liquidity,
            tick_lower,
            tick_upper,
            sqrt_price_x96
        )
        .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))
    }
    
    pub async fn compute_fee_amount_position(&self, token_id: U256) -> Result<(U256, U256), Error> {
        let Position {
            tick_lower,
            tick_upper,
            liquidity,
            fee_growth_inside_0_last_x128,
            fee_growth_inside_1_last_x128,
            token_owed_0,
            token_owed_1,
            ..} = self.fetch_position(token_id).await?;
        let Slot0 { tick, ..} = self.fetch_slot_0().await?;
        let tick_info_lower = self.fetch_tick(tick_lower).await?;
        let tick_info_upper = self.fetch_tick(tick_upper).await?;
        let (fee_growth_global_0_x128, fee_growth_global_1_x128) = self.fetch_fee_growth_global().await?;

        compute_fee_amount(
            liquidity,
            tick,
            tick_lower,
            tick_upper,
            (token_owed_0, token_owed_1),
            (fee_growth_inside_0_last_x128, fee_growth_inside_1_last_x128),
            (tick_info_lower.fee_growth_outside_0_x128, tick_info_lower.fee_growth_outside_1_x128),
            (tick_info_upper.fee_growth_outside_0_x128, tick_info_upper.fee_growth_outside_1_x128),
            (fee_growth_global_0_x128, fee_growth_global_1_x128)
        )
        .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))
    }
    
}