use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Addr;
use cosmwasm_std::Coin;
use cosmwasm_std::Uint128;

use crate::state::BetDetails;

#[cw_serde]
pub struct InstantiateMsg{
    #[serde(default)]
    pub minimum_amount : Coin
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(OwnerResp)]
    Owner {},
    #[returns(BetDetailResp)]
    BetDetails{
        bet_id : Uint128
    },
    #[returns(BetAmountResp)]
    BetAmount{
        sender : Addr,
        bet_id : Uint128
    }
}

#[cw_serde]
pub struct OwnerResp{
    pub owner : Addr
}

#[cw_serde]
pub struct BetDetailResp{
    pub detail : BetDetails
}

#[cw_serde]
pub struct BetAmountResp{
    pub team: Uint128,
    pub amount : Uint128
}

#[cw_serde]
pub enum ExecMsg{
    CreateBet{
        bet_title: String,
        team1_title: String,
        team2_title: String
    },
    Bet{
        bet_id : Uint128,
        bet_team : Uint128
    },
    UpdateWinner{
        bet_id: Uint128,
        winner: Uint128
    },
    GetReward{
        bet_id : Uint128,
    },
    GetBrokerage{}
}