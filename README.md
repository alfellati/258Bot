# 258 Bot

Built for the 258+ Automation

## Specification

### Presale

- `presale_balances`: Make an `presale_balances` storage map called `PresaleBalances` with the format `PresaleBalances: [timestamp, account, amount]` and store the `presale_balances` storage map in a JSON file called `presale_balances.json`. 

- `presale_pool`: Make an `presale_pool` storage map called `PresalePool` with the format `PresalePool: [account, presale_shares]` and store the `presale_pool` storage map in a JSON file called `presale_pool.json`. 

- `presale_allocations`: Make an `presale_allocations` storage map called `PresaleAllocations` with the format `PresaleAllocations: [account, alloc]` and store the `presale_allocations` storage map in a JSON file called `presale_allocations.json`.

- `presale_sequencing`: Check all SOL received transactions ranging from `0.1 SOL` to `10 SOL` for a `target_receiver_account` within a certain period of time `beginning_timestamp` to `ending_timestamp` and record the results in a `presale_balances` storage map. Calculate the `total_amount` which is a sum of all the `amount` from all the accounts, and then allocate `presale_shares` as percentage to each `account` relative to the percentage of `amount` of that `account` in relation to the `total_amount`, then store the results in a `presale_pool` storage map. Ensure that the accounts must be from unique accounts, if any account sends multiple transfers then they should be summed up into the account's `presale_balances.amount` to avoid missing multiple transfers from the same accounts while avoiding duplicate accounts.

- `presale_distribution`: In the `presale_allocations` storage map, distribute `presale_amount` which is `774_000_000 KAMA tokens` to the accounts (ensure must be unique accounts) with each `account` getting their `alloc` relative to their `presale_shares` from the `presale_pool` storage map and add the `alloc` to the `PresaleAllocations` storage map, handle duplicate accounts and ensure that if account already exists in the storage map the `alloc` allocated to it will be updated to add to the amount it already has in the storage map and make a batched airdrop with `presale_amount` of SPL `token` to distribute the airdrop `alloc` to the accounts each with their `alloc` from the `presale_amount`. Store the transaction hash in a JSON file called `presale_distribution_transaction_hash.json`.

### Airdrop

- `airdrop_pool`: Make an `airdrop_pool` storage map called `AirdropPool` with the format `AirdropPool: [account, airdrop_shares]` and store the `airdrop_pool` storage map in a JSON file called `airdrop_pool.json`. 

- `airdrop_allocations`: Make an `airdrop_allocations` storage map called `AirdropAllocations` with the format `AirdropAllocations: [account, alloc]` and store the `airdrop_allocations` storage map in a JSON file called `airdrop_allocations.json`.

- `referrals`: Make a `referrals` storage map called `Referrals` with the format `Referrals: [account, referral_count]` and store the `referrals` storage map in a JSON file called `referrals.json`.

- `airdrop_referrals_sequencing`: Check and sequence a CSV file for accounts with format `[account, referral_count]` and record the accounts with the most `referral_count` in the `referrals` storage map, and also calculate the `total_referral_count` which is a sum of all the `referral_count` from all the `referrals`. Then in the `airdrop_pool` storage map, allocate `airdrop_shares` as percentage to each `account` relative to the percentage of `referral_count` of that `account` in relation to the `total_referral_count`. Then store the results in the `airdrop_pool` storage map. 

- `airdrop_distribution`: In the `airdrop_allocations` storage map, distribute `airdrop_amount` which is `103_200_000 KAMA tokens` to the accounts (ensure must be unique accounts) with each `account` getting their `alloc` relative to their `airdrop_shares` from the `airdrop_pool` storage map and add the `alloc` to the `AirdropAllocations` storage map, handle duplicate accounts and ensure that if account already exists in the storage map the `alloc` allocated to it will be updated to add to the amount it already has in the storage map and make a batched airdrop with `airdrop_amount` of SPL `token` to distribute the airdrop `alloc` to the accounts each with their `alloc` from the `presale_amount`. Store the transaction hash in a JSON file called `airdrop_distribution_transaction_hash.json`.

### Bonuses

- `bonus_allocations`: Make a `bonus_allocations` storage map called `BonusAllocations` with format `BonusAllocations: [account, alloc]` and store the `bonus_allocations` storage map in a JSON file called `bonus_allocations.json`.

- `first_presale_bonuses`: Make a `first_presale_bonuses` storage map called `FirstPresaleBonuses` with format `FirstPresaleBonuses: [timestamp, account, amount]` and store the `first_presale_bonuses` storage map in a JSON file called `first_presale_bonuses.json`.

- `largest_presale_bonuses`: Make a `largest_presale_bonuses` storage map called `LargestPresaleBonuses` with format `LargestPresaleBonuses: [account, amount]` and store the `largest_presale_bonuses` storage map in a JSON file called `largest_presale_bonuses.json`.

- `first_presale_bonuses_sequencing`: Sequence the `presale_balances` storage map and filter out the first `258` transactions (ensure must be unique accounts) by `timestamp` and record the results in the `first_presale_bonuses` storage map. In the `bonus_allocations` storage map, distribute `bonus_amount` which is `12_900_000` KAMA tokens to the accounts with each `account` getting their `alloc` amount relative to their `presale_shares` from the `presale_pool` storage map and add the `alloc` to the `BonusAllocations` storage map, handle duplicate accounts and ensure that if account already exists in the storage map the `alloc` allocated to it will be updated to add to the amount it already has in the storage map, handle duplicate accounts and ensure that if account already exists in the storage map then the `alloc` allocated to it now will be added to the existing `alloc` it already has in the storage map.

- `largest_presale_bonuses_sequencing`: Sequence the `presale_pool` storage map and filter out the top `258` accounts (ensure must be unique accounts) with largest `presale_shares` and record the results in the `largest_presale_bonuses` storage map. In the `bonus_allocations` storage map, distribute `bonus_amount` which is `12_900_000` KAMA tokens to the accounts with each `account` getting their `alloc` amount relative to their `presale_shares` from the `presale_pool` storage map and add the `alloc` to the `BonusAllocations` storage map, handle duplicate accounts and ensure that if account already exists in the storage map the `alloc` allocated to it will be updated to add to the amount it already has in the storage map, handle duplicate accounts and ensure that if account already exists in the storage map then the `alloc` allocated to it now will be added to the existing `alloc` it already has in the storage map.

## Build and Run

### Install Dependencies

To install, use `make init`

### Build

To build, use `make build`

### Run

To run, use `make run`

## License: MIT

Check the [LICENSE](LICENSE) file for more details.
