use curta::chip::ec::edwards::ed25519::gadget::CompressedPointWriter;
use curta::chip::ec::edwards::ed25519::params::Ed25519BaseField;
use curta::chip::trace::generator::ArithmeticGenerator;
use curta::chip::utils::bigint_into_u16_digits;
use curta::maybe_rayon::*;
use curta::plonky2::stark::config::StarkyConfig;
use curta::plonky2::stark::prover::StarkyProver;
use curta::plonky2::stark::verifier::StarkyVerifier;
use curta::plonky2::stark::Starky;
use curve25519_dalek::edwards::CompressedEdwardsY;
use log::debug;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::frontend::curta::ec::point::CompressedEdwardsYVariable;
use crate::frontend::curta::ec::sig_verification::air::Ed25519VerificationAir;
use crate::frontend::curta::field::variable::FieldVariable;
use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::{PlonkParameters, ValueStream};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ed25519VerificationHint {}

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for Ed25519VerificationHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        const NUM_VERIFICATIONS: usize = 256;

        let mut public_keys_values = Vec::with_capacity(NUM_VERIFICATIONS);
        let mut sigr_values = Vec::with_capacity(NUM_VERIFICATIONS);
        let mut sigs_values = Vec::with_capacity(NUM_VERIFICATIONS);
        let mut h_values = Vec::with_capacity(NUM_VERIFICATIONS);

        for _ in 0..NUM_VERIFICATIONS {
            let pk: CompressedEdwardsY = input_stream
                .read_value::<CompressedEdwardsYVariable>()
                .into();
            let sig_r: CompressedEdwardsY = input_stream
                .read_value::<CompressedEdwardsYVariable>()
                .into();
            let sig_s: BigUint = input_stream
                .read_value::<FieldVariable<Ed25519BaseField>>()
                .into();
            let h: BigUint = input_stream
                .read_value::<FieldVariable<Ed25519BaseField>>()
                .into();
            public_keys_values.push(pk);
            sigr_values.push(sig_r);
            sigs_values.push(sig_s);
            h_values.push(h);
        }

        let Ed25519VerificationAir {
            air,
            trace_data,
            public_keys,
            sigs_s_limbs,
            sigr_s,
            h_s_limbs,
        } = Ed25519VerificationAir::<L::Field, L::CubicParams>::new();

        let trace_generator = ArithmeticGenerator::new(trace_data, 1 << 16);
        let writer = trace_generator.new_writer();

        public_keys
            .par_iter()
            .zip(public_keys_values.par_iter())
            .for_each(|(pk, pk_value)| {
                writer.write_ec_compressed_point(pk, pk_value, 0);
            });

        sigr_s
            .par_iter()
            .zip(sigr_values.par_iter())
            .for_each(|(sigr, sigr_value)| {
                writer.write_ec_compressed_point(sigr, sigr_value, 0);
            });

        sigs_s_limbs
            .par_iter()
            .zip(sigs_values.par_iter())
            .for_each(|(sigs, sigs_value)| {
                writer.write_array(sigs, bigint_into_u16_digits(sigs_value, 16), 0);
            });

        h_s_limbs
            .par_iter()
            .zip(h_values.par_iter())
            .for_each(|(h, h_value)| {
                writer.write_array(h, bigint_into_u16_digits(h_value, 16), 0);
            });

        writer.write_global_instructions(&trace_generator.air_data);

        let stark = Starky::new(air);
        let config = StarkyConfig::standard_fast_config(1 << 16);

        let public_inputs: Vec<L::Field> = writer.public().unwrap().clone();

        let proof = StarkyProver::<L::Field, L::CurtaConfig, D>::prove(
            &config,
            &stark,
            &trace_generator,
            &public_inputs,
        )
        .unwrap();
        debug!("Generated proof");

        // Verify the proof to make sure it's valid.
        StarkyVerifier::verify(&config, &stark, proof.clone(), &public_inputs).unwrap();

        // Return the aggregated public key and the proof.
        output_stream.write_stark_proof(proof);
        output_stream.write_slice(&public_inputs);
    }
}

#[cfg(test)]
mod tests {

    use curta::air::RAirData;
    use curta::math::goldilocks::cubic::GoldilocksCubicParameters;

    use super::*;
    use crate::frontend::curta::field::variable::FieldVariable;
    use crate::prelude::*;
    use crate::utils::setup_logger;

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
    fn test_ed25519_verification_hint() {
        setup_logger();

        type F = GoldilocksField;
        type R = GoldilocksCubicParameters;
        type C = <DefaultParameters as PlonkParameters<2>>::CurtaConfig;

        let num_rows = 1 << 16;
        const NUM_VERIFICATIONS: usize = 256;

        let mut builder = DefaultBuilder::new();

        let Ed25519VerificationAir { air, .. } = Ed25519VerificationAir::<F, R>::new();

        let stark = Starky::new(air);
        let config = StarkyConfig::<C, 2>::standard_fast_config(num_rows);

        let mut input_stream = VariableStream::new();
        for i in 0..NUM_VERIFICATIONS {
            let compressed_p_bytes = hex::decode(PUB_KEYS[i % NUM_SIGS]).unwrap();
            let compressed_p = builder.constant::<CompressedEdwardsYVariable>(CompressedEdwardsY(
                compressed_p_bytes.try_into().unwrap(),
            ));

            let sig_bytes = hex::decode(SIGS[i % NUM_SIGS]).unwrap();
            let sig_r = builder.constant::<CompressedEdwardsYVariable>(CompressedEdwardsY(
                sig_bytes[0..32].try_into().unwrap(),
            ));

            let sig_s_biguint = builder.constant::<FieldVariable<Ed25519BaseField>>(
                BigUint::from_bytes_le(sig_bytes[32..64].try_into().unwrap()),
            );

            let h_biguint = builder.constant::<FieldVariable<Ed25519BaseField>>(
                BigUint::from_bytes_le(&hex::decode(H[i % NUM_SIGS]).unwrap()),
            );

            input_stream.write::<CompressedEdwardsYVariable>(&compressed_p);
            input_stream.write::<CompressedEdwardsYVariable>(&sig_r);
            input_stream.write::<FieldVariable<Ed25519BaseField>>(&sig_s_biguint);
            input_stream.write::<FieldVariable<Ed25519BaseField>>(&h_biguint);
        }

        let hint = Ed25519VerificationHint {};
        let outputs = builder.hint(input_stream, hint);

        // Read the stark proof and stark public inputs from the output stream.
        let proof = outputs.read_stark_proof(&mut builder, &stark, &config);
        let public_inputs = outputs.read_exact_unsafe(&mut builder, stark.air.num_public_inputs());
        builder.verify_stark_proof(&config, &stark, proof, &public_inputs);

        let circuit = builder.build();
        let input = circuit.input();

        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
