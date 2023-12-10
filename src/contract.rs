use cosmwasm_std::{DepsMut, MessageInfo, Uint128, StdResult, Response, Coin};

use crate::state::{MINIMUM_AMOUNT, OWNER, BET_COUNTER, BROKERAGE};

pub fn instantiate(deps: DepsMut, info:MessageInfo, minimum_amount: Coin) ->  StdResult<Response>{
    MINIMUM_AMOUNT.save(deps.storage, &minimum_amount)?;
    BET_COUNTER.save(deps.storage, &Uint128::new(0))?;
    OWNER.save(deps.storage, &info.sender)?;
    BROKERAGE.save(deps.storage, &Uint128::new(0))?;
    Ok(Response::new())
}

pub mod query{
    use cosmwasm_std::{Deps, StdResult, Uint128, Addr};

    use crate::{msg::{OwnerResp, BetDetailResp, BetAmountResp}, state::{OWNER, BET_DETAILS, BET_AMOUNT}};


    pub fn owner(deps: Deps) ->  StdResult<OwnerResp>{
        let res = OWNER.load(deps.storage)?;
        Ok(OwnerResp { owner: res })
    }

    pub fn bet_details(deps: Deps, bet_id:Uint128) ->  StdResult<BetDetailResp>{
        let res = BET_DETAILS.load(deps.storage, bet_id.u128())?;
        Ok(BetDetailResp { detail: res })
    }

    pub fn bet_amount(deps: Deps, bet_id: Uint128, sender: Addr) ->  StdResult<BetAmountResp>{
        let res = BET_AMOUNT.load(deps.storage, (&sender, bet_id.u128()))?;
        Ok(BetAmountResp { team: res.0, amount: res.1 })
    }

}

pub mod execute{
    use std::ops::Add;

    use cosmwasm_std::{DepsMut, MessageInfo, Uint128, Response, StdResult, BankMsg, Coin, Env, Decimal};

    use crate::{ContractError, state::{MINIMUM_AMOUNT, BET_COUNTER, BET_DETAILS, BetDetails, BET_AMOUNT, OWNER, BROKERAGE}};

    pub fn create_bet(deps: DepsMut, info:MessageInfo, bet_title: String, team1_title: String, team2_title: String) -> StdResult<Response>{
        let counter = BET_COUNTER.load(deps.storage)?;
        BET_DETAILS.save(deps.storage, counter.u128(), &BetDetails{bet_id: counter, bet_title:bet_title, team1:Uint128::new(0), team1_title: team1_title, team2:Uint128::new(0), team2_title: team2_title, team1_amount:Uint128::new(0), team2_amount:Uint128::new(0), bet_winner:None})?;
        BET_COUNTER.save(deps.storage, &counter.add(Uint128::new(1)))?;
        Ok(Response::new())
    }


    pub fn bet(deps: DepsMut, info:MessageInfo, bet_id: Uint128, bet_team: Uint128) -> Result<Response, ContractError>{
        let minimum_amount = MINIMUM_AMOUNT.load(deps.storage)?;
        if info.funds.iter().any(|coin|{
            coin.denom == minimum_amount.denom && coin.amount >= minimum_amount.amount
        }){
            BET_AMOUNT.save(deps.storage, (&info.sender, bet_id.u128()), &(bet_team, minimum_amount.amount))?;
            let mut details = BET_DETAILS.load(deps.storage, bet_id.u128())?;
            if bet_team == Uint128::new(1){
                details.team1 = details.team1.add(Uint128::new(1));
                details.team1_amount = details.team1_amount.add(minimum_amount.amount);
            }else if  bet_team == Uint128::new(2){
                details.team2 = details.team2.add(Uint128::new(1));
                details.team2_amount = details.team2_amount.add(minimum_amount.amount);
            }
            BET_DETAILS.save(deps.storage, bet_id.u128(), &details)?;

        }
        Ok(Response::new())
    }

    pub fn update_winner(deps: DepsMut, info:MessageInfo, bet_id: Uint128, winner_team: Uint128) -> Result<Response, ContractError>{
        let owner = OWNER.load(deps.storage)?;
        if owner != info.sender{
            return Err(ContractError::Unauthorized { owner: owner.into() })
        }
        let mut details = BET_DETAILS.load(deps.storage, bet_id.u128())?;
        details.bet_winner = Some(winner_team);
        BET_DETAILS.save(deps.storage, bet_id.u128(), &details)?;
        Ok(Response::new())
    }

    pub fn get_reward(deps:DepsMut, info:MessageInfo, env:Env, bet_id: Uint128) -> Result<Response, ContractError>{
        let bet_details = BET_DETAILS.load(deps.storage, bet_id.u128())?;
        if bet_details.bet_winner == None{
            return Err(ContractError::ResultNotDeclared {  })
        }
        let bet_amount = BET_AMOUNT.load(deps.storage, (&info.sender, bet_id.u128()))?;
        if bet_amount.0 != bet_details.bet_winner.unwrap(){
            return  Err(ContractError::NotWinner {  });
        }
        let amount ;
        if bet_amount.0 == Uint128::new(1){
            amount = bet_amount.1.add(Uint128::new((bet_amount.1.u128()/bet_details.team1.u128())*bet_details.team2.u128()));
        }else{
            amount = bet_amount.1.add(Uint128::new((bet_amount.1.u128()/bet_details.team2.u128())*bet_details.team1.u128()));
        }
        let mut coin_amount = deps.querier.query_balance(&env.contract.address, "ATOM")?;
        coin_amount.amount = amount.multiply_ratio(Uint128::new(95),Uint128::new(100));
        BROKERAGE.update(deps.storage, |value| -> StdResult<_> {Ok(value + amount - coin_amount.amount)})?;
        let bank_msg = BankMsg::Send { to_address: info.sender.to_string(), amount:  vec![coin_amount]};
        let resp = Response::new()
            .add_message(bank_msg)
            .add_attribute("reward", amount.to_string())
            .add_attribute("receiver", info.sender.to_string());
        Ok(resp)
    }

    pub fn get_brokerage(deps:DepsMut, info:MessageInfo, env:Env) -> Result<Response, ContractError>{
        let owner = OWNER.load(deps.storage)?;
        if owner != info.sender{
            return Err(ContractError::Unauthorized { owner: owner.into() })
        }
        let brokerage = BROKERAGE.load(deps.storage)?;
        let mut coin_amount = deps.querier.query_balance(&env.contract.address, "ATOM")?;
        coin_amount.amount = brokerage;
        let bank_msg = BankMsg::Send { to_address: info.sender.to_string(), amount: vec![coin_amount] };
        let resp = Response::new()
            .add_message(bank_msg)
            .add_attribute("brokerage", brokerage.to_string())
            .add_attribute("owner", info.sender.to_string());
        Ok(resp)
    }

}