use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Uint128, Addr};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct BetDetails{
    pub bet_id : Uint128,
    pub bet_title: String,
    pub team1: Uint128,
    pub team1_title: String,
    pub team2: Uint128,
    pub team2_title: String,
    pub team1_amount: Uint128,
    pub team2_amount: Uint128,
    pub bet_winner : Option<Uint128>
}

pub const OWNER:Item<Addr> = Item::new("owner");
pub const BROKERAGE:Item<Uint128> = Item::new("brokerage");
pub const BET_COUNTER:Item<Uint128> = Item::new("bet_counter");
pub const MINIMUM_AMOUNT:Item<Coin> = Item::new("minimum_amount");
pub const BET_DETAILS:Map<u128, BetDetails> = Map::new("bet_details");
pub const BET_AMOUNT:Map<(&Addr, u128), (Uint128, Uint128)> = Map::new("bet_amount");