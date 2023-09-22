use core::marker::PhantomData;

use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::field::types::Field;
use plonky2::gates::gate::Gate;
use plonky2::gates::util::StridedConstraintConsumer;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::ext_target::ExtensionTarget;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator, WitnessGeneratorRef};
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::plonk::vars::{EvaluationTargets, EvaluationVars, EvaluationVarsBase};
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
pub mod u8;
pub mod util;

use util::{biguint_to_bits_target, bits_to_biguint_target};

use crate::frontend::num::biguint::CircuitBuilderBiguint;
use crate::prelude::{BoolVariable, ByteVariable, CircuitVariable};

pub fn convert_byte_target_to_byte_var<F: RichField + Extendable<D>, const D: usize>(
    byte_target: Target,
    plonky2_builder: &mut CircuitBuilder<F, D>,
) -> ByteVariable {
    // Note that we are assuming that the target is a byte
    let le_bits: [Target; 8] = plonky2_builder
        .low_bits(byte_target, 8, 8)
        .iter()
        .map(|x| x.target)
        .collect_vec()
        .try_into()
        .expect("Expected 8 bits.  Should never happen");
    let mut bool_variables: [BoolVariable; 8] = le_bits.map(|x| x.into());

    // Need to reverse it to big endian
    bool_variables.reverse();
    ByteVariable::from_be_bits(bool_variables)
}

pub fn convert_byte_var_to_target<F: RichField + Extendable<D>, const D: usize>(
    byte_var: ByteVariable,
    plonky2_builder: &mut CircuitBuilder<F, D>,
) -> Target {
    let le_bits = byte_var.as_le_bits();
    let le_targets = le_bits
        .iter()
        .map(|x| BoolTarget::new_unsafe(x.variables()[0].0));
    plonky2_builder.le_sum(le_targets)
}

/*
a ^ b ^ c = a+b+c - 2*a*b - 2*a*c - 2*b*c + 4*a*b*c
          = a*( 1 - 2*b - 2*c + 4*b*c ) + b + c - 2*b*c
          = a*( 1 - 2*b -2*c + 4*m ) + b + c - 2*m
where m = b*c
 */
pub fn xor3<F: RichField + Extendable<D>, const D: usize>(
    a: BoolTarget,
    b: BoolTarget,
    c: BoolTarget,
    builder: &mut CircuitBuilder<F, D>,
) -> BoolTarget {
    // let gate_type = XOR3Gate::new_from_config(&builder.config);
    // let gate = builder.add_gate(gate_type, vec![]);
    // // let (row, copy) = builder.find_slot(gate, &[], &[]);

    // builder.connect(Target::wire(gate, 0), a.target);
    // builder.connect(Target::wire(gate, 1), b.target);
    // builder.connect(Target::wire(gate, 2), c.target);
    // let output = BoolTarget::new_unsafe(Target::wire(gate, 3));
    // return output;

    let m = builder.mul(b.target, c.target);
    let two_b = builder.add(b.target, b.target);
    let two_c = builder.add(c.target, c.target);
    let two_m = builder.add(m, m);
    let four_m = builder.add(two_m, two_m);
    let one = builder.one();
    let one_sub_two_b = builder.sub(one, two_b);
    let one_sub_two_b_sub_two_c = builder.sub(one_sub_two_b, two_c);
    let one_sub_two_b_sub_two_c_add_four_m = builder.add(one_sub_two_b_sub_two_c, four_m);
    let mut res = builder.mul(a.target, one_sub_two_b_sub_two_c_add_four_m);
    res = builder.add(res, b.target);
    res = builder.add(res, c.target);

    BoolTarget::new_unsafe(builder.sub(res, two_m))
}

pub fn xor3_arr<F: RichField + Extendable<D>, const D: usize, const S: usize>(
    a: [BoolTarget; S],
    b: [BoolTarget; S],
    c: [BoolTarget; S],
    builder: &mut CircuitBuilder<F, D>,
) -> [BoolTarget; S] {
    let mut res = [None; S];
    for i in 0..S {
        res[i] = Some(xor3(a[i], b[i], c[i], builder));
    }

    let gate_type = XOR3Gate::new(16);
    let gate = builder.add_gate(gate_type, vec![]);

    for i in 0..16 {
        builder.connect(
            Target::wire(gate, XOR3Gate::wire_ith_a(i)),
            a[16 + i].target,
        );
        builder.connect(
            Target::wire(gate, XOR3Gate::wire_ith_b(i)),
            b[16 + i].target,
        );
        builder.connect(
            Target::wire(gate, XOR3Gate::wire_ith_c(i)),
            c[16 + i].target,
        );
        res[16 + i] = Some(BoolTarget::new_unsafe(Target::wire(
            gate,
            XOR3Gate::wire_ith_d(i),
        )));
    }

    res.map(|x| x.unwrap())
}

pub fn xor3_arr_slow<F: RichField + Extendable<D>, const D: usize, const S: usize>(
    a: [BoolTarget; S],
    b: [BoolTarget; S],
    c: [BoolTarget; S],
    builder: &mut CircuitBuilder<F, D>,
) -> [BoolTarget; S] {
    let mut res = [None; S];
    for i in 0..S {
        res[i] = Some(xor3(a[i], b[i], c[i], builder));
    }
    res.map(|x| x.unwrap())
}

pub fn xor2_arr<F: RichField + Extendable<D>, const D: usize, const S: usize>(
    a: [BoolTarget; S],
    b: [BoolTarget; S],
    builder: &mut CircuitBuilder<F, D>,
) -> [BoolTarget; S] {
    let mut res = [None; S];
    let zero = builder.zero();
    for i in 0..S {
        let a_b = builder.mul(a[i].target, b[i].target);
        let two_a_b = builder.mul_const(F::ONE + F::ONE, a_b);
        let a_plus_b = builder.add(a[i].target, b[i].target);
        res[i] = Some(BoolTarget::new_unsafe(builder.sub(a_plus_b, two_a_b)));
    }

    let gate_type = XOR3Gate::new(16);
    let gate = builder.add_gate(gate_type, vec![]);

    for i in 0..16 {
        builder.connect(
            Target::wire(gate, XOR3Gate::wire_ith_a(i)),
            a[16 + i].target,
        );
        builder.connect(
            Target::wire(gate, XOR3Gate::wire_ith_b(i)),
            b[16 + i].target,
        );
        builder.connect(Target::wire(gate, XOR3Gate::wire_ith_c(i)), zero);
        res[16 + i] = Some(BoolTarget::new_unsafe(Target::wire(
            gate,
            XOR3Gate::wire_ith_d(i),
        )));
    }

    res.map(|x| x.unwrap())
}

pub fn xor2_arr_slow<F: RichField + Extendable<D>, const D: usize, const S: usize>(
    a: [BoolTarget; S],
    b: [BoolTarget; S],
    builder: &mut CircuitBuilder<F, D>,
) -> [BoolTarget; S] {
    let mut res = [None; S];
    for i in 0..S {
        let a_b = builder.mul(a[i].target, b[i].target);
        let two_a_b = builder.mul_const(F::ONE + F::ONE, a_b);
        let a_plus_b = builder.add(a[i].target, b[i].target);
        res[i] = Some(BoolTarget::new_unsafe(builder.sub(a_plus_b, two_a_b)));
    }

    res.map(|x| x.unwrap())
}

pub fn and_arr<F: RichField + Extendable<D>, const D: usize, const S: usize>(
    a: [BoolTarget; S],
    b: [BoolTarget; S],
    builder: &mut CircuitBuilder<F, D>,
) -> [BoolTarget; S] {
    let mut res = [None; S];
    for i in 0..S {
        res[i] = Some(builder.and(a[i], b[i]));
    }
    res.map(|x| x.unwrap())
}

pub fn not_arr<F: RichField + Extendable<D>, const D: usize, const S: usize>(
    a: [BoolTarget; S],
    builder: &mut CircuitBuilder<F, D>,
) -> [BoolTarget; S] {
    let mut res = [None; S];
    for i in 0..S {
        res[i] = Some(builder.not(a[i]));
    }
    res.map(|x| x.unwrap())
}

pub fn zip_add<F: RichField + Extendable<D>, const D: usize, const S: usize>(
    a: [[BoolTarget; S]; 8],
    b: [[BoolTarget; S]; 8],
    builder: &mut CircuitBuilder<F, D>,
) -> [[BoolTarget; S]; 8] {
    let mut res = [None; 8];
    for i in 0..8 {
        res[i] = Some(add_arr(a[i], b[i], builder));
    }
    res.map(|x| x.unwrap())
}

pub fn add_arr<F: RichField + Extendable<D>, const D: usize, const S: usize>(
    a: [BoolTarget; S],
    b: [BoolTarget; S],
    builder: &mut CircuitBuilder<F, D>,
) -> [BoolTarget; S] {
    // First convert a, b into biguint with limbs of 32 bits each
    let a_biguint = bits_to_biguint_target(builder, a.to_vec());
    let b_biguint = bits_to_biguint_target(builder, b.to_vec());
    // Then add a and b are big uints
    let sum_biguint = builder.add_biguint(&a_biguint, &b_biguint);
    let mut sum_bits = biguint_to_bits_target::<F, D>(builder, &sum_biguint);

    // sum_bits is in big-endian format.
    // we need to return the S least significant bits in big-endian format
    let mut res = [None; S];
    sum_bits.reverse();
    for i in 0..S {
        res[i] = Some(sum_bits[S - 1 - i]);
    }
    res.map(|x| x.unwrap())
}

/// A gate which can perform a weighted multiply-add, i.e. `result = c0 x y + c1 z`. If the config
/// supports enough routed wires, it can support several such operations in one gate.
#[derive(Debug, Clone)]
pub struct XOR3Gate {
    pub num_xors: usize,
}

impl XOR3Gate {
    pub fn new(num_xors: usize) -> Self {
        Self { num_xors }
    }

    pub fn wire_ith_a(i: usize) -> usize {
        i * 4
    }

    pub fn wire_ith_b(i: usize) -> usize {
        i * 4 + 1
    }

    pub fn wire_ith_c(i: usize) -> usize {
        i * 4 + 2
    }

    pub fn wire_ith_d(i: usize) -> usize {
        i * 4 + 3
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Gate<F, D> for XOR3Gate {
    fn id(&self) -> String {
        format!("{self:?}")
    }
    fn serialize(&self, dst: &mut Vec<u8>, _common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        dst.write_usize(self.num_xors)
    }

    fn deserialize(src: &mut Buffer, _common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        let num_xors = src.read_usize()?;
        Ok(Self { num_xors })
    }

    fn eval_unfiltered(&self, vars: EvaluationVars<F, D>) -> Vec<F::Extension> {
        let mut constraints = Vec::new();

        let one = F::Extension::from_canonical_u64(1);
        let two = F::Extension::from_canonical_u64(2);
        let four = F::Extension::from_canonical_u64(4);
        let mut acc = F::Extension::from_canonical_u64(0);

        for i in 0..self.num_xors {
            let a = vars.local_wires[XOR3Gate::wire_ith_a(i)];
            let b = vars.local_wires[XOR3Gate::wire_ith_b(i)];
            let c = vars.local_wires[XOR3Gate::wire_ith_c(i)];
            let d = vars.local_wires[XOR3Gate::wire_ith_d(i)];
            let output = a * (one - two * b - two * c + four * b * c) + b + c - two * b * c - d;
            acc += output;
        }

        constraints.push(acc);

        constraints
    }

    fn eval_unfiltered_base_one(
        &self,
        vars: EvaluationVarsBase<F>,
        mut yield_constr: StridedConstraintConsumer<F>,
    ) {
        let one = F::from_canonical_u64(1);
        let two = F::from_canonical_u64(2);
        let four = F::from_canonical_u64(4);

        let mut acc = F::from_canonical_u64(0);
        for i in 0..self.num_xors {
            let a = vars.local_wires[XOR3Gate::wire_ith_a(i)];
            let b = vars.local_wires[XOR3Gate::wire_ith_b(i)];
            let c = vars.local_wires[XOR3Gate::wire_ith_c(i)];
            let d = vars.local_wires[XOR3Gate::wire_ith_d(i)];
            let output = a * (one - two * b - two * c + four * b * c) + b + c - two * b * c - d;
            acc += output;
        }

        yield_constr.one(acc);
    }

    fn eval_unfiltered_circuit(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        vars: EvaluationTargets<D>,
    ) -> Vec<ExtensionTarget<D>> {
        let one = builder.one_extension();
        let two = builder.two_extension();
        let four = builder.constant_extension(F::Extension::from_canonical_u64(4));
        let mut acc = builder.zero_extension();

        for i in 0..self.num_xors {
            let a = vars.local_wires[XOR3Gate::wire_ith_a(i)];
            let b = vars.local_wires[XOR3Gate::wire_ith_b(i)];
            let c = vars.local_wires[XOR3Gate::wire_ith_c(i)];
            let d = vars.local_wires[XOR3Gate::wire_ith_d(i)];

            let two_b = builder.mul_extension(two, b);
            let two_c = builder.mul_extension(two, c);
            let bc = builder.mul_extension(b, c);
            let four_bc = builder.mul_extension(four, bc);
            let two_bc = builder.mul_extension(two, bc);

            // a * (one - two * b - two * c + four * b * c) + b + c - two * b * c - d;

            let mut output = builder.sub_extension(one, two_b);
            output = builder.sub_extension(output, two_c);
            output = builder.add_extension(output, four_bc);
            output = builder.mul_extension(a, output);
            output = builder.add_extension(output, b);
            output = builder.add_extension(output, c);
            output = builder.sub_extension(output, two_bc);
            output = builder.sub_extension(output, d);

            acc = builder.add_extension(acc, output);
        }

        vec![acc]
    }

    fn generators(&self, row: usize, _local_constants: &[F]) -> Vec<WitnessGeneratorRef<F, D>> {
        let gen = XOR3Generator::<F, D> {
            row,
            num_xors: self.num_xors,
            _phantom: PhantomData,
        };
        vec![WitnessGeneratorRef::new(gen.adapter())]
    }

    fn num_wires(&self) -> usize {
        4
    }

    fn num_constants(&self) -> usize {
        0
    }

    fn degree(&self) -> usize {
        3
    }

    fn num_constraints(&self) -> usize {
        1
    }
}

#[derive(Debug, Default)]
pub struct XOR3Generator<F: RichField + Extendable<D>, const D: usize> {
    row: usize,
    num_xors: usize,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> XOR3Generator<F, D> {
    pub fn id() -> String {
        "XOR3Generator".to_string()
    }
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D> for XOR3Generator<F, D> {
    fn id(&self) -> String {
        Self::id()
    }

    fn serialize(&self, dst: &mut Vec<u8>, _common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        dst.write_usize(self.row)?;
        dst.write_usize(self.num_xors)
    }

    fn deserialize(src: &mut Buffer, _common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        let row = src.read_usize()?;
        let num_xors = src.read_usize()?;
        Ok(Self {
            row,
            num_xors,
            _phantom: PhantomData,
        })
    }

    fn dependencies(&self) -> Vec<Target> {
        let local_target = |column| Target::wire(self.row, column);
        let mut result: Vec<Target> = Vec::new();

        for i in 0..self.num_xors {
            result.push(local_target(i * 4));
            result.push(local_target(i * 4 + 1));
            result.push(local_target(i * 4 + 2));
        }

        result
    }

    /*
    a ^ b ^ c = a+b+c - 2*a*b - 2*a*c - 2*b*c + 4*a*b*c
            = a*( 1 - 2*b - 2*c + 4*b*c ) + b + c - 2*b*c
            = a*( 1 - 2*b -2*c + 4*m ) + b + c - 2*m
    where m = b*c
    */
    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let get_wire = |wire: usize| -> F { witness.get_target(Target::wire(self.row, wire)) };

        let one = F::from_canonical_u64(1);
        let two = F::from_canonical_u64(2);
        let four = F::from_canonical_u64(4);

        for i in 0..self.num_xors {
            let a = get_wire(4 * i);
            let b = get_wire(4 * i + 1);
            let c = get_wire(4 * i + 2);
            let d_target = Target::wire(self.row, 4 * i + 3);
            let computed_output =
                a * (one - two * b - two * c + four * b * c) + b + c - two * b * c;
            out_buffer.set_target(d_target, computed_output);
        }
    }
}
