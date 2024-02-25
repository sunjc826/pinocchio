use std::marker::PhantomData;

use crate::circuits::utility::{
    func::{alloc_num_equals, num_to_le_bit_nums, print_nums},
    wrapper::StepCircuitWrapper,
};
use bellpepper_core::{
    boolean::AllocatedBit, num::AllocatedNum, ConstraintSystem, LinearCombination, SynthesisError,
};
use ff::{derive::bitvec::store::BitStore, Field, PrimeField, PrimeFieldBits};
use nova_snark::traits::{circuit::StepCircuit, Group, PrimeFieldExt};
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct IvcIncrementCircuit<G: Group> {
    pub _phantom: PhantomData<G>,
    pub auxiliary_variables: [u64; 0],
}

impl<G: Group> IvcIncrementCircuit<G> {
    const ZERO: G::Scalar = G::Scalar::ZERO;
    const ONE: G::Scalar = G::Scalar::ONE;
}

impl<G: Group> StepCircuit<G::Scalar> for IvcIncrementCircuit<G> {
    fn arity(&self) -> usize {
        1
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

        // Pinocchio: ArithmeticInputBus
        // Pinocchio: OneBus

        let constant_1 =
            AllocatedNum::alloc(cs.namespace(|| "constant_1"), || Ok(G::Scalar::from(1)))?;

        nums.push(constant_1.clone());
        // Pinocchio: ArithZero

        let constant_0 =
            AllocatedNum::alloc(cs.namespace(|| "constant_0"), || Ok(G::Scalar::from(0)))?;

        nums.push(constant_0.clone());
        // Pinocchio: ConstantArithmeticBus
        nums.push(nums[1].mul(cs.namespace(|| ""), &constant_1)?);
        // Pinocchio: ArithAddBus
        nums.push(nums[0].add(cs.namespace(|| ""), &nums[3])?);
        // Pinocchio: ArithmeticOutputBus
        nums.push(nums[1].mul(cs.namespace(|| ""), &nums[4])?);

        let z_out = nums[5..]
            .to_vec()
            .into_iter()
            .map(|num| num.clone())
            .collect::<Vec<_>>();
        print_nums("output", &z_out);
        Ok(z_out)
    }
}

impl<G: Group> StepCircuitWrapper<G> for IvcIncrementCircuit<G> {}
