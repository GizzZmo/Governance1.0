module StakingTests {
    use std::signer;
    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};

    use super::*;

    #[test]
    fun test_initialize() {
        let admin = signer::test_signer(1);
        DelegationStaking::initialize(&admin);
        assert!(exists<Config>(signer::address(&admin)), "Config should be initialized");
        let config = borrow_global<Config>(signer::address(&admin));
        assert!(config.min_stake == 1000, "Minimum stake should be 1000");
    }

    #[test]
    fun test_delegate_stake() {
        let admin = signer::test_signer(1);
        DelegationStaking::initialize(&admin);

        let delegator = signer::test_signer(2);
        let validator_address = signer::address(&signer::test_signer(3));
        let stake_amount = 2000;

        // Need a way to mock validator existence and stake updates
        // This test requires more framework-specific mocking capabilities
        // DelegationStaking::delegate_stake(&delegator, validator_address, stake_amount);
        // assert!(/* check delegator stake and validator delegated stake */, "Stake not delegated");
    }

    #[test]
    fun test_update_reputation() {
        let validator_address = signer::address(&signer::test_signer(1));
        move_to(&signer::test_signer(1), Validator {
            address: validator_address,
            stake: 1000,
            delegated_stake: 0,
            reputation: 100,
            last_heartbeat: 0,
        });

        DelegationStaking::update_reputation(&signer::test_signer(0), validator_address, 10);
        let validator = borrow_global<Validator>(validator_address);
        assert!(validator.reputation == 110, "Reputation not updated correctly");

        DelegationStaking::update_reputation(&signer::test_signer(0), validator_address, -20);
        let updated_validator = borrow_global<Validator>(validator_address);
        assert!(updated_validator.reputation == 90, "Reputation not updated correctly with negative change");
    }

    #[test]
    fun test_record_heartbeat() {
        let validator_address = signer::address(&signer::test_signer(1));
        move_to(&signer::test_signer(1), Validator {
            address: validator_address,
            stake: 1000,
            delegated_stake: 0,
            reputation: 100,
            last_heartbeat: 0,
        });

        DelegationStaking::record_heartbeat(&signer::test_signer(1), validator_address);
        let validator = borrow_global<Validator>(validator_address);
        assert!(validator.last_heartbeat == current_time(), "Heartbeat not recorded");
    }

    // Mock native function for testing
    native fun current_time(): u64 {
        100
    }
}
