# Governance 1.0

Governance 1.0 is a modular, Move-based decentralized governance protocol. It features proposal management, hybrid voting (reputation/stake/token), validator delegation, a DAO token, and a treasury with automated audit hooks.

## Features

- **Modular Contracts:** `governance`, `delegation-staking`, `treasury`, `proposal-handler`, `governance_token`
- **Hybrid Voting:** Leverages validator reputation, staked tokens, and delegated stake.
- **DAO Integration:** Proposal submission and voting weighted by governance tokens.
- **Treasury:** Managed with multi-approval logic and AI-audit hooks.
- **Security:** Extensive input/state validation and test coverage.

## Contracts

- `contracts/governance.rs` – Proposal lifecycle and voting logic
- `contracts/delegation-staking.rs` – Validator registration, delegation, reputation
- `contracts/treasury.rs` – Treasury management and withdrawals
- `contracts/proposal-handler.rs` – Proposal execution logic
- `contracts/governance_token.rs` – DAO token contract

## Quick Start

```sh
bash scripts/deploy.sh
```

## Testing

```sh
move test ./tests
```

## CLI

```sh
bash scripts/governance-cli.sh submit-proposal <creator> <description> <type>
bash scripts/governance-cli.sh vote <proposal_id> <voter_addr> <votes> <support> <veto>
bash scripts/governance-cli.sh execute <proposal_id> <executor>
```

## Documentation

- [Governance Specs](governance-specs.md)
- [Staking Mechanics](staking-mechanics.md)
- [Use Cases](use_cases.md)