use super::super::utils::Error;

use ethers::types::{U256, I256};
use reqwest;
use serde::{Deserialize};

enum Beacon<'a>{
    ValidatorBalances(Vec<&'a str>)
}

impl<'a> Beacon<'a>{
    fn endpoint(&self) -> String {
        match self {
            Beacon::ValidatorBalances(id_vec) => format!("/states/head/validator_balances?id={}", id_vec.join("&id="))
        }
    }
}

#[derive(Deserialize)]
pub struct ResponseGetValidatorBalances {
    execution_optimistic: bool,
    data: Vec<ResponseGetValidatorBalancesData>,
}

#[derive(Deserialize)]
pub struct ResponseGetValidatorBalancesData {
    index: String,
    balance: String,
}

pub struct Client<'a> {
    reqwest_client: reqwest::Client,
    url: &'a str
}

impl<'a> Client<'a> {
    pub fn new (url: &'a str) -> Client<'a> {
        Client {
            reqwest_client:reqwest::Client::new(),
            url
        }
    }

    pub async fn fetch_validator_balances(&self, pubkey_vec: Vec<&str>) -> Result<Vec<U256>, Error>{
        let response: ResponseGetValidatorBalances = self.reqwest_client
            .get(format!("{}{}",self.url, Beacon::ValidatorBalances(pubkey_vec).endpoint()))
            .send().await.map_err(|e| Error::HTTPRequestError(e))?
            .error_for_status().map_err(|e| Error::HTTPResponseError(e))?
            .json().await.map_err(|e| Error::JsonError(e))?;
        
        let mut balances: Vec<U256> = vec![];
        for ResponseGetValidatorBalancesData {balance, ..} in response.data.iter(){
            balances.push(U256::from_dec_str(balance).map_err(|_| Error::CheckedMath)?
                .checked_mul(U256::exp10(9)).map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?);
        }

        Ok(balances)
    }

    pub async fn fetch_validator_rewards(
        &self,
        pubkey_vec: Vec<&str>
    ) -> Result<Vec<I256>, Error>{
        let balances = self.fetch_validator_balances(pubkey_vec).await?;
        let mut rewards: Vec<I256> = vec![];
        let full_deposit = U256::from(32).checked_mul(U256::exp10(18))
            .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?;

        for balance in balances.iter(){
            if balance.ge(&full_deposit) == true{
                rewards.push(
                    I256::from(
                        balance.checked_sub(full_deposit)
                            .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?
                            .as_u128()
                        )
                );
                continue;
            }
            rewards.push(
                I256::from(
                    full_deposit.checked_sub(*balance)
                        .map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?
                        .as_u128()
                ).checked_neg().map(|x| Ok(x)).unwrap_or(Err(Error::CheckedMath))?
            );
        }
        Ok(rewards)
    }
}