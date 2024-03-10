use nova_snark::{
    provider::{hyperkzg::Bn256EngineKZG, GrumpkinEngine},
    traits::{Engine, Group},
};
pub type E1 = Bn256EngineKZG;
pub type E2 = GrumpkinEngine;
pub type G1 = <E1 as Engine>::GE;
pub type G2 = <E2 as Engine>::GE;
pub type F1 = <G1 as Group>::Scalar;
pub type F2 = <G2 as Group>::Scalar;
pub type EE1 = nova_snark::provider::hyperkzg::EvaluationEngine<E1>;
pub type EE2 = nova_snark::provider::ipa_pc::EvaluationEngine<E2>;
pub type S1 = nova_snark::spartan::snark::RelaxedR1CSSNARK<E1, EE1>; // non-preprocessing SNARK
pub type S2 = nova_snark::spartan::snark::RelaxedR1CSSNARK<E2, EE2>; // non-preprocessing SNARK
