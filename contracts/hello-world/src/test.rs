#![cfg(test)]

use soroban_sdk::{Address, Env, String, testutils::Address as _, vec};

use crate::our_token::{SibToken, SibTokenClient};

struct SetUpResult<'a>{
    env: Env,
    client:SibTokenClient<'a>,
    sender: Address,
    receiver: Address,
}

fn setup<'a>() -> SetUpResult<'a>{

    let env = Env::default();
    let contract_id = env.register(SibToken, ());
    let client = SibTokenClient::new(&env, &contract_id);
    let sender = Address::generate(&env);
    let receiver = Address::generate(&env);
    SetUpResult { env, client, sender, receiver }
}

#[test]
fn test_balance() {

    let setup_result = setup();

    let sender_balance = setup_result.client.balance(&setup_result.sender);
    let bal = 0;
    assert_eq!(sender_balance, bal);

     let receiver_balance = setup_result.client.balance(&setup_result.receiver);
    assert_eq!(receiver_balance, bal);
}

#[test]
fn test_allowance() {
    let setup_result = setup();

    // let from_bal = 10;
    let spender_bal = 0;

    let allow = setup_result.client.allowance(&setup_result.sender, &setup_result.receiver);

    assert_eq!(allow, spender_bal);
}

#[test]
fn test_approval() {
    let setup_result = setup();

    let amount_app = 200;
    let live_in_ledger = 30;

    setup_result.client.approve(&setup_result.sender, &setup_result.receiver, &amount_app, &live_in_ledger);

    let allow = setup_result.client.allowance(&setup_result.sender, &setup_result.receiver);

    assert_eq!(allow, amount_app);

}

#[test]
fn test_transfer() {
    let setup_result = setup();
 
    setup_result.client.mint(&setup_result.sender, &1000);
    setup_result.client.transfer(&setup_result.sender, &setup_result.receiver, &400);
 
    assert_eq!(setup_result.client.balance(&setup_result.sender), 600);
    assert_eq!(setup_result.client.balance(&setup_result.receiver), 400);
}


#[test]
fn test_mint() {
    let setup_result = setup();
 
    setup_result.client.mint(&setup_result.sender, &1000);
 
    assert_eq!(setup_result.client.balance(&setup_result.sender), 1000);
}

#[test]
fn test_transfer_from() {
    let setup_result = setup();
    let spender = Address::generate(&setup_result.env);
 
    setup_result.client.mint(&setup_result.sender, &1000);
    setup_result.client.approve(&setup_result.sender, &spender, &500, &100u32);
 
    // Spender moves 200 of sender's tokens to receiver
    setup_result.client.transfer_from(&spender, &setup_result.sender, &setup_result.receiver, &200);
 
    // sender: 1000 - 200 = 800
    assert_eq!(setup_result.client.balance(&setup_result.sender), 800);
    // receiver: 0 + 200 = 200
    assert_eq!(setup_result.client.balance(&setup_result.receiver), 200);
    // allowance: 500 - 200 = 300 remaining
    assert_eq!(setup_result.client.allowance(&setup_result.sender, &spender), 300);
}

 
#[test]
fn test_burn() {
    let setup_result = setup();
 
    setup_result.client.mint(&setup_result.sender, &1000);
    setup_result.client.burn(&setup_result.sender, &400);
 
    // Balance reduced
    assert_eq!(setup_result.client.balance(&setup_result.sender), 600);
    // Total supply also reduced — burned tokens leave circulation
    assert_eq!(setup_result.client.total_supply(), 600);
}

#[test]
fn test_burn_from() {
    let setup_result = setup();
    let spender = Address::generate(&setup_result.env);
 
    setup_result.client.mint(&setup_result.sender, &1000);
    setup_result.client.approve(&setup_result.sender, &spender, &500, &100u32);
 
    // Spender burns 300 of sender's tokens using allowance
    setup_result.client.burn_from(&spender, &setup_result.sender, &300);
 
    // sender: 1000 - 300 = 700
    assert_eq!(setup_result.client.balance(&setup_result.sender), 700);
    // total supply reduced by 300
    assert_eq!(setup_result.client.total_supply(), 700);
    // allowance: 500 - 300 = 200 remaining
    assert_eq!(setup_result.client.allowance(&setup_result.sender, &spender), 200);
}