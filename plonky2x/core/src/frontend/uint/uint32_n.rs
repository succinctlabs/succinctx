#[macro_export]
macro_rules! make_uint32_n {
    ($a:ident, $b:ty, $c:expr) => {
        /// An integer type encoded as little-endian u32 limbs.
        #[derive(Debug, Clone, Copy)]
        pub struct $a {
            pub limbs: [U32Variable; $c]
        }

        impl CircuitVariable for $a {
            type ValueType<F: RichField> = $b;

            fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
                builder: &mut CircuitBuilder<L, D>,
            ) -> Self {
                Self {
                    limbs: array![_ => U32Variable::init_unsafe(builder); $c],
                }
            }

            fn variables(&self) -> Vec<Variable> {
                self.limbs.iter().map(|x| x.variable).collect()
            }

            fn from_variables_unsafe(variables: &[Variable]) -> Self {
                assert_eq!(variables.len(), $c);
                Self {
                    limbs: array![i => U32Variable::from_variables_unsafe(&[variables[i]]); $c],
                }
            }

            fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
                &self,
                builder: &mut CircuitBuilder<L, D>,
            ) {
                for limb in self.limbs.iter() {
                    limb.assert_is_valid(builder);
                }
            }

            fn nb_elements() -> usize {
                U32Variable::nb_elements() * $c
            }

            fn elements<F: RichField>(value: $b) -> Vec<F> {
                let limbs = <$b as Uint<$c>>::to_u32_limbs(value);
                limbs.iter().flat_map(|x| U32Variable::elements(*x)).collect()
            }

            fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
                let mut value_limbs: [u32; $c] = [0; $c];
                for i in 0..$c {
                    // There is 1 element in each U32 Variable
                    value_limbs[i] = U32Variable::from_elements(&elements[i .. i+1]);
                }
                <$b as Uint<$c>>::from_u32_limbs(value_limbs)
            }
        }


        impl EvmVariable for $a {
            fn encode<L: PlonkParameters<D>, const D: usize>(
                &self,
                builder: &mut CircuitBuilder<L, D>,
            ) -> Vec<ByteVariable> {
                self.limbs
                    .iter()
                    .rev()
                    .flat_map(|x| x.encode(builder))
                    .collect::<Vec<_>>()
            }

            fn decode<L: PlonkParameters<D>, const D: usize>(
                builder: &mut CircuitBuilder<L, D>,
                bytes: &[ByteVariable],
            ) -> Self {
                assert_eq!(bytes.len(), $c * 4);
                let mut limbs = [U32Variable::init_unsafe(builder); $c];
                for i in 0..$c {
                    limbs[i] = U32Variable::decode(builder, &bytes[i * 4..(i + 1) * 4]);
                }
                limbs.reverse();
                Self {
                    limbs
                }
            }

            fn encode_value<F: RichField>(value: Self::ValueType<F>) -> Vec<u8> {
                let mut bytes = vec![0u8; $c * 4];
                <$b as Uint<$c>>::to_big_endian(&value, &mut bytes);
                bytes
            }

            fn decode_value<F: RichField>(bytes: &[u8]) -> Self::ValueType<F> {
                <$b as Uint<$c>>::from_big_endian(bytes)
            }
        }

        impl SSZVariable for $a {
            fn hash_tree_root<L: PlonkParameters<D>, const D: usize>(
                &self,
                builder: &mut CircuitBuilder<L, D>,
            ) -> Bytes32Variable {
                let mut bytes = self.encode(builder);
                bytes.reverse();
                if bytes.len() < 32 {
                    let zero = builder.constant::<ByteVariable>(0);
                    bytes.extend(vec![zero; 32 - bytes.len()]);
                }
                Bytes32Variable(BytesVariable::<32>(bytes.try_into().unwrap()))
            }
        }

        impl<L: PlonkParameters<D>, const D: usize> Zero<L, D> for $a {
            fn zero(builder: &mut CircuitBuilder<L, D>) -> Self {
                let zero = U32Variable::zero(builder);
                Self {
                    limbs: [zero; $c],
                }
            }
        }

        impl<L: PlonkParameters<D>, const D: usize> One<L, D> for $a {
            fn one(builder: &mut CircuitBuilder<L, D>) -> Self {
                let zero = U32Variable::zero(builder);
                let one = U32Variable::one(builder);
                let mut limbs = [zero; $c];
                limbs[0] = one;
                Self {
                    limbs
                }
            }
        }

        impl<L: PlonkParameters<D>, const D: usize> Add<L, D> for $a {
            type Output = Self;

            fn add(self, rhs: $a, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
                let self_targets = self
                    .limbs
                    .iter()
                    .map(|x| U32Target::from(*x))
                    .collect::<Vec<_>>();
                let rhs_targets = rhs
                    .limbs
                    .iter()
                    .map(|x| U32Target::from(*x))
                    .collect::<Vec<_>>();
                assert_eq!(self_targets.len(), rhs_targets.len());
                assert_eq!(self_targets.len(), $c);

                let self_biguint = BigUintTarget {
                    limbs: self_targets,
                };
                let rhs_biguint = BigUintTarget { limbs: rhs_targets };
                let sum_biguint = builder.api.add_biguint(&self_biguint, &rhs_biguint);

                let mut limbs: [U32Variable; $c] = Self::zero(builder).limbs;
                for i in 0..$c {
                    limbs[i] = sum_biguint.limbs[i].into();
                }

                Self {
                    limbs
                }
            }
        }

        impl<L: PlonkParameters<D>, const D: usize> Sub<L, D> for $a {
            type Output = Self;

            fn sub(self, rhs: $a, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
                let self_targets = self
                    .limbs
                    .iter()
                    .map(|x| U32Target::from(*x))
                    .collect::<Vec<_>>();
                let rhs_targets = rhs
                    .limbs
                    .iter()
                    .map(|x| U32Target::from(*x))
                    .collect::<Vec<_>>();
                assert_eq!(self_targets.len(), rhs_targets.len());
                assert_eq!(self_targets.len(), $c);

                let self_biguint = BigUintTarget {
                    limbs: self_targets,
                };
                let rhs_biguint = BigUintTarget { limbs: rhs_targets };
                let diff_biguint = builder.api.sub_biguint(&self_biguint, &rhs_biguint);

                let mut limbs: [U32Variable; $c] = Self::zero(builder).limbs;
                for i in 0..$c {
                    limbs[i] = diff_biguint.limbs[i].into();
                }

                Self {
                    limbs
                }
            }
        }

        impl<L: PlonkParameters<D>, const D: usize> Mul<L, D> for $a {
            type Output = Self;

            fn mul(self, rhs: $a, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
                let self_targets = self
                    .limbs
                    .iter()
                    .map(|x| U32Target::from(*x))
                    .collect::<Vec<_>>();
                let rhs_targets = rhs
                    .limbs
                    .iter()
                    .map(|x| U32Target::from(*x))
                    .collect::<Vec<_>>();
                assert_eq!(self_targets.len(), rhs_targets.len());
                assert_eq!(self_targets.len(), $c);

                let self_biguint = BigUintTarget {
                    limbs: self_targets,
                };
                let rhs_biguint = BigUintTarget { limbs: rhs_targets };
                let product_biguint = builder.api.mul_biguint(&self_biguint, &rhs_biguint);

                let mut limbs: [U32Variable; $c] = Self::zero(builder).limbs;
                for i in 0..$c {
                    limbs[i] = product_biguint.limbs[i].into();
                }

                Self {
                    limbs
                }
            }
        }

        impl<L: PlonkParameters<D>, const D: usize> Div<L, D> for $a {
            type Output = Self;

            fn div(self, rhs: $a, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
                let self_targets = self
                    .limbs
                    .iter()
                    .map(|x| U32Target::from(*x))
                    .collect::<Vec<_>>();
                let rhs_targets = rhs
                    .limbs
                    .iter()
                    .map(|x| U32Target::from(*x))
                    .collect::<Vec<_>>();
                assert_eq!(self_targets.len(), rhs_targets.len());
                assert_eq!(self_targets.len(), $c);

                let self_biguint = BigUintTarget {
                    limbs: self_targets,
                };
                let rhs_biguint = BigUintTarget { limbs: rhs_targets };
                let quotient_biguint = builder.api.div_biguint(&self_biguint, &rhs_biguint);

                let mut limbs: [U32Variable; $c] = Self::zero(builder).limbs;
                for i in 0..$c {
                    limbs[i] = quotient_biguint.limbs[i].into();
                }

                Self {
                    limbs
                }
            }
        }

        impl<L: PlonkParameters<D>, const D: usize> Rem<L, D> for $a {
            type Output = Self;

            fn rem(self, rhs: $a, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
                let self_targets = self
                    .limbs
                    .iter()
                    .map(|x| U32Target::from(*x))
                    .collect::<Vec<_>>();
                let rhs_targets = rhs
                    .limbs
                    .iter()
                    .map(|x| U32Target::from(*x))
                    .collect::<Vec<_>>();
                assert_eq!(self_targets.len(), rhs_targets.len());
                assert_eq!(self_targets.len(), $c);

                let self_biguint = BigUintTarget {
                    limbs: self_targets,
                };
                let rhs_biguint = BigUintTarget { limbs: rhs_targets };
                let rem_biguint = builder.api.rem_biguint(&self_biguint, &rhs_biguint);

                let mut limbs: [U32Variable; $c] = Self::zero(builder).limbs;
                for i in 0..$c {
                    limbs[i] = rem_biguint.limbs[i].into();
                }

                Self {
                    limbs
                }
            }
        }

        impl<L: PlonkParameters<D>, const D: usize> LessThanOrEqual<L, D> for $a {
            #[must_use]
            fn lte(self, rhs: Self, builder: &mut CircuitBuilder<L, D>) -> BoolVariable {
                let mut lte_acc = builder.constant::<BoolVariable>(false);
                let mut equal_so_far = builder.constant::<BoolVariable>(true);
                for i in 0..$c {
                    let lhs = self.limbs[$c - i - 1];
                    let rhs = rhs.limbs[$c - i - 1];
                    let lte = builder.lte(lhs, rhs);
                    lte_acc = builder.select(equal_so_far, lte, lte_acc);
                    let equal = builder.is_equal(lhs, rhs);
                    equal_so_far = builder.and(equal_so_far, equal);
                }
                builder.or(lte_acc, equal_so_far)
            }
        }
    };
}

#[macro_export]
macro_rules! make_uint32_n_tests {
    ($a:ident, $b:ty, $c:expr) => {
        #[cfg(test)]
        mod tests {
            use rand::rngs::OsRng;
            use rand::Rng;
            use $crate::backend::circuit::DefaultParameters;
            use $crate::frontend::uint::Uint;
            use $crate::frontend::vars::EvmVariable;
            use $crate::prelude::*;

            #[allow(unused_imports)]
            use super::*;

            type L = DefaultParameters;
            const D: usize = 2;

            #[test]
            fn test_evm() {
                let num_bytes = $c * 4;
                let mut builder = CircuitBuilder::<L, D>::new();
                let mut var_bytes = vec![];
                for i in 0..(num_bytes) {
                    let byte = ByteVariable::constant(&mut builder, i as u8);
                    var_bytes.push(byte);
                }
                let decoded: $a = $a::decode(&mut builder, &var_bytes);
                let encoded = decoded.encode(&mut builder);
                let redecoded = $a::decode(&mut builder, &encoded[0..num_bytes]);

                builder.assert_is_equal(decoded, redecoded);
                for i in 0..(num_bytes) {
                    builder.assert_is_equal(var_bytes[i], encoded[i]);
                }

                let circuit = builder.build();
                let pw = PartialWitness::new();

                let proof = circuit.data.prove(pw).unwrap();
                circuit.data.verify(proof).unwrap();
            }

            #[test]
            fn test_u32n_evm_value() {
                type F = GoldilocksField;

                let limbs = [OsRng.gen::<u32>(); $c];
                let num = <$b as Uint<$c>>::from_u32_limbs(limbs);
                let encoded = $a::encode_value::<F>(num);
                let decoded: $b = $a::decode_value::<F>(&encoded);

                assert_eq!(decoded.to_u32_limbs(), num.to_u32_limbs());
            }

            #[test]
            fn test_u32n_add() {
                let mut rng = OsRng;

                let a = <$b as Uint<$c>>::from_u32_limbs([rng.gen(); $c]);
                let b = <$b as Uint<$c>>::from_u32_limbs([rng.gen(); $c]);

                let (expected_value, _) = a.overflowing_add(b);

                let mut builder = CircuitBuilder::<L, D>::new();

                let a = $a::constant(&mut builder, a);
                let b = $a::constant(&mut builder, b);
                let result = builder.add(a, b);
                let expected_result_var = $a::constant(&mut builder, expected_value);

                builder.assert_is_equal(result, expected_result_var);

                let circuit = builder.build();
                let pw = PartialWitness::new();

                let proof = circuit.data.prove(pw).unwrap();
                circuit.data.verify(proof).unwrap();
            }

            #[test]
            fn test_u256_sub() {
                let _num_bytes = $c * 4;

                let mut rng = OsRng;

                let a = <$b as Uint<$c>>::from_u32_limbs([rng.gen(); $c]);
                let b = <$b as Uint<$c>>::from_u32_limbs([rng.gen(); $c]);

                let (expected_value, _) = a.overflowing_sub(b);

                let mut builder = CircuitBuilder::<L, D>::new();

                let a = $a::constant(&mut builder, a);
                let b = $a::constant(&mut builder, b);
                let result = builder.sub(a, b);
                let expected_result_var = $a::constant(&mut builder, expected_value);

                builder.assert_is_equal(result, expected_result_var);

                let circuit = builder.build();
                let pw = PartialWitness::new();

                let proof = circuit.data.prove(pw).unwrap();
                circuit.data.verify(proof).unwrap();
            }

            #[test]
            fn test_u256_mul() {
                const D: usize = 2;

                let mut rng = OsRng;

                let a = <$b as Uint<$c>>::from_u32_limbs([rng.gen(); $c]);
                let b = <$b as Uint<$c>>::from_u32_limbs([rng.gen(); $c]);

                let (expected_value, _) = a.overflowing_mul(b);

                let mut builder = CircuitBuilder::<L, D>::new();

                let a = $a::constant(&mut builder, a);
                let b = $a::constant(&mut builder, b);
                let result = builder.mul(a, b);
                let expected_result_var = $a::constant(&mut builder, expected_value);

                builder.assert_is_equal(result, expected_result_var);

                let circuit = builder.build();
                let pw = PartialWitness::new();

                let proof = circuit.data.prove(pw).unwrap();
                circuit.data.verify(proof).unwrap();
            }
        }
    };
}
