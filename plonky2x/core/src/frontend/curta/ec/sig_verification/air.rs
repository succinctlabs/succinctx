use core::marker::PhantomData;

use curta::chip::builder::{AirBuilder, AirTraceData};
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
    pub p1_s: Vec<AffinePointRegister<Ed25519>>,
    pub p1_scalar_mul_gadget: EdScalarMulGadget<F, Ed25519Parameters>,
    pub p1_set_last: AirInstruction<F, Ed25519FpInstruction>,
    pub p1_set_bit: AirInstruction<F, Ed25519FpInstruction>,
    pub p2_s: Vec<AffinePointRegister<Ed25519>>,
    pub p2_scalar_mul_gadget: EdScalarMulGadget<F, Ed25519Parameters>,
    pub p2_set_last: AirInstruction<F, Ed25519FpInstruction>,
    pub p2_set_bit: AirInstruction<F, Ed25519FpInstruction>,
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

        let (air, trace_data) = builder.build();

        Self {
            air,
            trace_data,
            public_keys,
            sigs_s_limbs,
            sigr_s,
            h_s_limbs,
            p1_s,
            p1_scalar_mul_gadget,
            p1_set_last,
            p1_set_bit,
            p2_s,
            p2_scalar_mul_gadget,
            p2_set_last,
            p2_set_bit,
        }
    }
}

#[cfg(test)]
mod tests {

    use curta::chip::ec::edwards::ed25519::gadget::CompressedPointWriter;
    use curta::chip::ec::edwards::ed25519::params::Ed25519;
    use curta::chip::ec::gadget::EllipticCurveWriter;
    use curta::chip::trace::generator::ArithmeticGenerator;
    use curta::chip::utils::biguint_to_bits_le;
    use curta::math::goldilocks::cubic::GoldilocksCubicParameters;
    use curta::maybe_rayon::*;
    use curta::plonky2::stark::config::{
        CurtaPoseidonGoldilocksConfig, PoseidonGoldilocksStarkConfig,
    };
    use curta::plonky2::stark::gadget::StarkGadget;
    use curta::plonky2::stark::generator::simple::SimpleStarkWitnessGenerator;
    use curta::plonky2::stark::prover::StarkyProver;
    use curta::plonky2::stark::verifier::StarkyVerifier;
    use curta::plonky2::stark::Starky;
    use curve25519_dalek::edwards::CompressedEdwardsY;
    use curve25519_dalek::Scalar;
    use num_bigint::BigUint;
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::iop::witness::{PartialWitness, WitnessWrite};
    use plonky2::timed;
    use plonky2::util::timing::TimingTree;
    use rand::{thread_rng, Rng};

    use super::*;
    use crate::prelude::DefaultBuilder;
    use crate::utils::setup_logger;

    pub const MESSAGE: &str = "01c63e6b7db7863b35b289b35349a8a488ae886a59c37d4825577ddb9470c4537f972804002c3a0000000000000001000000000000";

    const NUM_SIGS: usize = 1;

    pub const H: [&str; NUM_SIGS] =
        ["9f81b8e0cf40cb8ec8f0bdcf8d4f7a9b56002e7e04b1ffe9790d27974519eb06"];

    pub const SIGS: [&str; NUM_SIGS] = [
        "186953830bb9c1f18b8ed096eef919bf6409d1921b8dc698fb39ca8135c2575f0b6feb4f0383b70f5156a99cd8210ec516bd1d702bdc8444ed3172b5f42fb008"];

    pub const PUB_KEYS: [&str; NUM_SIGS] =
        ["02f80695f0a4a2308246c88134b2de759e347d527189742dd42e98724bd5a9bc"];

    // const NUM_SIGS: usize = 51;

    // pub const SIGS: [&str; NUM_SIGS] = [
    //     "186953830bb9c1f18b8ed096eef919bf6409d1921b8dc698fb39ca8135c2575f0b6feb4f0383b70f5156a99cd8210ec516bd1d702bdc8444ed3172b5f42fb008", "24f12fd8775414dadb0a9bde425b814e2537b9b49805e555956fa64ed8821a8087f8834085ddad979fe773f81b485d94c22173798a0d124376bb0dd2a132fa09", "f57d6d2328a3bf25b8ce3062ad7e2eaac08bc426a70c88459d406bd22104a2f2d5332eb983ae4419cf55dfd9a69b99338cfbf5e58418b90fadf9d2d9dac59d02", "6be531403e56cb8195d571c40f8ca8d61c6a709baf2d663f24f298f634cba21e41d0cf0adece94289aa95b3150415e2f17d4a2301ce2868df95d084105c1de05", "f8895d802c7c9a5e4251d87285cba673145419f1518fb74681105aad7bcf5259883b553618b058da0505393936f60630bfbeef840e3cf225bfd9b4fd3df4d70c", "37299babd49cd50717986295a9de3c695583cf73b7a9d43e5c5fda44442841268b3702aba5cf118ef70554f98bc5f51c42bd34c7fce72e872478fe294eee940a", "82eace9a278c5623416539a931e70832079d83c87bf74168074e8d91465693193d23d12a1da93104ff9e94fd595f269ebab0923bd69224912572c0f4699e3e08", "ef6bab69352b3863ca444934042abebed113069b9c1363403dafbf62e8fd8462d7b1772743ee7c7ea48c7524e5afcc8719b7640cc3dd90075edeac1caa689f03", "099f2e36d97710e040f8989efa65f9f915247e184e4f31a78718af986daff98e0bdfc12faf3c5e2d0f33eed21139e3b15661060fdd200493ec5964a2781db104", "25a8b02a238389748102501be8691dd30f9d4c17c81e158be49c34823310460e34a159c9796a249540c3dbc587e1092ee47c35fdaccc85f755b0b02bba029106", "721b3af5bb191ed79b6d54a6aaf54055f5936e1eab6e12bd2a1195ea9872ccd1fcd3342acfe96a07a1768a260c3f2ec51b5b057e246d7bdcbd30a02607f1f80a", "d98f5bc7601fed4233c270ca52ee57fcfd149626a66fc9ae1d49aa6fd843c9e36282019b187a4b5b9aa90dbfd868142d53d2dd6b2399b2ecbaf53eda85999b07", "7c896c05af1ade3c4645d07c00fe9749eaef485c5987000e8031b384360e923eae792eb5b05cd8f5ea1261cdc9f98e89e57ba798c130ef273636f48887cace02", "c2b43ecd73d74e41abafb1b04a7a5e6b42f6cd1f16393918d2db7a50d5601a5a68a83a8a4258ae86e4905c4c3b159f1e2df1edba098716c581173fa8f5ae670a", "b5cfc35faf5ccf3af992853fc576538a3671df60b6245a5256a783ed7bb992c35642303cbb38405209b52caa4a092e3b8eeea0a5cbefa02efb0e13e28adc6e0d", "f852f065adf20f9a28516df7e07d22bc03e5971617d8e94f3c06711461e3f6deb5287a1b1ba685ce579aba0a10885fcb4c50c3c2e781bd5e4645f2abae953b0d", "6994b4c960a96545b71b30212a5c3ea5745d606f13a943184129a95f10acb6bcc67278858db307971f486d5645f8d8c0bb3c6d716e8f8ab1d5b82ad0ce7caa07", "c2f61aa990b4de006a1a6460b23075df139b28f2c79bc3a9f7d90567e3701afd3a856a9ca4bf4842423f1cceb1a0758a05cc40ca0e7e4f7ff3d37a9fae376005", "a7aedb9f4cbd056d46f4e5b37eadd22e65e9736e6a435acaf0be95b7720fa0cba651974b78e168cceffbc6e379d67f3477a2e5309b6136e72fbded31a69ba107", "6f9f2af53fe2e0159218d1530ed54be713c45a0cd9e5335ce528beb1fa7e4ac2dccd441db35a83f8ce110f636fac28464cb6c702876821e7953082c1dd867301", "e3076ce9d67b518b92fe22103613c112bd1002026d8044a04d9f588ddf10a4bb2ec0e32240b7b96df5c62c12869c331cf0b478bad941b087efc2a620b83b9e0b", "b60fad4974ab432f30d2fd70e60e35a090c126662d0795980c326e2eba6692e6bba6d95c1476420926a44ac1a5fd08a51668267d2526d83e09282c89ed3e5b03", "f408aaa06b3d8cab387de8334aadb9272eb83cd2e2f95410203d78b9b2bacba0d81abc1e75c6294e4eaf6be7fb48ccdcbb49f7abba7766cec43115471e5a210a", "773fa442ef54c1ba2cdfa6d7ff56dbbfef34ebd7ca89aee8f8e25d93634d7adb51ec0af680297e9e38a74eae70c2f55d7d2f244ceda34f565c80bbb3ce64670b", "0727a578bc3ceb571cae4bf8a30c7f3f858221d09beecc0bfdf4e9be98f460c23e830d137272f3cfa397e5c28f1406d1d8c8b577bf1c4013c33d52f5cde49906", "b6cc1062699761d72c7b75d103f30509ee24cd5c330f5b08d9a37ba5e0553e553f7f8446adea1f257d36fbfe958a24700bf9720712b80c5ee1d3ae75b9f7110e", "b85823305fa178ac1703d652199f0fed35648f44c2813a8f15ef2e3d8a1a13e92db30754c4afecd5075d9c29ef1e1cdc5ac3a32ef7f6d21a3fd123b2ba019203", "5a4d4804f7ce5bfa59162ebd8a6a430ab120549345eb8bf5cbce4c248fee84d056a78836c5ca0d00fc9984186a836b22883d0d026e5e914bd0df359628c4930f", "eb77d23762502f1160a76a8414aa8e44271a12e23c93895893bc0390b898a639d8ba185408bf665a26ce084da5a864814b74ab67a82b9fee468499ebb6136c07", "43d9db0cec5a3918ee5ec9287965514bef74e6058b7b2bc28a29ef35bdcd8fb3c9c8120d0fdc487d8f2fc32cfe6503c61cb84476c45b45ca579e720596a0db03", "53caeb00c825a02788247993d8f96a1713a35ce5f6d3319d9f856e038b3d5a30f2dfe184907f1490f52bc1dff31664072b74cc7e7ae2d52234ac592212423302", "a3c6d1be4a1da49c06b29d28b9a538248ce46422b8cb522d89cd4cbc32bd2f88696598bf7b4287a4a24acf239a07c033663e6055dcd251ec722f3a9fbd300a02", "b3388370b3be8a64634a613ee0dd42eac7fc4820e09b2722ed97c3b0a34645917cd52a41fdb2d5bea5a3171be352be0e04871c84ee91c3cc40dea0be9caa6102", "41ec8990922a06bc5bd272802e7fc7a187c34c323e5d1ce7353f03d01eaf050c4a8ee70238dbcac49ea2f2e3217014bc61ce8485b4b8a2559bb9ddf06b312f01", "3cecb0b108449adaf1fc98168c8d810ea785d265ee5cdb21d7e2c1aff800b6996e19236dae6e10af83281129d5bf7c0e1d2f9ee769cc4c3aeb903842f487e70f", "dd67eefab5c0bca27a2e90b1c66781df1996b0cf88792917358d3855c123516cf6a400c4f6eef092308e94b63b3c246009544aaaabc513569f83c1b54f4b2405", "356db1f67aa37d738dc31b554c4e3087841983c065bbcf167cac01d7fae5f74c320822275ec484a191b777f77119433ca2621caeccdd728374af9205bb5e5401", "d34b8e9949f57052bb6ebf5d8444f74e11bfd84da047e1662095154d698e093088a94c9c1321132b17694b7c07f54421179d0d5f05d6a44ac5b0cd844e5d880e", "3ad93049aa3526520732fd07dccec1881a604136e8febf38b073e1043db7035674b762876b0cf0e78235ce4b40f9806df38093e7de6bcb2613acf13e4585620b", "e9568f3cb4709d62266aaf884bd9aa14e68f28b9d49f03f8a30e6845adff51a409d9f9a2b6e9fdebef5768227cfda04f38b4c6a7712413d936894263cf2f7f08", "23ab01ce396ae086a444576eeb0c347cc35ba23dafac5e4b6613df4a84b7a363896b5687d05e2937d06b68cab262222eab5a0a429257021948720f40066b0900", "0c00929fb166a598871338eaac63bfb16acd73a0325c1b8c61e42ebd181e7f0457e35242cf8e6d1e226c7e1a98e85b6bd12a9f952d8ee28e02f56d580df8d607", "de870929242238195c7d4093cf2fe16b3da0ddba758c0209da1384ef0dc6a5018c765e48a66928eb891c602b24ce417ad6ce1414b56a0881457279809e054b03", "1f7e7416d4233a85b2d68b2580b665ca46fbf2f9ef607670ceb0659ca9da6a2f955ab8ca179baa399c4ea4f4174e95580cec37e630f9751086d985a0203e530e", "752cd790e2b9975f6ddbba27d73ba49ad8fcc8eeeecff0a2606b1ceaf939d2925e39d12dcb190f6bc648f65aa1d75f44ba1b53c06de886a39fa97fb0d8a8f704", "8530658de209d530137aeda8521a02e47bca7e29dc6da98e67fbe086e3f8cdc83523f101a779df511818d8b85bf65a4a59e3a1f7abd9004f5782d8968c8dd30a", "77b14eabf758517aa1ed6944ad2aaae8f18a0e0ae92e89743d60dd312817c3d9cbb9e2bafc8b63f009dbed0928c7ac98e562da3bb3a7667f7f7d3b851d9d5501", "70840804b93eee58509b780989c2aff0a6c92580368271d7d187a7bdb52915c4aced64c9f0f3addc42a3a1fd5f0af745c5409cf767d67f17d12cc92cba55d005", "c8974701d1f55baf846f38976e44ac6fde65c92115f5995dabfd8bd9cfad9868eadbf6b346ab6df82fdbe7b772a3b695d4a6458e0c3846c3f65380e68cd4c506", "4fd9185b3460544d96cc373037bfe1dc763edacb84856af6580e477245467c497e4408370a821c9bdbe93de79180749796e508d22fa4c40485548336ebad3f00", "c8fe860136d2d4396fb280c4010113e27fbdf0a8aff5fc6d9e4f7745d4fce5bd22040258438adb7f386286316ecba024c8204e767b758ee368a62ee9452a380d",];

    // pub const PUB_KEYS: [&str; NUM_SIGS] = [
    //     "02f80695f0a4a2308246c88134b2de759e347d527189742dd42e98724bd5a9bc",
    //     "04d3b737505bfbf1eee1375118f8d584302f2da7eefaa3c5d7b095f3cb485938",
    //     "064a31abd0d2431f3109a44c8b00e724f56731a99852408c83985157fa6276da",
    //     "092005a6f7a58a98df5f9b8d186b9877f12b603aa06c7debf0f610d5a49f9ed7",
    //     "0a978fd659c69448273e35554e21bac35458fe2b199f8b8fb81a6488ee99c734",
    //     "14574d49c457d877e91db73a93ac8ca5fc595ca25c25f156398f32322dd71f59",
    //     "1e05e4b40cf57ae8965cac3ea994a135f020b6a4e02ac478d3025dfe2f33d12c",
    //     "244f13c0835db4a3909cee6106b276684aba0f8d8b1b0ba02dff4d659b081adf",
    //     "262b5e095b309af2b0eae1c554e03b6cc4a5a0df207b662b329623f27fdce8d0",
    //     "290d9479d0c1998373900dfb72900d1c9f55c018dd7eeed4ce0e988bb3da03a2",
    //     "2f4515da05794e97d8a4b15679913ca844fa4e25cbe936aa92afce02ceac8071",
    //     "301d8f635ec49e983cc537c4b81399bb24027ac4be709ce1a4eeb448e98a9aec",
    //     "3ed9fc282b5a8c2203d79501f4d1ba9b673196c6684b17bba0e3380637ddf578",
    //     "496f13c8d934c49cf357ff744fe0adb9ba8bb4c0a6697194ab6c807be92f590a",
    //     "53bdf319c8bf5992e7ad6215aa2dda2db44e6cc0caad127544a0f83362bca022",
    //     "597f675a8d47cb4165e9eede08ef9f7cb5906cbc9b44da080a21f9f9e905f730",
    //     "5b8afe90b542f6e0fffb744f1dba74e34bb4d3ea6c84e49796f5e549781a2f5c",
    //     "5da3581ce065c8037217ecf718fc9b97e40bd40f36038f1136135c9ee5a9d5f2",
    //     "61771bd363e7deb0e9f260e9a7e0e36e646cdb30ae5e8b7ed55ce45411a4ac4c",
    //     "69cfb82e998e36468e79b3eaeec213d9ec9d76e25edc1b65d41c85af32cca365",
    //     "6d5e025e08e3ec72531c476098287b13295fa606fab8275418e0c4c54f236c9e",
    //     "6db5e3b16a2cc4fc1de701c21a073981eb1a50caeb2e0294a21b786e92f08475",
    //     "73071024c4dfbc81863e7201db88fc1a6fa6dcc4f6f2f0a8a33ef8ff3ef94473",
    //     "73fbfdaa00a5205310cb0d1bd54175647482fae300cc66b36e7846e82288e9f0",
    //     "7584413f927bee5a607d36d9eedc451ad5dab66af39d53c548649260e0676014",
    //     "8207bf4bf7e95006f8da5463b44c77c8e334456e2c4c4d4cd5ffb74e8e45d071",
    //     "840bd86fa97ef694a34fd47740c2d44ff7378d773ee090903796a719697e67d8",
    //     "88554d82b1e63bedeb3fe9bd7754c7deccdfe277bcbfad4bbaff6302d3488bd2",
    //     "88ae88472391e18ec1b416f69ccf13399c4a0f6cc6abe662a88e033fb3f74595",
    //     "8c9b6893834f9af8cf8410ed26f6e9c8029891b9b08e36d2706226d420580423",
    //     "94028d6eafa1c0d6218077d174cc59cea6f2ea17ef1c002160e549f43b03112b",
    //     "95a90ca29491de8982145611188f1a18bba22e29371d0ab494cd8bd0f02ab5b5",
    //     "95fefa2edb64720ba97bd4d5f82614236b3a1f5deb344df02d095fccfe1db9b0",
    //     "964906e479f2172f4eaea2446e44964680213193ac981c3ca77015df8b02f4c8",
    //     "978a06ff17956afd5cbb8a1e85267dccad9ea0aef5cb75f1c22af1baf2901358",
    //     "a1b99a0a349691e80429667831f9b818431514bb2763e26e94a65428d22f3827",
    //     "a8565f4f6e753fc7942fa29051e258da2e06d13b352220b9eadb31d8ead7f88b",
    //     "a9e1f3861a27d63510f7ff632506216abd09c1f9c0233c558bda127ef07f018e",
    //     "ad19893bd78c611087f21c0f33ca1edbb053aa5fdb3aa00ff6a4ba7d6ba286b8",
    //     "c3322124f3e4161dd1b57a7bb412c9352084215b1ceb15550850b3452a32d467",
    //     "c39ad005c6ce056b0fa7baff65f4d77ed645aef3488080396c568f1c7989c60a",
    //     "c4a1b230780455f5f18ec4b84386cde822e62ceefa8d32fbca0b9f398e434c86",
    //     "dd66c7a479fe0352fda60876f173519b4e567f0a0f0798d25e198603c1c5569b",
    //     "dea0d85a4cf74a91c85d7f90873bfbdc40c8c939377bec9a26d66b895a1bbeaa",
    //     "df4bc26d8aeb83ed380c04fe8aa4f23678989ebffd29c647eb96d4999b4a6736",
    //     "e2eefb4dd06c686ca29cdb2173a53ec8322a6cb9128e3b7cdf4bf5a5c2e8906b",
    //     "eab6f3a26d7fd65eff7c72a539dbeee68a9497476b69082958eae7d6a7f0f1d5",
    //     "f86c72277ed20efe15bab1abcf34656e7d2336e42133fa99331e874b5458b28f",
    //     "fa74875360c9fab71916824e96bccf6417c4ea23f01c298f31bc8775689dd93b",
    //     "fc8f03e8f87dc94bcb10b18b8ad3d2394533c41b51c0fc6f0c89d7498bc07cf2",
    //     "fe249696419a67cb9e0f29d0297d840048bddf6612a383f37d7b96348a1bc5f1",
    // ];

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sig_verification_air() {
        type C = CurtaPoseidonGoldilocksConfig;
        type SC = PoseidonGoldilocksStarkConfig;
        type E = Ed25519;

        setup_logger();

        let mut timing = TimingTree::new("Ed25519 Sig Verification aggregation", log::Level::Debug);

        let sig_verification_air =
            SigVerificationAir::<GoldilocksField, GoldilocksCubicParameters>::new();

        let SigVerificationAir {
            air,
            trace_data,
            public_keys,
            sigs_s_limbs,
            sigr_s,
            h_s_limbs,
            p1_s,
            p1_scalar_mul_gadget,
            p1_set_last,
            p1_set_bit,
            p2_s,
            p2_scalar_mul_gadget,
            p2_set_last,
            p2_set_bit,
        } = sig_verification_air;

        let trace_generator = ArithmeticGenerator::new(trace_data, 1 << 16);

        let writer = trace_generator.new_writer();
        timed!(
            timing,
            "Writing public keys",
            (0..256).into_par_iter().for_each(|i| {
                let compressed_p_bytes = hex::decode(PUB_KEYS[i % NUM_SIGS]).unwrap();
                let compressed_p = CompressedEdwardsY(compressed_p_bytes.try_into().unwrap());

                writer.write_ec_compressed_point(&public_keys[i], &compressed_p, i);
            })
        );

        timed!(
            timing,
            "Writing sigs",
            (0..256).into_par_iter().for_each(|i| {
                let sig_bytes = hex::decode(SIGS[i % NUM_SIGS]).unwrap();
                let sig_r = CompressedEdwardsY(sig_bytes[0..32].try_into().unwrap());

                writer.write_ec_compressed_point(&sigr_s[i], &sig_r, i);

                let sig_s_biguint = BigUint::from_bytes_le(sig_bytes[32..64].try_into().unwrap());
                let limbs = sig_s_biguint.to_u32_digits();
                for (j, limb) in limbs.iter().enumerate() {
                    writer.write(
                        &sigs_s_limbs[i].get(j),
                        &GoldilocksField::from_canonical_u32(*limb),
                        i,
                    );
                }
            })
        );

        timed!(
            timing,
            "Writing h limbs",
            (0..256).into_par_iter().for_each(|i| {
                let h_biguint = BigUint::from_bytes_le(&hex::decode(H[i % NUM_SIGS]).unwrap());
                let limbs = h_biguint.to_u32_digits();
                for (j, limb) in limbs.iter().enumerate() {
                    writer.write(
                        &h_s_limbs[i].get(j),
                        &GoldilocksField::from_canonical_u32(*limb),
                        i,
                    );
                }
            })
        );

        writer.write_global_instructions(&trace_generator.air_data);

        let (tx, rx) = channel();
        let res = p1_scalar_mul_gadget.double_and_add_gadget.result;
        let temp = p1_scalar_mul_gadget.double_and_add_gadget.temp;
        let scalar_bit = p1_scalar_mul_gadget.double_and_add_gadget.bit;
        let nb_bits = Ed25519::nb_scalar_bits();
        let tx_vec = (0..256).map(|_| tx.clone()).collect::<Vec<_>>();
        tx_vec.into_par_iter().enumerate().for_each(|(k, tx)| {
            let starting_row = 256 * k;
            writer.write_ec_point(&res, &Ed25519::neutral(), starting_row);
            writer.write_ec_point(&temp, &points[k], starting_row);
            let scalar_bits = biguint_to_bits_le(&scalars[k], nb_bits);
            for (i, bit) in scalar_bits.iter().enumerate() {
                let f_bit = GoldilocksField::from_canonical_u8(*bit as u8);
                writer.write(&scalar_bit, &f_bit, starting_row + i);
                writer.write_row_instructions(&trace_generator.air_data, starting_row + i);
            }
            tx.send((k, &points[k] * &scalars[k])).unwrap();
        });
        drop(tx);
        for (i, res) in rx.iter() {
            writer.write_ec_point(&p1_s[i], &res, 0);
        }
        for j in (0..(1 << 16)).rev() {
            writer.write_instruction(&p1_set_last, j);
            writer.write_instruction(&p1_set_bit, j);
        }

        let stark = Starky::new(air);
        let config = SC::standard_fast_config(1 << 16);
        let public_inputs = writer.public().unwrap().clone();

        let proof = timed!(
            timing,
            "Generate STARK proof",
            StarkyProver::<GoldilocksField, C, 2>::prove(
                &config,
                &stark,
                &trace_generator,
                &public_inputs,
            )
            .unwrap()
        );

        // Verify the proof as a stark
        StarkyVerifier::verify(&config, &stark, proof, &public_inputs).unwrap();

        timing.print();

        let mut builder = DefaultBuilder::new();
        let virtual_proof = builder.api.add_virtual_stark_proof(&stark, &config);

        let mut pw = PartialWitness::new();
        // Set public inputs.
        let public_input_targets = builder.api.add_virtual_targets(public_inputs.len());
        for (&pi_t, &pi) in public_input_targets.iter().zip(public_inputs.iter()) {
            pw.set_target(pi_t, pi);
        }
        builder
            .api
            .verify_stark_proof(&config, &stark, &virtual_proof, &public_input_targets);

        let generator = SimpleStarkWitnessGenerator::new(
            config,
            stark,
            virtual_proof,
            public_input_targets,
            trace_generator,
        );
        builder.add_simple_generator(generator);

        let data = builder.build().data;
        let mut timing = TimingTree::new("recursive_proof", log::Level::Debug);
        let recursive_proof =
            plonky2::plonk::prover::prove(&data.prover_only, &data.common, pw, &mut timing)
                .unwrap();
        timing.print();
        data.verify(recursive_proof).unwrap();
    }
}
