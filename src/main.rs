mod aux_vector_clock;
mod circuits;
mod ivc_increment;
mod minroot;
mod runner;
mod types;

fn main() {
    ivc_increment::run();
}

#[cfg(test)]
mod tests {
    use crate::types::E1;
    use ff::{Field, PrimeFieldBits};
    use nova_snark::traits::{Engine, Group};
    type GroupElement = <E1 as Engine>::GE;
    type FieldElement = <GroupElement as Group>::Scalar;

    #[test]
    fn group_test() {
        let fe = FieldElement::ONE;
        let bits = fe.to_le_bits();
        println!("{fe:?} {bits}");

        // binary rep of 14 is 1110
        // i.e. little endian would be 0111
        let fe = FieldElement::from(14);
        let bits = fe.to_le_bits();
        println!("{fe:?} {bits}");

        // binary rep of 46 is           0010,1110 (most significant bit to the left)
        // reverse the bytes that to get 1110,0010 (most significant bit to the right)
        // then make this little endian  0111,0100
        let fe = FieldElement::from(46);
        let bits = fe.to_le_bits();
        println!("{fe:?} {bits}");

        let fe = FieldElement::from([1, 1, 1, 1]);
        let bits = fe.to_le_bits();
        println!("{fe:?} {bits}");

        // MODULUS of Fr
        // Note: This is 32 *bytes*! i.e. 256 bits
        let fe = FieldElement::from([
            0x43e1f593f0000001,
            0x2833e84879b97091,
            0xb85045b68181585d,
            0x30644e72e131a029,
        ]);
        let bits = fe.to_le_bits();
        println!("{fe:?} {bits}"); // this should be 0
    }

    #[test]
    fn additive_inverse_test() {
        let fe = FieldElement::from(46);
        let bits = fe.to_le_bits();
        println!("{fe:?} {bits}");

        let neg_fe = -fe;
        let sum = fe + neg_fe;
        println!("{sum:?}");

        let one = FieldElement::from(1);
        let neg_one = -one;
        let sum = fe + neg_one * fe;
        println!("{sum:?}");
    }

    #[test]
    fn twos_complement_test() {
        // 32-bit two's complement negative one
        let m: u64 = 4294967296;
        let neg_one: u64 = 4294967295;
        let num: u64 = 5;
        let sum = (num + neg_one * num) % m;
        println!("{sum}");

        let fe_neg_one = FieldElement::from(neg_one);
        let fe_num = FieldElement::from(num);
        let fe_sum = fe_num + fe_neg_one * fe_num;
        // note that the first 4 bytes are 0
        println!("{fe_sum:?}");
    }
}
