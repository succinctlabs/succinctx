use core::marker::PhantomData;

use curta::chip::builder::AirBuilder;
use curta::chip::ec::edwards::ed25519::gadget::CompressedPointGadget;
use curta::chip::ec::edwards::ed25519::instruction::Ed25519FpInstruction;
use curta::chip::ec::edwards::ed25519::params::{Ed25519, Ed25519Parameters};
use curta::chip::ec::edwards::ed25519::point::CompressedPointRegister;
use curta::chip::ec::edwards::scalar_mul::gadget::EdScalarMulGadget;
use curta::chip::ec::point::AffinePointRegister;
use curta::chip::ec::EllipticCurve;
use curta::chip::instruction::set::AirInstruction;
use curta::chip::register::array::ArrayRegister;
use curta::chip::register::element::ElementRegister;
use curta::chip::trace::data::AirTraceData;
use curta::chip::{AirParameters, Chip};
use curta::math::prelude::{CubicParameters, *};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ed25519VerificationAirParameters<F: Field, R: CubicParameters<F>>(
    pub PhantomData<(F, R)>,
);

impl<F: PrimeField64, R: CubicParameters<F>> AirParameters
    for Ed25519VerificationAirParameters<F, R>
{
    type Field = F;
    type CubicParams = R;

    // TODO: specialize / implement as a function of E.
    const NUM_ARITHMETIC_COLUMNS: usize = 1000;
    const NUM_FREE_COLUMNS: usize = 9;
    const EXTENDED_COLUMNS: usize = 1527;

    type Instruction = Ed25519FpInstruction;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Ed25519VerificationAir<F: PrimeField64, R: CubicParameters<F>> {
    pub air: Chip<Ed25519VerificationAirParameters<F, R>>,
    pub trace_data: AirTraceData<Ed25519VerificationAirParameters<F, R>>,
    pub public_keys: Vec<CompressedPointRegister>,
    pub sigs_s_limbs: Vec<ArrayRegister<ElementRegister>>,
    pub sigr_s: Vec<CompressedPointRegister>,
    pub h_s_limbs: Vec<ArrayRegister<ElementRegister>>,
}

impl<F: PrimeField64, R: CubicParameters<F>> Ed25519VerificationAir<F, R> {
    /// Creates a new instance of the SigVerificationAir
    pub fn new() -> Self {
        let mut builder = AirBuilder::<Ed25519VerificationAirParameters<F, R>>::new();

        const NUM_VERIFICATIONS: usize = 256;

        // Allocate public inputs.
        let public_keys = (0..NUM_VERIFICATIONS)
            .map(|_| builder.alloc_public_ec_compressed_point())
            .collect::<Vec<_>>();

        // Each sig_s is 8 32-bit limbs
        let sigs_s_limbs = (0..NUM_VERIFICATIONS)
            .map(|_| builder.alloc_array_public::<ElementRegister>(8))
            .collect::<Vec<_>>();

        let sigr_s = (0..NUM_VERIFICATIONS)
            .map(|_| builder.alloc_public_ec_compressed_point())
            .collect::<Vec<_>>();

        // Each h is 8 32-bit limbs
        let h_s_limbs = (0..NUM_VERIFICATIONS)
            .map(|_| builder.alloc_array_public::<ElementRegister>(8))
            .collect::<Vec<_>>();

        let mut pub_keys_affine = Vec::new();
        let mut sigr_s_affine = Vec::new();
        // First decompress the public keys and sigr_s
        for i in 0..NUM_VERIFICATIONS {
            pub_keys_affine.push(builder.ed25519_decompress(&public_keys[i]));
            sigr_s_affine.push(builder.ed25519_decompress(&sigr_s[i]));
        }

        // Assert that the public keys and sig_r's are are on the curve.
        for i in 0..NUM_VERIFICATIONS {
            builder.ed_assert_valid(&pub_keys_affine[i]);
            builder.ed_assert_valid(&sigr_s_affine[i]);
        }

        /*
        // Calculate p1 = sig_s * G.  First create an array of generator constants
        let mut generators = Vec::new();
        for i in 0..NUM_VERIFICATIONS {
            generators.push(Ed25519::generator_affine_point_register(&mut builder));
        }
        let (p1_scalar_mul_gadget, p1_s, (p1_set_last, p1_set_bit)) =
            builder.batch_scalar_mul(&generators, &sigs_s_limbs);

        // Calculate p2 = h * pubKey.
        let (p2_scalar_mul_gadget, mut p2_s, (p2_set_last, p2_set_bit)) =
            builder.batch_scalar_mul(&pub_keys_affine, &h_s_limbs);

        // Calculate p2 = p2 + R.
        for i in 0..NUM_VERIFICATIONS {
            p2_s[i] = builder.ed_add(&p2_s[i], &sigr_s_affine[i]);
        }

        // Assert that p1 == p2
        for i in 0..NUM_VERIFICATIONS {
            builder.assert_equal(&p1_s[i].x, &p2_s[i].x);
            builder.assert_equal(&p1_s[i].y, &p2_s[i].y);
        }
        */

        let (air, trace_data) = builder.build();

        Self {
            air,
            trace_data,
            public_keys,
            sigs_s_limbs,
            sigr_s,
            h_s_limbs,
        }
    }
}
