pub mod spl_token_swap {

    use serde_derive::{Deserialize, Serialize};
    use serde_repr::{Deserialize_repr, Serialize_repr};
    use solana_sdk::pubkey::Pubkey;
    use std::{assert, cmp};

    #[derive(Debug, Serialize_repr, Deserialize_repr)]
    #[repr(u8)]
    pub enum SwapVersion {
        SwapV1 = 1,
    }

    #[derive(Debug, Serialize_repr, Deserialize_repr)]
    #[repr(u8)]
    pub enum CurveType {
        ConstantProduct = 0,
        Stable = 2,
    }

    #[repr(C)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SwapCurve {
        pub curve_type: CurveType,
        pub curve_params: [u8; 32],
    }

    #[repr(C)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FeeData {
        pub trade_fee_numerator: u64,
        pub trade_fee_denominator: u64,
        pub owner_trade_fee_numerator: u64,
        pub owner_trade_fee_denominator: u64,
        pub owner_withdraw_fee_numerator: u64,
        pub owner_withdraw_fee_denominator: u64,
        pub host_fee_numerator: u64,
        pub host_fee_denominator: u64,
        pub swap_curve: SwapCurve,
    }

    impl FeeData {
        fn calc_fee_(inputTokenAmt: u64, numerator: u64, denominator: u64) -> u64 {
            assert!(
                inputTokenAmt >= 0 && numerator > 0 && denominator > 0,
                "Invalid param(s) value(s) : inputTokenAmt{}, numerator{}, denominator{}",
                inputTokenAmt,
                numerator,
                denominator
            );

            // return a min fee of 1
            cmp::max(
                inputTokenAmt
                    .checked_mul(numerator)
                    .unwrap()
                    .checked_div(denominator),
                1,
            )
        }

        pub fn calc_fee(inputTokenAmt: u64) -> u64 {
            
        }
    }

    #[repr(C)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TokenSwapAccount {
        pub version: SwapVersion,
        pub is_initialized: bool,
        pub bumpSeed: u8,
        pub token_program_id: Pubkey,
        pub token_account_a: Pubkey,
        pub token_account_b: Pubkey,
        pub token_pool: Pubkey,
        pub mint_a: Pubkey,
        pub mint_b: Pubkey,
        pub fee_account: Pubkey,
        pub fee_data: FeeData,
    }

    pub trait TokenSwapAMM {
        fn get_fees(outputAmountWithoutFees: u64);
    }

    pub struct TokenSwapConstantProduct {}

    pub struct TokenSwapStable {}
}
