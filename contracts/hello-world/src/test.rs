#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, Env, String};

#[test]
fn test_subscription() {
    let env = Env::default();

    let contract_id = env.register(SubscriptionHub, ());
    let client = SubscriptionHubClient::new(&env, &contract_id);

    client.subscribe(
        &symbol_short!("alice"),
        &String::from_str(&env, "Tien"),
        &String::from_str(&env, "Netflix"),
    );

    let sub = client.get_subscription(
        &symbol_short!("alice")
    );

    assert_eq!(sub.username, String::from_str(&env, "Tien"));
    assert_eq!(sub.plan, String::from_str(&env, "Netflix"));
}