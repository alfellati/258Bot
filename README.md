# 258 Bot

Built for the 258+ Automation

## Features

- `presale_sequencing`: Check all SOL received transactions ranging from `0.1 SOL` to `10 SOL` for a `target_receiver_account` within a certain period of time `beginning_timestamp` to `ending_timestamp` and record the results in a `presale_balances` storage map called `PresaleBalances` that stores the results in the following format: `PresaleBalances: [timestamp, account, sent_amount]`. Calculate the `total_sent_amount` which is a sum of all the `sent_amount` from all the accounts, and then allocate `presale_shares` as percentage to each `account` relative to the percentage of `sent_amount` of that `account` in relation to the `total_sent_amount`, then store the results in a `presale_pool` storage map called `PresalePool` with the format `PresalePool: [account, presale_shares]`. Store the `presale_balances` storage map in a JSON file called `presale_balances.json` and store the `presale_pool` storage map in a JSON file called `presale_pool.json`.

- `airdrop_referrals_sequencing`: Check and sequence a CSV file for accounts with format `[account, referral_count]` and record the accounts with the most `referral_count` in a `referrals` storage map called `Referrals` that stores the results in the following format: `Referrals: [account, referral_count]`, and also calculate the `total_referral_count` which is a sum of all the `referral_count` from all the `referrals`, then allocate `airdrop_shares` as percentage to each `account` relative to the percentage of `referral_count` of that `account` in relation to the `total_referral_count`. Then store the results in an `airdrop_pool` storage map called `AirdropPool` with the format `AirdropPool: [account, airdrop_shares]`. Store the `referrals` storage map in a JSON file called `referrals.json` and store the `airdrop_pool` storage map in a JSON file called `airdrop_pool.json`.

- `airdrop_distribution`: Check the `airdrop_pool` storage map for the `account` and `airdrop_shares` and make a batched airdrop with `amount` of SPL `token` to distribute the airdrop `amount` to the accounts each with their `airdrop_shares` percentage of the airdrop. Store the transaction hash in an `airdrop_distribution_transaction_hash` storage map called `AirdropDistributionTransactionHash`. Store the `airdrop_distribution_transaction_hash` storage map in a JSON file called `airdrop_distribution_transaction_hash.json`. And store the `airdrop_allocations` storage map named `AirdropAllocations` with format `AirdropAllocations: [account, alloc]` in a JSON file called `airdrop_allocations.json`.

- `presale_distribution`: Check the `presale_pool` storage map for the `account` and `presale_shares` and make a batched airdrop with `amount` of SPL `token` to distribute the airdrop `amount` to the accounts each with their `presale_shares` percentage of the airdrop. Store the transaction hash in an `presale_distribution_transaction_hash` storage map called `PresaleDistributionTransactionHash`. Store the `presale_distribution_transaction_hash` storage map in a JSON file called `presale_distribution_transaction_hash.json`. And store the `presale_allocations` storage map named `PresaleAllocations` with format `PresaleAllocations: [account, alloc]` in a JSON file called `presale_allocations.json`.

- `tier_one_bonuses_sequencing`: Sequence the `presale_balances` storage map and filter out the first `258` transactions by `timestamp` that are ranging from `1 SOL` to `5 SOL` in size and record the results in a `tier_one_bonuses` storage map called `TierOneBonuses` with format `TierOneBonuses: [timestamp, account, amount]`, and also calculate the `total_amount` which is a sum of all the `amount` from all the `tier_one_bonuses`, then allocate `pool_shares` as percentage to each `account` relative to the percentage of `amount` of that `account` in relation to the `total_amount`. Make a `bonus_allocations` storage map called `BonusAllocations` with format `BonusAllocations: [account, bonus_alloc]` and then distribute `bonuses_amount` to the accounts in their `bonus_alloc` relative to their `pool_shares`, handle duplicate accounts and ensure that if account already exists in the storage map the `amount` allocated to it will be updated to add to the amount it already has in the storage map. Store the `bonus_allocations` storage map in a JSON file called `bonus_allocations.json`. Store the `tier_one_bonuses` storage map in a JSON file called `tier_one_bonuses.json`.

- `tier_two_bonuses_sequencing`: TODO

- `tier_three_bonuses_sequencing`: Sequence the `presale_balances` storage map and filter out the first `258` transactions by `timestamp` and record the results in a `tier_three_bonuses` storage map called `TierThreeBonuses` with format `TierThreeBonuses: [timestamp, account, amount]`, and also calculate the `total_amount` which is a sum of all the `amount` from all the `tier_three_bonuses`, then allocate `pool_shares` as percentage to each `account` relative to the percentage of `amount` of that `account` in relation to the `total_amount`. Make a `bonus_allocations` storage map called `BonusAllocations` with format `BonusAllocations: [account, bonus_alloc]` and then distribute `bonuses_amount` to the accounts in their `bonus_alloc` relative to their `pool_shares`, handle duplicate accounts and ensure that if account already exists in the storage map the `amount` allocated to it will be updated to add to the amount it already has in the storage map. Store the `bonus_allocations` storage map in a JSON file called `bonus_allocations.json`. Store the `tier_three_bonuses` storage map in a JSON file called `tier_three_bonuses.json`.

- `tier_four_bonuses_sequencing`: TODO

- `tier_five_bonuses_sequencing`: TODO

## Build and Run

### Install Dependencies

To install, use `make init`

### Build

To build, use `make build`

### Run

To run, use `make run`

## License: MIT

Check the [LICENSE](LICENSE) file for more details.
