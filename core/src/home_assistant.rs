use std::collections::HashMap;

use super::utils::Error;

use reqwest;
use serde::{Deserialize, Serialize};

pub enum State {
    TotalEur(f32),
    IlliquidEur(f32),
    LiquidEur(f32),
    Eth(f32),
    EthUsd(f32),
    Rpl(f32),
    RplUsd(f32),
    Blusd(f32),
    BlusdUsd(f32),
    Lusd(f32),
    LusdUsd(f32),
    EurUsd(f32),
}
impl State {
    fn value (&self) -> f32 {
        match self {
            State::TotalEur(v) => *v,
            State::IlliquidEur(v) =>  *v,
            State::LiquidEur(v) => *v,
            State::Eth(v) => *v,
            State::EthUsd(v) => *v,
            State::Rpl(v) => *v,
            State::RplUsd(v) => *v,
            State::Blusd(v) => *v,
            State::BlusdUsd(v) => *v,
            State::Lusd(v) => *v,
            State::LusdUsd(v) => *v,
            State::EurUsd(v) => *v,
        }
    }
    fn entity_id (&self) -> String {
        match self {
            State::TotalEur(_) => String::from("input_number.total_eur"),
            State::IlliquidEur(_) => String::from("input_number.illiquid_eur"),
            State::LiquidEur(_) => String::from("input_number.liquid_eur"),
            State::Eth(_) => String::from("input_number.eth"),
            State::EthUsd(_) => String::from("input_number.eth_usd"),
            State::Rpl(_) => String::from("input_number.rpl"),
            State::RplUsd(_) => String::from("input_number.rpl_usd"),
            State::Blusd(_) => String::from("input_number.blusd"),
            State::BlusdUsd(_) => String::from("input_number.blusd_usd"),
            State::Lusd(_) => String::from("input_number.lusd"),
            State::LusdUsd(_) => String::from("input_number.lusd_usd"),
            State::EurUsd(_) => String::from("input_number.eur_usd"),
        }
    }
    fn endpoint (&self) -> String {
        format!("/states/{}", self.entity_id())
    }
}

pub struct Client<'a> {
    reqwest_client: reqwest::Client,
    bearer_token: &'a str,
    url: &'a str
}
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ResponseApi {
    message: String,
}

#[derive(Serialize)]
struct RequestPostState<T> {
    state: T,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ResponsePostState {
    attributes: HashMap<String, String>,
    entity_id: String,
    last_changed: String,
    last_updated: String,
    state: String,
}

impl<'a> Client<'a> {
    pub fn new (bearer_token: &'a str, url: &'a str) -> Client<'a> {
        Client {
            reqwest_client:reqwest::Client::new(),
            bearer_token,
            url
        }
    }

    pub async fn test_client(&self) -> Result<ResponseApi, Error>{
        Ok(self
        .reqwest_client
        .get(format!("{}/",self.url))
        .bearer_auth(self.bearer_token.to_string())
        .send().await.map_err(|e| Error::HTTPRequestError(e))?
        .error_for_status().map_err(|e| Error::HTTPResponseError(e))?
        .json().await.map_err(|e| Error::JsonError(e))?)
    }

    pub async fn post_new_state(&self, state: State) -> Result<ResponsePostState, Error> {
        Ok(self
            .reqwest_client
            .post(format!("{}{}", self.url, state.endpoint()))
            .json::<RequestPostState<f32>>(&RequestPostState { state: state.value() })
            .bearer_auth(self.bearer_token.to_string())
            .send().await.map_err(|e| Error::HTTPRequestError(e))?
            .error_for_status().map_err(|e| Error::HTTPResponseError(e))?
            .json().await.map_err(|e| Error::JsonError(e))?)
    }
    
}