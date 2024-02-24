use crate::circuits::minroot::{MinRootCircuit, MinRootIteration};
use crate::runner;
use crate::types::E1;
use nova_snark::traits::Engine;
pub(crate) fn run() {
    let num_steps = 10;
    let num_iters_per_step = 1024;
    // number of iterations of MinRoot per Nova's recursive step
    let circuit_primary: MinRootCircuit<<E1 as Engine>::GE> = MinRootCircuit {
        seq: vec![
            MinRootIteration {
                x_i: <E1 as Engine>::Scalar::zero(),
                y_i: <E1 as Engine>::Scalar::zero(),
                x_i_plus_1: <E1 as Engine>::Scalar::zero(),
                y_i_plus_1: <E1 as Engine>::Scalar::zero(),
            };
            num_iters_per_step
        ],
    };

    // produce non-deterministic advice
    let (z0_primary, minroot_iterations) = MinRootIteration::<<E1 as Engine>::GE>::new(
        num_iters_per_step * num_steps,
        &<E1 as Engine>::Scalar::zero(),
        &<E1 as Engine>::Scalar::one(),
    );
    let minroot_circuits: Vec<MinRootCircuit<<E1 as Engine>::GE>> = (0..num_steps)
        .map(|i| MinRootCircuit {
            seq: (0..num_iters_per_step)
                .map(|j| MinRootIteration {
                    x_i: minroot_iterations[i * num_iters_per_step + j].x_i,
                    y_i: minroot_iterations[i * num_iters_per_step + j].y_i,
                    x_i_plus_1: minroot_iterations[i * num_iters_per_step + j].x_i_plus_1,
                    y_i_plus_1: minroot_iterations[i * num_iters_per_step + j].y_i_plus_1,
                })
                .collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();

    runner::run(
        "Nova-based VDF with MinRoot delay function",
        circuit_primary,
        minroot_circuits,
        &z0_primary,
    );
}
