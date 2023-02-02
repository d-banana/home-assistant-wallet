use std::sync::Arc;
use ethers::{
    providers::Provider,
    types::U256,
};

use super::home_assistant::{Client as ClientHA, State as StateHA};
use super::uniswap_v3_tracker::{UniswapV3PositionTracker, from_u256_to_f32};
use super::balance_tracker::{BalanceTracker, Token};
use super::rocket_pool_tracker::RocketPoolTracker;
use super::coingecko::{Client as ClientGecko, Token as TokenPrice};
use super::utils::Error;

pub struct Portfolio<'a> {
    client_ha: ClientHA<'a>,
    client_gecko: ClientGecko,
    lusd_blusd_uni_v3: UniswapV3PositionTracker,
    balance_tracker: BalanceTracker,
    rocket_pool_tracker: RocketPoolTracker<'a>
}

const LUSD_BLUSD_UNI_V3_POOL_ADDRESS: &str = "0x3e48df436b427c77Aad1Bae3536404AF27233682";

impl<'a> Portfolio<'a> {
    pub async fn new (
        bearer_token_ha: &'a str,
        url_ha: &'a str,
        url_eth: &'a str,
        url_beacon: &'a str,
        lusd_blusd_uni_v3_owner_address: &'a str,
        owner_address_vec: Vec<&'a str>,
        node_address: &'a str,
    ) -> Result<Portfolio<'a>, Error> {
        let client_ha = ClientHA::new(bearer_token_ha, url_ha);
        let client_gecko = ClientGecko::new();

        let client_eth = Arc::new(
            Provider::try_from(url_eth).map_err(|_| Error::CreateEthProvider)?
        );

        let lusd_blusd_uni_v3 = UniswapV3PositionTracker::new(
            &client_eth,
            LUSD_BLUSD_UNI_V3_POOL_ADDRESS,
            lusd_blusd_uni_v3_owner_address
        )?;

        let balance_tracker = BalanceTracker::new(&client_eth, owner_address_vec)?;

        let rocket_pool_tracker = RocketPoolTracker::new(&client_eth, node_address, url_beacon)?;

        Ok(Portfolio {
            client_ha, 
            client_gecko, 
            lusd_blusd_uni_v3, 
            balance_tracker, 
            rocket_pool_tracker
        })
    }

    pub async fn fetch_and_save(&self) -> Result<(), Error>{
        let (lusd_uni_v3, blusd_uni_v3) = self.lusd_blusd_univ3().await?;
        let lusd_hodl = self.balance_tracker.fetch_balance_of(Token::LUSD).await?;
        let blusd_hodl = self.balance_tracker.fetch_balance_of(Token::BLUSD).await?;
        let eth_hodl = self.balance_tracker.fetch_balance_of(Token::ETH).await?;
        let rpl_stake = self.rocket_pool_tracker.fetch_rpl_stake_rpl_max().await?;
        let eth_stake = self.rocket_pool_tracker.fetch_eth_stake().await?;
        let price_map = self.client_gecko.fetch_all_price().await?;
        let lusd_usd = price_map.get(&TokenPrice::LUSD.id())
            .map(|x| Ok(x)).unwrap_or(Err(Error::CoingecoPriceMissing))?;
        let blusd_usd = price_map.get(&TokenPrice::BLUSD.id())
            .map(|x| Ok(x)).unwrap_or(Err(Error::CoingecoPriceMissing))?;
        let eth_usd = price_map.get(&TokenPrice::ETH.id())
            .map(|x| Ok(x)).unwrap_or(Err(Error::CoingecoPriceMissing))?;
        let rpl_usd = price_map.get(&TokenPrice::RPL.id())
            .map(|x| Ok(x)).unwrap_or(Err(Error::CoingecoPriceMissing))?;
        let eur_usd = price_map.get(&TokenPrice::EUR.id())
            .map(|x| Ok(x)).unwrap_or(Err(Error::CoingecoPriceMissing))?;

        let lusd = lusd_uni_v3.checked_add(lusd_hodl)
            .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?;
        let lusd_f32 = from_u256_to_f32(lusd, 18, 2)
            .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))? * lusd_usd;
        self.client_ha.post_new_state(StateHA::Lusd(lusd_f32)).await?;
        self.client_ha.post_new_state(StateHA::LusdUsd(*lusd_usd)).await?;

        let blusd = blusd_uni_v3.checked_add(blusd_hodl)
            .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?;
        let blusd_f32 = from_u256_to_f32(blusd, 18, 2)
            .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))? * blusd_usd;
        self.client_ha.post_new_state(StateHA::Blusd(blusd_f32)).await?;
        self.client_ha.post_new_state(StateHA::BlusdUsd(*blusd_usd)).await?;

        let eth_illiquid_f32 = from_u256_to_f32(eth_stake, 18, 2)
            .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))? * eth_usd;
        let eth_liquid_f32 = from_u256_to_f32(eth_hodl, 18, 2)
            .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))? * eth_usd;
        self.client_ha.post_new_state(StateHA::Eth(eth_illiquid_f32 + eth_liquid_f32)).await?;
        self.client_ha.post_new_state(StateHA::EthUsd(*eth_usd)).await?;

        let rpl_f32 = from_u256_to_f32(rpl_stake, 18, 2)
            .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))? * rpl_usd;
        self.client_ha.post_new_state(StateHA::Rpl(rpl_f32)).await?;
        self.client_ha.post_new_state(StateHA::RplUsd(*rpl_usd)).await?;

        self.client_ha.post_new_state(StateHA::EurUsd(*eur_usd)).await?;

        let liquid_eur = (blusd_f32 + lusd_f32 + eth_liquid_f32) * eur_usd;
        self.client_ha.post_new_state(StateHA::LiquidEur(liquid_eur)).await?;

        let illiquid_eur = (eth_illiquid_f32 + rpl_f32) * eur_usd;
        self.client_ha.post_new_state(StateHA::IlliquidEur(illiquid_eur)).await?;

        self.client_ha.post_new_state(StateHA::TotalEur(liquid_eur + illiquid_eur)).await?;

        println!("Data saved !");
        Ok(())
    }

    async fn lusd_blusd_univ3(&self) -> Result<(U256, U256),Error>{
        let token_ids = self.lusd_blusd_uni_v3.fetch_all_owned_token_id().await;
        let (mut lusd_total, mut blusd_total) = (U256::zero(), U256::zero());

        for &token_id in token_ids.iter()  {
            let (lusd, blusd) = self.lusd_blusd_uni_v3.compute_liquidity_amount_position(token_id).await?;
            lusd_total = lusd_total.checked_add(lusd)
                .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?;
            blusd_total = blusd_total.checked_add(blusd)
                .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?;


            let (lusd, blusd) = self.lusd_blusd_uni_v3.compute_fee_amount_position(token_id).await?;
            lusd_total = lusd_total.checked_add(lusd)
                .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?;
            blusd_total = blusd_total.checked_add(blusd)
                .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?;
        }

        Ok((lusd_total, blusd_total))
    }
}