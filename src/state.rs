// state.rs

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

// Struct for the admin's profile
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AdminProfile {
    pub authority: Addr,
    pub collaterial_count: u64,
}

// Struct for the user profile
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserProfile {
    pub authority: Addr,
    pub loan_count: u64,
    pub last_loan: u64,
    pub can_borrow: bool,
    pub can_deposit: bool,
    pub coins_lent: Vec<LentCoin>,
    pub coins_deposited: Vec<DepositedCollateral>,
}

// Struct for the loan
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Loan {
    pub interest_rate: u64,
    pub lender: Addr,
    pub amount: Uint128,
    pub status: LoanStatus,
    pub duration: u64,
    pub authority: Addr,
    pub token_address: String,
    pub idx: u64,
}

// Enum for the loan status
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum LoanStatus {
    Open,
    Accepted,
    Closed,
}

// Struct for accepted collateral
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AcceptedCollateral {
    pub ticker: String,
    pub mint_address: String,
    pub pool_address: String,
    pub image: String,
    pub authority: Addr,
}

// Struct for collateral deposited by users
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DepositedCollateral {
    pub ticker: String,
    pub mint_address: String,
    pub pool_address: String,
    pub amount: Uint128,
    pub authority: Addr,
}

// Struct for coins lent by users
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LentCoin {
    pub ticker: String,
    pub amount: Uint128,
    pub token_address: String,
}

// Contract storage items
pub const ADMIN_PROFILE: Item<AdminProfile> = Item::new("admin_profile");
pub const USER_PROFILE: Map<&Addr, UserProfile> = Map::new("user_profile");
pub const LOAN: Map<&Addr, Loan> = Map::new("loan");
pub const ACCEPTED_COLLATERAL: Map<&Addr, AcceptedCollateral> = Map::new("accepted_collateral");
