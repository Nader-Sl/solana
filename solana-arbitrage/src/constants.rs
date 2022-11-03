 
 use std::collections::HashMap;

    pub const RPC_HTTPS_URL: &str = "https://solana-api.projectserum.com";
    pub const RPC_WSS_URL: &str = "wss://api.mainnet-beta.solana.com";

    //TokenSwap accounts
    pub const ORCA_USD_SOL_MARKET: &str = "EGZ7tiLeH62TPV1gL8WwbXGzEPa9zmcpVnnkPKKnrE2U";
    
    pub enum TOKEN {
        SOL, USDC
    }

    // #[macro_use]
    // extern crate lazy_static;
    // lazy_static! {
    //     static ref MARKETS: HashMap::from([
    //         (ORCA_USD_SOL_MARKET, HashMap::from([
    //             (TOKEN::SOL, 
    //     ]))
    //     ]);
    // }
    
 