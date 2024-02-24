use std::marker::PhantomData;

use bellpepper_core::{num::AllocatedNum, boolean::AllocatedBit, ConstraintSystem, SynthesisError, LinearCombination};
use crate::circuits::utility::{func::{num_to_le_bit_nums, alloc_num_equals, print_nums}, wrapper::StepCircuitWrapper};
use ff::{derive::bitvec::store::BitStore, Field, PrimeField, PrimeFieldBits};
use nova_snark::traits::{circuit::StepCircuit, Group, PrimeFieldExt};
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct {{ rs_circuit_struct_name }}<G: Group> {
    pub _phantom: PhantomData<G>,
    pub auxiliary_variables: [u64; {{ rs_num_auxiliary_variables }}]
}

impl<G: Group> {{ rs_circuit_struct_name }}<G> {
    const ZERO: G::Scalar = G::Scalar::ZERO;
    const ONE: G::Scalar = G::Scalar::ONE;
}

impl<G: Group> StepCircuit<G::Scalar> for {{ rs_circuit_struct_name }}<G> {
    fn arity(&self) -> usize {
        {{ rs_arity }}
    }

    fn synthesize<CS: ConstraintSystem<G::Scalar>>(
        &self,
        cs: &mut CS,
        z: &[AllocatedNum<G::Scalar>],
    ) -> Result<Vec<AllocatedNum<G::Scalar>>, SynthesisError> {
        let mut nums: Vec<AllocatedNum<G::Scalar>> = z.to_vec();
        for auxiliary_variable in self.auxiliary_variables {
            let allocated_auxiliary_variable = AllocatedNum::alloc(cs.namespace(|| ""), || {
                Ok(G::Scalar::from(auxiliary_variable))
            })?;
            nums.push(allocated_auxiliary_variable);
        }
        print_nums("inputs", &nums);

{{ rs_synthesize }}

        let z_out = nums[{{ rs_first_output_idx }}..].to_vec()
            .into_iter()
            .map(|num| num.clone())
            .collect::<Vec<_>>();
        print_nums("output", &z_out);
        Ok(z_out)
    }
}

impl<G: Group> StepCircuitWrapper<G> for {{ rs_circuit_struct_name }}<G> {}
