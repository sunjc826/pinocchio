use std::fmt;

use bellpepper_core::{
    boolean::AllocatedBit, num::AllocatedNum, ConstraintSystem, LinearCombination, SynthesisError,
};
use ff::{derive::bitvec::store::BitStore, Field, PrimeField, PrimeFieldBits};
use nova_snark::traits::{circuit::StepCircuit, Group, PrimeFieldExt};
// Adapted from Nova examples/and.rs
fn enforce_le_bits_to_num<Scalar, CS>(
    cs: &mut CS,
    bits: &[AllocatedBit],
    num: &AllocatedNum<Scalar>,
) where
    Scalar: PrimeField + PrimeFieldBits,
    CS: ConstraintSystem<Scalar>,
{
    let mut lc = LinearCombination::zero();
    let mut coeff = Scalar::ONE;
    for bit in bits.iter() {
        lc = lc + (coeff, bit.get_variable());
        coeff = coeff.double();
    }
    lc = lc - num.get_variable();
    cs.enforce(|| "compute number from bits", |lc| lc, |lc| lc, |_| lc);
}

fn num_to_le_bits<Scalar: PrimeFieldExt + PrimeFieldBits, CS: ConstraintSystem<Scalar>>(
    cs: &mut CS,
    num_active_bits: usize,
    num: &AllocatedNum<Scalar>,
) -> Result<Vec<AllocatedBit>, SynthesisError> {
    let field_value = num.get_value();
    let allocated_bits = match field_value {
        Some(field_value) => {
            let bits = field_value.to_le_bits();
            let bits_slice = &bits[0..num_active_bits];
            bits_slice
                .into_iter()
                .enumerate()
                .map(|(i, b)| AllocatedBit::alloc(cs.namespace(|| format!("bit {}", i)), Some(*b)))
                .collect::<Result<Vec<_>, SynthesisError>>()?
        }
        None => (0..num_active_bits)
            .into_iter()
            .map(|i| AllocatedBit::alloc(cs.namespace(|| format!("bit {}", i)), None))
            .collect::<Result<Vec<_>, SynthesisError>>()?,
    };
    enforce_le_bits_to_num(cs, &allocated_bits, &num);
    Ok(allocated_bits)
}

pub fn num_to_le_bit_nums<Scalar: PrimeFieldExt + PrimeFieldBits, CS: ConstraintSystem<Scalar>>(
    cs: &mut CS,
    num_active_bits: usize,
    num: &AllocatedNum<Scalar>,
) -> Result<Vec<AllocatedNum<Scalar>>, SynthesisError> {
    let allocated_bits = num_to_le_bits(cs, num_active_bits, num)?;
    let mut vec = Vec::new();
    for (i, allocated_bit) in allocated_bits.into_iter().enumerate() {
        let bit = allocated_bit.get_value();
        let allocated_num = AllocatedNum::alloc(cs.namespace(|| format!("{i}")), || match bit {
            Some(bit) => Ok(Scalar::from(bit as u64)),
            None => Err(SynthesisError::AssignmentMissing),
        })?;
        cs.enforce(
            || format!("bit equals num {i}"),
            |lc| lc,
            |lc| lc,
            |lc| lc + allocated_bit.get_variable() - allocated_num.get_variable(),
        );
        vec.push(allocated_num);
    }
    Ok(vec)
}

pub fn alloc_num_equals<Scalar: PrimeFieldExt + PrimeFieldBits, CS: ConstraintSystem<Scalar>>(
    cs: &mut CS,
    x: &AllocatedNum<Scalar>,
) -> Result<(AllocatedNum<Scalar>, AllocatedNum<Scalar>), SynthesisError> {
    let value = x.get_value();
    let r_value = value.map(|v| v == Scalar::ZERO);
    let y_value = r_value.map(|v| !v);

    let y_bit = AllocatedBit::alloc(cs.namespace(|| "y bit"), y_value)?;

    let t = AllocatedNum::alloc(cs.namespace(|| "t"), || {
        value
            .map(|value| {
                if value == Scalar::ZERO {
                    Scalar::ONE
                } else {
                    value.invert().unwrap()
                }
            })
            .map_or(Err(SynthesisError::AssignmentMissing), |value| Ok(value))
    })?;

    let m = t;

    cs.enforce(
        || "m * (a - b) = t * (a - b) = 1 - r = y",
        |lc| lc + m.get_variable(),
        |lc| lc + x.get_variable(),
        |lc| lc + y_bit.get_variable(),
    );

    cs.enforce(
        || "(1 - y) * (a - b) = r * (a - b) = 0",
        |lc| lc + CS::one() - y_bit.get_variable(),
        |lc| lc + x.get_variable(),
        |lc| lc,
    );

    let y = AllocatedNum::alloc(cs.namespace(|| "y"), || {
        y_bit
            .get_value()
            .map_or(Err(SynthesisError::AssignmentMissing), |v| {
                Ok(Scalar::from(v as u64))
            })
    })?;
    cs.enforce(
        || format!("bit equals num"),
        |lc| lc,
        |lc| lc,
        |lc| lc + y_bit.get_variable() - y.get_variable(),
    );
    Ok((y, m))
}

pub fn print_nums<Scalar: fmt::Debug + PrimeFieldExt + PrimeFieldBits>(
    desc: &str,
    nums: &Vec<AllocatedNum<Scalar>>,
) {
    if nums.is_empty() {
        return;
    }

    if nums.first().unwrap().get_value().is_none() {
        return;
    }

    println!("{desc} = (");
    nums.iter()
        .map(|num| num.get_value().unwrap())
        .for_each(|v| println!("{v:?},"));
    println!(")");
}
