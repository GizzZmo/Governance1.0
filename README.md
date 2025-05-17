# Governance 1.0

A modular Move-based decentralized governance protocol for DAOs, validator sets, and on-chain treasuries.

## Quick Start

1. Install prerequisites and Move toolchain.
2. Deploy contracts:  
   ```sh
   bash scripts/deploy.sh
   ```
3. Run tests:  
   ```sh
   move test ./tests
   ```

## Modules

- `governance.rs` – Proposals and voting
- `delegation-staking.rs` – Validators and delegation
- `treasury.rs` – Treasury management
- `governance_token.rs` – DAO token
- `proposal-handler.rs` – Proposal execution logic

## Scripts

- `scripts/deploy.sh`
- `scripts/governance-cli.sh`

## Docs

- `docs/governance-specs.md`
- `docs/staking-mechanics.md`
- `docs/use_cases.md`