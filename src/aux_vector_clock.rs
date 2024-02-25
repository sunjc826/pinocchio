use crate::circuits::aux_vector_clock::AuxVectorClockCircuit;
use crate::circuits::utility::func::print_nums;
use crate::runner;
use crate::types::E1;
use nova_snark::traits::Engine;
use std::marker::PhantomData;

/**
```c
    #define VECTOR_CLOCK_OP_INCREMENT 0
    #define VECTOR_CLOCK_OP_MERGE 1
    #define AuxiliaryInput \
        DOC(one of VECTOR_CLOCK_OP_XXX; otherwise no-op)\
        int operation_type;\
        DOC(for increment)\
        int increment_idx;\
        DOC(for merge)\
        int other_clock[VECTOR_CLOCK_SIZE];
```
*/
pub(crate) fn run() {
    let circuit_primary = AuxVectorClockCircuit {
        _phantom: PhantomData,
        auxiliary_variables: [0; 6],
    };
    let z0_primary = vec![
        <E1 as Engine>::Scalar::zero(),
        <E1 as Engine>::Scalar::zero(),
        <E1 as Engine>::Scalar::zero(),
        <E1 as Engine>::Scalar::zero(),
    ];

    println!("Initial State: {z0_primary:?}");

    let auxiliary_inputs: Vec<[u64; 6]> = vec![
        [0; 6],                   // increment idx 0 (1, 0, 0, 0)
        [0; 6],                   // increment idx 0 (2, 0, 0, 0)
        [0, 2, 0, 0, 0, 0],       // increment idx 2 (2, 0, 1, 0)
        [1, 0, 1, 3, 0, 9], // merge                 (3, 3, 1, 9); Note that idx 0 is the message receiver and thus next_clock = own_clock + 1
        [0, 3, 100, 42, 100, 42], // increment idx 3 (3, 3, 1, 10); Note that other_clock has garbage values
        [2, 1, 2, 3, 4, 5], // no-op                 (3, 3, 1, 10); Note that increment_idx, other_clock have garbage values
    ];

    let circuits = auxiliary_inputs
        .into_iter()
        .map(|auxiliary_input| AuxVectorClockCircuit {
            _phantom: PhantomData,
            auxiliary_variables: auxiliary_input,
        })
        .collect::<Vec<_>>();

    runner::run(
        "AuxVectorClockCircuit",
        circuit_primary,
        circuits,
        &z0_primary,
    );
}
