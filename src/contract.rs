#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, WasmMsg,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};

use crate::error::ContractError;
use crate::msg::{Cw20HookMsg, DepositMsg, ExecuteMsg, InstantiateMsg, QueryMsg, WithdrawMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Withdraw(msg) => withdraw_cw20(deps, info, msg),
        ExecuteMsg::Receive(msg) => deposit_cw20(deps, info, msg),
    }
}

fn withdraw_cw20(
    deps: DepsMut,
    info: MessageInfo,
    msg: WithdrawMsg,
) -> Result<Response, ContractError> {
    // Get the params from WithdrawMsg
    let cw20_address = msg.cw20_address;
    let to_sent = msg.amount;

    // Validations
    let cw20_address = deps.api.addr_validate(cw20_address.as_str())?;
    // check if the "to_sent" amount is greater than "max_cap" of "cw20_address" token.
    if to_sent.is_zero() {
        return Err(ContractError::Std(StdError::GenericErr {
            msg: "Invalid zero amount".to_string(),
        }));
    }

    // Handle the real "withdraw"
    let recipient = deps.api.addr_validate(info.sender.as_str())?;
    let msgs: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: cw20_address.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: recipient.to_string(),
            amount: to_sent,
        })?,
        funds: vec![],
    })];

    Ok(Response::default().add_messages(msgs))
}

fn deposit_cw20(
    deps: DepsMut,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let token_contract = info.sender;
    let sent_amount = cw20_msg.amount;

    // Deseralize the message for the params
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Deposit(msg)) => {
            let DepositMsg {
                cw20_address,
                amount,
            } = msg;
            // Validations
            if sent_amount != amount {
                return Err(ContractError::Std(StdError::GenericErr {
                    msg: "Invalid amount".to_string(),
                }));
            }
            if token_contract != deps.api.addr_validate(cw20_address.as_str())? {
                return Err(ContractError::Std(StdError::GenericErr {
                    msg: "Invalid amount".to_string(),
                }));
            }

            // Handle the real "deposit".

            Ok(Response::default())
        }
        Err(_) => Err(ContractError::Unauthorized {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
}
