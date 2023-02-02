pub use ethers::{
    prelude::*,
    abi::ethereum_types::U512
};

use crate::utils::checked_mul_div;

pub fn u256_x2_pow96() -> U256 {
    U256::from(2).pow(U256::from(96))
}

pub fn u256_x2_pow128() -> U256 {
    U256::from(2).pow(U256::from(128))
}

pub fn checked_mul_x2_pow96(x: U256) -> Option<U256> {
    x.checked_mul(u256_x2_pow96())
}

pub fn from_tick_to_price_sqrt_x96 (tick: i32) -> Option<U256> {
    let min_tick = 1.0001f64;
    let tick = tick as f64;
    let tick = min_tick.powf(tick).sqrt().to_string();

    let mut nb_decimal = tick.find('.').unwrap_or(0);
    if nb_decimal > 0 {
        nb_decimal = tick.len() - nb_decimal - 1 
    }

    U512::from_dec_str(&tick.replace(".", ""))
        .map(|x| Some(x)).unwrap_or(None)?
        .checked_mul(u256_x2_pow96().into())?
        .checked_div(U512::exp10(nb_decimal))?
        .try_into().map(|x| Some(x)).unwrap_or(None)
}

pub fn compute_liquidity_amount_0 (
    price_sqrt_a_x96: U256,
    price_sqrt_b_x96: U256,
    liquidity: u128
) -> Option<U256> {
    let (price_sqrt_a_x96, price_sqrt_b_x96) =  if price_sqrt_a_x96
        .gt(&price_sqrt_b_x96) {
            (price_sqrt_b_x96, price_sqrt_a_x96)
        } else {
            (price_sqrt_a_x96, price_sqrt_b_x96)
    };

    let numerator_1 = checked_mul_x2_pow96(U256::from(liquidity))?;
    let numerator_2 = price_sqrt_b_x96.checked_sub(price_sqrt_a_x96)?;

    checked_mul_div(numerator_1, numerator_2, price_sqrt_b_x96)?
        .checked_div(price_sqrt_a_x96)
}

pub fn compute_liquidity_amount_1 (
    price_sqrt_a_x96: U256,
    price_sqrt_b_x96: U256,
    liquidity: u128
) -> Option<U256> {
    let (price_sqrt_a_x96, price_sqrt_b_x96) =  if price_sqrt_a_x96
        .gt(&price_sqrt_b_x96) {
            (price_sqrt_b_x96, price_sqrt_a_x96)
        } else {
            (price_sqrt_a_x96, price_sqrt_b_x96)
    };

    let numerator_1 = U256::from(liquidity);
    let numerator_2 = price_sqrt_b_x96.checked_sub(price_sqrt_a_x96)?;

    checked_mul_div(numerator_1, numerator_2, u256_x2_pow96())
}

pub fn compute_liquidity_amount (
    liquidity: u128, 
    tick_lower: i32, 
    tick_upper: i32, 
    price_current_sqrt_x96: U256
) -> Option<(U256, U256)> {
    let price_lower_sqrt_x96 = from_tick_to_price_sqrt_x96(tick_lower)?;
    let price_upper_sqrt_x96 = from_tick_to_price_sqrt_x96(tick_upper)?;

    match price_current_sqrt_x96 {
        // current [lower, upper]
        price_current if price_current.lt(&price_lower_sqrt_x96) == true => {
            Some((compute_liquidity_amount_0(
                    price_lower_sqrt_x96,
                    price_upper_sqrt_x96,
                    liquidity)?,
                U256::zero()))
        },
        // [lower, upper] current
        price_current if price_current.gt(&price_upper_sqrt_x96) == true => {
            Some((U256::zero(),
                compute_liquidity_amount_1(
                    price_lower_sqrt_x96,
                    price_upper_sqrt_x96,
                    liquidity)?))
        },
        // [lower, current, upper]
        _ => {
            Some((compute_liquidity_amount_0(
                    price_current_sqrt_x96,
                    price_upper_sqrt_x96,
                    liquidity)?,
                compute_liquidity_amount_1(
                    price_lower_sqrt_x96,
                    price_current_sqrt_x96,
                    liquidity)?))
        }
    }
}

pub fn compute_fee_amount (
    liquidity: u128,
    tick_current: i32,
    tick_lower: i32,
    tick_upper: i32,
    (token_owed_0, token_owed_1): (u128, u128),
    (fee_growth_inside_0_last_x128, fee_growth_inside_1_last_x128):(U256, U256),
    (fee_growth_outside_lower_0_x128, fee_growth_outside_lower_1_x128):(U256, U256),
    (fee_growth_outside_upper_0_x128, fee_growth_outside_upper_1_x128): (U256, U256),
    (fee_growth_global_0_x128, fee_growth_global_1_x128): (U256, U256),
) -> Option<(U256, U256)> {
    let (fee_growth_below_0_x128, fee_growth_below_1_x128) = if tick_current.ge(&tick_lower) == true {
        (fee_growth_outside_lower_0_x128, fee_growth_outside_lower_1_x128)
    } else {
        (fee_growth_global_0_x128.checked_sub(fee_growth_outside_lower_0_x128)?,
        fee_growth_global_1_x128.checked_sub(fee_growth_outside_lower_1_x128)?)
    };

    let (fee_growth_above_0_x128, fee_growth_above_1_x128) = if tick_current.lt(&tick_upper) == true {
        (fee_growth_outside_upper_0_x128, fee_growth_outside_upper_1_x128)
    } else {
        (fee_growth_global_0_x128.checked_sub(fee_growth_outside_upper_0_x128)?,
        fee_growth_global_1_x128.checked_sub(fee_growth_outside_upper_1_x128)?)
    };

    let amount_0 = fee_growth_global_0_x128.checked_sub(fee_growth_below_0_x128)?
        .checked_sub(fee_growth_above_0_x128)?;
    let amount_1 = fee_growth_global_1_x128.checked_sub(fee_growth_below_1_x128)?
        .checked_sub(fee_growth_above_1_x128)?;
    Some((
        U256::from(token_owed_0).checked_add(
            checked_mul_div(
                amount_0.checked_sub(fee_growth_inside_0_last_x128)?,
                U256::from(liquidity),
                u256_x2_pow128())?
        )?,
        U256::from(token_owed_1).checked_add(
            checked_mul_div(
                amount_1.checked_sub(fee_growth_inside_1_last_x128)?,
                U256::from(liquidity),
                u256_x2_pow128())?
        )?
    ))
}

pub fn from_u256_to_f32(x: U256, decimal: usize, decimal_precision: usize) -> Option<f32> {
    if decimal < decimal_precision { return None }

    Some(x.checked_div(U256::exp10(decimal - decimal_precision))?
    .to_string().parse::<f32>().unwrap() / (10f32.powi(decimal_precision as i32)))
}