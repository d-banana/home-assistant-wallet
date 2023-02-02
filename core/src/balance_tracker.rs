mod blusd_token;
mod lusd_token;

use std::sync::Arc;

pub use ethers::{
    prelude::*,
    providers,
    providers::Provider,
    contract::builders::ContractCall
};

use super::utils::Error;
use lusd_token::LUSDToken;
use blusd_token::BLUSDToken;

pub enum Token {
    LUSD,
    BLUSD,
    ETH,
}

impl Token {
    pub fn address(&self) -> H160 {
        match self {
            Token::LUSD => "0x5f98805a4e8be255a32880fdec7f6728c6568ba0".parse::<Address>().unwrap(),
            Token::BLUSD => "0xb9d7dddca9a4ac480991865efef82e01273f79c3".parse::<Address>().unwrap(),
            _ => Address::default(),
        }
    }
}

pub struct BalanceTracker {
    owner_address_vec: Vec<H160>,
    client: Arc<Provider<providers::Http>>,
    lusd_token: LUSDToken<Provider<providers::Http>>,
    blusd_token: BLUSDToken<Provider<providers::Http>>,
}

impl BalanceTracker {
    pub fn new(
        client: &Arc<Provider<providers::Http>>,
        owner_address_vec: Vec<&str>,
    ) -> Result<BalanceTracker, Error>{
        let owner_address_vec = owner_address_vec.iter().map(|address| address.parse::<Address>()).into_iter();
        if let Some(Err(_)) = owner_address_vec.clone().find(|r| r.is_err())  {
            return Err(Error::ParseAddress);
        }

        Ok(
            BalanceTracker {
                owner_address_vec: owner_address_vec
                    .filter(|r| r.is_ok())
                    .map(|r| r.unwrap())
                    .collect(),
                client: Arc::clone(client),
                lusd_token: LUSDToken::new(Token::LUSD.address(), Arc::clone(client)),
                blusd_token: BLUSDToken::new(Token::BLUSD.address(), Arc::clone(client)),
            }
        )
    }

    pub async fn fetch_balance_of(&self, token: Token) -> Result<U256, Error>{
        let mut amount = U256::zero();
        for &owner in self.owner_address_vec.iter() {
            match token {
                Token::LUSD => {
                    if let Ok(x) = self.lusd_token.balance_of(owner).call().await{
                        if let Some(x) = amount.checked_add(x){
                            amount = x;
                            continue;
                        }
                    }
                    return Err(Error::ProviderRequestError);
                },
                Token::BLUSD => {
                    if let Ok(x) = self.blusd_token.balance_of(owner).call().await{
                        if let Some(x) = amount.checked_add(x){
                            amount = x;
                            continue;
                        }
                    }
                    return Err(Error::ProviderRequestError);
                },
                Token::ETH => {
                    if let Ok(x) = self.client.get_balance(owner, None).await{
                        if let Some(x) = amount.checked_add(x){
                            amount = x;
                            continue;
                        }
                    }
                    return Err(Error::ProviderRequestError);
                },
            }
        }

        Ok(amount)
    }
}