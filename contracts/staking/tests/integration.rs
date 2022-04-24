use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{coins, from_binary, OwnedDeps};
use cosmwasm_std::{to_binary, Addr, Uint128};
use cw20::{Cw20ReceiveMsg, Denom};
use staking::contract::{execute, instantiate, query};
use staking::msg::{
    BalanceResponse, ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg, TotalStackedResponse,
};

fn instantiate_contract() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info("owner", &[]);
    let msg = InstantiateMsg {
        incensitive_denom: Denom::Native("uusd".to_string()),
        stakable_denom: Denom::Cw20(Addr::unchecked("WTOKEN")),
    };
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    deps
}

#[test]
fn integration() {
    let mut deps = instantiate_contract();
    let user_1 = Addr::unchecked("olly");
    let user_2 = Addr::unchecked("francis");

    // deposit 1000 wtoken for user_1
    let msg = Cw20ReceiveMsg {
        sender: user_1.to_string(),
        amount: Uint128::from(2_000u128),
        msg: to_binary(&ReceiveMsg::Deposit {}).unwrap(),
    };

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("WTOKEN", &[]),
        ExecuteMsg::Receive(msg),
    )
    .unwrap();

    // deposit 1000 wtoken for user_2
    let msg = Cw20ReceiveMsg {
        sender: user_2.to_string(),
        amount: Uint128::from(1_000u128),
        msg: to_binary(&ReceiveMsg::Deposit {}).unwrap(),
    };

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("WTOKEN", &[]),
        ExecuteMsg::Receive(msg),
    )
    .unwrap();

    // check deposit was succesful
    let query_res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Balance {
            address: user_1.to_string(),
        },
    )
    .unwrap();
    let res: BalanceResponse = from_binary(&query_res).unwrap();
    assert_eq!(res.amount.u128(), 2000u128);

    let query_res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Balance {
            address: user_2.to_string(),
        },
    )
    .unwrap();
    let res: BalanceResponse = from_binary(&query_res).unwrap();
    assert_eq!(res.amount.u128(), 1000u128);

    let query_res = query(deps.as_ref(), mock_env(), QueryMsg::TotalDeposited {}).unwrap();
    let res: TotalStackedResponse = from_binary(&query_res).unwrap();
    assert_eq!(res.amount.u128(), 3000u128);

    // withdraw some token
    let res = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(&user_1.to_string(), &[]),
        ExecuteMsg::Withdraw {
            amount: Uint128::from(1_000u128),
        },
    )
    .unwrap();
    assert_eq!(res.messages.len(), 1);

    // check the remaining balance
    let query_res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Balance {
            address: user_1.to_string(),
        },
    )
    .unwrap();
    let res: BalanceResponse = from_binary(&query_res).unwrap();
    assert_eq!(res.amount.u128(), 1000u128);

    // Send reward to the contract
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("Someone", &coins(10_000u128, "uusd")),
        ExecuteMsg::Reward {},
    )
    .unwrap();

    // check the earnings
    let query_res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Earned {
            address: user_1.to_string(),
        },
    )
    .unwrap();
    let res: BalanceResponse = from_binary(&query_res).unwrap();
    assert_eq!(res.amount.u128(), 5000u128);
    let query_res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Earned {
            address: user_2.to_string(),
        },
    )
    .unwrap();
    let res: BalanceResponse = from_binary(&query_res).unwrap();
    assert_eq!(res.amount.u128(), 5000u128);

    // withdraw the earnings
    let res = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(&user_1.to_string(), &[]),
        ExecuteMsg::Claim {},
    )
    .unwrap();
    assert_eq!(res.messages.len(), 1);

    // we should have 0 balance now
    let query_res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Earned {
            address: user_1.to_string(),
        },
    )
    .unwrap();
    let res: BalanceResponse = from_binary(&query_res).unwrap();
    assert_eq!(res.amount.u128(), 0u128);
}
