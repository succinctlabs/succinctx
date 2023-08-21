
use plonky2::iop::witness::PartialWitness;
use plonky2::hash::hash_types::RichField;
use plonky2::field::extension::Extendable;
use ethers::types::H256;
use plonky2::plonk::config::GenericConfig;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::plonk::circuit_data::CircuitData;

pub struct CircuitBuild<F: RichField + Extendable<D>, const D: usize, C: GenericConfig<D, F=F>> {
    pub circuit_data: CircuitData<F, C, D>,
}

impl <F: RichField + Extendable<D>, const D: usize, C: GenericConfig<D, F=F>> CircuitBuild<F, D, C> {
    // TODO export circuit build to a file
    fn export() -> Vec<u8> {
        unimplemented!()
    }

    // TODO import a circuit build to a file
    fn import(bytes: Vec<u8>) -> Self {
        unimplemented!()
    }
}