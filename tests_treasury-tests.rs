module TreasuryTests {
    use std::signer;
    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};

    use super::*;

    #[test]
    fun test_initialize() {
        let admin = signer::test_signer(1);
        let initial_balance = 1000000;
        Treasury::initialize(&admin, initial_balance);
        assert!(exists<Balance>(signer::address(&admin)), "Treasury balance should exist");
        let balance = borrow_global<Balance>(signer::address(&admin));
        assert!(balance.value == initial_balance, "Initial balance mismatch");
    }

    #[test]
    fun test_deposit() {
        let admin = signer::test_signer(1);
        Treasury::initialize(&admin, 1000);
        Treasury::deposit(&admin, 500);
        let balance = borrow_global<Balance>(signer::address(&admin));
        assert!(balance.value == 1500, "Deposit failed");

        let other_account = signer::test_signer(2);
        Treasury::deposit(&other_account, 200);
        let other_balance = borrow_global<Balance>(signer::address(&other_account));
        assert!(other_balance.value == 200, "Deposit to new account failed");
    }

    #[test]
    fun test_submit_withdrawal_proposal() {
        let proposer = signer::test_signer(1);
        let proposal_id = 1;
        let amount = 500;
        move_to(&signer::test_signer(0), Balance { value: 10000 });
        Treasury::submit_withdrawal_proposal(&proposer, proposal_id, amount);
        // Need a way to verify the proposal was stored (implementation details depend on the framework)
        // assert!(/* proposal exists */, "Withdrawal proposal not submitted");
    }

    #[test]
    fun test_approve_withdrawal() {
        let approver = signer::test_signer(1);
        let proposal_id = 1;
        let amount = 500;
        Treasury::approve_withdrawal(&approver, proposal_id, amount);
        // Need a way to verify the approval was stored (implementation details depend on the framework)
        // assert!(approval.proposal_id == proposal_id, "Proposal ID mismatch in approval");
        // assert!(approval.amount == amount, "Approval amount mismatch");
        // assert!(approval.approver == signer::address(&approver), "Approver mismatch");
    }

    #[test]
    fun test_execute_withdrawal() {
        let admin = signer::test_signer(1);
        Treasury::initialize(&admin, 1000);
        // This test would require setup of approvals and a recipient
        // Treasury::execute_withdrawal(&admin, proposal_id, required_approvals, recipient);
        // assert!(/* funds transferred and proposal marked executed */, "Withdrawal not executed");
    }
}