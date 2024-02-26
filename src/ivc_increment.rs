use crate::circuits::ivc_increment::IvcIncrementCircuit;
use crate::runner;
use crate::types::E1;
use nova_snark::traits::Engine;

/**
```c
    #define Shared \
        int counter;
```
*/
pub(crate) fn run() {
    runner::run(
        "IvcIncrementCircuit",
        IvcIncrementCircuit::make_circuit_primary(),
        IvcIncrementCircuit::make_circuits(vec![[]; 10]),
        &IvcIncrementCircuit::<<E1 as Engine>::GE>::make_z0_primary_all_zero(),
    );
}
