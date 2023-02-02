use core::abi_parser::{build_smart_contract, AbiFile};
use core::portfolio::Portfolio;

use clap::Parser;

#[derive(Parser)]
#[command(
    author = "D-banana",
    version = "0.0.1",
    about = "Fetch info from the ethereum blockchain and push it to home assistant.",
    long_about = None
)]
struct Args {
    #[arg(
        help = "Generate rust file based on smart contract abi",
        short = 'b',
        long
    )]
    build_smart_contract: bool,

    #[arg(
        help = "Home assistant bearer token",
        short = 't',
        long
    )]
    bearer_token_ha: String,

    #[arg(
        help = "Home assistant URL(http://192.168.0.1:8123/api)",
        short = 'u',
        long
    )]
    url_ha: String,

    #[arg(
        help = "ETH execution node URL(http://192.168.0.1:8545)",
        short = 'e',
        long
    )]
    url_eth: String,

    #[arg(
        help = "ETH consensus node URL(http://192.168.0.1:8123/api)",
        short = 'c',
        long
    )]
    url_beacon: String,

    #[arg(
        help = "Owner address of lusd/blusd uniswap v3 position",
        short = 'p',
        long
    )]
    lusd_blusd_uni_v3_owner_address: String,

    #[arg(
        help = "Address of lusd,blusd,eth,rpl hodler",
        short = 'o',
        long
    )]
    hodl_owner_address_list: Vec<String>,

    #[arg(
        help = "Address of Rocket pool node",
        short = 'n',
        long
    )]
    node_address: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.build_smart_contract == true {
        println!("Start parsing smart contract.");
        build();
        return;
    }

    println!("Start fetching and push save data.");
    run(&args).await;
}


async fn run(args: &Args) {
    let portfolio = Portfolio::new(
        &args.bearer_token_ha,
        &args.url_ha,
        &args.url_eth,
        &args.url_beacon,
        &args.lusd_blusd_uni_v3_owner_address,
        args.hodl_owner_address_list.iter().map(|e| e.as_str()).collect(),
        &args.node_address
    ).await;
    if let Err(e) = portfolio {
        eprintln!("{}", e);
        return;
    }
    let portfolio = portfolio.unwrap();

    if let Err(e) = portfolio.fetch_and_save().await{
        eprintln!("{}", e);
    }
}

fn build() {
    let abi_file_list = vec![
        AbiFile {
            path_abi: String::from("./contract/NonfungiblePositionManager.json"),
            path_rs: String::from("./contract/nonfungible_position_manager.rs"),
            name: String::from("NonfungiblePositionManager")
        },
        AbiFile {
            path_abi: String::from("./contract/UniswapV3Pool.json"),
            path_rs: String::from("./contract/uniswap_v3_pool.rs"),
            name: String::from("UniswapV3Pool")
        },
        AbiFile {
            path_abi: String::from("./contract/LUSDToken.json"),
            path_rs: String::from("./contract/lusd_token.rs"),
            name: String::from("LUSDToken")
        },
        AbiFile {
            path_abi: String::from("./contract/BLUSDToken.json"),
            path_rs: String::from("./contract/blusd_token.rs"),
            name: String::from("BLUSDToken")
        },
        AbiFile {
            path_abi: String::from("./contract/RocketNodeManager.json"),
            path_rs: String::from("./contract/rocket_node_manager.rs"),
            name: String::from("RocketNodeManager")
        },
        AbiFile {
            path_abi: String::from("./contract/RocketMinipoolManager.json"),
            path_rs: String::from("./contract/rocket_minipool_manager.rs"),
            name: String::from("RocketMinipoolManager")
        },
    ];
    if let Err(_) = build_smart_contract(abi_file_list){
        eprintln!("Failed to build smart contract rust file.");
    }
}
