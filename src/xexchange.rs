use async_recursion::async_recursion;
use novax::Address;
use novax::caching::CachingStrategy;
use novax::executor::QueryExecutor;
use novax::pair::pair::PairContract;
use novax::router::router::RouterContract;
use tokio::join;
use num_bigint::{BigUint, ToBigInt};
use num_rational::BigRational;
use num_traits::cast::FromPrimitive;
use num_traits::Inv;
use crate::errors::{AppError, XExchangeError};
use crate::utils::constants::ONE_YEAR;

pub struct XExchangeService<Caching, Executor>
where
    Caching: CachingStrategy,
    Executor: QueryExecutor
{
    usdc_token_identifier: String,
    wegld_token_identifier: String,
    router_address: String,
    caching: Caching,
    executor: Executor
}

impl<Caching, Executor> XExchangeService<Caching, Executor>
where
    Caching: CachingStrategy,
    Executor: QueryExecutor
{

    pub fn new(
        usdc_token_identifier: String,
        wegld_token_identifier: String,
        router_address: String,
        caching: Caching,
        executor: Executor
    ) -> Self {
        XExchangeService {
            usdc_token_identifier,
            wegld_token_identifier,
            router_address,
            caching,
            executor,
        }
    }

    #[async_recursion]
    pub async fn get_fungible_price(
        &self,
        token_identifier: &str,
        token_decimals: u8
    ) -> Result<BigRational, AppError> {
        if token_identifier == self.usdc_token_identifier {
            return Ok(BigRational::from_i8(1).unwrap());
        }

        let usdc_token_decimals = 6u8;

        let (actual_token_identifier, actual_token_decimals) = if token_identifier == self.wegld_token_identifier {
            (&*self.usdc_token_identifier, usdc_token_decimals)
        } else {
            (token_identifier, token_decimals)
        };

        let Ok(pair) = self.get_pair_for_token(&self.wegld_token_identifier, &actual_token_identifier).await else {
            return Err(XExchangeError::PairNotFound { first_token_identifier: self.wegld_token_identifier.clone(), second_token_identifier: actual_token_identifier.to_string() }.into())
        };
        let pair_str = pair.to_bech32_string().unwrap();

        let results = join!(
            self.get_reserve_for_token_identifier(&pair_str, &self.wegld_token_identifier),
            self.get_reserve_for_token_identifier(&pair_str, actual_token_identifier)
        );

        let wegld_reserve = results.0?;
        let token_reserve = results.1?;

        let wegld_decimals = 18u8;
        let price_numerator = wegld_reserve * BigUint::from(10u8).pow(actual_token_decimals as u32);
        let price_denominator = token_reserve * BigUint::from(10u8).pow(wegld_decimals as u32);
        let mut price = BigRational::new(price_numerator.to_bigint().unwrap(), price_denominator.to_bigint().unwrap());

        if actual_token_identifier == self.usdc_token_identifier {
            price = price.inv()
        } else {
            let usd_price_of_wegld = self.get_fungible_price(&self.wegld_token_identifier, wegld_decimals).await?;
            price = usd_price_of_wegld * price
        }

        Ok(price)
    }

    async fn get_pair_for_token(
        &self,
        first_token_id: &str,
        second_token_id: &str
    ) -> Result<Address, AppError> {
        let router = RouterContract::new(
            &self.router_address
        );

        let pair_address = router
            .query(self.executor.clone())
            .with_caching_strategy(
                &self.caching
                    .with_duration(ONE_YEAR)
            )
            .get_pair(
                &first_token_id.to_string(),
                &second_token_id.to_string()
            )
            .await?;

        Ok(pair_address)
    }

    async fn get_reserve_for_token_identifier(
        &self,
        pair_address: &str,
        token_id: &str
    ) -> Result<BigUint, AppError> {
        let result = PairContract::new(
            pair_address
        )
            .query(self.executor.clone())
            .with_caching_strategy(
                &self.caching.until_next_block()
            )
            .get_reserve(&token_id.to_string())
            .await?;

        Ok(result.into())
    }
}