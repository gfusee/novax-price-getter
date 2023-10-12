use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use novax::caching::{CachingNone, CachingStrategy};
use novax_caching::local::caching_local::CachingLocal;
use novax_caching::locked::caching::CachingLocked;
use novax_token::properties::fetch::FetchTokenProperties;
use num_traits::ToPrimitive;
use novax_price_getter::XExchangeService;

const GATEWAY_URL: &str = "https://gateway.multiversx.com";

#[tokio::main]
async fn main() {
    let caching = CachingLocked::new(CachingLocal::empty());
    let service = XExchangeService::new(
        "USDC-c76f1f".to_string(),
        "WEGLD-bd4d79".to_string(),
        "erd1qqqqqqqqqqqqqpgqq66xk9gfr4esuhem3jru86wg5hvp33a62jps2fy57p".to_string(),
        CachingNone,
        GATEWAY_URL
    );

    let htm_token_identifier = "HTM-f51d55";
    let htm_token_decimals = get_token_decimals(htm_token_identifier, caching.clone()).await;

    let htm_price = service.get_fungible_price(
        htm_token_identifier,
        htm_token_decimals
    ).await.unwrap();

    println!("Current price of the HTM token: {}", htm_price.to_f64().unwrap())
}

async fn get_token_decimals(token_identifier: &str, caching: CachingLocked<CachingLocal>) -> u8 {
    let cache_key = {
        let mut hasher = DefaultHasher::new();
        token_identifier.hash(&mut hasher);
        hasher.finish()
    };

    caching.get_or_set_cache(
        cache_key,
        async {
            token_identifier.fetch_token_properties(GATEWAY_URL).await
        }
    )
        .await
        .unwrap()
        .decimals
}
