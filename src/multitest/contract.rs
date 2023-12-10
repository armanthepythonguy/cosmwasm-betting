use cosmwasm_std::{Addr, StdResult, Coin, Uint128, coin};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::{ContractError, query, instantiate, execute, msg::{InstantiateMsg, ExecMsg, BetDetailResp, QueryMsg, BetAmountResp}};

pub struct CWBetting(Addr);

impl CWBetting {
    pub fn addr(&self) -> &Addr{
        &self.0
    }

    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }

    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: u64,
        sender: &Addr,
        label: &str,
        minimal_donation: Coin,
    ) -> StdResult<Self>{
 
        app.instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {
                minimum_amount: minimal_donation,
            },
            &[],
            label,
            None,
        )
        .map(CWBetting)
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn create_bet(
        &self,
        app: &mut App,
        sender: &Addr,
        bet_title: String,
        team1_title: String,
        team2_title: String
    ) -> Result<(), ContractError>{
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::CreateBet { bet_title: bet_title, team1_title: team1_title, team2_title: team2_title }, &[])
            .map_err(|err| err.downcast().unwrap())
            .map(|_| ())
    }

    #[track_caller]
    pub fn get_bet(
        &self,
        app: &mut App,
        sender: &Addr,
        bet_id: Uint128
    ) -> StdResult<BetDetailResp>{
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::BetDetails { bet_id: bet_id })
    }

    #[track_caller]
    pub fn bet(
        &self,
        app: &mut App,
        sender: &Addr,
        bet_id: Uint128,
        team_id : Uint128,
        funds: &[Coin]
    ) -> Result<(), ContractError>{
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Bet { bet_id: bet_id, bet_team: team_id }, funds)
        .map_err(|err| err.downcast().unwrap())
        .map(|op| ())
    }

    #[track_caller]
    pub fn get_bet_details(
        &self,
        app: &mut App,
        sender: &Addr,
        bet_id: Uint128,
    ) -> StdResult<BetAmountResp>{
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::BetAmount { sender: sender.clone(), bet_id: bet_id })
    }

    #[track_caller]
    pub fn update_winner(
        &self,
        app: &mut App,
        sender: &Addr,
        bet_id: Uint128,
        winner: Uint128
    ) ->  Result<(), ContractError>{
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::UpdateWinner { bet_id: bet_id, winner: winner }, &[])
        .map_err(|err| err.downcast().unwrap())
        .map(|op| ())
    }

    pub fn get_reward(
        &self,
        app: &mut App,
        sender: &Addr,
        bet_id: Uint128,
    ) -> Result<(), ContractError>{
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::GetReward { bet_id: bet_id }, &[])
        .map_err(|err| err.downcast().unwrap())
        .map(|op| ())
    }

    pub fn get_brokerage(
        &self,
        app: &mut App,
        sender: &Addr
    ) -> Result<(), ContractError>{
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::GetBrokerage {  }, &[])
        .map_err(|err| err.downcast().unwrap())
        .map(|op| ())
    }
}

impl From<CWBetting> for Addr {
    fn from(contract: CWBetting) -> Self {
        contract.0
    }
}