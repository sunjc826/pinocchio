use std::marker::PhantomData;

use bellpepper_core::{num::AllocatedNum, ConstraintSystem, SynthesisError};
use ff::Field;
use nova_snark::traits::{circuit::StepCircuit, Group};
use num_bigint::BigUint;

use super::wrapper::StepCircuitWrapper;

#[derive(Clone, Debug)]
pub struct BloomFilterCircuit<G: Group> {
    
    _phantom: PhantomData<G>,
}

impl<G: Group> StepCircuit<G::Scalar> for BloomFilterCircuit<G> {
    fn arity(&self) -> usize {
        
    }

    fn synthesize<CS: ConstraintSystem<G::Scalar>>(
        &self,
        cs: &mut CS,
        z: &[AllocatedNum<G::Scalar>],
    ) -> Result<Vec<AllocatedNum<G::Scalar>>, SynthesisError> {
        
    }
}