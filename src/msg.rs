// msg.rs

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

// Message for instantiating the contract
#[cw_serde]
pub struct InstantiateMsg {}

// Messages for executing functions in the contract
#[cw_serde]
pub enum ExecuteMsg {
    InitializeUser {},

    AddAcceptedCollaterial {
        ticker: String,
        mint_address: String,
        pool_address: String,
        image: String,
    },

    DepositCollaterial {
        amount: Uint128,
        token_address: String,
    },

    WithdrawCollaterial {
        amount: Uint128,
        token_address: String,
    },

    CreateLoan {
        duration: u64,
        interest_rate: u64,
        amount: Uint128,
        token_address: String,
    },

    AcceptLoan {
        loan_idx: u64,
        token_address: String,
    },
    // RemoveLoan {
    //     loan_idx: u64,
    // },
}

#[cw_serde]
pub struct GetUserProfileResponse {
    pub address: String,
}

#[cw_serde]
pub struct GetLoanResponse {
    pub loan_idx: u64,
}

#[cw_serde]
pub struct GetAcceptedCollateralsResponse {}

// Messages for querying state from the contract
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetUserProfileResponse)]
    GetUserProfile,
    #[returns(GetLoanResponse)]
    GetLoan,
    #[returns(GetAcceptedCollateralsResponse)]
    GetAcceptedCollaterals {},
}
