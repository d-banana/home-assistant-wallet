use ethers::types::U256;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error  {
    #[error("Failed to create Ethereum provider.")]
    CreateEthProvider,
    #[error("Failed to make an HTTP request.")]
    HTTPRequestError(reqwest::Error),
    #[error("Server return an error.")]
    HTTPResponseError(reqwest::Error),
    #[error("Failed to parse JSON.")]
    JsonError(reqwest::Error),
    #[error("Failed to call a smart contract.")]
    ProviderRequestError,
    #[error("Failed to parse ethereum address.")]
    ParseAddress,
    #[error("Failed to do check math.")]
    CheckedMath,
    #[error("Some price was not returned by coingecko api.")]
    CoingecoPriceMissing,
}

pub fn checked_mul_div(
    a: U256,
    b: U256,
    denominator: U256
) -> Option<U256> {
    a.full_mul(b)
        .checked_div(denominator.into())?
        .try_into().map(|x| Some(x)).unwrap_or(None)
}
