# Staking & Delegation Mechanics

## Overview

- Validators register with minimum stake.
- Delegators can delegate to validators (cannot self-delegate).
- Validator reputation increases with uptime, proposal participation, and successful proposals.
- Reputation can be updated by governance admin.

## Functions

- `initialize`
- `delegate_stake`
- `update_reputation`
- `record_heartbeat`

## Security

- Only admin can initialize or update reputation.
- Heartbeats can only be recorded by validator.
