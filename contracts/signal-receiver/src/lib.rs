#![no_std]
extern crate alloc;
use alloy_sol_types::SolValue;
use groth16_verifier::RiscZeroGroth16VerifierClient;
use hex_literal::hex;
use signal_journal::SignalJournal;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Bytes, BytesN, Env,
    Symbol,
};

mod signal_journal;

/// Image ID for Mainnet Signal Ethereum v1.3.0 RISC0 program
/// https://github.com/boundless-xyz/Signal-Ethereum/releases/tag/v1.3.0
const IMAGE_ID: [u8; 32] = hex!("0ccb3d146a7f64e78cc1d146acc26912138ea39bb79b4ca74423389d61b2c30e");

const VERIFIER_ADDR: Symbol = symbol_short!("VERIFR");
const CURRENT_STATE: Symbol = symbol_short!("CURR_ST");

#[contract]
pub struct SignalReceiver;

#[contractimpl]
impl SignalReceiver {
    /// Initialize the receiver with a deployed RISC Zero verifier contract and initial consensus state
    pub fn __constructor(env: Env, risc0_verifier: Address, initial_state: BytesN<128>) {
        env.storage()
            .instance()
            .set(&VERIFIER_ADDR, &risc0_verifier);

        env.storage().instance().set(&CURRENT_STATE, &initial_state);
    }

    /// Receive a new signal proof and attempt to update the contract state and store the finalized root
    pub fn receive(env: Env, seal: Bytes, journal: BytesN<288>) -> Result<bool, Error> {
        // Call the verifier contract to verify the proof
        let verifier_addr = env
            .storage()
            .instance()
            .get(&VERIFIER_ADDR)
            .ok_or(Error::ContractNotInitialized)?;
        let verifier = RiscZeroGroth16VerifierClient::new(&env, &verifier_addr);

        let journal_digest = env.crypto().sha256(&journal.as_ref()).to_bytes();

        verifier
            .try_verify(&seal, &BytesN::from_array(&env, &IMAGE_ID), &journal_digest)
            .map_err(|_| Error::FailedToCallVerifier)?
            .map_err(|_| Error::ProofVerificationFailed)?;

        // Decode the journal
        let journal = SignalJournal::abi_decode(&journal.to_array())
            .map_err(|_| Error::FailedToDecodeJournal)?;

        // Check this journal's pre_state matches the current stored state
        // ensuring it is a valid consensus transition
        let curr_state: Bytes = env.storage().instance().get(&CURRENT_STATE).unwrap();
        if journal.pre_state.abi_encode() != curr_state.to_alloc_vec() {
            return Err(Error::JournalNotSuccessor);
        }

        // Store the finalized root for the finalized slot for later use
        env.storage().persistent().set(
            &DataKey::BeaconSlot(journal.finalized_slot),
            &journal.post_state.finalized.root.0,
        );

        // Update the contracts current view of the Ethereum consensus state
        env.storage()
            .instance()
            .set(&CURRENT_STATE, &journal.post_state.abi_encode().as_slice());

        Ok(true)
    }
}

#[contracttype]
pub enum DataKey {
    BeaconSlot(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    ContractNotInitialized = 1,
    FailedToDecodeJournal = 2,
    FailedToCallVerifier = 3,
    ProofVerificationFailed = 4,
    JournalNotSuccessor = 5,
}

mod test;
