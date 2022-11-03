 
pub mod constants;
pub mod dexes;
pub mod market_models;

use dexes::Dex;

use {
    anyhow::Result,
    dexes::orca,
    futures::prelude::*,
    serde_json::{json, Value},
    solana_account_decoder::UiAccountEncoding,
    solana_client::{
        nonblocking::{self, pubsub_client::PubsubClient, rpc_client::RpcClient},
        rpc_config::{
            RpcAccountInfoConfig, RpcBlockSubscribeConfig, RpcBlockSubscribeFilter,
            RpcProgramAccountsConfig, RpcTransactionLogsConfig, RpcTransactionLogsFilter,
        },
        rpc_response::SlotInfo,
    },
    solana_sdk::{
        account::Account,
        clock::Slot,
        commitment_config::{CommitmentConfig, CommitmentLevel},
        native_token::sol_to_lamports,
        pubkey::Pubkey,
        rpc_port,
        signature::{Keypair, Signer},
        system_program, system_transaction,
    },
    std::{
        collections::HashSet,
        net::{IpAddr, SocketAddr},
        str,
        str::FromStr,
        sync::{
            atomic::{AtomicBool, AtomicU64, Ordering},
            Arc, RwLock,
        },
        thread::sleep,
        time::{Duration, Instant},
    },
    tracing_subscriber,
};

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref RPC_CLIENT: RpcClient = RpcClient::new_with_commitment(
        constants::RPC_HTTPS_URL.to_owned(),
        CommitmentConfig::confirmed()
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    let dexes: Vec<Box<dyn Dex>> = vec![Box::new(daa::Orca::new())];

    //Collect market data per market per dex concurrently on a dedicated thread
    //per Dex.
    for dex in dexes.iter() {
        dex.collect_data().join();
    }

    Ok(())
}
