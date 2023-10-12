use std::path::PathBuf;
use std::sync::Arc;
use lazy_static::lazy_static;
use novax::Address;
use novax::caching::CachingNone;
use novax::executor::StandardMockExecutor;
use novax_mocking::ScenarioWorld;
use novax_mocking::world::infos::ScenarioWorldInfos;
use num_traits::ToPrimitive;
use tokio::sync::Mutex;
use novax_price_getter::XExchangeService;

lazy_static! {
    static ref WORLD: Arc<Mutex<ScenarioWorld>> = Arc::new(Mutex::new(get_world()));
}

const XEXCHANGE_ROUTER_ADDRESS: &str = "erd1qqqqqqqqqqqqqpgqq66xk9gfr4esuhem3jru86wg5hvp33a62jps2fy57p";

fn get_world() -> ScenarioWorld {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/xexchange_clone.json");
    let infos = ScenarioWorldInfos::from_file(path).unwrap();
    infos.into_world(|address, code_expr, world| {
        println!("{}", Address::from_bytes(address).to_bech32_string().unwrap());
        if address == Address::from(XEXCHANGE_ROUTER_ADDRESS).to_bytes() {
            world.register_contract(code_expr, mx_exchange_sc_router::ContractBuilder)
        } else {
            world.register_contract(code_expr, mx_exchange_sc_pair::ContractBuilder)
        }
    })
}

fn init_service() -> XExchangeService<CachingNone, StandardMockExecutor> {
    let executor = StandardMockExecutor::new(
        WORLD.clone(),
        None
    );

    XExchangeService::new(
        "USDC-c76f1f".to_string(),
        "WEGLD-bd4d79".to_string(),
        XEXCHANGE_ROUTER_ADDRESS.to_string(),
        CachingNone,
        executor
    )
}

#[tokio::test]
async fn test_get_price_with_mex() {
    let service = init_service();

    let result = service.get_fungible_price(
        "MEX-455c57",
        18
    ).await.unwrap().to_f64().unwrap();

    let expected = 0.000003663;

    assert_eq!(result, expected);
}