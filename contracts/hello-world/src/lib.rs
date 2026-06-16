#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, String, Symbol
};

#[contracttype]
#[derive(Clone)]
pub struct Subscription {
    pub username: String,
    pub plan: String,
}

#[contract]
pub struct SubscriptionHub;

#[contractimpl]
impl SubscriptionHub {

    pub fn subscribe(
        env: Env,
        user: Symbol,
        username: String,
        plan: String,
    ) {
        let sub = Subscription {
            username,
            plan,
        };

        env.storage().persistent().set(&user, &sub);
    }

    pub fn get_subscription(
        env: Env,
        user: Symbol,
    ) -> Subscription {
        env.storage().persistent().get(&user).unwrap()
    }
}