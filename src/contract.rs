use crate::msg::*;
use crate::state::*;
use cosmwasm_std::{
    attr, entry_point, to_json_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128, WasmMsg,
};
use cw20::Cw20ExecuteMsg;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct InstantiateMsg {}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let admin_profile = AdminProfile {
        authority: info.sender.clone(),
        collaterial_count: 0,
    };

    ADMIN_PROFILE.save(deps.storage, &admin_profile)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "initialize"),
        attr("admin", info.sender.to_string()),
    ]))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::InitializeUser {} => try_initialize_user(deps, info),
        ExecuteMsg::AddAcceptedCollaterial {
            ticker,
            mint_address,
            pool_address,
            image,
        } => try_add_accepted_collaterial(deps, info, ticker, mint_address, pool_address, image),
        ExecuteMsg::DepositCollaterial {
            amount,
            token_address,
        } => try_deposit_collaterial(deps, _env, info, amount, token_address),
        ExecuteMsg::WithdrawCollaterial {
            amount,
            token_address,
        } => try_withdraw_collaterial(deps, info, amount, token_address),
        ExecuteMsg::CreateLoan {
            duration,
            interest_rate,
            amount,
            token_address,
        } => try_create_loan(deps, info, duration, interest_rate, amount, token_address),
        ExecuteMsg::AcceptLoan {
            loan_idx,
            token_address,
        } => try_accept_loan(deps, info, loan_idx, token_address),
        // ExecuteMsg::RemoveLoan { loan_idx } => try_remove_loan(deps, info, loan_idx),
    }
}

pub fn try_initialize_user(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let user_profile = UserProfile {
        authority: info.sender.clone(),
        loan_count: 0,
        last_loan: 0,
        can_borrow: true,
        can_deposit: true,
        coins_lent: vec![],
        coins_deposited: vec![],
    };

    USER_PROFILE.save(deps.storage, &info.sender, &user_profile)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "initialize_user"),
        attr("user", info.sender.to_string()),
    ]))
}

pub fn try_add_accepted_collaterial(
    deps: DepsMut,
    info: MessageInfo,
    ticker: String,
    mint_address: String,
    pool_address: String,
    image: String,
) -> StdResult<Response> {
    let admin_profile = ADMIN_PROFILE.load(deps.storage)?;

    // Only admin can add collateral
    if admin_profile.authority != info.sender {
        return Err(StdError::generic_err("Unauthorized"));
    }

    let accepted_collateral = AcceptedCollateral {
        ticker,
        mint_address,
        pool_address,
        image,
        authority: info.sender.clone(),
    };

    ACCEPTED_COLLATERAL.save(deps.storage, &info.sender, &accepted_collateral)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "add_collateral"),
        attr("collateral", accepted_collateral.ticker),
        attr("admin", info.sender.to_string()),
    ]))
}

pub fn try_deposit_collaterial(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    token_address: String,
) -> StdResult<Response> {
    let mut user_profile = USER_PROFILE.load(deps.storage, &info.sender)?;

    let collateral = DepositedCollateral {
        ticker: "TOKEN".to_string(),
        mint_address: token_address.clone(),
        pool_address: "pool_address".to_string(),
        amount,
        authority: info.sender.clone(),
    };

    user_profile.coins_deposited.push(collateral);

    USER_PROFILE.save(deps.storage, &info.sender, &user_profile)?;

    // Transfer tokens from user to contract
    let transfer_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token_address.clone(),
        msg: to_json_binary(&Cw20ExecuteMsg::TransferFrom {
            owner: info.sender.clone().to_string(),
            recipient: _env.contract.address.to_string(),
            amount,
        })?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(transfer_msg)
        .add_attributes(vec![
            attr("action", "deposit_collateral"),
            attr("amount", amount.to_string()),
            attr("token_address", token_address),
            attr("user", info.sender.to_string()),
        ]))
}

pub fn try_withdraw_collaterial(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
    token_address: String,
) -> StdResult<Response> {
    let mut user_profile = USER_PROFILE.load(deps.storage, &info.sender)?;

    // Ensure sufficient balance
    if let Some(collateral) = user_profile
        .coins_deposited
        .iter_mut()
        .find(|c| c.amount >= amount && c.mint_address == token_address)
    {
        collateral.amount = collateral.amount.checked_sub(amount)?;
    } else {
        return Err(StdError::generic_err(
            "Insufficient collateral balance for the given token",
        ));
    }

    USER_PROFILE.save(deps.storage, &info.sender, &user_profile)?;

    let transfer_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token_address.clone(), // Use the actual token contract address
        msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
            recipient: info.sender.clone().to_string(),
            amount,
        })?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(transfer_msg)
        .add_attributes(vec![
            attr("action", "withdraw_collateral"),
            attr("amount", amount.to_string()),
            attr("token_address", token_address),
            attr("user", info.sender.to_string()),
        ]))
}

pub fn try_create_loan(
    deps: DepsMut,
    info: MessageInfo,
    duration: u64,
    interest_rate: u64,
    amount: Uint128,
    token_address: String,
) -> StdResult<Response> {
    let mut user_profile = USER_PROFILE.load(deps.storage, &info.sender)?;

    let loan = Loan {
        interest_rate,
        lender: info.sender.clone(),
        amount,
        status: LoanStatus::Open,
        duration,
        authority: info.sender.clone(),
        token_address: token_address.clone(),
        idx: user_profile.last_loan,
    };

    LOAN.save(deps.storage, &info.sender, &loan)?;

    user_profile.loan_count += 1;
    user_profile.last_loan += 1;

    USER_PROFILE.save(deps.storage, &info.sender, &user_profile)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "create_loan"),
        attr("loan_id", loan.idx.to_string()),
        attr("token_address", token_address),
        attr("user", info.sender.to_string()),
    ]))
}

pub fn try_accept_loan(
    deps: DepsMut,
    info: MessageInfo,
    loan_idx: u64,
    token_address: String,
) -> StdResult<Response> {
    let mut loan = LOAN.load(deps.storage, &info.sender)?;

    // Ensure the loan matches the token
    if loan.token_address != token_address {
        return Err(StdError::generic_err("Token mismatch"));
    }

    loan.status = LoanStatus::Accepted;

    LOAN.save(deps.storage, &info.sender, &loan)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "accept_loan"),
        attr("loan_id", loan_idx.to_string()),
        attr("token_address", token_address),
        attr("user", info.sender.to_string()),
    ]))
}
