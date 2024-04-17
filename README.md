# 258 Bot

Built for the 258+ Automation

## Specification

### Presale

- `presale_balances`: Make an `presale_balances` storage map called `PresaleBalances` with the format `PresaleBalances: [timestamp, account, amount]` and store the `presale_balances` storage map in a JSON file called `presale_balances.json`. 

- `presale_allocations`: Make an `presale_allocations` storage map called `PresaleAllocations` with the format `PresaleAllocations: [account, alloc]` and store the `presale_allocations` storage map in a JSON file called `presale_allocations.json`.

- `check_solana_transactions`: On the Solana network, Check all SOL received transactions with a minimum of `0.1 SOL` to a `target_receiver_account` within a certain period of time `beginning_timestamp` to `ending_timestamp` and record the results in a `presale_balances` storage map. Calculate the `total_amount` which is a sum of all the `amount` from all the accounts, and then allocate `presale_shares` as percentage to each `account` relative to the percentage of `amount` of that `account` in relation to the `total_amount`, then distribute `presale_amount` which is `774_000_000 KAMA tokens` to the accounts (ensure must be unique accounts) with each `account` getting their `alloc` relative to their `presale_shares`. Handle duplicate accounts and ensure that if account already exists in the storage map the `alloc` allocated to it will be updated to add to the amount it already has in the storage map.

## Build and Run

### Install Dependencies

To install, use `make init`

### Build

To build, use `make build`

### Run

To run, use `make run`

## License: MIT

Check the [LICENSE](LICENSE) file for more details.
