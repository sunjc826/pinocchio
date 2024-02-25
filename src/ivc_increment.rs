use crate::circuits::ivc_increment::IvcIncrementCircuit;
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
    let circuit_primary = IvcIncrementCircuit {
        _phantom: PhantomData,
        auxiliary_variables: [0; 0],
    };
    let z0_primary = vec![<E1 as Engine>::Scalar::zero()];

    println!("Initial State: {z0_primary:?}");

    let circuits = vec![
        IvcIncrementCircuit {
            _phantom: PhantomData,
            auxiliary_variables: [0; 0],
        };
        10
    ];

    runner::run(
        "IvcIncrementCircuit",
        circuit_primary,
        circuits,
        &z0_primary,
    );
}
