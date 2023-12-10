use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, StdError, StdResult, Response, Binary, Deps, to_json_binary};
use thiserror::Error;
pub mod msg;
mod state;
mod contract;


#[derive(Error, Debug, PartialEq)]
pub enum ContractError{
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized - only {owner} can call it")]
    Unauthorized {owner: String},

    #[error("Need minimum amount")]
    MinimumAmount {},

    #[error("Wrong team chosen")]
    WrongTeam {},

    #[error("Not a winner")]
    NotWinner {},

    #[error("Result is not yet declared")]
    ResultNotDeclared {}
}

#[entry_point]
pub fn instantiate(_deps: DepsMut, _env:Env, _info:MessageInfo, _msg:msg::InstantiateMsg) -> StdResult<Response>{
    contract::instantiate(_deps, _info, _msg.minimum_amount);
    Ok(Response::new())
}

#[entry_point]
pub fn query(_deps:Deps, _env:Env, _msg:msg::QueryMsg) -> StdResult<Binary>{
    use msg::QueryMsg::*;
    match _msg{
        Owner {} => to_json_binary(&contract::query::owner(_deps)?),
        BetDetails {bet_id} => to_json_binary(&contract::query::bet_details(_deps, bet_id)?),
        BetAmount {sender, bet_id} => to_json_binary(&contract::query::bet_amount(_deps, bet_id, sender)?)
    }
}

#[entry_point]
pub fn execute(_deps:DepsMut, _env:Env, _info:MessageInfo, _msg:msg::ExecMsg) -> Result<Response, ContractError>{
    use msg::ExecMsg::*;
    match _msg{
        CreateBet{bet_title, team1_title, team2_title} => contract::execute::create_bet(_deps, _info, bet_title, team1_title, team2_title).map_err(ContractError::Std),
        Bet{bet_id, bet_team} => contract::execute::bet(_deps, _info, bet_id, bet_team),
        UpdateWinner{bet_id, winner} => contract::execute::update_winner(_deps, _info, bet_id, winner),
        GetReward{bet_id} => contract::execute::get_reward(_deps, _info, _env, bet_id),
        GetBrokerage{} => contract::execute::get_brokerage(_deps, _info, _env)
    }
}


#[cfg(test)]
pub mod multitest;