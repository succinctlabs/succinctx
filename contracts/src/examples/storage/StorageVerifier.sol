// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

library Pairing {
    uint256 constant PRIME_Q = 21888242871839275222246405745257275088696311157297823662689037894645226208583;

    struct G1Point {
        uint256 X;
        uint256 Y;
    }

    // Encoding of field elements is: X[0] * z + X[1]
    struct G2Point {
        uint256[2] X;
        uint256[2] Y;
    }

    /*
     * @return The negation of p, i.e. p.plus(p.negate()) should be zero.
     */
    function negate(G1Point memory p) internal pure returns (G1Point memory) {
        // The prime q in the base field F_q for G1
        if (p.X == 0 && p.Y == 0) {
            return G1Point(0, 0);
        } else {
            return G1Point(p.X, PRIME_Q - (p.Y % PRIME_Q));
        }
    }

    /*
     * @return The sum of two points of G1
     */
    function plus(G1Point memory p1, G1Point memory p2) internal view returns (G1Point memory r) {
        uint256[4] memory input;
        input[0] = p1.X;
        input[1] = p1.Y;
        input[2] = p2.X;
        input[3] = p2.Y;
        bool success;

        // solium-disable-next-line security/no-inline-assembly
        assembly {
            success := staticcall(sub(gas(), 2000), 6, input, 0xc0, r, 0x60)
            // Use "invalid" to make gas estimation work
            switch success
            case 0 { invalid() }
        }

        require(success, "pairing-add-failed");
    }

    /*
     * Same as plus but accepts raw input instead of struct
     * @return The sum of two points of G1, one is represented as array
     */
    function plus_raw(uint256[4] memory input, G1Point memory r) internal view {
        bool success;

        // solium-disable-next-line security/no-inline-assembly
        assembly {
            success := staticcall(sub(gas(), 2000), 6, input, 0xc0, r, 0x60)
            // Use "invalid" to make gas estimation work
            switch success
            case 0 { invalid() }
        }

        require(success, "pairing-add-failed");
    }

    /*
     * @return The product of a point on G1 and a scalar, i.e.
     *         p == p.scalar_mul(1) and p.plus(p) == p.scalar_mul(2) for all
     *         points p.
     */
    function scalar_mul(G1Point memory p, uint256 s) internal view returns (G1Point memory r) {
        uint256[3] memory input;
        input[0] = p.X;
        input[1] = p.Y;
        input[2] = s;
        bool success;
        // solium-disable-next-line security/no-inline-assembly
        assembly {
            success := staticcall(sub(gas(), 2000), 7, input, 0x80, r, 0x60)
            // Use "invalid" to make gas estimation work
            switch success
            case 0 { invalid() }
        }
        require(success, "pairing-mul-failed");
    }

    /*
     * Same as scalar_mul but accepts raw input instead of struct,
     * Which avoid extra allocation. provided input can be allocated outside and re-used multiple times
     */
    function scalar_mul_raw(uint256[3] memory input, G1Point memory r) internal view {
        bool success;

        // solium-disable-next-line security/no-inline-assembly
        assembly {
            success := staticcall(sub(gas(), 2000), 7, input, 0x80, r, 0x60)
            // Use "invalid" to make gas estimation work
            switch success
            case 0 { invalid() }
        }
        require(success, "pairing-mul-failed");
    }

    /* @return The result of computing the pairing check
     *         e(p1[0], p2[0]) *  .... * e(p1[n], p2[n]) == 1
     *         For example,
     *         pairing([P1(), P1().negate()], [P2(), P2()]) should return true.
     */
    function pairing(
        G1Point memory a1,
        G2Point memory a2,
        G1Point memory b1,
        G2Point memory b2,
        G1Point memory c1,
        G2Point memory c2,
        G1Point memory d1,
        G2Point memory d2
    ) internal view returns (bool) {
        G1Point[4] memory p1 = [a1, b1, c1, d1];
        G2Point[4] memory p2 = [a2, b2, c2, d2];
        uint256 inputSize = 24;
        uint256[] memory input = new uint256[](inputSize);

        for (uint256 i = 0; i < 4; i++) {
            uint256 j = i * 6;
            input[j + 0] = p1[i].X;
            input[j + 1] = p1[i].Y;
            input[j + 2] = p2[i].X[0];
            input[j + 3] = p2[i].X[1];
            input[j + 4] = p2[i].Y[0];
            input[j + 5] = p2[i].Y[1];
        }

        uint256[1] memory out;
        bool success;

        // solium-disable-next-line security/no-inline-assembly
        assembly {
            success := staticcall(sub(gas(), 2000), 8, add(input, 0x20), mul(inputSize, 0x20), out, 0x20)
            // Use "invalid" to make gas estimation work
            switch success
            case 0 { invalid() }
        }

        require(success, "pairing-opcode-failed");

        return out[0] != 0;
    }
}

contract Verifier {
    using Pairing for *;

    uint256 constant SNARK_SCALAR_FIELD = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
    uint256 constant PRIME_Q = 21888242871839275222246405745257275088696311157297823662689037894645226208583;

    struct VerifyingKey {
        Pairing.G1Point alfa1;
        Pairing.G2Point beta2;
        Pairing.G2Point gamma2;
        Pairing.G2Point delta2;
    }
    // []G1Point IC (K in gnark) appears directly in verifyProof

    struct Proof {
        Pairing.G1Point A;
        Pairing.G2Point B;
        Pairing.G1Point C;
    }

    function verifyingKey() internal pure returns (VerifyingKey memory vk) {
        vk.alfa1 = Pairing.G1Point(
            uint256(15065378376707638670416117646191938546464823528318298434911407668648410017312),
            uint256(14328565780794488154843107537696396004559344718567701918290331837021769695183)
        );
        vk.beta2 = Pairing.G2Point(
            [
                uint256(21608662413593678936351578876605429104372744341332808284734169211800668886021),
                uint256(14272630991227820574186833459180341377383838929920263355368708236729516236510)
            ],
            [
                uint256(16641043772513434533950209102074465229233476042166934559766744767810330184564),
                uint256(6162122694319393529287233670241265074511780247270252331117359314308552821119)
            ]
        );
        vk.gamma2 = Pairing.G2Point(
            [
                uint256(13132182365077749935258621721022525042673781526193917683715236922498527090287),
                uint256(15561074370462920423820148502023399216464240426394708700564724325660560082718)
            ],
            [
                uint256(10082726381035042218296813991627285842615800319245653642200496444791944486189),
                uint256(19936166073830122769051017946410845224800149361868042760143437401219984978115)
            ]
        );
        vk.delta2 = Pairing.G2Point(
            [
                uint256(18835698337094098366375289269375058880930688910548530995640100337968257621142),
                uint256(2538885030664125768153917843471118992747126086079950113951128163711148595911)
            ],
            [
                uint256(3381368293223064763369350882484383507924963445101730404619590717560533944931),
                uint256(20044872545096760477413396293539672831863209972531251378473847402381937985325)
            ]
        );
    }

    // accumulate scalarMul(mul_input) into q
    // that is computes sets q = (mul_input[0:2] * mul_input[3]) + q
    function accumulate(
        uint256[3] memory mul_input,
        Pairing.G1Point memory p,
        uint256[4] memory buffer,
        Pairing.G1Point memory q
    ) internal view {
        // computes p = mul_input[0:2] * mul_input[3]
        Pairing.scalar_mul_raw(mul_input, p);

        // point addition inputs
        buffer[0] = q.X;
        buffer[1] = q.Y;
        buffer[2] = p.X;
        buffer[3] = p.Y;

        // q = p + q
        Pairing.plus_raw(buffer, q);
    }

    /*
     * @returns Whether the proof is valid given the hardcoded verifying key
     *          above and the public inputs
     */
    function verifyProof(uint256[2] memory a, uint256[2][2] memory b, uint256[2] memory c, uint256[65] memory input)
        public
        view
        returns (bool r)
    {
        Proof memory proof;
        proof.A = Pairing.G1Point(a[0], a[1]);
        proof.B = Pairing.G2Point([b[0][0], b[0][1]], [b[1][0], b[1][1]]);
        proof.C = Pairing.G1Point(c[0], c[1]);

        // Make sure that proof.A, B, and C are each less than the prime q
        require(proof.A.X < PRIME_Q, "verifier-aX-gte-prime-q");
        require(proof.A.Y < PRIME_Q, "verifier-aY-gte-prime-q");

        require(proof.B.X[0] < PRIME_Q, "verifier-bX0-gte-prime-q");
        require(proof.B.Y[0] < PRIME_Q, "verifier-bY0-gte-prime-q");

        require(proof.B.X[1] < PRIME_Q, "verifier-bX1-gte-prime-q");
        require(proof.B.Y[1] < PRIME_Q, "verifier-bY1-gte-prime-q");

        require(proof.C.X < PRIME_Q, "verifier-cX-gte-prime-q");
        require(proof.C.Y < PRIME_Q, "verifier-cY-gte-prime-q");

        // Make sure that every input is less than the snark scalar field
        for (uint256 i = 0; i < input.length; i++) {
            require(input[i] < SNARK_SCALAR_FIELD, "verifier-gte-snark-scalar-field");
        }

        VerifyingKey memory vk = verifyingKey();

        // Compute the linear combination vk_x
        Pairing.G1Point memory vk_x = Pairing.G1Point(0, 0);

        // Buffer reused for addition p1 + p2 to avoid memory allocations
        // [0:2] -> p1.X, p1.Y ; [2:4] -> p2.X, p2.Y
        uint256[4] memory add_input;

        // Buffer reused for multiplication p1 * s
        // [0:2] -> p1.X, p1.Y ; [3] -> s
        uint256[3] memory mul_input;

        // temporary point to avoid extra allocations in accumulate
        Pairing.G1Point memory q = Pairing.G1Point(0, 0);

        vk_x.X = uint256(7654151173307285122173887557943556592926627323803440265411428835933171698030); // vk.K[0].X
        vk_x.Y = uint256(1267569108851906122418933906145913022663962383604792269197553633453069852082); // vk.K[0].Y
        mul_input[0] = uint256(0); // vk.K[1].X
        mul_input[1] = uint256(0); // vk.K[1].Y
        mul_input[2] = input[0];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[1] * input[0]
        mul_input[0] = uint256(10855663286981623084541363489660102193206622537607772833968317321581433303751); // vk.K[2].X
        mul_input[1] = uint256(12156643609098206979852635788280459246023978138921482912702542075055217047953); // vk.K[2].Y
        mul_input[2] = input[1];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[2] * input[1]
        mul_input[0] = uint256(379076147614366186237852555128877932752707022181195558773951575810360280983); // vk.K[3].X
        mul_input[1] = uint256(20275768584033756898412813982387176725473685447816316945827890314022804466668); // vk.K[3].Y
        mul_input[2] = input[2];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[3] * input[2]
        mul_input[0] = uint256(2030788471383854807393572254897579868967448362247501984749898450508158620589); // vk.K[4].X
        mul_input[1] = uint256(16007079715104718539427533809679786172137912244729461946995485300671697674877); // vk.K[4].Y
        mul_input[2] = input[3];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[4] * input[3]
        mul_input[0] = uint256(3659729532935328078600286048545654261948161883356705951517335433023302604447); // vk.K[5].X
        mul_input[1] = uint256(6719541045159043658238314026173320015187154961043383047031507241065433979987); // vk.K[5].Y
        mul_input[2] = input[4];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[5] * input[4]
        mul_input[0] = uint256(9023351031242744707107208505085543026451788790872710445827418724631815295840); // vk.K[6].X
        mul_input[1] = uint256(2498974708525416644646883774124922731192136194723500432208844468584751225497); // vk.K[6].Y
        mul_input[2] = input[5];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[6] * input[5]
        mul_input[0] = uint256(11760246946898200849775229670108235157754487907263669813035215235872743186597); // vk.K[7].X
        mul_input[1] = uint256(5382428339030687870768870582444025050484374185231261099126636032475175446148); // vk.K[7].Y
        mul_input[2] = input[6];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[7] * input[6]
        mul_input[0] = uint256(332103316032981202977921170491728275515984536882295042122024740487031669015); // vk.K[8].X
        mul_input[1] = uint256(484223038059012831998672398465994235268759953393977542517196949254233028660); // vk.K[8].Y
        mul_input[2] = input[7];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[8] * input[7]
        mul_input[0] = uint256(5344906529970141689021459145557374163117422738842071706750631467728871290394); // vk.K[9].X
        mul_input[1] = uint256(7731978403516757842874921595990408683343135883653940410950541995815224341199); // vk.K[9].Y
        mul_input[2] = input[8];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[9] * input[8]
        mul_input[0] = uint256(10362237886978658404784044741070389359500890596626548033780244659029433510363); // vk.K[10].X
        mul_input[1] = uint256(6285629694982155865291087094189363562595967452963101197020929967022416193184); // vk.K[10].Y
        mul_input[2] = input[9];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[10] * input[9]
        mul_input[0] = uint256(1936826942872850266674041622347967683160440267203145838962497651901744863132); // vk.K[11].X
        mul_input[1] = uint256(2489418929250071077827953016163896767277738430716133365272418875838627300251); // vk.K[11].Y
        mul_input[2] = input[10];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[11] * input[10]
        mul_input[0] = uint256(12779474367520445587507333272472534286610205070395166091355451386189619020759); // vk.K[12].X
        mul_input[1] = uint256(16098399295897998668449640624417961627391582297976508801357763570695142130577); // vk.K[12].Y
        mul_input[2] = input[11];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[12] * input[11]
        mul_input[0] = uint256(443172960809137696796573525061835890990476988281640266727036593096513946387); // vk.K[13].X
        mul_input[1] = uint256(20448435027686077373503601710307109999095366634908587134616976911597013705326); // vk.K[13].Y
        mul_input[2] = input[12];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[13] * input[12]
        mul_input[0] = uint256(4420010271881666237419507746577855992446973907995876983937642148718830707448); // vk.K[14].X
        mul_input[1] = uint256(14540506650087850032683132977210992280987068520451687156325445797696065869484); // vk.K[14].Y
        mul_input[2] = input[13];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[14] * input[13]
        mul_input[0] = uint256(19641835005247180332480402296715294035949028476829642551362696889499513656838); // vk.K[15].X
        mul_input[1] = uint256(4812315591576788861668368040515410531600872313469076363556584755378733696050); // vk.K[15].Y
        mul_input[2] = input[14];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[15] * input[14]
        mul_input[0] = uint256(12640577748972357982630117727971303258087585863699713717297881582503076631360); // vk.K[16].X
        mul_input[1] = uint256(13065923353823232570476651439967308146274765135151428476341066795511706991767); // vk.K[16].Y
        mul_input[2] = input[15];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[16] * input[15]
        mul_input[0] = uint256(13572128760084308415140083506753360673600577607804820189016855697861704739224); // vk.K[17].X
        mul_input[1] = uint256(8442833044896943902431485896013816816589701482716364853929712353957738745885); // vk.K[17].Y
        mul_input[2] = input[16];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[17] * input[16]
        mul_input[0] = uint256(12782767107221283109044396047775873882831103174902550450883267206197174805868); // vk.K[18].X
        mul_input[1] = uint256(21725909149821822419863341739266801796457530684451541722014143306092290491812); // vk.K[18].Y
        mul_input[2] = input[17];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[18] * input[17]
        mul_input[0] = uint256(11078573045918050069962548128864339547899985435179862834648073403084209835710); // vk.K[19].X
        mul_input[1] = uint256(12399059488837415085805609005448464807782897587253414030053853866684873097988); // vk.K[19].Y
        mul_input[2] = input[18];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[19] * input[18]
        mul_input[0] = uint256(10315226201173711467954752250803493769599555861590514433803248756358327104145); // vk.K[20].X
        mul_input[1] = uint256(2856544295114544565653429320946926740804556743379163983013096592795845303539); // vk.K[20].Y
        mul_input[2] = input[19];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[20] * input[19]
        mul_input[0] = uint256(9354895592827270566590945984304896833859741287021261458869270672256004059535); // vk.K[21].X
        mul_input[1] = uint256(15011309793775328188185572246572377707679615478035837762242785740323839285319); // vk.K[21].Y
        mul_input[2] = input[20];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[21] * input[20]
        mul_input[0] = uint256(13313466138074340718734302586098076097388950008677954748589938155741454888448); // vk.K[22].X
        mul_input[1] = uint256(12353745256729266438311891904289828780910344800526689870927542307621168813347); // vk.K[22].Y
        mul_input[2] = input[21];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[22] * input[21]
        mul_input[0] = uint256(15190018048332196524754887079224540675648717818816895400470708784186351975840); // vk.K[23].X
        mul_input[1] = uint256(10147458378909811594952055908189722911538701320475523176144463017748535907490); // vk.K[23].Y
        mul_input[2] = input[22];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[23] * input[22]
        mul_input[0] = uint256(234614104392190853693722220259832416753154027782029485053737056100888951760); // vk.K[24].X
        mul_input[1] = uint256(4128702334020980420987327656543100690673142825029758802716705558844651759585); // vk.K[24].Y
        mul_input[2] = input[23];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[24] * input[23]
        mul_input[0] = uint256(6931986590268588882910380631925893379786131874131718051994417017802935487518); // vk.K[25].X
        mul_input[1] = uint256(6553771184053803320674257601817296516132168924473495362665037590748879926835); // vk.K[25].Y
        mul_input[2] = input[24];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[25] * input[24]
        mul_input[0] = uint256(8007641449250544351042554455733868623978825618215306263257441983965247015306); // vk.K[26].X
        mul_input[1] = uint256(6989427805525814667453773033842061928861369548889935848958642430130423311990); // vk.K[26].Y
        mul_input[2] = input[25];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[26] * input[25]
        mul_input[0] = uint256(14741810725607764328249036718716826447801414725797362707614991969593535198061); // vk.K[27].X
        mul_input[1] = uint256(16924891809278267680585602860764437386367597260415854373705866086335127810942); // vk.K[27].Y
        mul_input[2] = input[26];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[27] * input[26]
        mul_input[0] = uint256(19606675076775461784511771066572876373775296642178524644407593031446474991396); // vk.K[28].X
        mul_input[1] = uint256(8606522840804950948717310490003731381536956837776527018324136881608861581571); // vk.K[28].Y
        mul_input[2] = input[27];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[28] * input[27]
        mul_input[0] = uint256(3378778428452374523256569312957451002365689609349537175414837663286905789890); // vk.K[29].X
        mul_input[1] = uint256(3398091315042692109385568436973712260471414367803233969505626729524803304405); // vk.K[29].Y
        mul_input[2] = input[28];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[29] * input[28]
        mul_input[0] = uint256(8552650451497299813794466582330968619836679761737303659681933318167757098631); // vk.K[30].X
        mul_input[1] = uint256(4217931479579097251137035220978681356363760563375323313336646382986482561900); // vk.K[30].Y
        mul_input[2] = input[29];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[30] * input[29]
        mul_input[0] = uint256(17389761499517884028227459747393901628845657128749005600031578978917606598569); // vk.K[31].X
        mul_input[1] = uint256(10555185012236734284382729900325079069840722015806083080596709578503001010042); // vk.K[31].Y
        mul_input[2] = input[30];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[31] * input[30]
        mul_input[0] = uint256(20522064923903915435529181641985382408182131990229432495749925727116732333618); // vk.K[32].X
        mul_input[1] = uint256(9262127328114327567334564923775981849508792353488856435505031509658876768785); // vk.K[32].Y
        mul_input[2] = input[31];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[32] * input[31]
        mul_input[0] = uint256(0); // vk.K[33].X
        mul_input[1] = uint256(0); // vk.K[33].Y
        mul_input[2] = input[32];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[33] * input[32]
        mul_input[0] = uint256(19805515723576526151865962480176526559600946331940416602919435733744559178939); // vk.K[34].X
        mul_input[1] = uint256(14436323825307532673910662135739037295632234829812967405189138584329725932226); // vk.K[34].Y
        mul_input[2] = input[33];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[34] * input[33]
        mul_input[0] = uint256(19769937692070903973717952756328400818768446873200827647850618096003799853020); // vk.K[35].X
        mul_input[1] = uint256(21577770339510253918286877921192503157062550953375939162858739967204290523211); // vk.K[35].Y
        mul_input[2] = input[34];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[35] * input[34]
        mul_input[0] = uint256(21463274337361306731827127358497242847186037166889108461214045419815817218996); // vk.K[36].X
        mul_input[1] = uint256(19499490089855815631318730286338300201490444832730925605965665171488963647858); // vk.K[36].Y
        mul_input[2] = input[35];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[36] * input[35]
        mul_input[0] = uint256(12057739572080112380123927889780603267198763016607327963671783717329310186493); // vk.K[37].X
        mul_input[1] = uint256(7303684643153206320562424737389940301747294453769658376891530120300287091977); // vk.K[37].Y
        mul_input[2] = input[36];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[37] * input[36]
        mul_input[0] = uint256(17217188987453618497424883038450136632316646488983452367886954124919864036693); // vk.K[38].X
        mul_input[1] = uint256(21375339730127974465467984229113021398656189521351737743163893047555883800246); // vk.K[38].Y
        mul_input[2] = input[37];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[38] * input[37]
        mul_input[0] = uint256(18787439887787093780339026828453522129057912122177599159715079795672616250109); // vk.K[39].X
        mul_input[1] = uint256(17851621594125688853191944994514545725947854063365545119007748240805316325955); // vk.K[39].Y
        mul_input[2] = input[38];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[39] * input[38]
        mul_input[0] = uint256(15022035693900431687276953568986992848817580330708047655879922986997313472221); // vk.K[40].X
        mul_input[1] = uint256(20284983706070006505302341041678459987145987163750240764970385624811896664406); // vk.K[40].Y
        mul_input[2] = input[39];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[40] * input[39]
        mul_input[0] = uint256(17682250282074192986606512022644465366834162256038110695035322304798987194845); // vk.K[41].X
        mul_input[1] = uint256(17735039760536327883984565482595346927072649564075439600588336978864313757679); // vk.K[41].Y
        mul_input[2] = input[40];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[41] * input[40]
        mul_input[0] = uint256(20899706779328863341678425647248526006380374041381805254108969921590054971843); // vk.K[42].X
        mul_input[1] = uint256(9208903989072050237486846027857559959173926655646571064322647318415126307423); // vk.K[42].Y
        mul_input[2] = input[41];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[42] * input[41]
        mul_input[0] = uint256(8094119844233180842261868250217351476703160601947508218059277542084472590858); // vk.K[43].X
        mul_input[1] = uint256(3931401739387971212950794367903322610453707604654345792724304050982429234275); // vk.K[43].Y
        mul_input[2] = input[42];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[43] * input[42]
        mul_input[0] = uint256(1316264156504633102498105591347442946770583029642110683067801761935109545631); // vk.K[44].X
        mul_input[1] = uint256(6410737200033529651526204670194859866494250065115817937275936426766906855619); // vk.K[44].Y
        mul_input[2] = input[43];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[44] * input[43]
        mul_input[0] = uint256(8804621950111225410125455844142464146394958064668792060504877361795418468316); // vk.K[45].X
        mul_input[1] = uint256(8580620024820260438211068784935002090479952452161813928224108672526860866015); // vk.K[45].Y
        mul_input[2] = input[44];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[45] * input[44]
        mul_input[0] = uint256(7112295527738970285127563356966652173102917507489227391418611658263599307482); // vk.K[46].X
        mul_input[1] = uint256(17275281338238142402668320571083598275071996182405634500711023887464421538127); // vk.K[46].Y
        mul_input[2] = input[45];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[46] * input[45]
        mul_input[0] = uint256(5240121977703409724976930147474666123582523807199050824430926013248898323413); // vk.K[47].X
        mul_input[1] = uint256(9235777244560018425574571644168716210830462792585067951593424650639702291943); // vk.K[47].Y
        mul_input[2] = input[46];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[47] * input[46]
        mul_input[0] = uint256(16709515988692486378095964497727607992954421154155836143184835325120234108282); // vk.K[48].X
        mul_input[1] = uint256(4318821262168551260213596218763477445736619275692986821452072174624316039747); // vk.K[48].Y
        mul_input[2] = input[47];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[48] * input[47]
        mul_input[0] = uint256(4635653803757786734812939730130448826232706399998169438201812455241736781365); // vk.K[49].X
        mul_input[1] = uint256(10620534372183167886846599595246470641725726870973800884133180230395135737336); // vk.K[49].Y
        mul_input[2] = input[48];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[49] * input[48]
        mul_input[0] = uint256(21882091146466759727378676055635488784699217830186438786659067805274266463217); // vk.K[50].X
        mul_input[1] = uint256(18598242203333244012781528775935406341731159877256787836648474120945984728668); // vk.K[50].Y
        mul_input[2] = input[49];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[50] * input[49]
        mul_input[0] = uint256(4981598058944746539451200624832095463060756413653505740624778630043872771559); // vk.K[51].X
        mul_input[1] = uint256(15417437381372953121848338347341631007478788246258161716837437944605430967710); // vk.K[51].Y
        mul_input[2] = input[50];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[51] * input[50]
        mul_input[0] = uint256(16449857651479430087361783294716085780818735843608843357258053633847274952841); // vk.K[52].X
        mul_input[1] = uint256(15723174467364037738901289426375812656268714827433964282590968293223190370878); // vk.K[52].Y
        mul_input[2] = input[51];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[52] * input[51]
        mul_input[0] = uint256(20648916574105536651189253176665249822733862047163598918925707075866654074662); // vk.K[53].X
        mul_input[1] = uint256(21287059011152424060114653424447126287308325233923052797964482922876748269568); // vk.K[53].Y
        mul_input[2] = input[52];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[53] * input[52]
        mul_input[0] = uint256(18585476542396457205150744199897004875593607482225637447845157197513269605578); // vk.K[54].X
        mul_input[1] = uint256(3417167586405724706465908229613681922657557710181148039128574753784005202656); // vk.K[54].Y
        mul_input[2] = input[53];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[54] * input[53]
        mul_input[0] = uint256(4511397141251924031887085372482825080786918659993462057314369239464754481694); // vk.K[55].X
        mul_input[1] = uint256(21717095464432043238756681501430279756357661473638833021102155654431510563342); // vk.K[55].Y
        mul_input[2] = input[54];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[55] * input[54]
        mul_input[0] = uint256(9212519189230801322001536903365645225220584836288401322046467666908189548807); // vk.K[56].X
        mul_input[1] = uint256(19744538517056645677142028979018732961431154170994932788904230767809445689768); // vk.K[56].Y
        mul_input[2] = input[55];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[56] * input[55]
        mul_input[0] = uint256(12289453430414039567836724210064005227666479474586979621733298761776796474516); // vk.K[57].X
        mul_input[1] = uint256(12989757651650597082755681430685414608745195391709803644101621833221841663099); // vk.K[57].Y
        mul_input[2] = input[56];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[57] * input[56]
        mul_input[0] = uint256(2369478084041109780531328278734355652837607008951232860940114157378030098765); // vk.K[58].X
        mul_input[1] = uint256(6119868655329830957213547984204983011004726394242293527463828569451316899897); // vk.K[58].Y
        mul_input[2] = input[57];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[58] * input[57]
        mul_input[0] = uint256(17989837724212112366551871898291137740340429905303785911117451851352882896345); // vk.K[59].X
        mul_input[1] = uint256(17949950156855941131881511270898739855917513493341349952799783132793577741533); // vk.K[59].Y
        mul_input[2] = input[58];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[59] * input[58]
        mul_input[0] = uint256(7262439193785268583033241221853354235600977055409379173808433414317405448832); // vk.K[60].X
        mul_input[1] = uint256(12203906070920291015993347316731095119713139475700145895368333405554307230470); // vk.K[60].Y
        mul_input[2] = input[59];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[60] * input[59]
        mul_input[0] = uint256(1162813742019157110963043810774081780095578126075902578179759093147701028966); // vk.K[61].X
        mul_input[1] = uint256(6196461946796674808960015791266782945323722628464646545853640633136894953642); // vk.K[61].Y
        mul_input[2] = input[60];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[61] * input[60]
        mul_input[0] = uint256(8734960838152226512446036029393702945097778032815532218825582662229148105809); // vk.K[62].X
        mul_input[1] = uint256(14156597841489472902105242066512520003665698393998522446948180722237464995747); // vk.K[62].Y
        mul_input[2] = input[61];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[62] * input[61]
        mul_input[0] = uint256(4584525156462878436907385844466134332162203762915420480339839395820190826598); // vk.K[63].X
        mul_input[1] = uint256(20896772351738173920864818723101902251047201025669140610634794223821078661262); // vk.K[63].Y
        mul_input[2] = input[62];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[63] * input[62]
        mul_input[0] = uint256(15548856620589266889065140246191848830841596843403455464590867915077143726220); // vk.K[64].X
        mul_input[1] = uint256(19703685940179675295187023238588296384286868782044376229887047852708735602340); // vk.K[64].Y
        mul_input[2] = input[63];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[64] * input[63]
        mul_input[0] = uint256(6512240350257283986195670322307359040229223793918266787712417443520969192740); // vk.K[65].X
        mul_input[1] = uint256(17883424236536710893435059391856918208546300448278439854180388920157605103236); // vk.K[65].Y
        mul_input[2] = input[64];
        accumulate(mul_input, q, add_input, vk_x); // vk_x += vk.K[65] * input[64]

        return
            Pairing.pairing(Pairing.negate(proof.A), proof.B, vk.alfa1, vk.beta2, vk_x, vk.gamma2, proof.C, vk.delta2);
    }
}

interface IFunctionVerifier {
    function verify(bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool);

    function verificationKeyHash() external pure returns (bytes32);
}

contract StorageVerifier is IFunctionVerifier, Verifier {
    function verify(bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external pure returns (bool) {
        (uint256[2] memory a, uint256[2][2] memory b, uint256[2] memory c) =
            abi.decode(_proof, (uint256[2], uint256[2][2], uint256[2]));

        uint256[65] memory input;
        input[0] = uint256(_inputHash);
        input[1] = uint256(_outputHash);

        // remove unused warnings
        a = a;
        b = b;
        c = c;
        // temporary, should do:
        // return verifyProof(a, b, c, input);
        return true;
    }

    function verificationKeyHash() external pure returns (bytes32) {
        return keccak256(abi.encode(verifyingKey()));
    }
}
