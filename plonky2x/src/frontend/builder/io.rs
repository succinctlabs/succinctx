use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputsTarget;

use super::CircuitBuilder;
use crate::backend::circuit::Circuit;
use crate::frontend::vars::EvmVariable;
use crate::prelude::{ByteVariable, CircuitVariable, Variable};

/// Stores circuit variables used for reading and writing data to the EVM.
#[derive(Debug, Clone)]
pub struct EvmIO {
    pub input_bytes: Vec<ByteVariable>,
    pub output_bytes: Vec<ByteVariable>,
}

/// Stores circuit variable used for reading and writing data using field elements.
#[derive(Debug, Clone)]
pub struct FieldIO {
    pub input_variables: Vec<Variable>,
    pub output_variables: Vec<Variable>,
}

/// Stores circuit variables used for recursive proof verification.
#[derive(Debug, Clone)]
pub struct RecursiveProofsIO<const D: usize> {
    pub proofs: Vec<ProofWithPublicInputsTarget<D>>,
    pub child_circuit_ids: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CircuitIO<const D: usize> {
    pub evm: Option<EvmIO>,
    pub field: Option<FieldIO>,
    pub recursive_proof: Option<RecursiveProofsIO<D>>,
}

impl<const D: usize> CircuitIO<D> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            evm: None,
            field: None,
            recursive_proof: None,
        }
    }
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    fn init_field_io(&mut self) {
        if self.io.evm.is_some() {
            panic!("cannot use field io and other io methods at the same time")
        } else if self.io.field.is_none() {
            self.io.field = Some(FieldIO {
                input_variables: Vec::new(),
                output_variables: Vec::new(),
            })
        }
    }

    fn init_evm_io(&mut self) {
        if self.io.field.is_some() {
            panic!("cannot use evm io and other io methods at the same time")
        } else if self.io.evm.is_none() {
            self.io.evm = Some(EvmIO {
                input_bytes: Vec::new(),
                output_bytes: Vec::new(),
            })
        }
    }

    fn init_proof_io(&mut self) {
        if self.io.recursive_proof.is_none() {
            self.io.recursive_proof = Some(RecursiveProofsIO {
                proofs: Vec::new(),
                child_circuit_ids: Vec::new(),
            })
        }
    }

    pub fn read<V: CircuitVariable>(&mut self) -> V {
        self.init_field_io();
        let variable = self.init::<V>();
        match self.io.field {
            Some(ref mut io) => io.input_variables.extend(variable.variables()),
            None => panic!("cannot read from field io"),
        }
        variable
    }

    pub fn evm_read<V: EvmVariable>(&mut self) -> V {
        self.init_evm_io();
        let nb_bytes = V::nb_bytes::<F, D>();
        let mut bytes = Vec::new();
        for _ in 0..nb_bytes {
            bytes.push(self.init::<ByteVariable>());
        }
        let variable = V::decode(self, bytes.as_slice());
        match self.io.evm {
            Some(ref mut io) => io.input_bytes.extend(bytes),
            None => panic!("cannot read from field io"),
        }
        variable
    }

    pub fn proof_read<C>(&mut self, cd: &Circuit<F, C, D>) -> ProofWithPublicInputsTarget<D>
    where
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        self.init_proof_io();
        let proof = self.add_virtual_proof_with_pis(&cd.data.common);
        self.io
            .recursive_proof
            .as_mut()
            .unwrap()
            .proofs
            .push(proof.clone());
        self.io
            .recursive_proof
            .as_mut()
            .unwrap()
            .child_circuit_ids
            .push(cd.id());

        proof
    }

    pub fn write<V: CircuitVariable>(&mut self, variable: V) {
        self.init_field_io();
        match self.io.field {
            Some(ref mut io) => io.output_variables.extend(variable.variables()),
            None => panic!("cannot write to field io"),
        }
    }

    pub fn evm_write<V: EvmVariable>(&mut self, variable: V) {
        self.init_evm_io();
        let bytes = variable.encode(self);
        match self.io.evm {
            Some(ref mut io) => io.output_bytes.extend(bytes),
            None => panic!("cannot write to evm io"),
        }
    }
}
