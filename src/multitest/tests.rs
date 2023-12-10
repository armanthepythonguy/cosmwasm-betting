use cosmwasm_std::{Addr, coins, coin, Uint128, BankMsg, CosmosMsg};
use cw_multi_test::{App, Executor};

use crate::{ContractError, msg::{BetDetailResp, BetAmountResp}, state::BetDetails};

use super::contract::CWBetting;



#[test]
fn create_bet_and_test(){
    let owner = Addr::unchecked("owner");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &owner, coins(10, "ATOM"))
            .unwrap();
    });

    let code_id = CWBetting::store_code(&mut app);

    let contract = CWBetting::instantiate(
        &mut app,
        code_id, 
        &owner, 
        "CW Betting", 
        coin(10, "ATOM")).unwrap();

    contract.create_bet(&mut app, &owner, String::from("Bet"), String::from("Team1"), String::from("Team2"))
                                                .unwrap();

    let res = contract.get_bet(&mut app, &owner, Uint128::new(0)).unwrap();
    let resp = BetDetails{
        bet_id : Uint128::new(0), 
        bet_title : String::from("Bet"),
        team1: Uint128::new(0),
        team1_title:String::from("Team1"),
        team2 : Uint128::new(0),
        team2_title : String::from("Team2"),
        team1_amount : Uint128::new(0),
        team2_amount : Uint128::new(0),
        bet_winner : None
    };
    assert_eq!(res, BetDetailResp{detail : resp})
}

#[test]
fn bet_and_test(){
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(100, "ATOM"))
            .unwrap();
    });

    let code_id = CWBetting::store_code(&mut app);

    let contract = CWBetting::instantiate(
        &mut app,
        code_id, 
        &owner, 
        "CW Betting", 
        coin(10, "ATOM")).unwrap();

    contract.create_bet(&mut app, &owner, String::from("Bet"), String::from("Team1"), String::from("Team2"))
                                                .unwrap();

    // let err = contract.bet(&mut app, &sender, Uint128::new(0), Uint128::new(1), &coins(5, "ATOM"))
    //                                                 .unwrap_err();

    // assert_eq!(err, ContractError::MinimumAmount {  });

    contract.bet(&mut app, &sender, Uint128::new(0), Uint128::new(1), &coins(10, "ATOM"))
                                                    .unwrap();

    let res = contract.get_bet(&mut app, &owner, Uint128::new(0)).unwrap();
    
    let resp = BetDetails{
        bet_id : Uint128::new(0), 
        bet_title : String::from("Bet"),
        team1: Uint128::new(1),
        team1_title:String::from("Team1"),
        team2 : Uint128::new(0),
        team2_title : String::from("Team2"),
        team1_amount : Uint128::new(10),
        team2_amount : Uint128::new(0),
        bet_winner : None
    };

    assert_eq!(res, BetDetailResp{detail : resp});

}

#[test]
pub fn execute_bet_and_test(){
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");
    let sender2 = Addr::unchecked("sender2");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(100, "ATOM"))
            .unwrap();
    });

    let msg : CosmosMsg = BankMsg::Send { to_address: sender2.clone().into(), amount: vec![coin(50, "ATOM")] }.into();
    app.execute(sender.clone(), msg.clone()).unwrap();

    let code_id = CWBetting::store_code(&mut app);

    let contract = CWBetting::instantiate(
        &mut app,
        code_id, 
        &owner, 
        "CW Betting", 
        coin(10, "ATOM")).unwrap();

    contract.create_bet(&mut app, &owner, String::from("Bet"), String::from("Team1"), String::from("Team2"))
                                                .unwrap();

    contract.bet(&mut app, &sender, Uint128::new(0), Uint128::new(1), &coins(10, "ATOM"))
    .unwrap();

    contract.bet(&mut app, &sender2, Uint128::new(0), Uint128::new(2), &coins(10, "ATOM"))
                                                    .unwrap();

    let res = contract.get_bet(&mut app, &owner, Uint128::new(0)).unwrap();

    let resp = BetDetails{
        bet_id : Uint128::new(0), 
        bet_title : String::from("Bet"),
        team1: Uint128::new(1),
        team1_title:String::from("Team1"),
        team2 : Uint128::new(1),
        team2_title : String::from("Team2"),
        team1_amount : Uint128::new(10),
        team2_amount : Uint128::new(10),
        bet_winner : None
    };

    assert_eq!(res, BetDetailResp{detail : resp});

    let res = contract.get_bet_details(&mut app, &sender, Uint128::new(0)).unwrap();

    assert_eq!(res, BetAmountResp{team : Uint128::new(1), amount:Uint128::new(10)});

    let err = contract.update_winner(&mut app, &sender, Uint128::new(0), Uint128::new(1)).unwrap_err();

    assert_eq!(err, ContractError::Unauthorized { owner: owner.clone().into_string() });

    contract.update_winner(&mut app, &owner, Uint128::new(0), Uint128::new(1)).unwrap();

    let res = contract.get_bet(&mut app, &owner, Uint128::new(0)).unwrap();

    let resp = BetDetails{
        bet_id : Uint128::new(0), 
        bet_title : String::from("Bet"),
        team1: Uint128::new(1),
        team1_title:String::from("Team1"),
        team2 : Uint128::new(1),
        team2_title : String::from("Team2"),
        team1_amount : Uint128::new(10),
        team2_amount : Uint128::new(10),
        bet_winner : Some(Uint128::new(1))
    };

    assert_eq!(res, BetDetailResp{detail : resp});

    contract.get_reward(&mut app, &sender, Uint128::new(0)).unwrap();

    assert_eq!(app.wrap().query_all_balances(sender2.clone()).unwrap(), coins(40, "ATOM"));
    assert_eq!(app.wrap().query_all_balances(sender.clone()).unwrap(), coins(59, "ATOM"));

    let err=contract.get_brokerage(&mut app, &sender).unwrap_err();
    assert_eq!(err, ContractError::Unauthorized { owner: owner.to_string() });

    contract.get_brokerage(&mut app, &owner).unwrap();
    assert_eq!(app.wrap().query_all_balances(owner.clone()).unwrap(), coins(1, "ATOM"));
}