use futures::StreamExt;

use tokio::{
    runtime::{Builder, Runtime},
    task,
};

use solana_account_decoder::UiAccountEncoding;

use solana_client::{
    nonblocking::{self, pubsub_client::PubsubClient, rpc_client::RpcClient},
    rpc_config::{
        RpcAccountInfoConfig, RpcBlockSubscribeConfig, RpcBlockSubscribeFilter,
        RpcProgramAccountsConfig, RpcTransactionLogsConfig, RpcTransactionLogsFilter,
    },
    rpc_response::SlotInfo,
};
use solana_sdk::{
    account::Account,
    account::{ReadableAccount, WritableAccount},
    clock::Slot,
    commitment_config::{CommitmentConfig, CommitmentLevel},
    native_token::sol_to_lamports,
    pubkey::Pubkey,
    rpc_port,
    signature::{Keypair, Signer},
    system_program, system_transaction,
};

use std::{
    collections::HashSet,
    net::{IpAddr, SocketAddr},
    str,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, RwLock,
    },
    thread::{self, sleep, JoinHandle},
    time::{Duration, Instant},
};

use crate::RPC_CLIENT;

use {
    super::common::token_swap::spl_token_swap,
    super::Dex,
    crate::constants,
    anyhow::Result,
    bincode::Config,
    byteorder::{LittleEndian, ReadBytesExt},
    serde_derive::{Deserialize, Serialize},
    serde_repr::{Deserialize_repr, Serialize_repr},
    std::io::Cursor,
    tracing::log,
};

pub struct Orca {}

impl Orca {
    pub fn new() -> Orca {
        Orca {}
    }
}

impl Dex for Orca {
    fn collect_data(&self) -> JoinHandle<()> {
        thread::spawn(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async move {
                    let account = RPC_CLIENT
                        .get_account(&Pubkey::from_str(constants::ORCA_USD_SOL_MARKET).unwrap())
                        .await
                        .unwrap();

                    let token_swap_account = account.deserialize_data::<spl_token_swap::TokenSwapAccount>().unwrap();

                    log::info!("Parsed Orca SOL/USDC Market");
                    log::info!("{:?}", token_swap_account);

                    let token_a_acc = RPC_CLIENT.get_account(&token_swap_account.mint_a).await;

                    println!("{:?}", token_a_acc);
                    let pubsub_client = PubsubClient::new(&format!("{}", constants::RPC_WSS_URL))
                        .await
                        .unwrap();

                    let acc_config = Some(RpcAccountInfoConfig {
                        commitment: Some(CommitmentConfig::confirmed()),
                        encoding: Some(UiAccountEncoding::Base64Zstd),
                        data_slice: None,
                        min_context_slot: None,
                    });

                    let (mut token_acc_a_stream, _) = pubsub_client
                        .account_subscribe(&token_swap_account.token_account_a, acc_config.clone())
                        .await
                        .unwrap();

                    let (mut token_acc_b_stream, _) = pubsub_client
                        .account_subscribe(&token_swap_account.token_account_b, acc_config)
                        .await
                        .unwrap();

                    log::info!(
                        "Reading token streams ({},{})",
                        token_swap_account.token_account_a,
                        token_swap_account.token_account_b
                    );

                    loop {
                        match token_acc_a_stream.next().await {
                            Some(notification) => {
                                if let Some(acc) = notification.value.decode::<Account>() {
                                    log::info!(
                                        "[{}] Latest lamports amount : {:?}",
                                        token_swap_account.token_account_a,
                                        acc.lamports
                                    );
                                }
                            }
                            None => {}
                        }

                        match token_acc_b_stream.next().await {
                            Some(notification) => {
                                if let Some(acc) = notification.value.decode::<Account>() {
                                    log::info!(
                                        "[{}]  Latest lamports amount : {:?}",
                                        token_swap_account.token_account_b,
                                        acc.lamports
                                    );
                                }
                            }
                            None => {}
                        }
                    }
                })
        })
    }
}
