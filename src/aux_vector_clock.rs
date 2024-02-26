use crate::circuits::aux_vector_clock::AuxVectorClockCircuit;
use crate::runner;
use crate::types::E1;
use nova_snark::traits::Engine;

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
    let auxiliary_inputs: Vec<[u64; 6]> = vec![
        [0; 6],                   // increment idx 0 (1, 0, 0, 0)
        [0; 6],                   // increment idx 0 (2, 0, 0, 0)
        [0, 2, 0, 0, 0, 0],       // increment idx 2 (2, 0, 1, 0)
        [1, 0, 1, 3, 0, 9], // merge                 (3, 3, 1, 9); Note that idx 0 is the message receiver and thus next_clock = own_clock + 1
        [0, 3, 100, 42, 100, 42], // increment idx 3 (3, 3, 1, 10); Note that other_clock has garbage values
        [2, 1, 2, 3, 4, 5], // no-op                 (3, 3, 1, 10); Note that increment_idx, other_clock have garbage values
    ];
    runner::run(
        "AuxVectorClockCircuit",
        AuxVectorClockCircuit::make_circuit_primary(),
        AuxVectorClockCircuit::make_circuits(auxiliary_inputs),
        &AuxVectorClockCircuit::<<E1 as Engine>::GE>::make_z0_primary_all_zero(),
    );
}
