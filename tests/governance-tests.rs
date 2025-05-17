module GovernanceTests {
    use std::signer;
    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};

    use super::*;

    #[test]
    fun test_submit_proposal() {
        let creator = signer::test_signer(1);
        let description = string::utf8(b"Test proposal");
        let proposal_type = 0;

        let proposal_id = Governance::submit_proposal(
            &creator,
            description,
            proposal_type,
        );

        let proposal = borrow_global<Proposal>(signer::address(&creator));
        assert!(proposal.id == proposal_id, "Proposal ID mismatch");
        assert!(proposal.creator == signer::address(&creator), "Creator mismatch");
        assert!(proposal.description == string::utf8(b"Test proposal"), "Description mismatch");
        assert!(proposal.proposal_type == 0, "Proposal type mismatch");
        assert!(!proposal.executed, "Proposal should not be executed");
    }

    #[test]
    fun test_hybrid_vote() {
        let voter_address = signer::address(&signer::test_signer(2));
        let voter = Voter {
            address: voter_address,
            stake: 1000,
            delegate: option::none(),
            reputation: 50,
        };
        move_to(&signer::test_signer(2), voter);

        let creator = signer::test_signer(1);
        let description = string::utf8(b"Test proposal");
        let proposal_type = 0;
        let proposal_id = Governance::submit_proposal(
            &creator,
            description,
            proposal_type,
        );
        let proposal = borrow_global_mut<Proposal>(signer::address(&creator));

        Governance::hybrid_vote(&mut proposal, voter_address, 100, true, false);
        assert!(proposal.votes_for > 0, "Votes for should be greater than 0");
    }

    #[test]
    fun test_execute_proposal() {
        let creator = signer::test_signer(1);
        let description = string::utf8(b"Test proposal");
        let proposal_type = 0;
        let proposal_id = Governance::submit_proposal(
            &creator,
            description,
            proposal_type,
        );
        let proposal = borrow_global_mut<Proposal>(signer::address(&creator));
        proposal.votes_for = Governance::total_stake(); // Mock enough votes
        Governance::execute_proposal(&creator, &mut proposal);
        assert!(proposal.executed, "Proposal should be executed");
    }

    // Mock native functions for testing
    native fun generate_id(): u64 {
        1
    }

    native fun total_stake(): u128 {
        10000
    }
}
