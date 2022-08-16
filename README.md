# cetra-chamber

## Description
Proof Of Concept for deltra-neutral strategy with leveraged yield farming on Solana blockchain.

## How to test?
1. Ensure that you have latest [Rust](https://rust-lang.com/) and [Solana CLI](https://docs.solana.com/ru/cli/install-solana-cli-tools) versions installed.
2. Open terminal in repo root folder and write: `cargo test-bpf -- --nocapture`.

## Architecture
Main idea of protocol based on concept called `chamber`. Definition for this concept is - `protocol agnostic vault`. Technically `chamber` provide state for entire strategy, farming options, tokens pool accounts, etc..
Each chamber has(currently) the following number of instructions:
- `InitializeChamber` - initializes `chamber` state with farming configuration, rates, fees, strategy type, and other important properties.
- `InitializeChamberStrategy` - creates strategy specific accounts, positions.
- `DepositChamber` - used to deposit funds in `chamber` strategy.
- `SettleChamberPosition` - used to finish internal strategy deposit stage.
- `SettleChamberPosition2` - same as above(this split help us save computation units).

To make deposit, user must create `UserPosition` account. This account is unique per each `chamber`. Currently `UserPosition` is used to track deposited amounts of tokens, but in near future we will change this mechanic to interest bearing tokens. In general case this structure can store various metadata about depositor. `UserPosition` should be created with `InitializeUserPosition` instruction.

In order for strategies to be as stable and effective as possible and work, we must integrate other protocols. Therefore, the `src/handler` folder shows a basic implementation of `wrapper functions` for tulip. Support for other protocols will be added in a similar way.

We reach `protocol-agnostic` mechanism by using various strategies implementations based on internal `chamber` protocol configuration. `DepositChamber` instruction expects remaining accounts related to internal `chamber` strategy option. Therefore, the client needs to lookup strategy specific accounts for specific `chamber`. Obviously this will be possible with our SDK.

## Links
- Protocol [documentation](https://cetra.gitbook.io/welcome/).
