#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::{StellarAssetClient, TokenClient},
    Address, Env, String, Vec,
};

fn create_token_contract<'a>(
    env: &Env,
    admin: &Address,
) -> (TokenClient<'a>, StellarAssetClient<'a>) {
    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    (
        TokenClient::new(env, &sac.address()),
        StellarAssetClient::new(env, &sac.address()),
    )
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(SplitPayContract, ());
    let client = SplitPayContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    assert_eq!(client.get_group_count(), 0);
}

#[test]
fn test_create_group() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);

    let (_, _) = create_token_contract(&env, &admin);
    let token_address = env
        .register_stellar_asset_contract_v2(admin.clone())
        .address();

    let contract_id = env.register(SplitPayContract, ());
    let client = SplitPayContractClient::new(&env, &contract_id);

    client.initialize(&admin);

    let mut members = Vec::new(&env);
    members.push_back(alice.clone());
    members.push_back(bob.clone());
    members.push_back(charlie.clone());

    let group_id = client.create_group(
        &alice,
        &String::from_str(&env, "Dinner Party"),
        &token_address,
        &members,
    );

    assert_eq!(group_id, 0);
    assert_eq!(client.get_group_count(), 1);

    let group = client.get_group(&group_id);
    assert_eq!(group.total_members, 3);
    assert_eq!(group.is_settled, false);
}

#[test]
fn test_add_expense() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);

    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    let token_address = sac.address();
    let token_admin = StellarAssetClient::new(&env, &token_address);

    // Mint tokens to alice
    token_admin.mint(&alice, &3000);

    let contract_id = env.register(SplitPayContract, ());
    let client = SplitPayContractClient::new(&env, &contract_id);

    client.initialize(&admin);

    let mut members = Vec::new(&env);
    members.push_back(alice.clone());
    members.push_back(bob.clone());
    members.push_back(charlie.clone());

    let group_id = client.create_group(
        &alice,
        &String::from_str(&env, "Dinner Party"),
        &token_address,
        &members,
    );

    client.add_expense(
        &group_id,
        &alice,
        &3000,
        &String::from_str(&env, "Restaurant bill"),
    );

    let group = client.get_group(&group_id);
    assert_eq!(group.total_paid, 3000);

    let payments = client.get_payments(&group_id);
    assert_eq!(payments.len(), 1);

    // Alice paid 3000, split 3 ways = 1000 each
    // Alice is owed 2000 (she paid for bob and charlie)
    let alice_balance = client.get_balance(&group_id, &alice);
    assert_eq!(alice_balance, 2000);

    let bob_balance = client.get_balance(&group_id, &bob);
    assert_eq!(bob_balance, -1000);

    let charlie_balance = client.get_balance(&group_id, &charlie);
    assert_eq!(charlie_balance, -1000);
}

#[test]
fn test_split_payment() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let payer = Address::generate(&env);
    let recipient1 = Address::generate(&env);
    let recipient2 = Address::generate(&env);

    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    let token_address = sac.address();
    let token_admin = StellarAssetClient::new(&env, &token_address);
    let token_client = TokenClient::new(&env, &token_address);

    // Mint tokens to payer
    token_admin.mint(&payer, &1000);

    let contract_id = env.register(SplitPayContract, ());
    let client = SplitPayContractClient::new(&env, &contract_id);

    client.initialize(&admin);

    let mut recipients = Vec::new(&env);
    recipients.push_back(recipient1.clone());
    recipients.push_back(recipient2.clone());

    client.split_payment(&payer, &token_address, &1000, &recipients);

    assert_eq!(token_client.balance(&recipient1), 500);
    assert_eq!(token_client.balance(&recipient2), 500);
    assert_eq!(token_client.balance(&payer), 0);
}

#[test]
fn test_settle_group() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    let token_address = sac.address();
    let token_admin = StellarAssetClient::new(&env, &token_address);
    let token_client = TokenClient::new(&env, &token_address);

    // Mint tokens
    token_admin.mint(&alice, &1000);
    token_admin.mint(&bob, &1000);

    let contract_id = env.register(SplitPayContract, ());
    let client = SplitPayContractClient::new(&env, &contract_id);

    client.initialize(&admin);

    let mut members = Vec::new(&env);
    members.push_back(alice.clone());
    members.push_back(bob.clone());

    let group_id = client.create_group(
        &alice,
        &String::from_str(&env, "Trip"),
        &token_address,
        &members,
    );

    // Alice pays 600
    client.add_expense(&group_id, &alice, &600, &String::from_str(&env, "Hotel"));

    // Bob pays 400
    client.add_expense(&group_id, &bob, &400, &String::from_str(&env, "Food"));

    // Total: 1000, each should pay 500
    // Alice paid 600, so owed 100
    // Bob paid 400, so owes 100

    client.settle_group(&group_id, &alice);

    let group = client.get_group(&group_id);
    assert_eq!(group.is_settled, true);

    // After settlement:
    // Alice: started 1000, paid 600 to contract = 400, gets back 600 (500 fair + 100 credit) = 1000...
    // Actually let's check: alice starts 1000, transfers 600 -> 400 left
    // Bob starts 1000, transfers 400 -> 600 left
    // Contract has 1000
    // Settlement: alice gets 500 + 100 = 600, bob gets 500 - 100 = 400
    // Final: alice = 400 + 600 = 1000, bob = 600 + 400 = 1000
    assert_eq!(token_client.balance(&alice), 1000);
    assert_eq!(token_client.balance(&bob), 1000);
}
