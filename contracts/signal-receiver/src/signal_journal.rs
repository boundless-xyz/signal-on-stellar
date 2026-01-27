// Defines the Solidity struct layouts for public types
alloy_sol_types::sol! {
    struct Checkpoint {
        uint64 epoch;
        bytes32 root;
    }

    struct State {
        Checkpoint current_justified;
        Checkpoint finalized;
    }

    struct SignalJournal {
        State pre_state;
        State post_state;
        uint64 finalized_slot;
    }
}
