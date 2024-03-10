// This is adapted from the nova-snark package's min_root example.
// We don't care too much about compression for the moment,
// so the parts related to CompressedSNARK are removed.
// This file provides a somewhat unified/simplified way to operate on a StepCircuit impl.

use crate::{circuits::utility::wrapper::StepCircuitWrapper, types::*};
use nova_snark::{
    traits::{circuit::TrivialCircuit, snark::RelaxedR1CSSNARKTrait, Engine},
    PublicParams, RecursiveSNARK,
};
use std::{fmt, time::Instant};

pub(crate) fn run<C1: StepCircuitWrapper<<E1 as Engine>::GE>>(
    description: &str,
    circuit_primary: C1,
    circuits: Vec<C1>,
    z0_primary: &[<E1 as Engine>::Scalar],
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

use std::{collections::HashMap, env::current_dir};

use nova_scotia::{
    circom::reader::load_r1cs, continue_recursive_circuit, create_public_params,
    create_recursive_circuit, FileLocation,
};
use serde_json::Value;

pub fn print_nums<Scalar: fmt::Debug>(desc: &str, nums: &Vec<Scalar>) {
    println!("{desc} = (");
    nums.iter().for_each(|v| println!("{v:?},"));
    println!(")");
}

pub(crate) fn run_circom(
    circuit_filepath: &str,
    witness_gen_filepath: &str,
    z0_primary: &[<E1 as Engine>::Scalar],
    private_inputs: Vec<HashMap<String, serde_json::Value>>,
) {
    println!(
        "Running test with witness generator: {} and group: {}",
        witness_gen_filepath,
        std::any::type_name::<G1>()
    );
    let iteration_count = private_inputs.len();
    let root = current_dir().unwrap();

    let circuit_file = root.join(circuit_filepath);
    let r1cs = load_r1cs::<G1, G2>(&FileLocation::PathBuf(circuit_file));
    let witness_generator_file = root.join(witness_gen_filepath);

    // let start_public_input = [F1::from(10), F1::from(10)];

    let pp: PublicParams<E1, E2, _, _> = create_public_params(r1cs.clone());

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

    println!("Creating a RecursiveSNARK...");
    let start = Instant::now();
    let recursive_snark = create_recursive_circuit(
        FileLocation::PathBuf(witness_generator_file.clone()),
        r1cs.clone(),
        private_inputs,
        z0_primary.to_vec(),
        &pp,
    )
    .unwrap();
    println!("RecursiveSNARK creation took {:?}", start.elapsed());

    let z0_secondary = [F2::from(0)];

    // verify the recursive SNARK
    println!("Verifying a RecursiveSNARK...");
    let start = Instant::now();
    let res = recursive_snark.verify(&pp, iteration_count, z0_primary, &z0_secondary);
    println!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res.is_ok(),
        start.elapsed()
    );
    assert!(res.is_ok());
    let z_last = res.unwrap().0;
    print_nums("Final result", &z_last);

    // continue recursive circuit by adding 2 further steps
    // println!("Adding steps to our RecursiveSNARK...");
    // let start = Instant::now();

    // let iteration_count_continue = 2;

    // let mut private_inputs_continue = Vec::new();
    // for i in 0..iteration_count_continue {
    //     let mut private_input = HashMap::new();
    //     private_input.insert("adder".to_string(), json!(5 + i));
    //     private_inputs_continue.push(private_input);
    // }

    // let res = continue_recursive_circuit(
    //     &mut recursive_snark,
    //     z_last,
    //     FileLocation::PathBuf(witness_generator_file),
    //     r1cs,
    //     private_inputs_continue,
    //     z0_primary.to_vec(),
    //     &pp,
    // );
    // assert!(res.is_ok());
    // println!(
    //     "Adding 2 steps to our RecursiveSNARK took {:?}",
    //     start.elapsed()
    // );

    // verify the recursive SNARK with the added steps
    // println!("Verifying a RecursiveSNARK...");
    // let start = Instant::now();
    // let res = recursive_snark.verify(
    //     &pp,
    //     iteration_count + iteration_count_continue,
    //     z0_primary,
    //     &z0_secondary,
    // );
    // println!(
    //     "RecursiveSNARK::verify: {:?}, took {:?}",
    //     res,
    //     start.elapsed()
    // );
    // assert!(res.is_ok());

    // assert_eq!(res.clone().unwrap().0[0], F1::from(31));
    // assert_eq!(res.unwrap().0[1], F1::from(115));
}
