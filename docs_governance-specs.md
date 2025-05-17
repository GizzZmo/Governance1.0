# Governance Module Specifications

## Proposal Types

- **0:** General proposals
- **1:** Parameter changes
- **2:** Upgrade/veto proposals

## Proposal Lifecycle

1. Submission: Stake/token/reputation requirements enforced.
2. Voting: Hybrid weight (stake, reputation, DAO token).
3. Execution: Requires passing votes and quorum.

## Security

- All entrypoints check caller permissions and invariants.
- Only governance admin can execute proposals.
- Automatic assertion checks for proposal state.

## Example

```move
let proposal_id = Governance::submit_proposal(&signer, "Add feature", 0);
Governance::hybrid_vote(&mut proposal, voter_addr, 100, true, false);
Governance::execute_proposal(&signer, &mut proposal);
```