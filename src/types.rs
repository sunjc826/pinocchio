use nova_snark::provider::{hyperkzg::Bn256EngineKZG, GrumpkinEngine};
pub type E1 = Bn256EngineKZG;
pub type E2 = GrumpkinEngine;
pub type EE1 = nova_snark::provider::hyperkzg::EvaluationEngine<E1>;
pub type EE2 = nova_snark::provider::ipa_pc::EvaluationEngine<E2>;
pub type S1 = nova_snark::spartan::snark::RelaxedR1CSSNARK<E1, EE1>; // non-preprocessing SNARK
pub type S2 = nova_snark::spartan::snark::RelaxedR1CSSNARK<E2, EE2>; // non-preprocessing SNARK
