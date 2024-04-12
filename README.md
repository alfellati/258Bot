# 258 Bot

Built for the 258+ Automation

## Features

### Presale

- `presale_pool`: Make an `presale_pool` storage map called `PresalePool` with the format `PresalePool: [account, presale_shares]` and store the `presale_pool` storage map in a JSON file called `presale_pool.json`. 

- `presale_allocations`: Make an `presale_allocations` storage map called `PresaleAllocations` with the format `PresaleAllocations: [account, alloc]` and store the `presale_allocations` storage map in a JSON file called `presale_allocations.json`.

- `presale_sequencing`: Check all SOL received transactions ranging from `0.1 SOL` to `10 SOL` for a `target_receiver_account` within a certain period of time `beginning_timestamp` to `ending_timestamp` and record the results in a `presale_balances` storage map called `PresaleBalances` that stores the results in the following format: `PresaleBalances: [timestamp, account, sent_amount]`. Calculate the `total_sent_amount` which is a sum of all the `sent_amount` from all the accounts, and then allocate `presale_shares` as percentage to each `account` relative to the percentage of `sent_amount` of that `account` in relation to the `total_sent_amount`, then store the results in a `presale_pool` storage map called `PresalePool` with the format `PresalePool: [account, presale_shares]`. Store the `presale_balances` storage map in a JSON file called `presale_balances.json` and store the `presale_pool` storage map in a JSON file called `presale_pool.json`.

- `presale_distribution`: In the `presale_allocations` storage map, distribute `presale_amount` which is `774_000_000 KAMA tokens` to the accounts each getting their `alloc` relative to their `presale_shares` from the `presale_pool` storage map and add the `alloc` to the `PresaleAllocations` storage map, handle duplicate accounts and ensure that if account already exists in the storage map the `alloc` allocated to it will be updated to add to the amount it already has in the storage map and make a batched airdrop with `presale_amount` of SPL `token` to distribute the airdrop `alloc` to the accounts each with their `alloc` from the `presale_amount`. Store the transaction hash in a JSON file called `presale_distribution_transaction_hash.json`.

### Airdrop

- `airdrop_pool`: Make an `airdrop_pool` storage map called `AirdropPool` with the format `AirdropPool: [account, airdrop_shares]` and store the `airdrop_pool` storage map in a JSON file called `airdrop_pool.json`. 

- `airdrop_allocations`: Make an `airdrop_allocations` storage map called `AirdropAllocations` with the format `AirdropAllocations: [account, alloc]` and store the `airdrop_allocations` storage map in a JSON file called `airdrop_allocations.json`.

- `referrals`: Make a `referrals` storage map called `Referrals` with the format `Referrals: [account, referral_count]` and store the `referrals` storage map in a JSON file called `referrals.json`.

- `airdrop_referrals_sequencing`: Check and sequence a CSV file for accounts with format `[account, referral_count]` and record the accounts with the most `referral_count` in the `referrals` storage map, and also calculate the `total_referral_count` which is a sum of all the `referral_count` from all the `referrals`. Then in the `airdrop_pool` storage map, allocate `airdrop_shares` as percentage to each `account` relative to the percentage of `referral_count` of that `account` in relation to the `total_referral_count`. Then store the results in the `airdrop_pool` storage map. 

- `airdrop_distribution`: In the `airdrop_allocations` storage map, distribute `airdrop_amount` which is `103_200_000 KAMA tokens` to the accounts each getting their `alloc` relative to their `airdrop_shares` from the `airdrop_pool` storage map and add the `alloc` to the `AirdropAllocations` storage map, handle duplicate accounts and ensure that if account already exists in the storage map the `alloc` allocated to it will be updated to add to the amount it already has in the storage map and make a batched airdrop with `airdrop_amount` of SPL `token` to distribute the airdrop `alloc` to the accounts each with their `alloc` from the `presale_amount`. Store the transaction hash in a JSON file called `airdrop_distribution_transaction_hash.json`.

### Bonuses

- `bonus_allocations`: Make a `bonus_allocations` storage map called `BonusAllocations` with format `BonusAllocations: [account, alloc]` and store the `bonus_allocations` storage map in a JSON file called `bonus_allocations.json`.

- `tier_one_bonuses`: Make a `tier_one_bonuses` storage map called `TierOneBonuses` with format `TierOneBonuses: [timestamp, account, amount]` and store the `tier_one_bonuses` storage map in a JSON file called `tier_one_bonuses.json`.

- `tier_one_bonuses_sequencing`: Sequence the `presale_balances` storage map and filter out the first `258` transactions by `timestamp` that are ranging from `5.1 SOL` to `10 SOL` in size and record the results in the `tier_one_bonuses` storage map, and also calculate the `total_amount` which is a sum of all the `amount` from all the bonuses. Using data from `tier_one_bonuses`, allocate `pool_shares` as percentage to each `account` relative to the percentage of `amount` of that `account` in relation to the `total_amount`. In the `bonus_allocations` storage map, distribute `tier_one_bonuses_amount` which is `15_480_000 KAMA tokens` to the accounts each getting their `alloc` relative to their `pool_shares` and add it to the `BonusAllocations` storage map, handle duplicate accounts and ensure that if account already exists in the storage map the `alloc` allocated to it will be updated to add to the amount it already has in the storage map.

- `tier_two_bonuses_sequencing`: Sequence the `presale_balances` storage map and filter out the first `258` transactions by `timestamp` that are ranging from `1 SOL` to `5 SOL` in size and record the results in a `tier_two_bonuses` storage map called `TierTwoBonuses` with format `TierTwoBonuses: [timestamp, account, amount]`, and also calculate the `total_amount` which is a sum of all the `amount` from all the bonuses. Then using `tier_two_bonuses`, allocate `pool_shares` as percentage to each `account` relative to the percentage of `amount` of that `account` in relation to the `total_amount`. In the `bonus_allocations` storage map, distribute `tier_two_bonuses_amount` which is `12_900_000 KAMA tokens` to the accounts each getting their `alloc` relative to their `pool_shares` and add it to the `BonusAllocations` storage map, handle duplicate accounts and ensure that if account already exists in the storage map the `alloc` allocated to it will be updated to add to the amount it already has in the storage map. Store the `tier_two_bonuses` storage map in a JSON file called `tier_two_bonuses.json`.

- `tier_three_bonuses_sequencing`: Sequence the `presale_balances` storage map and filter out the first `258` transactions by `timestamp` and record the results in a `tier_three_bonuses` storage map called `TierThreeBonuses` with format `TierThreeBonuses: [timestamp, account, amount]`, and also calculate the `total_amount` which is a sum of all the `amount` from all the `tier_three_bonuses`. Then using `tier_three_bonuses`, allocate `pool_shares` as percentage to each `account` relative to the percentage of `amount` of that `account` in relation to the `total_amount`. Make a `bonus_allocations` storage map called `BonusAllocations` with format `BonusAllocations: [account, bonus_alloc]` and then distribute `bonuses_amount` to the accounts in their `bonus_alloc` relative to their `pool_shares`, handle duplicate accounts and ensure that if account already exists in the storage map the `amount` allocated to it will be updated to add to the amount it already has in the storage map. Store the `bonus_allocations` storage map in a JSON file called `bonus_allocations.json`. Store the `tier_three_bonuses` storage map in a JSON file called `tier_three_bonuses.json`.

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
