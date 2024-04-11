# 258 Bot

Built for the 258+ Automation

## Features

- `presale_sequencing`: Check all SOL received transactions ranging from `0.1 SOL` to `10 SOL` for a `target_receiver_account` within a certain period of time `beginning_timestamp` to `ending_timestamp` in a CSV file with format [timestamp, received_amount, sender_account] and record the results in a CSV file.

- `make_airdrop`: Make batched airdrop of SPL token to accounts from a CSV file with format [account, amount]

- `tier_three_sequencing`: Sequence a CSV file of SOL transactions in a certain period of time `beginning_timestamp` to `ending_timestamp` in a CSV file with format [timestamp, received_amount, sender_account] and filter out the first 258 transactions by timestamp and record the results in a CSV file.

## License: MIT

Check the [LICENSE](LICENSE) file for more details.
