use std::path::PathBuf;
use std::sync::Arc;
use lazy_static::lazy_static;
use novax::Address;
use novax::caching::CachingNone;
use novax::executor::StandardMockExecutor;
use novax_mocking::world::infos::ScenarioWorldInfos;
use num_traits::ToPrimitive;
use tokio::sync::Mutex;
use novax_price_getter::errors::{AppError, XExchangeError};
use novax_price_getter::XExchangeService;

lazy_static! {
    static ref WORLD_INFOS: ScenarioWorldInfos = get_world_infos();
}

const XEXCHANGE_ROUTER_ADDRESS: &str = "erd1qqqqqqqqqqqqqpgqg2esr6d6tfd250x4n3tkhfkw8cc4p2x50n4swatdz6";

fn get_world_infos() -> ScenarioWorldInfos {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/xexchange_clone.json");
    ScenarioWorldInfos::from_file(path).unwrap()
}

fn init_service() -> XExchangeService<CachingNone, StandardMockExecutor> {
    let world = WORLD_INFOS.clone().into_world(|address, code_expr, world| {
        if address == Address::from(XEXCHANGE_ROUTER_ADDRESS).to_bytes() {
            world.register_contract(code_expr, mx_exchange_sc_router::ContractBuilder)
        } else {
            world.register_contract(code_expr, mx_exchange_sc_pair::ContractBuilder)
        }
    });
    let executor = StandardMockExecutor::new(
        Arc::new(Mutex::new(world)),
        None
    );

    XExchangeService::new(
        "USDC-8d4068".to_string(),
        "WEGLD-d7c6bb".to_string(),
        XEXCHANGE_ROUTER_ADDRESS.to_string(),
        CachingNone,
        executor
    )
}

#[tokio::test]
async fn test_get_price_with_usdc() {
    let service = init_service();

    let result = service.get_fungible_price(
        "USDC-8d4068",
        18
    ).await.unwrap().to_f64().unwrap();

    let expected = 1f64;

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_get_price_with_wegld() {
    let service = init_service();

    let result = service.get_fungible_price(
        "WEGLD-d7c6bb",
        18
    ).await.unwrap().to_f64().unwrap();

    let expected = 14.856761003204669;

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_get_price_with_mex() {
    let service = init_service();

    let result = service.get_fungible_price(
        "MEX-dc289c",
        18
    ).await.unwrap().to_f64().unwrap();

    let expected = 0.00007160202800139497;

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_get_price_with_unknown_token() {
    let service = init_service();

    let result = service.get_fungible_price(
        "ABC-648400",
        18
    ).await.unwrap_err();

    let expected = AppError::XExchange(XExchangeError::PairNotFound {
        first_token_identifier: "WEGLD-d7c6bb".to_string(),
        second_token_identifier: "ABC-648400".to_string()
    });

    assert_eq!(result, expected);
}