# Tien Soroban Project

A simple subscription management smart contract built with Stellar Soroban.

## Overview

Subscription Hub allows users to register and store subscription information directly on the Stellar blockchain. Each user can save their subscription plan and retrieve it later.

This project was developed as a learning project for Stellar Soroban smart contract development.

## Features

* Register a subscription
* Store subscription information on-chain
* Retrieve subscription details
* Built with Rust and Soroban SDK

## Smart Contract Functions

### subscribe()

Stores a user's subscription information.

Parameters:

* `user` - User identifier
* `username` - User name
* `plan` - Subscription plan

### get_subscription()

Returns the stored subscription information for a user.

## Example

Register a subscription:

```bash
subscribe(
    user = "alice",
    username = "Tien",
    plan = "Netflix"
)
```

Retrieve subscription:

```bash
get_subscription("alice")
```

Expected result:

```text
Username: Tien
Plan: Netflix
```

## Technologies Used

* Rust
* Stellar Soroban SDK
* Stellar Testnet
* Soroban Studio

## Project Structure

```text
src/
├── lib.rs
└── test.rs
```

## Test

The project includes unit tests to verify:

* Subscription creation
* Subscription retrieval
* Data integrity

## Future Improvements

* Multiple subscriptions per user
* Subscription expiration dates
* Automatic renewals
* Payment tracking

## Author

Tien Truong

## License

MIT License
