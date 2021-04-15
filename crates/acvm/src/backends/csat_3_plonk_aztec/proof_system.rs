use std::collections::BTreeMap;

use acir::{circuit::Circuit, native_types::Witness};
use aztec_backend::barretenberg_rs::composer::{Assignments, StandardComposer};
use noir_field::Bn254Scalar;

use crate::{Language, ProofSystemCompiler};

use super::Plonk;

impl ProofSystemCompiler<Bn254Scalar> for Plonk {
    fn prove_with_meta(
        &self,
        circuit: Circuit<Bn254Scalar>,
        witness_values: BTreeMap<Witness, Bn254Scalar>,
    ) -> Vec<u8> {
        let constraint_system = aztec_backend::serialise_circuit(&circuit);

        let mut composer = StandardComposer::new(constraint_system);

        // Add witnesses in the correct order
        // Note: The witnesses are sorted via their witness index, since we implement Ord on Witness and use a BTreeMap
        let mut sorted_witness = Assignments::new();
        for (_, value) in witness_values.iter() {
            sorted_witness.push(*value);
        }

        composer.create_proof(sorted_witness)
    }

    fn verify_from_cs(
        &self,
        proof: &[u8],
        public_inputs: Vec<Bn254Scalar>,
        circuit: Circuit<Bn254Scalar>,
    ) -> bool {
        let constraint_system = aztec_backend::serialise_circuit(&circuit);

        let mut composer = StandardComposer::new(constraint_system);

        composer.verify(&proof, Some(Assignments::from_vec(public_inputs)))
    }

    fn np_language(&self) -> Language {
        Language::PLONKCSat { width: 3 }
    }
}
