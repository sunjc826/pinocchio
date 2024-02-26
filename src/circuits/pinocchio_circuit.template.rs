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

    pub fn make_circuit_primary() -> {{ rs_circuit_struct_name }}<G> {
        {{ rs_circuit_struct_name }} {
            _phantom: PhantomData,
            auxiliary_variables: [0; {{ rs_num_auxiliary_variables }}],
        }
    }

    pub fn make_z0_primary_all_zero() -> Vec<G::Scalar> {
        vec![G::Scalar::from(0 as u64); {{ rs_arity }}]
    }

    // The following function only accepts u64 as each input element (for simplicity).
    // Usually, the fields are large so z0_primary can actually consist of much larger numbers. 
    pub fn make_z0_primary(initial_public_inputs: [u64; {{ rs_arity }}]) -> Vec<G::Scalar> {
        initial_public_inputs
            .into_iter()
            .map(|v| G::Scalar::from(v))
            .collect::<Vec<_>>()
    }

    pub fn make_circuits(auxiliary_inputs: Vec<[u64; {{ rs_num_auxiliary_variables }}]>) -> Vec<{{ rs_circuit_struct_name }}<G>> {
        auxiliary_inputs
        .into_iter()
        .map(|auxiliary_input| {{ rs_circuit_struct_name }} {
            _phantom: PhantomData,
            auxiliary_variables: auxiliary_input,
        })
        .collect::<Vec<_>>()
    }
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
