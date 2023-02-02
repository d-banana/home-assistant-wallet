use std::collections::HashMap;
use reqwest;
use serde::{Deserialize, Serialize};
use super::utils::Error;

const URL: &str = "https://api.coingecko.com/api/v3/simple/price";


pub enum Token {
    LUSD,
    BLUSD,
    ETH,
    RPL,
    EUR    
}

impl Token {
    pub fn id (&self) -> String {
        match self {
            Token::LUSD => String::from("liquity-usd"),
            Token::BLUSD => String::from("boosted-lusd"),
            Token::ETH => String::from("ethereum"),
            Token::RPL => String::from("rocket-pool"),
            Token::EUR => String::from("ageur"),
        }
    }
}

pub struct Client {
    reqwest_client: reqwest::Client,
    tokens: Vec<Token> 
}

impl Client {
    pub fn new () -> Client {
        Client {
            reqwest_client:reqwest::Client::new(),
            tokens: vec![Token::LUSD, Token::BLUSD, Token::ETH, Token::RPL, Token:: EUR]
        }
    }

    pub async fn fetch_all_price(&self) -> Result<HashMap<String, f32>, Error> {
        let response = self
            .reqwest_client
            .get(format!("{}?ids={}&vs_currencies=usd",
                URL,
                self.tokens.iter().map(|t| t.id()).collect::<Vec<String>>().join("%2C")
            ))
            .send().await.map_err(|e| Error::HTTPRequestError(e))?
            .error_for_status().map_err(|e| Error::HTTPResponseError(e))?
            .json::<HashMap<String, HashMap<String, f32>>>().await.map_err(|e| Error::JsonError(e))?;

        let mut price_map:HashMap<String, f32> = HashMap::new();

        for token in self.tokens.iter(){
            println!();
            if let Some(usd_map) = response.get(&token.id()){
                if let Some(usd) = usd_map.get("usd"){
                    if *usd == 0. {return Err(Error::CoingecoPriceMissing) }
                    price_map.insert(token.id().to_string(), *usd);

                    continue;
                }
            }
            return Err(Error::CoingecoPriceMissing)
        }

        Ok(price_map)
    }
}