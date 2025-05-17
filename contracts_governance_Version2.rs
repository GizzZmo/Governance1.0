module Governance {
    struct Proposal {
        id: u64,
        creator: address,
        description: string,
        proposal_type: u8,
        votes_for: u128,
        votes_against: u128,
        veto_votes: u128,
        quorum: u128,
        executed: bool,
    }

    struct Voter {
        address: address,
        stake: u128,
        delegate: option<address>,
        reputation: u128,
    }

    const GOVERNANCE_ADMIN: address = @0x123; // Replace with actual admin address

    public fun submit_proposal(creator: &signer, description: string, proposal_type: u8): u64 {
        assert!(string::length(&description) > 0, "Proposal description cannot be empty");
        assert!(proposal_type < 3, "Invalid proposal type");
        let proposal_id = generate_id();
        let quorum = determine_quorum(proposal_type);
        let new_proposal = Proposal { id: proposal_id, creator: signer::address(creator), description, proposal_type, votes_for: 0, votes_against: 0, veto_votes: 0, quorum, executed: false };
        move_to(creator, new_proposal);
        return proposal_id;
    }

    public fun hybrid_vote(proposal: &mut Proposal, voter_address: address, votes: u128, support: bool, veto: bool) {
        assert!(exists<Voter>(voter_address), "Voter not registered");
        let voter = borrow_global<Voter>(voter_address);
        assert!(voter.stake >= votes, "Insufficient stake to cast these many votes");
        assert!(!proposal.executed, "Proposal has already been executed");

        let effective_votes = sqrt(votes);
        let reputation_weight = 100 + voter.reputation / 100;
        let final_votes = effective_votes * reputation_weight / 100;

        let voting_address = voter.delegate.unwrap_or(voter_address);

        if veto && proposal.proposal_type == 2 {
            proposal.veto_votes += final_votes;
        } else if support {
            proposal.votes_for += final_votes;
        } else {
            proposal.votes_against += final_votes;
        }
    }

    public fun execute_proposal(account: &signer, proposal: &mut Proposal) {
        assert!(signer::address(account) == GOVERNANCE_ADMIN, "Only the governance admin can execute proposals");
        assert!(proposal.votes_for + proposal.votes_against >= proposal.quorum, "Quorum not met");
        assert!(proposal.votes_for > proposal.votes_against, "Proposal rejected");
        assert!(!proposal.executed, "Proposal has already been executed");
        proposal.executed = true;
    }

    fun determine_quorum(proposal_type: u8) -> u128 {
        assert!(proposal_type < 3, "Invalid proposal type");
        match proposal_type {
            0 => total_stake() * 10 / 100,
            1 => total_stake() * 30 / 100,
            2 => total_stake() * 50 / 100,
            _ => abort("Invalid proposal type"),
        }
    }

    fun sqrt(x: u128) -> u128 {
        let mut guess = x / 2;
        while guess * guess > x {
            guess = (guess + x / guess) / 2;
        }
        guess
    }

    native fun generate_id(): u64;
    native fun total_stake(): u128;
}