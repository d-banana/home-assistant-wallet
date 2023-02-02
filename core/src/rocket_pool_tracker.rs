mod rocket_node_manager;
mod rocket_minipool_manager;
mod eth_beacon;

pub use ethers::{
    prelude::*,
    providers,
    providers::Provider
};
use std::sync::Arc;

use rocket_node_manager::RocketNodeManager;
use rocket_minipool_manager::RocketMinipoolManager;
use eth_beacon::Client as ClientBeacon;
use crate::utils::checked_mul_div;

use super::utils::Error;

pub const ROCKET_NODE_MANAGER_ADDRESS: &str = "0x372236c940f572020c0c0eb1ac7212460e4e5a33";
pub const ROCKET_MINIPOOL_MANAGER_ADDRESS: &str = "0x6293B8abC1F36aFB22406Be5f96D893072A8cF3a";
pub const MINIPOOL_ETH: u32 = 16;

pub struct RocketPoolTracker<'a> {
    node_address: H160,
    rocket_node_manager: RocketNodeManager<Provider<providers::Http>>,
    rocket_minipool_manager: RocketMinipoolManager<Provider<providers::Http>>,
    client_beacon: ClientBeacon<'a>
}

impl<'a> RocketPoolTracker<'a> {
    pub fn new(
        client: &Arc<Provider<providers::Http>>,
        node_address: &str,
        url_beacon: &'a str,
    ) -> Result<RocketPoolTracker<'a>, Error>{
        let node_address = node_address
            .parse::<Address>().map_err(|_| Error::ParseAddress)?;

        let rocket_node_address = ROCKET_NODE_MANAGER_ADDRESS
            .parse::<Address>().map_err(|_| Error::ParseAddress)?;

        let rocket_minipool_address = ROCKET_MINIPOOL_MANAGER_ADDRESS
            .parse::<Address>().map_err(|_| Error::ParseAddress)?;

        Ok(RocketPoolTracker {
            node_address,
            rocket_node_manager: RocketNodeManager::new(rocket_node_address, Arc::clone(client)),
            rocket_minipool_manager: RocketMinipoolManager::new(rocket_minipool_address, Arc::clone(client)),
            client_beacon: ClientBeacon::new(url_beacon)
        })
    }

    pub async fn fetch_rpl_stake_rpl_max (&self) -> Result<U256, Error> {
        self.rocket_node_manager
            .get_node_details(self.node_address)
            .call().await
            .map(|detail| Ok(detail.rpl_stake))
            .map_err(|_| Error::ProviderRequestError)?
    }

    async fn fetch_all_minipool_pubkey(&self) -> Result<Vec<Bytes>, Error> {
        let minipool_count: U256 = self.rocket_node_manager
            .get_node_details(self.node_address)
            .call().await
            .map(|detail| detail.minipool_count)
            .map_err(|_| Error::ProviderRequestError)?;

        let mut pubkey_vec: Vec<Bytes> = vec![];

        for i in 0..minipool_count.as_u32() {
            let address = self.rocket_minipool_manager
                .get_node_minipool_at(self.node_address, U256::from(i))
                .call().await
                .map_err(|_| Error::ProviderRequestError)?;

            let pubkey = self.rocket_minipool_manager
                .get_minipool_pubkey(address)
                .call().await
                .map_err(|_| Error::ProviderRequestError)?;
            
            pubkey_vec.push(pubkey);
        }

        Ok(pubkey_vec)
    }

    async fn fetch_average_node_fee (&self) -> Result<U256, Error> {
        self.rocket_node_manager
            .get_average_node_fee(self.node_address)
            .call().await
            .map(|fee| Ok(fee))
            .map_err(|_| Error::ProviderRequestError)?
    }

    pub async fn fetch_eth_stake (&self) -> Result<U256, Error>{
        let pubkey_vec = self.fetch_all_minipool_pubkey().await?;
        let average_fee = self.fetch_average_node_fee().await?;
        let pubkey_vec:Vec<String> = pubkey_vec.iter().map(|p| format!("{p:#x}")).collect();
        let rewards = self.client_beacon
            .fetch_validator_rewards(
                pubkey_vec.iter().map(|p| p.as_str()).collect()
            ).await?;

        // minipool_count * 16 ETH 
        let mut eth = U256::exp10(18).checked_mul(U256::from(MINIPOOL_ETH))
            .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?
            .checked_mul(U256::from(pubkey_vec.len()))
            .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?;
        
        // Half reward + x% of the other half reward(reward * 0.5 * (1+~fee%)) 
        let pow18 = U256::exp10(18);
        let average_fee = average_fee.checked_add(pow18)
            .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?;
        let reward_share = U256::from(5).checked_mul(U256::exp10(17))
            .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?;
        let reward_share = checked_mul_div(reward_share, average_fee, pow18)
            .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?;

        for reward in rewards.iter(){
            let (sign, reward_abs) = reward.into_sign_and_abs();
            if sign == Sign::Negative {
                eth = eth.checked_sub(
                    checked_mul_div(reward_abs, reward_share, pow18)
                        .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?
                ).map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?;
                continue;
            }

            eth = eth.checked_add(
                checked_mul_div(reward_abs, reward_share, pow18)
                    .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?
            ).map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?;
        }

        Ok(eth)
    }
}