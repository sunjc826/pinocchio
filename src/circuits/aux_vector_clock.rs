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
pub struct AuxVectorClockCircuit<G: Group> {
    pub _phantom: PhantomData<G>,
    pub auxiliary_variables: [u64; 6],
}

impl<G: Group> AuxVectorClockCircuit<G> {
    const ZERO: G::Scalar = G::Scalar::ZERO;
    const ONE: G::Scalar = G::Scalar::ONE;

    pub fn make_circuit_primary() -> AuxVectorClockCircuit<G> {
        AuxVectorClockCircuit {
            _phantom: PhantomData,
            auxiliary_variables: [0; 6],
        }
    }

    pub fn make_z0_primary_all_zero() -> Vec<G::Scalar> {
        vec![G::Scalar::from(0 as u64); 4]
    }

    // The following function only accepts u64 as each input element (for simplicity).
    // Usually, the fields are large so z0_primary can actually consist of much larger numbers.
    pub fn make_z0_primary(initial_public_inputs: [u64; 4]) -> Vec<G::Scalar> {
        initial_public_inputs
            .into_iter()
            .map(|v| G::Scalar::from(v))
            .collect::<Vec<_>>()
    }

    pub fn make_circuits(auxiliary_inputs: Vec<[u64; 6]>) -> Vec<AuxVectorClockCircuit<G>> {
        auxiliary_inputs
            .into_iter()
            .map(|auxiliary_input| AuxVectorClockCircuit {
                _phantom: PhantomData,
                auxiliary_variables: auxiliary_input,
            })
            .collect::<Vec<_>>()
    }
}

impl<G: Group> StepCircuit<G::Scalar> for AuxVectorClockCircuit<G> {
    fn arity(&self) -> usize {
        4
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
        // Pinocchio: ArithmeticInputBus
        // Pinocchio: ArithmeticInputBus
        // Pinocchio: ArithmeticInputBus
        // Pinocchio: ArithmeticInputBus
        // Pinocchio: ArithmeticInputBus
        // Pinocchio: ArithmeticInputBus
        // Pinocchio: ArithmeticInputBus
        // Pinocchio: ArithmeticInputBus
        // Pinocchio: ArithmeticInputBus
        // Pinocchio: OneBus

        let constant_1 =
            AllocatedNum::alloc(cs.namespace(|| "constant_1"), || Ok(G::Scalar::from(1)))?;

        nums.push(constant_1.clone());
        // Pinocchio: ArithZero

        let constant_0 =
            AllocatedNum::alloc(cs.namespace(|| "constant_0"), || Ok(G::Scalar::from(0)))?;

        nums.push(constant_0.clone());
        // Pinocchio: ConstantMultiplyBus

        let constant_4294967295 =
            AllocatedNum::alloc(cs.namespace(|| "constant_4294967295"), || {
                Ok(G::Scalar::from(4294967295))
            })?;

        nums.push(nums[0].mul(cs.namespace(|| ""), &constant_4294967295)?);
        // Pinocchio: ArithAddBus
        nums.push(nums[6].add(cs.namespace(|| ""), &nums[12])?);
        // Pinocchio: SplitBus

        let multiple_nums = num_to_le_bit_nums(cs, 65, &nums[13])?;
        nums.extend(multiple_nums);

        // Pinocchio: LeftShiftBus
        // Pinocchio: JoinBus

        match nums[45].get_value() {
            Some(v) => println!("Comparison result nums[45] = {v:?}"),
            None => (),
        };

        // Pinocchio: SplitBus
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[45].mul(cs.namespace(|| ""), &nums[0])?);

        let constant_neg_1 =
            AllocatedNum::alloc(cs.namespace(|| "constant_-1"), || Ok(-G::Scalar::from(1)))?;

        nums.push(nums[45].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[80])?);
        nums.push(nums[81].mul(cs.namespace(|| ""), &nums[6])?);
        nums.push(nums[79].add(cs.namespace(|| ""), &nums[82])?);
        // Pinocchio: ConstantArithmeticBus
        nums.push(nums[10].mul(cs.namespace(|| ""), &constant_1)?);
        // Pinocchio: ArithAddBus
        nums.push(nums[83].add(cs.namespace(|| ""), &nums[84])?);
        // Pinocchio: ConstantArithmeticBus
        nums.push(nums[10].mul(cs.namespace(|| ""), &constant_0)?);
        // Pinocchio: ArithmeticZeroPBus
        nums.push(nums[5].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[86].add(cs.namespace(|| ""), &nums[87])?);

        let (y, m) = alloc_num_equals(cs, &nums[88])?;
        nums.push(y);
        nums.push(m);

        nums.push(nums[89].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[91])?);
        // Pinocchio: SplitBus
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[92].mul(cs.namespace(|| ""), &nums[85])?);
        nums.push(nums[92].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[94])?);
        nums.push(nums[95].mul(cs.namespace(|| ""), &nums[83])?);
        nums.push(nums[93].add(cs.namespace(|| ""), &nums[96])?);
        // Pinocchio: ArithmeticZeroPBus
        nums.push(nums[84].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[4].add(cs.namespace(|| ""), &nums[98])?);

        let (y, m) = alloc_num_equals(cs, &nums[99])?;
        nums.push(y);
        nums.push(m);

        nums.push(nums[100].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[102])?);
        // Pinocchio: SplitBus
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[103].mul(cs.namespace(|| ""), &nums[97])?);
        nums.push(nums[103].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[105])?);
        nums.push(nums[106].mul(cs.namespace(|| ""), &nums[0])?);
        nums.push(nums[104].add(cs.namespace(|| ""), &nums[107])?);
        // Pinocchio: ArithAddBus
        nums.push(nums[0].add(cs.namespace(|| ""), &nums[84])?);
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[92].mul(cs.namespace(|| ""), &nums[109])?);
        nums.push(nums[92].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[111])?);
        nums.push(nums[112].mul(cs.namespace(|| ""), &nums[0])?);
        nums.push(nums[110].add(cs.namespace(|| ""), &nums[113])?);
        // Pinocchio: ArithmeticZeroPBus
        nums.push(nums[86].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[4].add(cs.namespace(|| ""), &nums[115])?);

        let (y, m) = alloc_num_equals(cs, &nums[116])?;
        nums.push(y);
        nums.push(m);

        nums.push(nums[117].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[119])?);
        // Pinocchio: SplitBus
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[120].mul(cs.namespace(|| ""), &nums[114])?);
        nums.push(nums[120].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[122])?);
        nums.push(nums[123].mul(cs.namespace(|| ""), &nums[108])?);
        nums.push(nums[121].add(cs.namespace(|| ""), &nums[124])?);
        // Pinocchio: ConstantMultiplyBus
        nums.push(nums[1].mul(cs.namespace(|| ""), &constant_4294967295)?);
        // Pinocchio: ArithAddBus
        nums.push(nums[7].add(cs.namespace(|| ""), &nums[126])?);
        // Pinocchio: SplitBus

        let multiple_nums = num_to_le_bit_nums(cs, 65, &nums[127])?;
        nums.extend(multiple_nums);

        // Pinocchio: LeftShiftBus
        // Pinocchio: JoinBus

        match nums[159].get_value() {
            Some(v) => println!("Comparison result nums[159] = {v:?}"),
            None => (),
        };

        // Pinocchio: SplitBus
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[159].mul(cs.namespace(|| ""), &nums[1])?);
        nums.push(nums[159].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[194])?);
        nums.push(nums[195].mul(cs.namespace(|| ""), &nums[7])?);
        nums.push(nums[193].add(cs.namespace(|| ""), &nums[196])?);
        // Pinocchio: ArithAddBus
        nums.push(nums[197].add(cs.namespace(|| ""), &nums[84])?);
        // Pinocchio: ArithmeticZeroPBus
        nums.push(nums[5].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[84].add(cs.namespace(|| ""), &nums[199])?);

        let (y, m) = alloc_num_equals(cs, &nums[200])?;
        nums.push(y);
        nums.push(m);

        nums.push(nums[201].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[203])?);
        // Pinocchio: SplitBus
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[204].mul(cs.namespace(|| ""), &nums[198])?);
        nums.push(nums[204].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[206])?);
        nums.push(nums[207].mul(cs.namespace(|| ""), &nums[197])?);
        nums.push(nums[205].add(cs.namespace(|| ""), &nums[208])?);
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[103].mul(cs.namespace(|| ""), &nums[209])?);
        nums.push(nums[103].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[211])?);
        nums.push(nums[212].mul(cs.namespace(|| ""), &nums[1])?);
        nums.push(nums[210].add(cs.namespace(|| ""), &nums[213])?);
        // Pinocchio: ArithAddBus
        nums.push(nums[1].add(cs.namespace(|| ""), &nums[84])?);
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[204].mul(cs.namespace(|| ""), &nums[215])?);
        nums.push(nums[204].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[217])?);
        nums.push(nums[218].mul(cs.namespace(|| ""), &nums[1])?);
        nums.push(nums[216].add(cs.namespace(|| ""), &nums[219])?);
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[120].mul(cs.namespace(|| ""), &nums[220])?);
        nums.push(nums[120].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[222])?);
        nums.push(nums[223].mul(cs.namespace(|| ""), &nums[214])?);
        nums.push(nums[221].add(cs.namespace(|| ""), &nums[224])?);
        // Pinocchio: ConstantMultiplyBus
        nums.push(nums[2].mul(cs.namespace(|| ""), &constant_4294967295)?);
        // Pinocchio: ArithAddBus
        nums.push(nums[8].add(cs.namespace(|| ""), &nums[226])?);
        // Pinocchio: SplitBus

        let multiple_nums = num_to_le_bit_nums(cs, 65, &nums[227])?;
        nums.extend(multiple_nums);

        // Pinocchio: LeftShiftBus
        // Pinocchio: JoinBus

        match nums[259].get_value() {
            Some(v) => println!("Comparison result nums[259] = {v:?}"),
            None => (),
        };

        // Pinocchio: SplitBus
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[259].mul(cs.namespace(|| ""), &nums[2])?);
        nums.push(nums[259].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[294])?);
        nums.push(nums[295].mul(cs.namespace(|| ""), &nums[8])?);
        nums.push(nums[293].add(cs.namespace(|| ""), &nums[296])?);
        // Pinocchio: ArithAddBus
        nums.push(nums[297].add(cs.namespace(|| ""), &nums[84])?);
        // Pinocchio: ConstantArithmeticBus

        let constant_2 =
            AllocatedNum::alloc(cs.namespace(|| "constant_2"), || Ok(G::Scalar::from(2)))?;

        nums.push(nums[10].mul(cs.namespace(|| ""), &constant_2)?);
        // Pinocchio: ArithmeticZeroPBus
        nums.push(nums[5].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[299].add(cs.namespace(|| ""), &nums[300])?);

        let (y, m) = alloc_num_equals(cs, &nums[301])?;
        nums.push(y);
        nums.push(m);

        nums.push(nums[302].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[304])?);
        // Pinocchio: SplitBus
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[305].mul(cs.namespace(|| ""), &nums[298])?);
        nums.push(nums[305].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[307])?);
        nums.push(nums[308].mul(cs.namespace(|| ""), &nums[297])?);
        nums.push(nums[306].add(cs.namespace(|| ""), &nums[309])?);
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[103].mul(cs.namespace(|| ""), &nums[310])?);
        nums.push(nums[103].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[312])?);
        nums.push(nums[313].mul(cs.namespace(|| ""), &nums[2])?);
        nums.push(nums[311].add(cs.namespace(|| ""), &nums[314])?);
        // Pinocchio: ArithAddBus
        nums.push(nums[2].add(cs.namespace(|| ""), &nums[84])?);
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[305].mul(cs.namespace(|| ""), &nums[316])?);
        nums.push(nums[305].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[318])?);
        nums.push(nums[319].mul(cs.namespace(|| ""), &nums[2])?);
        nums.push(nums[317].add(cs.namespace(|| ""), &nums[320])?);
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[120].mul(cs.namespace(|| ""), &nums[321])?);
        nums.push(nums[120].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[323])?);
        nums.push(nums[324].mul(cs.namespace(|| ""), &nums[315])?);
        nums.push(nums[322].add(cs.namespace(|| ""), &nums[325])?);
        // Pinocchio: ConstantMultiplyBus
        nums.push(nums[3].mul(cs.namespace(|| ""), &constant_4294967295)?);
        // Pinocchio: ArithAddBus
        nums.push(nums[9].add(cs.namespace(|| ""), &nums[327])?);
        // Pinocchio: SplitBus

        let multiple_nums = num_to_le_bit_nums(cs, 65, &nums[328])?;
        nums.extend(multiple_nums);

        // Pinocchio: LeftShiftBus
        // Pinocchio: JoinBus

        match nums[360].get_value() {
            Some(v) => println!("Comparison result nums[360] = {v:?}"),
            None => (),
        };

        // Pinocchio: SplitBus
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[360].mul(cs.namespace(|| ""), &nums[3])?);
        nums.push(nums[360].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[395])?);
        nums.push(nums[396].mul(cs.namespace(|| ""), &nums[9])?);
        nums.push(nums[394].add(cs.namespace(|| ""), &nums[397])?);
        // Pinocchio: ArithAddBus
        nums.push(nums[398].add(cs.namespace(|| ""), &nums[84])?);
        // Pinocchio: ConstantArithmeticBus

        let constant_3 =
            AllocatedNum::alloc(cs.namespace(|| "constant_3"), || Ok(G::Scalar::from(3)))?;

        nums.push(nums[10].mul(cs.namespace(|| ""), &constant_3)?);
        // Pinocchio: ArithmeticZeroPBus
        nums.push(nums[5].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[400].add(cs.namespace(|| ""), &nums[401])?);

        let (y, m) = alloc_num_equals(cs, &nums[402])?;
        nums.push(y);
        nums.push(m);

        nums.push(nums[403].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[405])?);
        // Pinocchio: SplitBus
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[406].mul(cs.namespace(|| ""), &nums[399])?);
        nums.push(nums[406].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[408])?);
        nums.push(nums[409].mul(cs.namespace(|| ""), &nums[398])?);
        nums.push(nums[407].add(cs.namespace(|| ""), &nums[410])?);
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[103].mul(cs.namespace(|| ""), &nums[411])?);
        nums.push(nums[103].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[413])?);
        nums.push(nums[414].mul(cs.namespace(|| ""), &nums[3])?);
        nums.push(nums[412].add(cs.namespace(|| ""), &nums[415])?);
        // Pinocchio: ArithAddBus
        nums.push(nums[3].add(cs.namespace(|| ""), &nums[84])?);
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[406].mul(cs.namespace(|| ""), &nums[417])?);
        nums.push(nums[406].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[419])?);
        nums.push(nums[420].mul(cs.namespace(|| ""), &nums[3])?);
        nums.push(nums[418].add(cs.namespace(|| ""), &nums[421])?);
        // Pinocchio: ArithmeticConditionalBus
        nums.push(nums[120].mul(cs.namespace(|| ""), &nums[422])?);
        nums.push(nums[120].mul(cs.namespace(|| ""), &constant_neg_1)?);
        nums.push(nums[10].add(cs.namespace(|| ""), &nums[424])?);
        nums.push(nums[425].mul(cs.namespace(|| ""), &nums[416])?);
        nums.push(nums[423].add(cs.namespace(|| ""), &nums[426])?);
        // Pinocchio: ArithmeticOutputBus
        nums.push(nums[10].mul(cs.namespace(|| ""), &nums[125])?);
        // Pinocchio: ArithmeticOutputBus
        nums.push(nums[10].mul(cs.namespace(|| ""), &nums[225])?);
        // Pinocchio: ArithmeticOutputBus
        nums.push(nums[10].mul(cs.namespace(|| ""), &nums[326])?);
        // Pinocchio: ArithmeticOutputBus
        nums.push(nums[10].mul(cs.namespace(|| ""), &nums[427])?);

        let z_out = nums[428..]
            .to_vec()
            .into_iter()
            .map(|num| num.clone())
            .collect::<Vec<_>>();
        print_nums("output", &z_out);
        Ok(z_out)
    }
}

impl<G: Group> StepCircuitWrapper<G> for AuxVectorClockCircuit<G> {}
