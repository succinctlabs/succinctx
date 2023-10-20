use crate::frontend::builder::CircuitBuilder;
use crate::prelude::{BoolVariable, CircuitVariable, PlonkParameters, U32Variable, Variable};

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn reshape(&self, arr: Vec<BoolVariable>) -> Vec<[BoolVariable; 32]> {
        arr.chunks(32).map(|x| x.try_into().unwrap()).collect()
    }

    pub fn _right_rotate<const S: usize>(
        &self,
        arr: [BoolVariable; S],
        bits: usize,
    ) -> [BoolVariable; S] {
        let mut res = Vec::new();
        for i in 0..S {
            res.push(arr[((S - bits) + i) % S])
        }
        res.try_into().unwrap()
    }

    pub fn _shr<const S: usize>(
        &mut self,
        arr: [BoolVariable; S],
        bits: usize,
    ) -> [BoolVariable; S] {
        let mut res = Vec::new();
        for i in 0..S {
            if i < bits {
                res.push(self._false());
            } else {
                res.push(arr[i - bits]);
            }
        }
        res.try_into().unwrap()
    }

    /// It is assumed that `a`, `b`, `c` are already rangechecked from BoolVariable
    /// below is the algebraic expression for a ^ b ^ c
    // a ^ b ^ c = a+b+c - 2*a*b - 2*a*c - 2*b*c + 4*a*b*c
    // = a*( 1 - 2*b - 2*c + 4*b*c ) + b + c - 2*b*c
    // = a*( 1 - 2*b -2*c + 4*m ) + b + c - 2*m
    // where m = b*c
    //
    pub fn xor3(&mut self, a: Variable, b: Variable, c: Variable) -> BoolVariable {
        let m = self.mul(b, c);
        let two_b = self.add(b, b);
        let two_c = self.add(c, c);
        let two_m = self.add(m, m);
        let four_m = self.add(two_m, two_m);
        let one = self.one::<Variable>();
        let one_sub_two_b = self.sub(one, two_b);
        let one_sub_two_b_sub_two_c = self.sub(one_sub_two_b, two_c);
        let one_sub_two_b_sub_two_c_add_four_m = self.add(one_sub_two_b_sub_two_c, four_m);
        let mut res = self.mul(a, one_sub_two_b_sub_two_c_add_four_m);
        res = self.add(res, b);
        res = self.add(res, c);
        let v = self.sub(res, two_m);

        // It's okay because this is an algebraic expression where if a, b, c are bits
        // then the result is a bit
        BoolVariable::from_variables_unsafe(&[v])
    }

    pub fn add_arr<const S: usize>(
        &mut self,
        a: [BoolVariable; S],
        b: [BoolVariable; S],
    ) -> [BoolVariable; S] {
        if S == 32 {
            let a_u32 = U32Variable::from_be_bits(&a, self);
            let b_u32 = U32Variable::from_be_bits(&b, self);

            let a_u64 = a_u32.to_u64(self);
            let b_u64 = b_u32.to_u64(self);
            let c_u64 = self.add(a_u64, b_u64);
            let c_u32 = c_u64.limbs[0];

            c_u32.to_be_bits(self).to_vec().try_into().unwrap()
        } else {
            todo!();
        }
    }

    pub fn zip_add<const S: usize>(
        &mut self,
        a: [[BoolVariable; S]; 8],
        b: [[BoolVariable; S]; 8],
    ) -> [[BoolVariable; S]; 8] {
        a.iter()
            .zip(b.iter())
            .map(|(a, b)| self.add_arr(*a, *b))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

pub fn xor3_arr<const S: usize, L: PlonkParameters<D>, const D: usize>(
    a: [BoolVariable; S],
    b: [BoolVariable; S],
    c: [BoolVariable; S],
    builder: &mut CircuitBuilder<L, D>,
) -> [BoolVariable; S] {
    let mut res = Vec::new();
    for i in 0..S {
        let o = builder.xor3(a[i].0, b[i].0, c[i].0);
        // builder.watch_slice(&[a[i], b[i], c[i], o], "xor3_arr");
        res.push(o);
    }
    res.try_into().unwrap()
}

pub fn xor2_arr<const S: usize, L: PlonkParameters<D>, const D: usize>(
    a: [BoolVariable; S],
    b: [BoolVariable; S],
    builder: &mut CircuitBuilder<L, D>,
) -> [BoolVariable; S] {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| builder.xor(*a, *b))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub fn not_arr<const S: usize, L: PlonkParameters<D>, const D: usize>(
    a: [BoolVariable; S],
    builder: &mut CircuitBuilder<L, D>,
) -> [BoolVariable; S] {
    a.iter()
        .map(|a| builder.not(*a))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub fn and_arr<const S: usize, L: PlonkParameters<D>, const D: usize>(
    a: [BoolVariable; S],
    b: [BoolVariable; S],
    builder: &mut CircuitBuilder<L, D>,
) -> [BoolVariable; S] {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| builder.and(*a, *b))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
