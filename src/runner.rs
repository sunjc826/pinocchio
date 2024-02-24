// This is adapted from the nova-snark package's min_root example.
// We don't care too much about compression for the moment,
// so the parts related to CompressedSNARK are removed.
// This file provides a somewhat unified/simplified way to operate on a StepCircuit impl.

use crate::{circuits::utility::wrapper::StepCircuitWrapper, types::*};
use nova_snark::{
    provider::hyperkzg::Bn256EngineKZG,
    traits::{circuit::TrivialCircuit, snark::RelaxedR1CSSNARKTrait, Engine},
    PublicParams, RecursiveSNARK,
};
use std::time::Instant;

pub fn run<C1: StepCircuitWrapper<<E1 as Engine>::GE>>(
    description: &str,
    circuit_primary: C1,
    circuits: Vec<C1>,
    z0_primary: &[<Bn256EngineKZG as Engine>::Scalar],
) {
    println!("{}", description);
    println!("=========================================================");

    let num_steps = circuits.len();

    let circuit_secondary = TrivialCircuit::default();

    // produce public parameters
    let start = Instant::now();
    println!("Producing public parameters...");
    let pp = PublicParams::<E1, E2, C1, TrivialCircuit<<E2 as Engine>::Scalar>>::setup(
        &circuit_primary,
        &circuit_secondary,
        &*S1::ck_floor(),
        &*S2::ck_floor(),
    );
    println!("PublicParams::setup, took {:?} ", start.elapsed());

    println!(
        "Number of constraints per step (primary circuit): {}",
        pp.num_constraints().0
    );
    println!(
        "Number of constraints per step (secondary circuit): {}",
        pp.num_constraints().1
    );

    println!(
        "Number of variables per step (primary circuit): {}",
        pp.num_variables().0
    );
    println!(
        "Number of variables per step (secondary circuit): {}",
        pp.num_variables().1
    );

    let z0_secondary = vec![<E2 as Engine>::Scalar::zero()];

    // type C1 = SCW;
    type C2 = TrivialCircuit<<E2 as Engine>::Scalar>;
    // produce a recursive SNARK
    println!("Generating a RecursiveSNARK...");
    let mut recursive_snark: RecursiveSNARK<E1, E2, C1, C2> =
        RecursiveSNARK::<E1, E2, C1, C2>::new(
            &pp,
            &circuits[0],
            &circuit_secondary,
            &z0_primary,
            &z0_secondary,
        )
        .unwrap();

    for (i, circuit_primary) in circuits.iter().enumerate() {
        let start = Instant::now();
        let res = recursive_snark.prove_step(&pp, circuit_primary, &circuit_secondary);
        assert!(res.is_ok());
        println!(
            "RecursiveSNARK::prove_step {}: {:?}, took {:?} ",
            i,
            res.is_ok(),
            start.elapsed()
        );
    }

    // verify the recursive SNARK
    println!("Verifying a RecursiveSNARK...");
    let start = Instant::now();
    let res = recursive_snark.verify(&pp, num_steps, &z0_primary, &z0_secondary);
    println!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res.is_ok(),
        start.elapsed()
    );
    assert!(res.is_ok());
}
