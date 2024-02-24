use nova_snark::traits::{circuit::StepCircuit, Group};
pub trait StepCircuitWrapper<G: Group>: StepCircuit<G::Scalar> {}
