# Abra Liquidation Bot
Rust implementation of an abracadabra liquidator

## Prerequisites
- Rust
- Foundry (optional to test & deploy new contracts)

## Wallet
This is an immature software, only use with wallet with smaller ETH amount.

## How to run
```
make build-release
```

- create `.env` from `.env.example` and fill the necessary endpoints. `RPC_URL` is the current chain websocket rpc.
- `launch_one.sh` and `launch_all.sh` contains examples how to run the bot, modify and adapt at will.
- each bot process are isolated instance and corresponding to one cauldron listener and liquidator using it's own parameters.
- full archives nodes needs to be used to properly work

## Improvement
The bot needs a lot of improvements and polishing:

### TODO
- Everytime the bot is started, it's fetching previous blocks to build a in-memory database of user's positions. It should be cached locally and incrementaly only fetch the newer block when initialized.
- Support 0x swappers
- Having module so that cauldron's specific features are handled properly. For example, with GLP cauldron's it needs to use a Lens contract to resolve the best asset to burn GLP to and use it to build the 0x api request.
- Support cook based liquidation, where the master contract is approved/disapproved using a signature instead of approving the master contract globally.
- Better code architecture divided in module
- Passing less parameters and instead having a struct accross the multiple functions
- Better way to handle launching the bots
- Support for better profitability checks. Current bot state is making very minor checks for profitability, like the `MIN_BORROW_PART` part of the configuration which ignores user positions that are inferior to the value.
- Telegram/Discord notifications handling

### Bugs / issues
- Revise why sending a transaction using web socket fails and find a way to only use websocket or http so it's simpler to handle.
- Something the gas estimate pass but the transactions fails, spending gas.
- The bot can drain all your wallet ETH as it doesn't have any support for profitability checks and killswitch when something goes wrong.
