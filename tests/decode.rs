use arm64utils::instruction::*;
use arm64utils::*;

#[test]
fn test_adcs() {
    // adcs w4, w4, w8
    assert_eq!(
        decode_root_unwrap(0x3A080084),
        InstructionKind::ADCS32AddsubCarry {
            Rd: 4,
            Rm: 8,
            Rn: 4
        }
    );
    // adcs x13, x3, x13
    assert_eq!(
        decode_root_unwrap(0xBA0D006D),
        InstructionKind::ADCS64AddsubCarry {
            Rd: 13,
            Rm: 13,
            Rn: 3
        }
    );
    // adcs x6, x6, x14
    assert_eq!(
        decode_root_unwrap(0xBA0E00C6),
        InstructionKind::ADCS64AddsubCarry {
            Rd: 6,
            Rm: 14,
            Rn: 6
        }
    );
    // adcs x9, x9, x17
    assert_eq!(
        decode_root_unwrap(0xBA110129),
        InstructionKind::ADCS64AddsubCarry {
            Rd: 9,
            Rm: 17,
            Rn: 9
        }
    );
    // adcs x11, x21, xzr
    assert_eq!(
        decode_root_unwrap(0xBA1F02AB),
        InstructionKind::ADCS64AddsubCarry {
            Rd: 11,
            Rm: 31,
            Rn: 21
        }
    );
    // adcs x0, xzr, xzr
    assert_eq!(
        decode_root_unwrap(0xBA1F03E0),
        InstructionKind::ADCS64AddsubCarry {
            Rd: 0,
            Rm: 31,
            Rn: 31
        }
    );
}
#[test]
fn test_add() {
    // add w6, w5, w0
    assert_eq!(
        decode_root_unwrap(0xB0000A6),
        InstructionKind::ADD32AddsubShift {
            Rd: 6,
            Rm: 0,
            Rn: 5,
            imm6: 0,
            shift: 0
        }
    );
    // add w18, w18, #0x100
    assert_eq!(
        decode_root_unwrap(0x11040252),
        InstructionKind::ADD32AddsubImm {
            Rd: 18,
            Rn: 18,
            imm12: 256,
            shift: 0
        }
    );
    // add w13, w21, #0x770, lsl #12
    assert_eq!(
        decode_root_unwrap(0x115DC2AD),
        InstructionKind::ADD32AddsubImm {
            Rd: 13,
            Rn: 21,
            imm12: 1904,
            shift: 1
        }
    );
    // add w9, w8, #0x900, lsl #12
    assert_eq!(
        decode_root_unwrap(0x11640109),
        InstructionKind::ADD32AddsubImm {
            Rd: 9,
            Rn: 8,
            imm12: 2304,
            shift: 1
        }
    );
    // add x9, x11, w10, sxtw
    assert_eq!(
        decode_root_unwrap(0x8B2AC169),
        InstructionKind::ADD64AddsubExt {
            Rd: 9,
            Rm: 10,
            Rn: 11,
            imm3: 0,
            option: 6
        }
    );
    // add x8, x8, w11, sxtw #1
    assert_eq!(
        decode_root_unwrap(0x8B2BC508),
        InstructionKind::ADD64AddsubExt {
            Rd: 8,
            Rm: 11,
            Rn: 8,
            imm3: 1,
            option: 6
        }
    );
    // add x29, sp, #0x10
    assert_eq!(
        decode_root_unwrap(0x910043FD),
        InstructionKind::ADD64AddsubImm {
            Rd: 29,
            Rn: 31,
            imm12: 16,
            shift: 0
        }
    );
    // add x10, x8, #0xb8
    assert_eq!(
        decode_root_unwrap(0x9102E10A),
        InstructionKind::ADD64AddsubImm {
            Rd: 10,
            Rn: 8,
            imm12: 184,
            shift: 0
        }
    );
    // add x25, x26, #0x190
    assert_eq!(
        decode_root_unwrap(0x91064359),
        InstructionKind::ADD64AddsubImm {
            Rd: 25,
            Rn: 26,
            imm12: 400,
            shift: 0
        }
    );
}
#[test]
fn test_adds() {
    // adds w8, w8, w0
    assert_eq!(
        decode_root_unwrap(0x2B000108),
        InstructionKind::ADDS32AddsubShift {
            Rd: 8,
            Rm: 0,
            Rn: 8,
            imm6: 0,
            shift: 0
        }
    );
    // adds w13, w10, #0x10, lsl #12
    assert_eq!(
        decode_root_unwrap(0x3140414D),
        InstructionKind::ADDS32SAddsubImm {
            Rd: 13,
            Rn: 10,
            imm12: 16,
            shift: 1
        }
    );
    // adds x14, x14, x3
    assert_eq!(
        decode_root_unwrap(0xAB0301CE),
        InstructionKind::ADDS64AddsubShift {
            Rd: 14,
            Rm: 3,
            Rn: 14,
            imm6: 0,
            shift: 0
        }
    );
    // adds x13, x13, x5
    assert_eq!(
        decode_root_unwrap(0xAB0501AD),
        InstructionKind::ADDS64AddsubShift {
            Rd: 13,
            Rm: 5,
            Rn: 13,
            imm6: 0,
            shift: 0
        }
    );
    // adds x14, x14, x6
    assert_eq!(
        decode_root_unwrap(0xAB0601CE),
        InstructionKind::ADDS64AddsubShift {
            Rd: 14,
            Rm: 6,
            Rn: 14,
            imm6: 0,
            shift: 0
        }
    );
    // adds x11, x8, x23
    assert_eq!(
        decode_root_unwrap(0xAB17010B),
        InstructionKind::ADDS64AddsubShift {
            Rd: 11,
            Rm: 23,
            Rn: 8,
            imm6: 0,
            shift: 0
        }
    );
    // adds x4, x4, #1
    assert_eq!(
        decode_root_unwrap(0xB1000484),
        InstructionKind::ADDS64SAddsubImm {
            Rd: 4,
            Rn: 4,
            imm12: 1,
            shift: 0
        }
    );
}
#[test]
fn test_addv() {
    // addv b1, v0.8b
    assert_eq!(
        decode_root_unwrap(0xE31B801),
        InstructionKind::ADDVAsimdallOnly {
            Q: 0,
            Rd: 1,
            Rn: 0,
            size: 0
        }
    );
    // addv s1, v1.4s
    assert_eq!(
        decode_root_unwrap(0x4EB1B821),
        InstructionKind::ADDVAsimdallOnly {
            Q: 1,
            Rd: 1,
            Rn: 1,
            size: 2
        }
    );
    // addv s3, v3.4s
    assert_eq!(
        decode_root_unwrap(0x4EB1B863),
        InstructionKind::ADDVAsimdallOnly {
            Q: 1,
            Rd: 3,
            Rn: 3,
            size: 2
        }
    );
    // addv s6, v6.4s
    assert_eq!(
        decode_root_unwrap(0x4EB1B8C6),
        InstructionKind::ADDVAsimdallOnly {
            Q: 1,
            Rd: 6,
            Rn: 6,
            size: 2
        }
    );
    // addv s7, v7.4s
    assert_eq!(
        decode_root_unwrap(0x4EB1B8E7),
        InstructionKind::ADDVAsimdallOnly {
            Q: 1,
            Rd: 7,
            Rn: 7,
            size: 2
        }
    );
    // addv s17, v17.4s
    assert_eq!(
        decode_root_unwrap(0x4EB1BA31),
        InstructionKind::ADDVAsimdallOnly {
            Q: 1,
            Rd: 17,
            Rn: 17,
            size: 2
        }
    );
}
#[test]
fn test_adrp() {
    // adrp x2, #0x1140000
    assert_eq!(
        decode_root_unwrap(0x90008A02),
        InstructionKind::ADRPOnlyPcreladdr {
            Rd: 2,
            immhi: 1104,
            immlo: 0
        }
    );
    // adrp x10, #0x53d000
    assert_eq!(
        decode_root_unwrap(0xB00029EA),
        InstructionKind::ADRPOnlyPcreladdr {
            Rd: 10,
            immhi: 335,
            immlo: 1
        }
    );
    // adrp x9, #0x1895000
    assert_eq!(
        decode_root_unwrap(0xB000C4A9),
        InstructionKind::ADRPOnlyPcreladdr {
            Rd: 9,
            immhi: 1573,
            immlo: 1
        }
    );
    // adrp x20, #0x19fd000
    assert_eq!(
        decode_root_unwrap(0xB000CFF4),
        InstructionKind::ADRPOnlyPcreladdr {
            Rd: 20,
            immhi: 1663,
            immlo: 1
        }
    );
    // adrp x8, #0xfc2000
    assert_eq!(
        decode_root_unwrap(0xD0007E08),
        InstructionKind::ADRPOnlyPcreladdr {
            Rd: 8,
            immhi: 1008,
            immlo: 2
        }
    );
    // adrp x2, #0x3000
    assert_eq!(
        decode_root_unwrap(0xF0000002),
        InstructionKind::ADRPOnlyPcreladdr {
            Rd: 2,
            immhi: 0,
            immlo: 3
        }
    );
    // adrp x21, #0xfffffffffffff000
    assert_eq!(
        decode_root_unwrap(0xF0FFFFF5),
        InstructionKind::ADRPOnlyPcreladdr {
            Rd: 21,
            immhi: 524287,
            immlo: 3
        }
    );
}
#[test]
fn test_and() {
    // and w2, w8, #1
    assert_eq!(
        decode_root_unwrap(0x12000102),
        InstructionKind::AND32LogImm {
            Rd: 2,
            Rn: 8,
            immr: 0,
            imms: 0
        }
    );
    // and w11, w0, #3
    assert_eq!(
        decode_root_unwrap(0x1200040B),
        InstructionKind::AND32LogImm {
            Rd: 11,
            Rn: 0,
            immr: 0,
            imms: 1
        }
    );
    // and w18, w4, #0xffff
    assert_eq!(
        decode_root_unwrap(0x12003C92),
        InstructionKind::AND32LogImm {
            Rd: 18,
            Rn: 4,
            immr: 0,
            imms: 15
        }
    );
    // and w17, w2, #0xff0000
    assert_eq!(
        decode_root_unwrap(0x12101C51),
        InstructionKind::AND32LogImm {
            Rd: 17,
            Rn: 2,
            immr: 16,
            imms: 7
        }
    );
    // and w8, w10, #2
    assert_eq!(
        decode_root_unwrap(0x121F0148),
        InstructionKind::AND32LogImm {
            Rd: 8,
            Rn: 10,
            immr: 31,
            imms: 0
        }
    );
    // and x26, x26, #0xfffffffffffffffe
    assert_eq!(
        decode_root_unwrap(0x927FFB5A),
        InstructionKind::AND64LogImm {
            N: 1,
            Rd: 26,
            Rn: 26,
            immr: 63,
            imms: 62
        }
    );
}
#[test]
fn test_ands() {
    // ands w10, w10, #1
    assert_eq!(
        decode_root_unwrap(0x7200014A),
        InstructionKind::ANDS32SLogImm {
            Rd: 10,
            Rn: 10,
            immr: 0,
            imms: 0
        }
    );
    // ands w13, w10, #0x4000
    assert_eq!(
        decode_root_unwrap(0x7212014D),
        InstructionKind::ANDS32SLogImm {
            Rd: 13,
            Rn: 10,
            immr: 18,
            imms: 0
        }
    );
    // ands w28, w8, #4
    assert_eq!(
        decode_root_unwrap(0x721E011C),
        InstructionKind::ANDS32SLogImm {
            Rd: 28,
            Rn: 8,
            immr: 30,
            imms: 0
        }
    );
    // ands w11, w11, #0xfffffffe
    assert_eq!(
        decode_root_unwrap(0x721F796B),
        InstructionKind::ANDS32SLogImm {
            Rd: 11,
            Rn: 11,
            immr: 31,
            imms: 30
        }
    );
    // ands x13, x13, #7
    assert_eq!(
        decode_root_unwrap(0xF24009AD),
        InstructionKind::ANDS64SLogImm {
            N: 1,
            Rd: 13,
            Rn: 13,
            immr: 0,
            imms: 2
        }
    );
}
#[test]
fn test_asr() {
    // asr w4, w2, #1
    assert_eq!(
        decode_root_unwrap(0x13017C44),
        InstructionKind::SBFM32MBitfield {
            Rd: 4,
            Rn: 2,
            immr: 1,
            imms: 31
        }
    );
    // asr w2, w2, #3
    assert_eq!(
        decode_root_unwrap(0x13037C42),
        InstructionKind::SBFM32MBitfield {
            Rd: 2,
            Rn: 2,
            immr: 3,
            imms: 31
        }
    );
    // asr w15, w15, #3
    assert_eq!(
        decode_root_unwrap(0x13037DEF),
        InstructionKind::SBFM32MBitfield {
            Rd: 15,
            Rn: 15,
            immr: 3,
            imms: 31
        }
    );
    // asr w23, w23, w1
    assert_eq!(
        decode_root_unwrap(0x1AC12AF7),
        InstructionKind::ASRV32Dp2Src {
            Rd: 23,
            Rm: 1,
            Rn: 23
        }
    );
    // asr x10, x10, #0x3f
    assert_eq!(
        decode_root_unwrap(0x937FFD4A),
        InstructionKind::SBFM64MBitfield {
            Rd: 10,
            Rn: 10,
            immr: 63,
            imms: 63
        }
    );
    // asr x14, x8, x11
    assert_eq!(
        decode_root_unwrap(0x9ACB290E),
        InstructionKind::ASRV64Dp2Src {
            Rd: 14,
            Rm: 11,
            Rn: 8
        }
    );
    // asr x0, x1, x3
    assert_eq!(
        decode_root_unwrap(0x9AC32820),
        InstructionKind::ASRV64Dp2Src {
            Rd: 0,
            Rm: 3,
            Rn: 1
        }
    );
}
#[test]
fn test_bfi() {
    // bfi w9, w8, #0x18, #4
    assert_eq!(
        decode_root_unwrap(0x33080D09),
        InstructionKind::BFM32MBitfield {
            Rd: 9,
            Rn: 8,
            immr: 8,
            imms: 3
        }
    );
    // bfi w26, w26, #0x10, #0x10
    assert_eq!(
        decode_root_unwrap(0x33103F5A),
        InstructionKind::BFM32MBitfield {
            Rd: 26,
            Rn: 26,
            immr: 16,
            imms: 15
        }
    );
    // bfi w10, w11, #8, #6
    assert_eq!(
        decode_root_unwrap(0x3318156A),
        InstructionKind::BFM32MBitfield {
            Rd: 10,
            Rn: 11,
            immr: 24,
            imms: 5
        }
    );
    // bfi w1, w9, #5, #1
    assert_eq!(
        decode_root_unwrap(0x331B0121),
        InstructionKind::BFM32MBitfield {
            Rd: 1,
            Rn: 9,
            immr: 27,
            imms: 0
        }
    );
    // bfi w9, w11, #3, #1
    assert_eq!(
        decode_root_unwrap(0x331D0169),
        InstructionKind::BFM32MBitfield {
            Rd: 9,
            Rn: 11,
            immr: 29,
            imms: 0
        }
    );
    // bfi w10, w9, #1, #0x1f
    assert_eq!(
        decode_root_unwrap(0x331F792A),
        InstructionKind::BFM32MBitfield {
            Rd: 10,
            Rn: 9,
            immr: 31,
            imms: 30
        }
    );
}
#[test]
fn test_bfxil() {
    // bfxil w0, w17, #0, #1
    assert_eq!(
        decode_root_unwrap(0x33000220),
        InstructionKind::BFM32MBitfield {
            Rd: 0,
            Rn: 17,
            immr: 0,
            imms: 0
        }
    );
    // bfxil w22, w8, #0, #3
    assert_eq!(
        decode_root_unwrap(0x33000916),
        InstructionKind::BFM32MBitfield {
            Rd: 22,
            Rn: 8,
            immr: 0,
            imms: 2
        }
    );
    // bfxil w6, w1, #0, #6
    assert_eq!(
        decode_root_unwrap(0x33001426),
        InstructionKind::BFM32MBitfield {
            Rd: 6,
            Rn: 1,
            immr: 0,
            imms: 5
        }
    );
    // bfxil w12, w9, #2, #0x1e
    assert_eq!(
        decode_root_unwrap(0x33027D2C),
        InstructionKind::BFM32MBitfield {
            Rd: 12,
            Rn: 9,
            immr: 2,
            imms: 31
        }
    );
    // bfxil w8, w1, #6, #5
    assert_eq!(
        decode_root_unwrap(0x33062828),
        InstructionKind::BFM32MBitfield {
            Rd: 8,
            Rn: 1,
            immr: 6,
            imms: 10
        }
    );
    // bfxil x16, x11, #0x15, #0x1f
    assert_eq!(
        decode_root_unwrap(0xB355CD70),
        InstructionKind::BFM64MBitfield {
            Rd: 16,
            Rn: 11,
            immr: 21,
            imms: 51
        }
    );
    // bfxil x5, x7, #4, #5
    assert_eq!(
        decode_root_unwrap(0xB34420E5),
        InstructionKind::BFM64MBitfield {
            Rd: 5,
            Rn: 7,
            immr: 4,
            imms: 8
        }
    );
}
#[test]
fn test_bic() {
    // bic w2, w6, w2
    assert_eq!(
        decode_root_unwrap(0xA2200C2),
        InstructionKind::BIC32LogShift {
            Rd: 2,
            Rm: 2,
            Rn: 6,
            imm6: 0,
            shift: 0
        }
    );
    // bic w8, w1, w8
    assert_eq!(
        decode_root_unwrap(0xA280028),
        InstructionKind::BIC32LogShift {
            Rd: 8,
            Rm: 8,
            Rn: 1,
            imm6: 0,
            shift: 0
        }
    );
    // bic w2, w13, w2, asr #31
    assert_eq!(
        decode_root_unwrap(0xAA27DA2),
        InstructionKind::BIC32LogShift {
            Rd: 2,
            Rm: 2,
            Rn: 13,
            imm6: 31,
            shift: 2
        }
    );
    // bic w7, w12, w7, asr #31
    assert_eq!(
        decode_root_unwrap(0xAA77D87),
        InstructionKind::BIC32LogShift {
            Rd: 7,
            Rm: 7,
            Rn: 12,
            imm6: 31,
            shift: 2
        }
    );
    // bic v2.16b, v2.16b, v0.16b
    assert_eq!(
        decode_root_unwrap(0x4E601C42),
        InstructionKind::BICAsimdsameOnly {
            Q: 1,
            Rd: 2,
            Rm: 0,
            Rn: 2
        }
    );
    // bic v30.16b, v30.16b, v27.16b
    assert_eq!(
        decode_root_unwrap(0x4E7B1FDE),
        InstructionKind::BICAsimdsameOnly {
            Q: 1,
            Rd: 30,
            Rm: 27,
            Rn: 30
        }
    );
}
#[test]
fn test_bit() {
    // bit v2.16b, v1.16b, v0.16b
    assert_eq!(
        decode_root_unwrap(0x6EA01C22),
        InstructionKind::BITAsimdsameOnly {
            Q: 1,
            Rd: 2,
            Rm: 0,
            Rn: 1
        }
    );
    // bit v2.16b, v1.16b, v3.16b
    assert_eq!(
        decode_root_unwrap(0x6EA31C22),
        InstructionKind::BITAsimdsameOnly {
            Q: 1,
            Rd: 2,
            Rm: 3,
            Rn: 1
        }
    );
    // bit v16.16b, v1.16b, v4.16b
    assert_eq!(
        decode_root_unwrap(0x6EA41C30),
        InstructionKind::BITAsimdsameOnly {
            Q: 1,
            Rd: 16,
            Rm: 4,
            Rn: 1
        }
    );
    // bit v5.16b, v3.16b, v4.16b
    assert_eq!(
        decode_root_unwrap(0x6EA41C65),
        InstructionKind::BITAsimdsameOnly {
            Q: 1,
            Rd: 5,
            Rm: 4,
            Rn: 3
        }
    );
    // bit v7.16b, v0.16b, v5.16b
    assert_eq!(
        decode_root_unwrap(0x6EA51C07),
        InstructionKind::BITAsimdsameOnly {
            Q: 1,
            Rd: 7,
            Rm: 5,
            Rn: 0
        }
    );
    // bit v20.16b, v3.16b, v22.16b
    assert_eq!(
        decode_root_unwrap(0x6EB61C74),
        InstructionKind::BITAsimdsameOnly {
            Q: 1,
            Rd: 20,
            Rm: 22,
            Rn: 3
        }
    );
}
#[test]
fn test_bsl() {
    // bsl v5.8b, v3.8b, v0.8b
    assert_eq!(
        decode_root_unwrap(0x2E601C65),
        InstructionKind::BSLAsimdsameOnly {
            Q: 0,
            Rd: 5,
            Rm: 0,
            Rn: 3
        }
    );
    // bsl v5.16b, v6.16b, v7.16b
    assert_eq!(
        decode_root_unwrap(0x6E671CC5),
        InstructionKind::BSLAsimdsameOnly {
            Q: 1,
            Rd: 5,
            Rm: 7,
            Rn: 6
        }
    );
    // bsl v6.16b, v7.16b, v17.16b
    assert_eq!(
        decode_root_unwrap(0x6E711CE6),
        InstructionKind::BSLAsimdsameOnly {
            Q: 1,
            Rd: 6,
            Rm: 17,
            Rn: 7
        }
    );
    // bsl v25.16b, v30.16b, v27.16b
    assert_eq!(
        decode_root_unwrap(0x6E7B1FD9),
        InstructionKind::BSLAsimdsameOnly {
            Q: 1,
            Rd: 25,
            Rm: 27,
            Rn: 30
        }
    );
    // bsl v26.16b, v31.16b, v30.16b
    assert_eq!(
        decode_root_unwrap(0x6E7E1FFA),
        InstructionKind::BSLAsimdsameOnly {
            Q: 1,
            Rd: 26,
            Rm: 30,
            Rn: 31
        }
    );
    // bsl v27.16b, v30.16b, v31.16b
    assert_eq!(
        decode_root_unwrap(0x6E7F1FDB),
        InstructionKind::BSLAsimdsameOnly {
            Q: 1,
            Rd: 27,
            Rm: 31,
            Rn: 30
        }
    );
}
#[test]
fn test_ccmn() {
    // ccmn w8, #1, #4, eq
    assert_eq!(
        decode_root_unwrap(0x3A410904),
        InstructionKind::CCMN32CondcmpImm {
            Rn: 8,
            cond: 0,
            imm5: 1,
            nzcv: 4
        }
    );
    // ccmn w9, #1, #4, ne
    assert_eq!(
        decode_root_unwrap(0x3A411924),
        InstructionKind::CCMN32CondcmpImm {
            Rn: 9,
            cond: 1,
            imm5: 1,
            nzcv: 4
        }
    );
    // ccmn w13, #1, #0, ne
    assert_eq!(
        decode_root_unwrap(0x3A4119A0),
        InstructionKind::CCMN32CondcmpImm {
            Rn: 13,
            cond: 1,
            imm5: 1,
            nzcv: 0
        }
    );
    // ccmn w8, #1, #4, lo
    assert_eq!(
        decode_root_unwrap(0x3A413904),
        InstructionKind::CCMN32CondcmpImm {
            Rn: 8,
            cond: 3,
            imm5: 1,
            nzcv: 4
        }
    );
    // ccmn w22, #0x11, #4, ne
    assert_eq!(
        decode_root_unwrap(0x3A511AC4),
        InstructionKind::CCMN32CondcmpImm {
            Rn: 22,
            cond: 1,
            imm5: 17,
            nzcv: 4
        }
    );
    // ccmn x1, #1, #4, ne
    assert_eq!(
        decode_root_unwrap(0xBA411824),
        InstructionKind::CCMN64CondcmpImm {
            Rn: 1,
            cond: 1,
            imm5: 1,
            nzcv: 4
        }
    );
}
#[test]
fn test_ccmp() {
    // ccmp w1, w4, #4, lt
    assert_eq!(
        decode_root_unwrap(0x7A44B024),
        InstructionKind::CCMP32CondcmpReg {
            Rm: 4,
            Rn: 1,
            cond: 11,
            nzcv: 4
        }
    );
    // ccmp w23, #0x1f, #2, ne
    assert_eq!(
        decode_root_unwrap(0x7A5F1AE2),
        InstructionKind::CCMP32CondcmpImm {
            Rn: 23,
            cond: 1,
            imm5: 31,
            nzcv: 2
        }
    );
    // ccmp x8, #0, #0, eq
    assert_eq!(
        decode_root_unwrap(0xFA400900),
        InstructionKind::CCMP64CondcmpImm {
            Rn: 8,
            cond: 0,
            imm5: 0,
            nzcv: 0
        }
    );
    // ccmp x9, x2, #2, hs
    assert_eq!(
        decode_root_unwrap(0xFA422122),
        InstructionKind::CCMP64CondcmpReg {
            Rm: 2,
            Rn: 9,
            cond: 2,
            nzcv: 2
        }
    );
    // ccmp x10, x14, #0, lo
    assert_eq!(
        decode_root_unwrap(0xFA4E3140),
        InstructionKind::CCMP64CondcmpReg {
            Rm: 14,
            Rn: 10,
            cond: 3,
            nzcv: 0
        }
    );
    // ccmp x8, x26, #4, ne
    assert_eq!(
        decode_root_unwrap(0xFA5A1104),
        InstructionKind::CCMP64CondcmpReg {
            Rm: 26,
            Rn: 8,
            cond: 1,
            nzcv: 4
        }
    );
}
#[test]
fn test_cinc() {
    // cinc w8, w0, eq
    assert_eq!(
        decode_root_unwrap(0x1A801408),
        InstructionKind::CSINC32Condsel {
            Rd: 8,
            Rm: 0,
            Rn: 0,
            cond: 1
        }
    );
    // cinc w12, w10, lt
    assert_eq!(
        decode_root_unwrap(0x1A8AA54C),
        InstructionKind::CSINC32Condsel {
            Rd: 12,
            Rm: 10,
            Rn: 10,
            cond: 10
        }
    );
    // cinc w8, w17, lt
    assert_eq!(
        decode_root_unwrap(0x1A91A628),
        InstructionKind::CSINC32Condsel {
            Rd: 8,
            Rm: 17,
            Rn: 17,
            cond: 10
        }
    );
    // cinc w19, w19, ne
    assert_eq!(
        decode_root_unwrap(0x1A930673),
        InstructionKind::CSINC32Condsel {
            Rd: 19,
            Rm: 19,
            Rn: 19,
            cond: 0
        }
    );
    // cinc w5, w23, ne
    assert_eq!(
        decode_root_unwrap(0x1A9706E5),
        InstructionKind::CSINC32Condsel {
            Rd: 5,
            Rm: 23,
            Rn: 23,
            cond: 0
        }
    );
    // cinc x7, x2, lt
    assert_eq!(
        decode_root_unwrap(0x9A82A447),
        InstructionKind::CSINC64Condsel {
            Rd: 7,
            Rm: 2,
            Rn: 2,
            cond: 10
        }
    );
}
#[test]
fn test_cinv() {
    // cinv w0, w8, ne
    assert_eq!(
        decode_root_unwrap(0x5A880100),
        InstructionKind::CSINV32Condsel {
            Rd: 0,
            Rm: 8,
            Rn: 8,
            cond: 0
        }
    );
    // cinv w8, w8, ge
    assert_eq!(
        decode_root_unwrap(0x5A88B108),
        InstructionKind::CSINV32Condsel {
            Rd: 8,
            Rm: 8,
            Rn: 8,
            cond: 11
        }
    );
    // cinv w22, w8, le
    assert_eq!(
        decode_root_unwrap(0x5A88C116),
        InstructionKind::CSINV32Condsel {
            Rd: 22,
            Rm: 8,
            Rn: 8,
            cond: 12
        }
    );
    // cinv w10, w10, lt
    assert_eq!(
        decode_root_unwrap(0x5A8AA14A),
        InstructionKind::CSINV32Condsel {
            Rd: 10,
            Rm: 10,
            Rn: 10,
            cond: 10
        }
    );
    // cinv w11, w11, lt
    assert_eq!(
        decode_root_unwrap(0x5A8BA16B),
        InstructionKind::CSINV32Condsel {
            Rd: 11,
            Rm: 11,
            Rn: 11,
            cond: 10
        }
    );
    // cinv x0, x8, ge
    assert_eq!(
        decode_root_unwrap(0xDA88B100),
        InstructionKind::CSINV64Condsel {
            Rd: 0,
            Rm: 8,
            Rn: 8,
            cond: 11
        }
    );
}
#[test]
fn test_clz() {
    // clz w9, w6
    assert_eq!(
        decode_root_unwrap(0x5AC010C9),
        InstructionKind::CLZ32Dp1Src { Rd: 9, Rn: 6 }
    );
    // clz w21, w6
    assert_eq!(
        decode_root_unwrap(0x5AC010D5),
        InstructionKind::CLZ32Dp1Src { Rd: 21, Rn: 6 }
    );
    // clz w12, w12
    assert_eq!(
        decode_root_unwrap(0x5AC0118C),
        InstructionKind::CLZ32Dp1Src { Rd: 12, Rn: 12 }
    );
    // clz w13, w12
    assert_eq!(
        decode_root_unwrap(0x5AC0118D),
        InstructionKind::CLZ32Dp1Src { Rd: 13, Rn: 12 }
    );
    // clz w19, w19
    assert_eq!(
        decode_root_unwrap(0x5AC01273),
        InstructionKind::CLZ32Dp1Src { Rd: 19, Rn: 19 }
    );
    // clz w10, w25
    assert_eq!(
        decode_root_unwrap(0x5AC0132A),
        InstructionKind::CLZ32Dp1Src { Rd: 10, Rn: 25 }
    );
}
#[test]
fn test_cmeq() {
    // cmeq v1.4s, v0.4s, #0
    assert_eq!(
        decode_root_unwrap(0x4EA09801),
        InstructionKind::CMEQAsimdmiscZ {
            Q: 1,
            Rd: 1,
            Rn: 0,
            size: 2
        }
    );
    // cmeq v16.4s, v7.4s, #0
    assert_eq!(
        decode_root_unwrap(0x4EA098F0),
        InstructionKind::CMEQAsimdmiscZ {
            Q: 1,
            Rd: 16,
            Rn: 7,
            size: 2
        }
    );
    // cmeq v19.16b, v16.16b, v2.16b
    assert_eq!(
        decode_root_unwrap(0x6E228E13),
        InstructionKind::CMEQAsimdsameOnly {
            Q: 1,
            Rd: 19,
            Rm: 2,
            Rn: 16,
            size: 0
        }
    );
    // cmeq v4.16b, v4.16b, v3.16b
    assert_eq!(
        decode_root_unwrap(0x6E238C84),
        InstructionKind::CMEQAsimdsameOnly {
            Q: 1,
            Rd: 4,
            Rm: 3,
            Rn: 4,
            size: 0
        }
    );
    // cmeq v16.16b, v5.16b, v3.16b
    assert_eq!(
        decode_root_unwrap(0x6E238CB0),
        InstructionKind::CMEQAsimdsameOnly {
            Q: 1,
            Rd: 16,
            Rm: 3,
            Rn: 5,
            size: 0
        }
    );
    // cmeq v18.16b, v7.16b, v5.16b
    assert_eq!(
        decode_root_unwrap(0x6E258CF2),
        InstructionKind::CMEQAsimdsameOnly {
            Q: 1,
            Rd: 18,
            Rm: 5,
            Rn: 7,
            size: 0
        }
    );
}
#[test]
fn test_cmhi() {
    // cmhi v2.8h, v2.8h, v0.8h
    assert_eq!(
        decode_root_unwrap(0x6E603442),
        InstructionKind::CMHIAsimdsameOnly {
            Q: 1,
            Rd: 2,
            Rm: 0,
            Rn: 2,
            size: 1
        }
    );
    // cmhi v3.8h, v3.8h, v0.8h
    assert_eq!(
        decode_root_unwrap(0x6E603463),
        InstructionKind::CMHIAsimdsameOnly {
            Q: 1,
            Rd: 3,
            Rm: 0,
            Rn: 3,
            size: 1
        }
    );
}
#[test]
fn test_cmhs() {
    // cmhs v3.2s, v1.2s, v3.2s
    assert_eq!(
        decode_root_unwrap(0x2EA33C23),
        InstructionKind::CMHSAsimdsameOnly {
            Q: 0,
            Rd: 3,
            Rm: 3,
            Rn: 1,
            size: 2
        }
    );
    // cmhs v4.2s, v1.2s, v4.2s
    assert_eq!(
        decode_root_unwrap(0x2EA43C24),
        InstructionKind::CMHSAsimdsameOnly {
            Q: 0,
            Rd: 4,
            Rm: 4,
            Rn: 1,
            size: 2
        }
    );
    // cmhs v5.2s, v1.2s, v5.2s
    assert_eq!(
        decode_root_unwrap(0x2EA53C25),
        InstructionKind::CMHSAsimdsameOnly {
            Q: 0,
            Rd: 5,
            Rm: 5,
            Rn: 1,
            size: 2
        }
    );
}
#[test]
fn test_cmlt() {
    // cmlt d0, d0, #0
    assert_eq!(
        decode_root_unwrap(0x5EE0A800),
        InstructionKind::CMLTAsisdmiscZ {
            Rd: 0,
            Rn: 0,
            size: 3
        }
    );
}
#[test]
fn test_cmn() {
    // cmn w1, #1
    assert_eq!(
        decode_root_unwrap(0x3100043F),
        InstructionKind::ADDS32SAddsubImm {
            Rd: 31,
            Rn: 1,
            imm12: 1,
            shift: 0
        }
    );
    // cmn w9, #1
    assert_eq!(
        decode_root_unwrap(0x3100053F),
        InstructionKind::ADDS32SAddsubImm {
            Rd: 31,
            Rn: 9,
            imm12: 1,
            shift: 0
        }
    );
    // cmn w9, #0xf
    assert_eq!(
        decode_root_unwrap(0x31003D3F),
        InstructionKind::ADDS32SAddsubImm {
            Rd: 31,
            Rn: 9,
            imm12: 15,
            shift: 0
        }
    );
    // cmn w11, #0x380, lsl #12
    assert_eq!(
        decode_root_unwrap(0x314E017F),
        InstructionKind::ADDS32SAddsubImm {
            Rd: 31,
            Rn: 11,
            imm12: 896,
            shift: 1
        }
    );
    // cmn x10, #1
    assert_eq!(
        decode_root_unwrap(0xB100055F),
        InstructionKind::ADDS64SAddsubImm {
            Rd: 31,
            Rn: 10,
            imm12: 1,
            shift: 0
        }
    );
    // cmn x1, #3
    assert_eq!(
        decode_root_unwrap(0xB1000C3F),
        InstructionKind::ADDS64SAddsubImm {
            Rd: 31,
            Rn: 1,
            imm12: 3,
            shift: 0
        }
    );
    // cmn x9, #0x10
    assert_eq!(
        decode_root_unwrap(0xB100413F),
        InstructionKind::ADDS64SAddsubImm {
            Rd: 31,
            Rn: 9,
            imm12: 16,
            shift: 0
        }
    );
}
#[test]
fn test_cmp() {
    // cmp w2, w8
    assert_eq!(
        decode_root_unwrap(0x6B08005F),
        InstructionKind::SUBS32AddsubShift {
            Rd: 31,
            Rm: 8,
            Rn: 2,
            imm6: 0,
            shift: 0
        }
    );
    // cmp w26, w13
    assert_eq!(
        decode_root_unwrap(0x6B0D035F),
        InstructionKind::SUBS32AddsubShift {
            Rd: 31,
            Rm: 13,
            Rn: 26,
            imm6: 0,
            shift: 0
        }
    );
    // cmp w18, w4, uxth
    assert_eq!(
        decode_root_unwrap(0x6B24225F),
        InstructionKind::SUBS32SAddsubExt {
            Rd: 31,
            Rm: 4,
            Rn: 18,
            imm3: 0,
            option: 1
        }
    );
    // cmp w9, w8, asr #3
    assert_eq!(
        decode_root_unwrap(0x6B880D3F),
        InstructionKind::SUBS32AddsubShift {
            Rd: 31,
            Rm: 8,
            Rn: 9,
            imm6: 3,
            shift: 2
        }
    );
    // cmp w1, #0
    assert_eq!(
        decode_root_unwrap(0x7100003F),
        InstructionKind::SUBS32SAddsubImm {
            Rd: 31,
            Rn: 1,
            imm12: 0,
            shift: 0
        }
    );
    // cmp w2, #0x93
    assert_eq!(
        decode_root_unwrap(0x71024C5F),
        InstructionKind::SUBS32SAddsubImm {
            Rd: 31,
            Rn: 2,
            imm12: 147,
            shift: 0
        }
    );
    // cmp w12, #0x7fe, lsl #12
    assert_eq!(
        decode_root_unwrap(0x715FF99F),
        InstructionKind::SUBS32SAddsubImm {
            Rd: 31,
            Rn: 12,
            imm12: 2046,
            shift: 1
        }
    );
    // cmp x8, x20
    assert_eq!(
        decode_root_unwrap(0xEB14011F),
        InstructionKind::SUBS64AddsubShift {
            Rd: 31,
            Rm: 20,
            Rn: 8,
            imm6: 0,
            shift: 0
        }
    );
    // cmp x6, #0xa
    assert_eq!(
        decode_root_unwrap(0xF10028DF),
        InstructionKind::SUBS64SAddsubImm {
            Rd: 31,
            Rn: 6,
            imm12: 10,
            shift: 0
        }
    );
    // cmp x19, #1, lsl #12
    assert_eq!(
        decode_root_unwrap(0xF140067F),
        InstructionKind::SUBS64SAddsubImm {
            Rd: 31,
            Rn: 19,
            imm12: 1,
            shift: 1
        }
    );
    // cmp x8, #0x80, lsl #12
    assert_eq!(
        decode_root_unwrap(0xF142011F),
        InstructionKind::SUBS64SAddsubImm {
            Rd: 31,
            Rn: 8,
            imm12: 128,
            shift: 1
        }
    );
}
#[test]
fn test_cmtst() {
    // cmtst v18.4s, v17.4s, v0.4s
    assert_eq!(
        decode_root_unwrap(0x4EA08E32),
        InstructionKind::CMTSTAsimdsameOnly {
            Q: 1,
            Rd: 18,
            Rm: 0,
            Rn: 17,
            size: 2
        }
    );
    // cmtst v19.4s, v17.4s, v1.4s
    assert_eq!(
        decode_root_unwrap(0x4EA18E33),
        InstructionKind::CMTSTAsimdsameOnly {
            Q: 1,
            Rd: 19,
            Rm: 1,
            Rn: 17,
            size: 2
        }
    );
    // cmtst v19.4s, v17.4s, v3.4s
    assert_eq!(
        decode_root_unwrap(0x4EA38E33),
        InstructionKind::CMTSTAsimdsameOnly {
            Q: 1,
            Rd: 19,
            Rm: 3,
            Rn: 17,
            size: 2
        }
    );
    // cmtst v19.4s, v17.4s, v5.4s
    assert_eq!(
        decode_root_unwrap(0x4EA58E33),
        InstructionKind::CMTSTAsimdsameOnly {
            Q: 1,
            Rd: 19,
            Rm: 5,
            Rn: 17,
            size: 2
        }
    );
    // cmtst v19.4s, v17.4s, v6.4s
    assert_eq!(
        decode_root_unwrap(0x4EA68E33),
        InstructionKind::CMTSTAsimdsameOnly {
            Q: 1,
            Rd: 19,
            Rm: 6,
            Rn: 17,
            size: 2
        }
    );
    // cmtst v17.4s, v17.4s, v7.4s
    assert_eq!(
        decode_root_unwrap(0x4EA78E31),
        InstructionKind::CMTSTAsimdsameOnly {
            Q: 1,
            Rd: 17,
            Rm: 7,
            Rn: 17,
            size: 2
        }
    );
}
#[test]
fn test_cneg() {
    // cneg w9, w0, mi
    assert_eq!(
        decode_root_unwrap(0x5A805409),
        InstructionKind::CSNEG32Condsel {
            Rd: 9,
            Rm: 0,
            Rn: 0,
            cond: 5
        }
    );
    // cneg w14, w4, lt
    assert_eq!(
        decode_root_unwrap(0x5A84A48E),
        InstructionKind::CSNEG32Condsel {
            Rd: 14,
            Rm: 4,
            Rn: 4,
            cond: 10
        }
    );
    // cneg w14, w11, eq
    assert_eq!(
        decode_root_unwrap(0x5A8B156E),
        InstructionKind::CSNEG32Condsel {
            Rd: 14,
            Rm: 11,
            Rn: 11,
            cond: 1
        }
    );
    // cneg w17, w14, mi
    assert_eq!(
        decode_root_unwrap(0x5A8E55D1),
        InstructionKind::CSNEG32Condsel {
            Rd: 17,
            Rm: 14,
            Rn: 14,
            cond: 5
        }
    );
    // cneg x8, x8, mi
    assert_eq!(
        decode_root_unwrap(0xDA885508),
        InstructionKind::CSNEG64Condsel {
            Rd: 8,
            Rm: 8,
            Rn: 8,
            cond: 5
        }
    );
    // cneg x17, x17, mi
    assert_eq!(
        decode_root_unwrap(0xDA915631),
        InstructionKind::CSNEG64Condsel {
            Rd: 17,
            Rm: 17,
            Rn: 17,
            cond: 5
        }
    );
}
#[test]
fn test_crc32ch() {
    // crc32ch w0, w1, w2
    assert_eq!(
        decode_root_unwrap(0x1AC25420),
        InstructionKind::CRC32CH32CDp2Src {
            Rd: 0,
            Rm: 2,
            Rn: 1
        }
    );
}
#[test]
fn test_csel() {
    // csel w0, w2, w0, lo
    assert_eq!(
        decode_root_unwrap(0x1A803040),
        InstructionKind::CSEL32Condsel {
            Rd: 0,
            Rm: 0,
            Rn: 2,
            cond: 3
        }
    );
    // csel w4, w25, w4, lo
    assert_eq!(
        decode_root_unwrap(0x1A843324),
        InstructionKind::CSEL32Condsel {
            Rd: 4,
            Rm: 4,
            Rn: 25,
            cond: 3
        }
    );
    // csel w0, wzr, w9, eq
    assert_eq!(
        decode_root_unwrap(0x1A8903E0),
        InstructionKind::CSEL32Condsel {
            Rd: 0,
            Rm: 9,
            Rn: 31,
            cond: 0
        }
    );
    // csel w30, wzr, w17, lt
    assert_eq!(
        decode_root_unwrap(0x1A91B3FE),
        InstructionKind::CSEL32Condsel {
            Rd: 30,
            Rm: 17,
            Rn: 31,
            cond: 11
        }
    );
    // csel w16, w16, w18, hi
    assert_eq!(
        decode_root_unwrap(0x1A928210),
        InstructionKind::CSEL32Condsel {
            Rd: 16,
            Rm: 18,
            Rn: 16,
            cond: 8
        }
    );
    // csel x9, x8, x10, ne
    assert_eq!(
        decode_root_unwrap(0x9A8A1109),
        InstructionKind::CSEL64Condsel {
            Rd: 9,
            Rm: 10,
            Rn: 8,
            cond: 1
        }
    );
    // csel x1, x25, x23, eq
    assert_eq!(
        decode_root_unwrap(0x9A970321),
        InstructionKind::CSEL64Condsel {
            Rd: 1,
            Rm: 23,
            Rn: 25,
            cond: 0
        }
    );
}
#[test]
fn test_cset() {
    // cset w13, lo
    assert_eq!(
        decode_root_unwrap(0x1A9F27ED),
        InstructionKind::CSINC32Condsel {
            Rd: 13,
            Rm: 31,
            Rn: 31,
            cond: 2
        }
    );
    // cset w0, mi
    assert_eq!(
        decode_root_unwrap(0x1A9F57E0),
        InstructionKind::CSINC32Condsel {
            Rd: 0,
            Rm: 31,
            Rn: 31,
            cond: 5
        }
    );
    // cset w4, hi
    assert_eq!(
        decode_root_unwrap(0x1A9F97E4),
        InstructionKind::CSINC32Condsel {
            Rd: 4,
            Rm: 31,
            Rn: 31,
            cond: 9
        }
    );
    // cset w26, lt
    assert_eq!(
        decode_root_unwrap(0x1A9FA7FA),
        InstructionKind::CSINC32Condsel {
            Rd: 26,
            Rm: 31,
            Rn: 31,
            cond: 10
        }
    );
    // cset w18, ge
    assert_eq!(
        decode_root_unwrap(0x1A9FB7F2),
        InstructionKind::CSINC32Condsel {
            Rd: 18,
            Rm: 31,
            Rn: 31,
            cond: 11
        }
    );
    // cset w28, gt
    assert_eq!(
        decode_root_unwrap(0x1A9FD7FC),
        InstructionKind::CSINC32Condsel {
            Rd: 28,
            Rm: 31,
            Rn: 31,
            cond: 13
        }
    );
}
#[test]
fn test_csetm() {
    // csetm w23, ne
    assert_eq!(
        decode_root_unwrap(0x5A9F03F7),
        InstructionKind::CSINV32Condsel {
            Rd: 23,
            Rm: 31,
            Rn: 31,
            cond: 0
        }
    );
    // csetm w0, hi
    assert_eq!(
        decode_root_unwrap(0x5A9F93E0),
        InstructionKind::CSINV32Condsel {
            Rd: 0,
            Rm: 31,
            Rn: 31,
            cond: 9
        }
    );
    // csetm w14, hi
    assert_eq!(
        decode_root_unwrap(0x5A9F93EE),
        InstructionKind::CSINV32Condsel {
            Rd: 14,
            Rm: 31,
            Rn: 31,
            cond: 9
        }
    );
    // csetm w11, lt
    assert_eq!(
        decode_root_unwrap(0x5A9FA3EB),
        InstructionKind::CSINV32Condsel {
            Rd: 11,
            Rm: 31,
            Rn: 31,
            cond: 10
        }
    );
    // csetm w23, le
    assert_eq!(
        decode_root_unwrap(0x5A9FC3F7),
        InstructionKind::CSINV32Condsel {
            Rd: 23,
            Rm: 31,
            Rn: 31,
            cond: 12
        }
    );
    // csetm x0, eq
    assert_eq!(
        decode_root_unwrap(0xDA9F13E0),
        InstructionKind::CSINV64Condsel {
            Rd: 0,
            Rm: 31,
            Rn: 31,
            cond: 1
        }
    );
}
#[test]
fn test_csinc() {
    // csinc w12, w13, w8, lt
    assert_eq!(
        decode_root_unwrap(0x1A88B5AC),
        InstructionKind::CSINC32Condsel {
            Rd: 12,
            Rm: 8,
            Rn: 13,
            cond: 11
        }
    );
    // csinc w9, w15, w9, lt
    assert_eq!(
        decode_root_unwrap(0x1A89B5E9),
        InstructionKind::CSINC32Condsel {
            Rd: 9,
            Rm: 9,
            Rn: 15,
            cond: 11
        }
    );
    // csinc w10, wzr, w10, ge
    assert_eq!(
        decode_root_unwrap(0x1A8AA7EA),
        InstructionKind::CSINC32Condsel {
            Rd: 10,
            Rm: 10,
            Rn: 31,
            cond: 10
        }
    );
    // csinc x6, x9, x0, ne
    assert_eq!(
        decode_root_unwrap(0x9A801526),
        InstructionKind::CSINC64Condsel {
            Rd: 6,
            Rm: 0,
            Rn: 9,
            cond: 1
        }
    );
    // csinc x1, x9, x11, ne
    assert_eq!(
        decode_root_unwrap(0x9A8B1521),
        InstructionKind::CSINC64Condsel {
            Rd: 1,
            Rm: 11,
            Rn: 9,
            cond: 1
        }
    );
    // csinc x8, x9, x25, ne
    assert_eq!(
        decode_root_unwrap(0x9A991528),
        InstructionKind::CSINC64Condsel {
            Rd: 8,
            Rm: 25,
            Rn: 9,
            cond: 1
        }
    );
}
#[test]
fn test_csinv() {
    // csinv w8, w8, w3, ge
    assert_eq!(
        decode_root_unwrap(0x5A83A108),
        InstructionKind::CSINV32Condsel {
            Rd: 8,
            Rm: 3,
            Rn: 8,
            cond: 10
        }
    );
    // csinv w8, w9, w8, ls
    assert_eq!(
        decode_root_unwrap(0x5A889128),
        InstructionKind::CSINV32Condsel {
            Rd: 8,
            Rm: 8,
            Rn: 9,
            cond: 9
        }
    );
    // csinv w12, w13, w24, le
    assert_eq!(
        decode_root_unwrap(0x5A98D1AC),
        InstructionKind::CSINV32Condsel {
            Rd: 12,
            Rm: 24,
            Rn: 13,
            cond: 13
        }
    );
    // csinv w8, w9, wzr, ge
    assert_eq!(
        decode_root_unwrap(0x5A9FA128),
        InstructionKind::CSINV32Condsel {
            Rd: 8,
            Rm: 31,
            Rn: 9,
            cond: 10
        }
    );
    // csinv w25, w9, wzr, le
    assert_eq!(
        decode_root_unwrap(0x5A9FD139),
        InstructionKind::CSINV32Condsel {
            Rd: 25,
            Rm: 31,
            Rn: 9,
            cond: 13
        }
    );
    // csinv w18, w16, wzr, le
    assert_eq!(
        decode_root_unwrap(0x5A9FD212),
        InstructionKind::CSINV32Condsel {
            Rd: 18,
            Rm: 31,
            Rn: 16,
            cond: 13
        }
    );
}
#[test]
fn test_csneg() {
    // csneg w8, w8, w2, ge
    assert_eq!(
        decode_root_unwrap(0x5A82A508),
        InstructionKind::CSNEG32Condsel {
            Rd: 8,
            Rm: 2,
            Rn: 8,
            cond: 10
        }
    );
    // csneg w8, w9, w8, gt
    assert_eq!(
        decode_root_unwrap(0x5A88C528),
        InstructionKind::CSNEG32Condsel {
            Rd: 8,
            Rm: 8,
            Rn: 9,
            cond: 12
        }
    );
    // csneg w8, w10, w8, gt
    assert_eq!(
        decode_root_unwrap(0x5A88C548),
        InstructionKind::CSNEG32Condsel {
            Rd: 8,
            Rm: 8,
            Rn: 10,
            cond: 12
        }
    );
    // csneg w21, w21, w8, le
    assert_eq!(
        decode_root_unwrap(0x5A88D6B5),
        InstructionKind::CSNEG32Condsel {
            Rd: 21,
            Rm: 8,
            Rn: 21,
            cond: 13
        }
    );
    // csneg w12, w10, w22, gt
    assert_eq!(
        decode_root_unwrap(0x5A96C54C),
        InstructionKind::CSNEG32Condsel {
            Rd: 12,
            Rm: 22,
            Rn: 10,
            cond: 12
        }
    );
    // csneg w11, w23, w27, lo
    assert_eq!(
        decode_root_unwrap(0x5A9B36EB),
        InstructionKind::CSNEG32Condsel {
            Rd: 11,
            Rm: 27,
            Rn: 23,
            cond: 3
        }
    );
}
#[test]
fn test_dc() {
    // dc cvau, x11
    assert_eq!(
        decode_root_unwrap(0xD50B7B2B),
        InstructionKind::SYSCrSystem {
            CRm: 11,
            CRn: 7,
            Rt: 11,
            op1: 3,
            op2: 1
        }
    );
    // dc civac, x10
    assert_eq!(
        decode_root_unwrap(0xD50B7E2A),
        InstructionKind::SYSCrSystem {
            CRm: 14,
            CRn: 7,
            Rt: 10,
            op1: 3,
            op2: 1
        }
    );
}
#[test]
fn test_dmb() {
    // dmb ishld
    assert_eq!(
        decode_root_unwrap(0xD50339BF),
        InstructionKind::DMBBoSystem { CRm: 9 }
    );
    // dmb ishst
    assert_eq!(
        decode_root_unwrap(0xD5033ABF),
        InstructionKind::DMBBoSystem { CRm: 10 }
    );
    // dmb ish
    assert_eq!(
        decode_root_unwrap(0xD5033BBF),
        InstructionKind::DMBBoSystem { CRm: 11 }
    );
}
#[test]
fn test_dsb() {
    // dsb ish
    assert_eq!(
        decode_root_unwrap(0xD5033B9F),
        InstructionKind::DSBBoSystem { CRm: 11 }
    );
    // dsb sy
    assert_eq!(
        decode_root_unwrap(0xD5033F9F),
        InstructionKind::DSBBoSystem { CRm: 15 }
    );
}
#[test]
fn test_dup() {
    // dup v31.2s, v24.s[1]
    assert_eq!(
        decode_root_unwrap(0xE0C071F),
        InstructionKind::DUPAsimdinsDvV {
            Q: 0,
            Rd: 31,
            Rn: 24,
            imm5: 12
        }
    );
    // dup v17.4s, v17.s[0]
    assert_eq!(
        decode_root_unwrap(0x4E040631),
        InstructionKind::DUPAsimdinsDvV {
            Q: 1,
            Rd: 17,
            Rn: 17,
            imm5: 4
        }
    );
    // dup v21.4s, v21.s[0]
    assert_eq!(
        decode_root_unwrap(0x4E0406B5),
        InstructionKind::DUPAsimdinsDvV {
            Q: 1,
            Rd: 21,
            Rn: 21,
            imm5: 4
        }
    );
    // dup v20.4s, v25.s[0]
    assert_eq!(
        decode_root_unwrap(0x4E040734),
        InstructionKind::DUPAsimdinsDvV {
            Q: 1,
            Rd: 20,
            Rn: 25,
            imm5: 4
        }
    );
    // dup v0.4s, w1
    assert_eq!(
        decode_root_unwrap(0x4E040C20),
        InstructionKind::DUPAsimdinsDrR {
            Q: 1,
            Rd: 0,
            Rn: 1,
            imm5: 4
        }
    );
    // dup v3.2d, v2.d[1]
    assert_eq!(
        decode_root_unwrap(0x4E180443),
        InstructionKind::DUPAsimdinsDvV {
            Q: 1,
            Rd: 3,
            Rn: 2,
            imm5: 24
        }
    );
}
#[test]
fn test_eon() {
    // eon w10, w10, w9
    assert_eq!(
        decode_root_unwrap(0x4A29014A),
        InstructionKind::EON32LogShift {
            Rd: 10,
            Rm: 9,
            Rn: 10,
            imm6: 0,
            shift: 0
        }
    );
    // eon w12, w12, w11
    assert_eq!(
        decode_root_unwrap(0x4A2B018C),
        InstructionKind::EON32LogShift {
            Rd: 12,
            Rm: 11,
            Rn: 12,
            imm6: 0,
            shift: 0
        }
    );
    // eon w14, w14, w13
    assert_eq!(
        decode_root_unwrap(0x4A2D01CE),
        InstructionKind::EON32LogShift {
            Rd: 14,
            Rm: 13,
            Rn: 14,
            imm6: 0,
            shift: 0
        }
    );
    // eon w15, w15, w14
    assert_eq!(
        decode_root_unwrap(0x4A2E01EF),
        InstructionKind::EON32LogShift {
            Rd: 15,
            Rm: 14,
            Rn: 15,
            imm6: 0,
            shift: 0
        }
    );
    // eon w16, w17, w16
    assert_eq!(
        decode_root_unwrap(0x4A300230),
        InstructionKind::EON32LogShift {
            Rd: 16,
            Rm: 16,
            Rn: 17,
            imm6: 0,
            shift: 0
        }
    );
    // eon w17, w18, w17
    assert_eq!(
        decode_root_unwrap(0x4A310251),
        InstructionKind::EON32LogShift {
            Rd: 17,
            Rm: 17,
            Rn: 18,
            imm6: 0,
            shift: 0
        }
    );
}
#[test]
fn test_eor() {
    // eor w8, w1, w0
    assert_eq!(
        decode_root_unwrap(0x4A000028),
        InstructionKind::EOR32LogShift {
            Rd: 8,
            Rm: 0,
            Rn: 1,
            imm6: 0,
            shift: 0
        }
    );
    // eor w18, w18, w6
    assert_eq!(
        decode_root_unwrap(0x4A060252),
        InstructionKind::EOR32LogShift {
            Rd: 18,
            Rm: 6,
            Rn: 18,
            imm6: 0,
            shift: 0
        }
    );
    // eor w18, w0, w18
    assert_eq!(
        decode_root_unwrap(0x4A120012),
        InstructionKind::EOR32LogShift {
            Rd: 18,
            Rm: 18,
            Rn: 0,
            imm6: 0,
            shift: 0
        }
    );
    // eor w27, w27, #0x80000000
    assert_eq!(
        decode_root_unwrap(0x5201037B),
        InstructionKind::EOR32LogImm {
            Rd: 27,
            Rn: 27,
            immr: 1,
            imms: 0
        }
    );
    // eor w0, w8, #0x7fff0000
    assert_eq!(
        decode_root_unwrap(0x52103900),
        InstructionKind::EOR32LogImm {
            Rd: 0,
            Rn: 8,
            immr: 16,
            imms: 14
        }
    );
    // eor x15, x12, #0x7fff000000000000
    assert_eq!(
        decode_root_unwrap(0xD250398F),
        InstructionKind::EOR64LogImm {
            N: 1,
            Rd: 15,
            Rn: 12,
            immr: 16,
            imms: 14
        }
    );
}
#[test]
fn test_ext() {
    // ext v1.8b, v2.8b, v3.8b, #4
    assert_eq!(
        decode_root_unwrap(0x2E032041),
        InstructionKind::EXTAsimdextOnly {
            Q: 0,
            Rd: 1,
            Rm: 3,
            Rn: 2,
            imm4: 4
        }
    );
    // ext v2.16b, v2.16b, v2.16b, #8
    assert_eq!(
        decode_root_unwrap(0x6E024042),
        InstructionKind::EXTAsimdextOnly {
            Q: 1,
            Rd: 2,
            Rm: 2,
            Rn: 2,
            imm4: 8
        }
    );
    // ext v28.16b, v3.16b, v3.16b, #8
    assert_eq!(
        decode_root_unwrap(0x6E03407C),
        InstructionKind::EXTAsimdextOnly {
            Q: 1,
            Rd: 28,
            Rm: 3,
            Rn: 3,
            imm4: 8
        }
    );
    // ext v18.16b, v17.16b, v17.16b, #8
    assert_eq!(
        decode_root_unwrap(0x6E114232),
        InstructionKind::EXTAsimdextOnly {
            Q: 1,
            Rd: 18,
            Rm: 17,
            Rn: 17,
            imm4: 8
        }
    );
    // ext v26.16b, v21.16b, v21.16b, #8
    assert_eq!(
        decode_root_unwrap(0x6E1542BA),
        InstructionKind::EXTAsimdextOnly {
            Q: 1,
            Rd: 26,
            Rm: 21,
            Rn: 21,
            imm4: 8
        }
    );
    // ext v27.16b, v25.16b, v25.16b, #8
    assert_eq!(
        decode_root_unwrap(0x6E19433B),
        InstructionKind::EXTAsimdextOnly {
            Q: 1,
            Rd: 27,
            Rm: 25,
            Rn: 25,
            imm4: 8
        }
    );
}
#[test]
fn test_extr() {
    // extr w4, w3, w2, #8
    assert_eq!(
        decode_root_unwrap(0x13822064),
        InstructionKind::EXTR32Extract {
            Rd: 4,
            Rm: 2,
            Rn: 3,
            imms: 8
        }
    );
    // extr w10, w13, w10, #0x18
    assert_eq!(
        decode_root_unwrap(0x138A61AA),
        InstructionKind::EXTR32Extract {
            Rd: 10,
            Rm: 10,
            Rn: 13,
            imms: 24
        }
    );
    // extr x2, x4, x2, #0x20
    assert_eq!(
        decode_root_unwrap(0x93C28082),
        InstructionKind::EXTR64Extract {
            Rd: 2,
            Rm: 2,
            Rn: 4,
            imms: 32
        }
    );
    // extr x10, x9, x8, #0x28
    assert_eq!(
        decode_root_unwrap(0x93C8A12A),
        InstructionKind::EXTR64Extract {
            Rd: 10,
            Rm: 8,
            Rn: 9,
            imms: 40
        }
    );
    // extr x12, x21, x23, #0x3f
    assert_eq!(
        decode_root_unwrap(0x93D7FEAC),
        InstructionKind::EXTR64Extract {
            Rd: 12,
            Rm: 23,
            Rn: 21,
            imms: 63
        }
    );
    // extr x10, x21, x24, #0x3e
    assert_eq!(
        decode_root_unwrap(0x93D8FAAA),
        InstructionKind::EXTR64Extract {
            Rd: 10,
            Rm: 24,
            Rn: 21,
            imms: 62
        }
    );
}
#[test]
fn test_fabs() {
    // fabs s2, s0
    assert_eq!(
        decode_root_unwrap(0x1E20C002),
        InstructionKind::FABSSFloatdp1 { Rd: 2, Rn: 0 }
    );
    // fabs s0, s1
    assert_eq!(
        decode_root_unwrap(0x1E20C020),
        InstructionKind::FABSSFloatdp1 { Rd: 0, Rn: 1 }
    );
    // fabs s3, s3
    assert_eq!(
        decode_root_unwrap(0x1E20C063),
        InstructionKind::FABSSFloatdp1 { Rd: 3, Rn: 3 }
    );
    // fabs s0, s6
    assert_eq!(
        decode_root_unwrap(0x1E20C0C0),
        InstructionKind::FABSSFloatdp1 { Rd: 0, Rn: 6 }
    );
    // fabs s1, s9
    assert_eq!(
        decode_root_unwrap(0x1E20C121),
        InstructionKind::FABSSFloatdp1 { Rd: 1, Rn: 9 }
    );
    // fabs v25.4s, v29.4s
    assert_eq!(
        decode_root_unwrap(0x4EA0FBB9),
        InstructionKind::FABSAsimdmiscR {
            Q: 1,
            Rd: 25,
            Rn: 29,
            size: 2
        }
    );
}
#[test]
fn test_fadd() {
    // fadd v3.2s, v5.2s, v3.2s
    assert_eq!(
        decode_root_unwrap(0xE23D4A3),
        InstructionKind::FADDAsimdsameOnly {
            Q: 0,
            Rd: 3,
            Rm: 3,
            Rn: 5,
            size: 0
        }
    );
    // fadd s2, s0, s0
    assert_eq!(
        decode_root_unwrap(0x1E202802),
        InstructionKind::FADDSFloatdp2 {
            Rd: 2,
            Rm: 0,
            Rn: 0
        }
    );
    // fadd s16, s16, s3
    assert_eq!(
        decode_root_unwrap(0x1E232A10),
        InstructionKind::FADDSFloatdp2 {
            Rd: 16,
            Rm: 3,
            Rn: 16
        }
    );
    // fadd s22, s17, s4
    assert_eq!(
        decode_root_unwrap(0x1E242A36),
        InstructionKind::FADDSFloatdp2 {
            Rd: 22,
            Rm: 4,
            Rn: 17
        }
    );
    // fadd s3, s4, s18
    assert_eq!(
        decode_root_unwrap(0x1E322883),
        InstructionKind::FADDSFloatdp2 {
            Rd: 3,
            Rm: 18,
            Rn: 4
        }
    );
    // fadd d1, d19, d1
    assert_eq!(
        decode_root_unwrap(0x1E612A61),
        InstructionKind::FADDDFloatdp2 {
            Rd: 1,
            Rm: 1,
            Rn: 19
        }
    );
    // fadd v30.4s, v31.4s, v31.4s
    assert_eq!(
        decode_root_unwrap(0x4E3FD7FE),
        InstructionKind::FADDAsimdsameOnly {
            Q: 1,
            Rd: 30,
            Rm: 31,
            Rn: 31,
            size: 0
        }
    );
}
#[test]
fn test_faddp() {
    // faddp v2.2s, v1.2s, v1.2s
    assert_eq!(
        decode_root_unwrap(0x2E21D422),
        InstructionKind::FADDPAsimdsameOnly {
            Q: 0,
            Rd: 2,
            Rm: 1,
            Rn: 1,
            size: 0
        }
    );
    // faddp v17.2s, v17.2s, v18.2s
    assert_eq!(
        decode_root_unwrap(0x2E32D631),
        InstructionKind::FADDPAsimdsameOnly {
            Q: 0,
            Rd: 17,
            Rm: 18,
            Rn: 17,
            size: 0
        }
    );
    // faddp v18.2s, v18.2s, v19.2s
    assert_eq!(
        decode_root_unwrap(0x2E33D652),
        InstructionKind::FADDPAsimdsameOnly {
            Q: 0,
            Rd: 18,
            Rm: 19,
            Rn: 18,
            size: 0
        }
    );
    // faddp v21.2s, v21.2s, v22.2s
    assert_eq!(
        decode_root_unwrap(0x2E36D6B5),
        InstructionKind::FADDPAsimdsameOnly {
            Q: 0,
            Rd: 21,
            Rm: 22,
            Rn: 21,
            size: 0
        }
    );
    // faddp s3, v5.2s
    assert_eq!(
        decode_root_unwrap(0x7E30D8A3),
        InstructionKind::FADDPAsisdpairOnlySd {
            Rd: 3,
            Rn: 5,
            size: 0
        }
    );
    // faddp s23, v20.2s
    assert_eq!(
        decode_root_unwrap(0x7E30DA97),
        InstructionKind::FADDPAsisdpairOnlySd {
            Rd: 23,
            Rn: 20,
            size: 0
        }
    );
}
#[test]
fn test_fccmp() {
    // fccmp s8, s0, #4, mi
    assert_eq!(
        decode_root_unwrap(0x1E204504),
        InstructionKind::FCCMPSFloatccmp {
            Rm: 0,
            Rn: 8,
            cond: 4,
            nzcv: 4
        }
    );
    // fccmp s2, s0, #0, le
    assert_eq!(
        decode_root_unwrap(0x1E20D440),
        InstructionKind::FCCMPSFloatccmp {
            Rm: 0,
            Rn: 2,
            cond: 13,
            nzcv: 0
        }
    );
    // fccmp s3, s7, #8, pl
    assert_eq!(
        decode_root_unwrap(0x1E275468),
        InstructionKind::FCCMPSFloatccmp {
            Rm: 7,
            Rn: 3,
            cond: 5,
            nzcv: 8
        }
    );
    // fccmp s1, s9, #8, ls
    assert_eq!(
        decode_root_unwrap(0x1E299428),
        InstructionKind::FCCMPSFloatccmp {
            Rm: 9,
            Rn: 1,
            cond: 9,
            nzcv: 8
        }
    );
    // fccmp s0, s9, #2, ge
    assert_eq!(
        decode_root_unwrap(0x1E29A402),
        InstructionKind::FCCMPSFloatccmp {
            Rm: 9,
            Rn: 0,
            cond: 10,
            nzcv: 2
        }
    );
    // fccmp s28, s23, #8, ls
    assert_eq!(
        decode_root_unwrap(0x1E379788),
        InstructionKind::FCCMPSFloatccmp {
            Rm: 23,
            Rn: 28,
            cond: 9,
            nzcv: 8
        }
    );
}
#[test]
fn test_fcmeq() {
    // fcmeq v2.4s, v1.4s, #0.0
    assert_eq!(
        decode_root_unwrap(0x4EA0D822),
        InstructionKind::FCMEQAsimdmiscFz {
            Q: 1,
            Rd: 2,
            Rn: 1,
            size: 2
        }
    );
    // fcmeq v0.4s, v2.4s, #0.0
    assert_eq!(
        decode_root_unwrap(0x4EA0D840),
        InstructionKind::FCMEQAsimdmiscFz {
            Q: 1,
            Rd: 0,
            Rn: 2,
            size: 2
        }
    );
    // fcmeq v2.4s, v2.4s, #0.0
    assert_eq!(
        decode_root_unwrap(0x4EA0D842),
        InstructionKind::FCMEQAsimdmiscFz {
            Q: 1,
            Rd: 2,
            Rn: 2,
            size: 2
        }
    );
    // fcmeq v1.4s, v4.4s, #0.0
    assert_eq!(
        decode_root_unwrap(0x4EA0D881),
        InstructionKind::FCMEQAsimdmiscFz {
            Q: 1,
            Rd: 1,
            Rn: 4,
            size: 2
        }
    );
    // fcmeq v2.4s, v7.4s, #0.0
    assert_eq!(
        decode_root_unwrap(0x4EA0D8E2),
        InstructionKind::FCMEQAsimdmiscFz {
            Q: 1,
            Rd: 2,
            Rn: 7,
            size: 2
        }
    );
    // fcmeq v23.4s, v25.4s, #0.0
    assert_eq!(
        decode_root_unwrap(0x4EA0DB37),
        InstructionKind::FCMEQAsimdmiscFz {
            Q: 1,
            Rd: 23,
            Rn: 25,
            size: 2
        }
    );
}
#[test]
fn test_fcmge() {
    // fcmge v2.4s, v3.4s, v2.4s
    assert_eq!(
        decode_root_unwrap(0x6E22E462),
        InstructionKind::FCMGEAsimdsameOnly {
            Q: 1,
            Rd: 2,
            Rm: 2,
            Rn: 3,
            size: 0
        }
    );
    // fcmge v27.4s, v25.4s, v8.4s
    assert_eq!(
        decode_root_unwrap(0x6E28E73B),
        InstructionKind::FCMGEAsimdsameOnly {
            Q: 1,
            Rd: 27,
            Rm: 8,
            Rn: 25,
            size: 0
        }
    );
    // fcmge v24.4s, v24.4s, v28.4s
    assert_eq!(
        decode_root_unwrap(0x6E3CE718),
        InstructionKind::FCMGEAsimdsameOnly {
            Q: 1,
            Rd: 24,
            Rm: 28,
            Rn: 24,
            size: 0
        }
    );
    // fcmge v4.4s, v3.4s, #0.0
    assert_eq!(
        decode_root_unwrap(0x6EA0C864),
        InstructionKind::FCMGEAsimdmiscFz {
            Q: 1,
            Rd: 4,
            Rn: 3,
            size: 2
        }
    );
    // fcmge v6.4s, v4.4s, #0.0
    assert_eq!(
        decode_root_unwrap(0x6EA0C886),
        InstructionKind::FCMGEAsimdmiscFz {
            Q: 1,
            Rd: 6,
            Rn: 4,
            size: 2
        }
    );
    // fcmge v29.4s, v27.4s, #0.0
    assert_eq!(
        decode_root_unwrap(0x6EA0CB7D),
        InstructionKind::FCMGEAsimdmiscFz {
            Q: 1,
            Rd: 29,
            Rn: 27,
            size: 2
        }
    );
}
#[test]
fn test_fcmgt() {
    // fcmgt v1.4s, v3.4s, v1.4s
    assert_eq!(
        decode_root_unwrap(0x6EA1E461),
        InstructionKind::FCMGTAsimdsameOnly {
            Q: 1,
            Rd: 1,
            Rm: 1,
            Rn: 3,
            size: 2
        }
    );
    // fcmgt v2.4s, v3.4s, v2.4s
    assert_eq!(
        decode_root_unwrap(0x6EA2E462),
        InstructionKind::FCMGTAsimdsameOnly {
            Q: 1,
            Rd: 2,
            Rm: 2,
            Rn: 3,
            size: 2
        }
    );
    // fcmgt v26.4s, v8.4s, v5.4s
    assert_eq!(
        decode_root_unwrap(0x6EA5E51A),
        InstructionKind::FCMGTAsimdsameOnly {
            Q: 1,
            Rd: 26,
            Rm: 5,
            Rn: 8,
            size: 2
        }
    );
    // fcmgt v3.4s, v23.4s, v8.4s
    assert_eq!(
        decode_root_unwrap(0x6EA8E6E3),
        InstructionKind::FCMGTAsimdsameOnly {
            Q: 1,
            Rd: 3,
            Rm: 8,
            Rn: 23,
            size: 2
        }
    );
    // fcmgt v7.4s, v3.4s, v24.4s
    assert_eq!(
        decode_root_unwrap(0x6EB8E467),
        InstructionKind::FCMGTAsimdsameOnly {
            Q: 1,
            Rd: 7,
            Rm: 24,
            Rn: 3,
            size: 2
        }
    );
    // fcmgt v30.4s, v17.4s, v28.4s
    assert_eq!(
        decode_root_unwrap(0x6EBCE63E),
        InstructionKind::FCMGTAsimdsameOnly {
            Q: 1,
            Rd: 30,
            Rm: 28,
            Rn: 17,
            size: 2
        }
    );
}
#[test]
fn test_fcmle() {
    // fcmle d3, d1, #0.0
    assert_eq!(
        decode_root_unwrap(0x7EE0D823),
        InstructionKind::FCMLEAsisdmiscFz {
            Rd: 3,
            Rn: 1,
            size: 3
        }
    );
}
#[test]
fn test_fcmlt() {
    // fcmlt v19.4s, v7.4s, #0.0
    assert_eq!(
        decode_root_unwrap(0x4EA0E8F3),
        InstructionKind::FCMLTAsimdmiscFz {
            Q: 1,
            Rd: 19,
            Rn: 7,
            size: 2
        }
    );
    // fcmlt v27.4s, v29.4s, #0.0
    assert_eq!(
        decode_root_unwrap(0x4EA0EBBB),
        InstructionKind::FCMLTAsimdmiscFz {
            Q: 1,
            Rd: 27,
            Rn: 29,
            size: 2
        }
    );
}
#[test]
fn test_fcmp() {
    // fcmp s1, s0
    assert_eq!(
        decode_root_unwrap(0x1E202020),
        InstructionKind::FCMPSFloatcmp { Rm: 0, Rn: 1 }
    );
    // fcmp s25, #0.0
    assert_eq!(
        decode_root_unwrap(0x1E202328),
        InstructionKind::FCMPSzFloatcmp { Rm: 0, Rn: 25 }
    );
    // fcmp s17, s8
    assert_eq!(
        decode_root_unwrap(0x1E282220),
        InstructionKind::FCMPSFloatcmp { Rm: 8, Rn: 17 }
    );
    // fcmp s0, s13
    assert_eq!(
        decode_root_unwrap(0x1E2D2000),
        InstructionKind::FCMPSFloatcmp { Rm: 13, Rn: 0 }
    );
    // fcmp s3, s14
    assert_eq!(
        decode_root_unwrap(0x1E2E2060),
        InstructionKind::FCMPSFloatcmp { Rm: 14, Rn: 3 }
    );
    // fcmp d11, #0.0
    assert_eq!(
        decode_root_unwrap(0x1E602168),
        InstructionKind::FCMPDzFloatcmp { Rm: 0, Rn: 11 }
    );
}
#[test]
fn test_fcsel() {
    // fcsel s0, s1, s0, mi
    assert_eq!(
        decode_root_unwrap(0x1E204C20),
        InstructionKind::FCSELSFloatsel {
            Rd: 0,
            Rm: 0,
            Rn: 1,
            cond: 4
        }
    );
    // fcsel s3, s10, s3, gt
    assert_eq!(
        decode_root_unwrap(0x1E23CD43),
        InstructionKind::FCSELSFloatsel {
            Rd: 3,
            Rm: 3,
            Rn: 10,
            cond: 12
        }
    );
    // fcsel s3, s1, s4, gt
    assert_eq!(
        decode_root_unwrap(0x1E24CC23),
        InstructionKind::FCSELSFloatsel {
            Rd: 3,
            Rm: 4,
            Rn: 1,
            cond: 12
        }
    );
    // fcsel s2, s1, s13, eq
    assert_eq!(
        decode_root_unwrap(0x1E2D0C22),
        InstructionKind::FCSELSFloatsel {
            Rd: 2,
            Rm: 13,
            Rn: 1,
            cond: 0
        }
    );
    // fcsel s20, s20, s21, gt
    assert_eq!(
        decode_root_unwrap(0x1E35CE94),
        InstructionKind::FCSELSFloatsel {
            Rd: 20,
            Rm: 21,
            Rn: 20,
            cond: 12
        }
    );
    // fcsel d3, d20, d3, vs
    assert_eq!(
        decode_root_unwrap(0x1E636E83),
        InstructionKind::FCSELDFloatsel {
            Rd: 3,
            Rm: 3,
            Rn: 20,
            cond: 6
        }
    );
}
#[test]
fn test_fcvt() {
    // fcvt d16, s3
    assert_eq!(
        decode_root_unwrap(0x1E22C070),
        InstructionKind::FCVTDsFloatdp1 { Rd: 16, Rn: 3 }
    );
    // fcvt d6, s5
    assert_eq!(
        decode_root_unwrap(0x1E22C0A6),
        InstructionKind::FCVTDsFloatdp1 { Rd: 6, Rn: 5 }
    );
    // fcvt s2, d0
    assert_eq!(
        decode_root_unwrap(0x1E624002),
        InstructionKind::FCVTSdFloatdp1 { Rd: 2, Rn: 0 }
    );
    // fcvt s14, d0
    assert_eq!(
        decode_root_unwrap(0x1E62400E),
        InstructionKind::FCVTSdFloatdp1 { Rd: 14, Rn: 0 }
    );
    // fcvt s5, d5
    assert_eq!(
        decode_root_unwrap(0x1E6240A5),
        InstructionKind::FCVTSdFloatdp1 { Rd: 5, Rn: 5 }
    );
    // fcvt s16, d16
    assert_eq!(
        decode_root_unwrap(0x1E624210),
        InstructionKind::FCVTSdFloatdp1 { Rd: 16, Rn: 16 }
    );
}
#[test]
fn test_fcvtas() {
    // fcvtas x0, s0
    assert_eq!(
        decode_root_unwrap(0x9E240000),
        InstructionKind::FCVTAS64SFloat2Int { Rd: 0, Rn: 0 }
    );
    // fcvtas x0, d0
    assert_eq!(
        decode_root_unwrap(0x9E640000),
        InstructionKind::FCVTAS64DFloat2Int { Rd: 0, Rn: 0 }
    );
}
#[test]
fn test_fcvtms() {
    // fcvtms w0, s0
    assert_eq!(
        decode_root_unwrap(0x1E300000),
        InstructionKind::FCVTMS32SFloat2Int { Rd: 0, Rn: 0 }
    );
    // fcvtms w8, s0
    assert_eq!(
        decode_root_unwrap(0x1E300008),
        InstructionKind::FCVTMS32SFloat2Int { Rd: 8, Rn: 0 }
    );
    // fcvtms w19, s0
    assert_eq!(
        decode_root_unwrap(0x1E300013),
        InstructionKind::FCVTMS32SFloat2Int { Rd: 19, Rn: 0 }
    );
    // fcvtms w9, s16
    assert_eq!(
        decode_root_unwrap(0x1E300209),
        InstructionKind::FCVTMS32SFloat2Int { Rd: 9, Rn: 16 }
    );
    // fcvtms w21, s16
    assert_eq!(
        decode_root_unwrap(0x1E300215),
        InstructionKind::FCVTMS32SFloat2Int { Rd: 21, Rn: 16 }
    );
    // fcvtms w22, s19
    assert_eq!(
        decode_root_unwrap(0x1E300276),
        InstructionKind::FCVTMS32SFloat2Int { Rd: 22, Rn: 19 }
    );
}
#[test]
fn test_fcvtps() {
    // fcvtps w1, s0
    assert_eq!(
        decode_root_unwrap(0x1E280001),
        InstructionKind::FCVTPS32SFloat2Int { Rd: 1, Rn: 0 }
    );
    // fcvtps w8, s0
    assert_eq!(
        decode_root_unwrap(0x1E280008),
        InstructionKind::FCVTPS32SFloat2Int { Rd: 8, Rn: 0 }
    );
    // fcvtps w12, s1
    assert_eq!(
        decode_root_unwrap(0x1E28002C),
        InstructionKind::FCVTPS32SFloat2Int { Rd: 12, Rn: 1 }
    );
    // fcvtps w8, s2
    assert_eq!(
        decode_root_unwrap(0x1E280048),
        InstructionKind::FCVTPS32SFloat2Int { Rd: 8, Rn: 2 }
    );
    // fcvtps w18, s6
    assert_eq!(
        decode_root_unwrap(0x1E2800D2),
        InstructionKind::FCVTPS32SFloat2Int { Rd: 18, Rn: 6 }
    );
    // fcvtps w15, s17
    assert_eq!(
        decode_root_unwrap(0x1E28022F),
        InstructionKind::FCVTPS32SFloat2Int { Rd: 15, Rn: 17 }
    );
}
#[test]
fn test_fcvtpu() {
    // fcvtpu w8, s0
    assert_eq!(
        decode_root_unwrap(0x1E290008),
        InstructionKind::FCVTPU32SFloat2Int { Rd: 8, Rn: 0 }
    );
    // fcvtpu x0, s0
    assert_eq!(
        decode_root_unwrap(0x9E290000),
        InstructionKind::FCVTPU64SFloat2Int { Rd: 0, Rn: 0 }
    );
    // fcvtpu x9, s0
    assert_eq!(
        decode_root_unwrap(0x9E290009),
        InstructionKind::FCVTPU64SFloat2Int { Rd: 9, Rn: 0 }
    );
}
#[test]
fn test_fcvtzs() {
    // fcvtzs w10, s0
    assert_eq!(
        decode_root_unwrap(0x1E38000A),
        InstructionKind::FCVTZS32SFloat2Int { Rd: 10, Rn: 0 }
    );
    // fcvtzs w10, d0, #0x18
    assert_eq!(
        decode_root_unwrap(0x1E58A00A),
        InstructionKind::FCVTZS32DFloat2Fix {
            Rd: 10,
            Rn: 0,
            scale: 40
        }
    );
    // fcvtzs w9, d0, #8
    assert_eq!(
        decode_root_unwrap(0x1E58E009),
        InstructionKind::FCVTZS32DFloat2Fix {
            Rd: 9,
            Rn: 0,
            scale: 56
        }
    );
    // fcvtzs w8, d2, #8
    assert_eq!(
        decode_root_unwrap(0x1E58E048),
        InstructionKind::FCVTZS32DFloat2Fix {
            Rd: 8,
            Rn: 2,
            scale: 56
        }
    );
    // fcvtzs x8, s0
    assert_eq!(
        decode_root_unwrap(0x9E380008),
        InstructionKind::FCVTZS64SFloat2Int { Rd: 8, Rn: 0 }
    );
    // fcvtzs x13, s4
    assert_eq!(
        decode_root_unwrap(0x9E38008D),
        InstructionKind::FCVTZS64SFloat2Int { Rd: 13, Rn: 4 }
    );
}
#[test]
fn test_fcvtzu() {
    // fcvtzu w16, s0, #0x10
    assert_eq!(
        decode_root_unwrap(0x1E19C010),
        InstructionKind::FCVTZU32SFloat2Fix {
            Rd: 16,
            Rn: 0,
            scale: 48
        }
    );
    // fcvtzu w3, s0
    assert_eq!(
        decode_root_unwrap(0x1E390003),
        InstructionKind::FCVTZU32SFloat2Int { Rd: 3, Rn: 0 }
    );
    // fcvtzu w10, s0
    assert_eq!(
        decode_root_unwrap(0x1E39000A),
        InstructionKind::FCVTZU32SFloat2Int { Rd: 10, Rn: 0 }
    );
    // fcvtzu w11, s8
    assert_eq!(
        decode_root_unwrap(0x1E39010B),
        InstructionKind::FCVTZU32SFloat2Int { Rd: 11, Rn: 8 }
    );
    // fcvtzu x9, d0, #0xe
    assert_eq!(
        decode_root_unwrap(0x9E59C809),
        InstructionKind::FCVTZU64DFloat2Fix {
            Rd: 9,
            Rn: 0,
            scale: 50
        }
    );
    // fcvtzu x0, d0
    assert_eq!(
        decode_root_unwrap(0x9E790000),
        InstructionKind::FCVTZU64DFloat2Int { Rd: 0, Rn: 0 }
    );
}
#[test]
fn test_fdiv() {
    // fdiv s5, s2, s0
    assert_eq!(
        decode_root_unwrap(0x1E201845),
        InstructionKind::FDIVSFloatdp2 {
            Rd: 5,
            Rm: 0,
            Rn: 2
        }
    );
    // fdiv s2, s7, s0
    assert_eq!(
        decode_root_unwrap(0x1E2018E2),
        InstructionKind::FDIVSFloatdp2 {
            Rd: 2,
            Rm: 0,
            Rn: 7
        }
    );
    // fdiv s20, s7, s3
    assert_eq!(
        decode_root_unwrap(0x1E2318F4),
        InstructionKind::FDIVSFloatdp2 {
            Rd: 20,
            Rm: 3,
            Rn: 7
        }
    );
    // fdiv s15, s0, s11
    assert_eq!(
        decode_root_unwrap(0x1E2B180F),
        InstructionKind::FDIVSFloatdp2 {
            Rd: 15,
            Rm: 11,
            Rn: 0
        }
    );
    // fdiv s21, s22, s21
    assert_eq!(
        decode_root_unwrap(0x1E351AD5),
        InstructionKind::FDIVSFloatdp2 {
            Rd: 21,
            Rm: 21,
            Rn: 22
        }
    );
    // fdiv d10, d5, d6
    assert_eq!(
        decode_root_unwrap(0x1E6618AA),
        InstructionKind::FDIVDFloatdp2 {
            Rd: 10,
            Rm: 6,
            Rn: 5
        }
    );
}
#[test]
fn test_fmax() {
    // fmax s1, s1, s0
    assert_eq!(
        decode_root_unwrap(0x1E204821),
        InstructionKind::FMAXSFloatdp2 {
            Rd: 1,
            Rm: 0,
            Rn: 1
        }
    );
    // fmax s4, s0, s1
    assert_eq!(
        decode_root_unwrap(0x1E214804),
        InstructionKind::FMAXSFloatdp2 {
            Rd: 4,
            Rm: 1,
            Rn: 0
        }
    );
    // fmax s6, s1, s2
    assert_eq!(
        decode_root_unwrap(0x1E224826),
        InstructionKind::FMAXSFloatdp2 {
            Rd: 6,
            Rm: 2,
            Rn: 1
        }
    );
    // fmax s1, s1, s12
    assert_eq!(
        decode_root_unwrap(0x1E2C4821),
        InstructionKind::FMAXSFloatdp2 {
            Rd: 1,
            Rm: 12,
            Rn: 1
        }
    );
    // fmax s0, s0, s15
    assert_eq!(
        decode_root_unwrap(0x1E2F4800),
        InstructionKind::FMAXSFloatdp2 {
            Rd: 0,
            Rm: 15,
            Rn: 0
        }
    );
    // fmax s22, s20, s21
    assert_eq!(
        decode_root_unwrap(0x1E354A96),
        InstructionKind::FMAXSFloatdp2 {
            Rd: 22,
            Rm: 21,
            Rn: 20
        }
    );
}
#[test]
fn test_fmaxnm() {
    // fmaxnm s0, s3, s0
    assert_eq!(
        decode_root_unwrap(0x1E206860),
        InstructionKind::FMAXNMSFloatdp2 {
            Rd: 0,
            Rm: 0,
            Rn: 3
        }
    );
    // fmaxnm s1, s8, s0
    assert_eq!(
        decode_root_unwrap(0x1E206901),
        InstructionKind::FMAXNMSFloatdp2 {
            Rd: 1,
            Rm: 0,
            Rn: 8
        }
    );
    // fmaxnm s8, s8, s0
    assert_eq!(
        decode_root_unwrap(0x1E206908),
        InstructionKind::FMAXNMSFloatdp2 {
            Rd: 8,
            Rm: 0,
            Rn: 8
        }
    );
    // fmaxnm s2, s9, s2
    assert_eq!(
        decode_root_unwrap(0x1E226922),
        InstructionKind::FMAXNMSFloatdp2 {
            Rd: 2,
            Rm: 2,
            Rn: 9
        }
    );
    // fmaxnm s2, s2, s17
    assert_eq!(
        decode_root_unwrap(0x1E316842),
        InstructionKind::FMAXNMSFloatdp2 {
            Rd: 2,
            Rm: 17,
            Rn: 2
        }
    );
    // fmaxnm s18, s18, s20
    assert_eq!(
        decode_root_unwrap(0x1E346A52),
        InstructionKind::FMAXNMSFloatdp2 {
            Rd: 18,
            Rm: 20,
            Rn: 18
        }
    );
}
#[test]
fn test_fmin() {
    // fmin s0, s1, s0
    assert_eq!(
        decode_root_unwrap(0x1E205820),
        InstructionKind::FMINSFloatdp2 {
            Rd: 0,
            Rm: 0,
            Rn: 1
        }
    );
    // fmin s2, s3, s2
    assert_eq!(
        decode_root_unwrap(0x1E225862),
        InstructionKind::FMINSFloatdp2 {
            Rd: 2,
            Rm: 2,
            Rn: 3
        }
    );
    // fmin s9, s1, s5
    assert_eq!(
        decode_root_unwrap(0x1E255829),
        InstructionKind::FMINSFloatdp2 {
            Rd: 9,
            Rm: 5,
            Rn: 1
        }
    );
    // fmin s2, s11, s10
    assert_eq!(
        decode_root_unwrap(0x1E2A5962),
        InstructionKind::FMINSFloatdp2 {
            Rd: 2,
            Rm: 10,
            Rn: 11
        }
    );
    // fmin s0, s0, s15
    assert_eq!(
        decode_root_unwrap(0x1E2F5800),
        InstructionKind::FMINSFloatdp2 {
            Rd: 0,
            Rm: 15,
            Rn: 0
        }
    );
    // fmin s2, s17, s16
    assert_eq!(
        decode_root_unwrap(0x1E305A22),
        InstructionKind::FMINSFloatdp2 {
            Rd: 2,
            Rm: 16,
            Rn: 17
        }
    );
}
#[test]
fn test_fminnm() {
    // fminnm s1, s1, s0
    assert_eq!(
        decode_root_unwrap(0x1E207821),
        InstructionKind::FMINNMSFloatdp2 {
            Rd: 1,
            Rm: 0,
            Rn: 1
        }
    );
    // fminnm s1, s8, s1
    assert_eq!(
        decode_root_unwrap(0x1E217901),
        InstructionKind::FMINNMSFloatdp2 {
            Rd: 1,
            Rm: 1,
            Rn: 8
        }
    );
    // fminnm s9, s0, s2
    assert_eq!(
        decode_root_unwrap(0x1E227809),
        InstructionKind::FMINNMSFloatdp2 {
            Rd: 9,
            Rm: 2,
            Rn: 0
        }
    );
    // fminnm s1, s0, s10
    assert_eq!(
        decode_root_unwrap(0x1E2A7801),
        InstructionKind::FMINNMSFloatdp2 {
            Rd: 1,
            Rm: 10,
            Rn: 0
        }
    );
    // fminnm s3, s16, s17
    assert_eq!(
        decode_root_unwrap(0x1E317A03),
        InstructionKind::FMINNMSFloatdp2 {
            Rd: 3,
            Rm: 17,
            Rn: 16
        }
    );
    // fminnm s24, s23, s24
    assert_eq!(
        decode_root_unwrap(0x1E387AF8),
        InstructionKind::FMINNMSFloatdp2 {
            Rd: 24,
            Rm: 24,
            Rn: 23
        }
    );
}
#[test]
fn test_fminv() {
    // fminv s0, v0.4s
    assert_eq!(
        decode_root_unwrap(0x6EB0F800),
        InstructionKind::FMINVAsimdallOnlySd {
            Q: 1,
            Rd: 0,
            Rn: 0,
            size: 2
        }
    );
}
#[test]
fn test_fmla() {
    // fmla v3.4s, v1.4s, v0.4s
    assert_eq!(
        decode_root_unwrap(0x4E20CC23),
        InstructionKind::FMLAAsimdsameOnly {
            Q: 1,
            Rd: 3,
            Rm: 0,
            Rn: 1,
            size: 0
        }
    );
    // fmla v1.4s, v6.4s, v0.s[2]
    assert_eq!(
        decode_root_unwrap(0x4F8018C1),
        InstructionKind::FMLAAsimdelemRSd {
            H: 1,
            L: 0,
            M: 0,
            Q: 1,
            Rd: 1,
            Rm: 0,
            Rn: 6,
            size: 2
        }
    );
    // fmla v31.4s, v2.4s, v3.s[2]
    assert_eq!(
        decode_root_unwrap(0x4F83185F),
        InstructionKind::FMLAAsimdelemRSd {
            H: 1,
            L: 0,
            M: 0,
            Q: 1,
            Rd: 31,
            Rm: 3,
            Rn: 2,
            size: 2
        }
    );
    // fmla v16.4s, v17.4s, v3.s[2]
    assert_eq!(
        decode_root_unwrap(0x4F831A30),
        InstructionKind::FMLAAsimdelemRSd {
            H: 1,
            L: 0,
            M: 0,
            Q: 1,
            Rd: 16,
            Rm: 3,
            Rn: 17,
            size: 2
        }
    );
    // fmla v18.4s, v3.4s, v16.s[2]
    assert_eq!(
        decode_root_unwrap(0x4F901872),
        InstructionKind::FMLAAsimdelemRSd {
            H: 1,
            L: 0,
            M: 1,
            Q: 1,
            Rd: 18,
            Rm: 0,
            Rn: 3,
            size: 2
        }
    );
    // fmla v22.4s, v20.4s, v16.s[2]
    assert_eq!(
        decode_root_unwrap(0x4F901A96),
        InstructionKind::FMLAAsimdelemRSd {
            H: 1,
            L: 0,
            M: 1,
            Q: 1,
            Rd: 22,
            Rm: 0,
            Rn: 20,
            size: 2
        }
    );
}
#[test]
fn test_fmls() {
    // fmls v6.4s, v2.4s, v0.4s
    assert_eq!(
        decode_root_unwrap(0x4EA0CC46),
        InstructionKind::FMLSAsimdsameOnly {
            Q: 1,
            Rd: 6,
            Rm: 0,
            Rn: 2,
            size: 2
        }
    );
    // fmls v2.4s, v16.4s, v4.4s
    assert_eq!(
        decode_root_unwrap(0x4EA4CE02),
        InstructionKind::FMLSAsimdsameOnly {
            Q: 1,
            Rd: 2,
            Rm: 4,
            Rn: 16,
            size: 2
        }
    );
    // fmls v3.4s, v0.4s, v2.s[2]
    assert_eq!(
        decode_root_unwrap(0x4F825803),
        InstructionKind::FMLSAsimdelemRSd {
            H: 1,
            L: 0,
            M: 0,
            Q: 1,
            Rd: 3,
            Rm: 2,
            Rn: 0,
            size: 2
        }
    );
    // fmls v20.4s, v18.4s, v5.s[0]
    assert_eq!(
        decode_root_unwrap(0x4F855254),
        InstructionKind::FMLSAsimdelemRSd {
            H: 0,
            L: 0,
            M: 0,
            Q: 1,
            Rd: 20,
            Rm: 5,
            Rn: 18,
            size: 2
        }
    );
    // fmls v30.4s, v24.4s, v8.s[2]
    assert_eq!(
        decode_root_unwrap(0x4F885B1E),
        InstructionKind::FMLSAsimdelemRSd {
            H: 1,
            L: 0,
            M: 0,
            Q: 1,
            Rd: 30,
            Rm: 8,
            Rn: 24,
            size: 2
        }
    );
    // fmls v27.4s, v26.4s, v28.s[0]
    assert_eq!(
        decode_root_unwrap(0x4F9C535B),
        InstructionKind::FMLSAsimdelemRSd {
            H: 0,
            L: 0,
            M: 1,
            Q: 1,
            Rd: 27,
            Rm: 12,
            Rn: 26,
            size: 2
        }
    );
}
#[test]
fn test_fmov() {
    // fmov s0, #2.00000000
    assert_eq!(
        decode_root_unwrap(0x1E201000),
        InstructionKind::FMOVSFloatimm { Rd: 0, imm8: 0 }
    );
    // fmov s0, #6.00000000
    assert_eq!(
        decode_root_unwrap(0x1E231000),
        InstructionKind::FMOVSFloatimm { Rd: 0, imm8: 24 }
    );
    // fmov w9, s0
    assert_eq!(
        decode_root_unwrap(0x1E260009),
        InstructionKind::FMOV32SFloat2Int { Rd: 9, Rn: 0 }
    );
    // fmov w0, s8
    assert_eq!(
        decode_root_unwrap(0x1E260100),
        InstructionKind::FMOV32SFloat2Int { Rd: 0, Rn: 8 }
    );
    // fmov s13, w27
    assert_eq!(
        decode_root_unwrap(0x1E27036D),
        InstructionKind::FMOVS32Float2Int { Rd: 13, Rn: 27 }
    );
    // fmov v27.4s, #-1.00000000
    assert_eq!(
        decode_root_unwrap(0x4F07F61B),
        InstructionKind::FMOVAsimdimmSS {
            Q: 1,
            Rd: 27,
            a: 1,
            b: 1,
            c: 1,
            d: 1,
            e: 0,
            f: 0,
            g: 0,
            h: 0
        }
    );
    // fmov x11, d8
    assert_eq!(
        decode_root_unwrap(0x9E66010B),
        InstructionKind::FMOV64DFloat2Int { Rd: 11, Rn: 8 }
    );
}
#[test]
fn test_fmul() {
    // fmul s6, s0, s0
    assert_eq!(
        decode_root_unwrap(0x1E200806),
        InstructionKind::FMULSFloatdp2 {
            Rd: 6,
            Rm: 0,
            Rn: 0
        }
    );
    // fmul s16, s15, s2
    assert_eq!(
        decode_root_unwrap(0x1E2209F0),
        InstructionKind::FMULSFloatdp2 {
            Rd: 16,
            Rm: 2,
            Rn: 15
        }
    );
    // fmul s7, s13, s3
    assert_eq!(
        decode_root_unwrap(0x1E2309A7),
        InstructionKind::FMULSFloatdp2 {
            Rd: 7,
            Rm: 3,
            Rn: 13
        }
    );
    // fmul s0, s0, s5
    assert_eq!(
        decode_root_unwrap(0x1E250800),
        InstructionKind::FMULSFloatdp2 {
            Rd: 0,
            Rm: 5,
            Rn: 0
        }
    );
    // fmul s20, s6, s24
    assert_eq!(
        decode_root_unwrap(0x1E3808D4),
        InstructionKind::FMULSFloatdp2 {
            Rd: 20,
            Rm: 24,
            Rn: 6
        }
    );
    // fmul v31.4s, v21.4s, v31.s[0]
    assert_eq!(
        decode_root_unwrap(0x4F9F92BF),
        InstructionKind::FMULAsimdelemRSd {
            H: 0,
            L: 0,
            M: 1,
            Q: 1,
            Rd: 31,
            Rm: 15,
            Rn: 21,
            size: 2
        }
    );
}
#[test]
fn test_fneg() {
    // fneg s11, s6
    assert_eq!(
        decode_root_unwrap(0x1E2140CB),
        InstructionKind::FNEGSFloatdp1 { Rd: 11, Rn: 6 }
    );
    // fneg s0, s11
    assert_eq!(
        decode_root_unwrap(0x1E214160),
        InstructionKind::FNEGSFloatdp1 { Rd: 0, Rn: 11 }
    );
    // fneg s4, s12
    assert_eq!(
        decode_root_unwrap(0x1E214184),
        InstructionKind::FNEGSFloatdp1 { Rd: 4, Rn: 12 }
    );
    // fneg s14, s14
    assert_eq!(
        decode_root_unwrap(0x1E2141CE),
        InstructionKind::FNEGSFloatdp1 { Rd: 14, Rn: 14 }
    );
    // fneg d0, d0
    assert_eq!(
        decode_root_unwrap(0x1E614000),
        InstructionKind::FNEGDFloatdp1 { Rd: 0, Rn: 0 }
    );
    // fneg v24.4s, v16.4s
    assert_eq!(
        decode_root_unwrap(0x6EA0FA18),
        InstructionKind::FNEGAsimdmiscR {
            Q: 1,
            Rd: 24,
            Rn: 16,
            size: 2
        }
    );
}
#[test]
fn test_fnmul() {
    // fnmul s2, s2, s0
    assert_eq!(
        decode_root_unwrap(0x1E208842),
        InstructionKind::FNMULSFloatdp2 {
            Rd: 2,
            Rm: 0,
            Rn: 2
        }
    );
    // fnmul s6, s4, s2
    assert_eq!(
        decode_root_unwrap(0x1E228886),
        InstructionKind::FNMULSFloatdp2 {
            Rd: 6,
            Rm: 2,
            Rn: 4
        }
    );
    // fnmul s1, s1, s3
    assert_eq!(
        decode_root_unwrap(0x1E238821),
        InstructionKind::FNMULSFloatdp2 {
            Rd: 1,
            Rm: 3,
            Rn: 1
        }
    );
    // fnmul s0, s0, s6
    assert_eq!(
        decode_root_unwrap(0x1E268800),
        InstructionKind::FNMULSFloatdp2 {
            Rd: 0,
            Rm: 6,
            Rn: 0
        }
    );
    // fnmul s3, s6, s8
    assert_eq!(
        decode_root_unwrap(0x1E2888C3),
        InstructionKind::FNMULSFloatdp2 {
            Rd: 3,
            Rm: 8,
            Rn: 6
        }
    );
    // fnmul s20, s23, s21
    assert_eq!(
        decode_root_unwrap(0x1E358AF4),
        InstructionKind::FNMULSFloatdp2 {
            Rd: 20,
            Rm: 21,
            Rn: 23
        }
    );
}
#[test]
fn test_frecpe() {
    // frecpe v3.4s, v2.4s
    assert_eq!(
        decode_root_unwrap(0x4EA1D843),
        InstructionKind::FRECPEAsimdmiscR {
            Q: 1,
            Rd: 3,
            Rn: 2,
            size: 2
        }
    );
    // frecpe v18.4s, v6.4s
    assert_eq!(
        decode_root_unwrap(0x4EA1D8D2),
        InstructionKind::FRECPEAsimdmiscR {
            Q: 1,
            Rd: 18,
            Rn: 6,
            size: 2
        }
    );
    // frecpe v18.4s, v17.4s
    assert_eq!(
        decode_root_unwrap(0x4EA1DA32),
        InstructionKind::FRECPEAsimdmiscR {
            Q: 1,
            Rd: 18,
            Rn: 17,
            size: 2
        }
    );
    // frecpe v19.4s, v18.4s
    assert_eq!(
        decode_root_unwrap(0x4EA1DA53),
        InstructionKind::FRECPEAsimdmiscR {
            Q: 1,
            Rd: 19,
            Rn: 18,
            size: 2
        }
    );
    // frecpe v31.4s, v25.4s
    assert_eq!(
        decode_root_unwrap(0x4EA1DB3F),
        InstructionKind::FRECPEAsimdmiscR {
            Q: 1,
            Rd: 31,
            Rn: 25,
            size: 2
        }
    );
    // frecpe v5.4s, v28.4s
    assert_eq!(
        decode_root_unwrap(0x4EA1DB85),
        InstructionKind::FRECPEAsimdmiscR {
            Q: 1,
            Rd: 5,
            Rn: 28,
            size: 2
        }
    );
}
#[test]
fn test_frecps() {
    // frecps v1.4s, v2.4s, v1.4s
    assert_eq!(
        decode_root_unwrap(0x4E21FC41),
        InstructionKind::FRECPSAsimdsameOnly {
            Q: 1,
            Rd: 1,
            Rm: 1,
            Rn: 2,
            size: 0
        }
    );
    // frecps v26.4s, v23.4s, v3.4s
    assert_eq!(
        decode_root_unwrap(0x4E23FEFA),
        InstructionKind::FRECPSAsimdsameOnly {
            Q: 1,
            Rd: 26,
            Rm: 3,
            Rn: 23,
            size: 0
        }
    );
    // frecps v18.4s, v17.4s, v6.4s
    assert_eq!(
        decode_root_unwrap(0x4E26FE32),
        InstructionKind::FRECPSAsimdsameOnly {
            Q: 1,
            Rd: 18,
            Rm: 6,
            Rn: 17,
            size: 0
        }
    );
    // frecps v25.4s, v21.4s, v18.4s
    assert_eq!(
        decode_root_unwrap(0x4E32FEB9),
        InstructionKind::FRECPSAsimdsameOnly {
            Q: 1,
            Rd: 25,
            Rm: 18,
            Rn: 21,
            size: 0
        }
    );
    // frecps v12.4s, v11.4s, v30.4s
    assert_eq!(
        decode_root_unwrap(0x4E3EFD6C),
        InstructionKind::FRECPSAsimdsameOnly {
            Q: 1,
            Rd: 12,
            Rm: 30,
            Rn: 11,
            size: 0
        }
    );
    // frecps v30.4s, v31.4s, v30.4s
    assert_eq!(
        decode_root_unwrap(0x4E3EFFFE),
        InstructionKind::FRECPSAsimdsameOnly {
            Q: 1,
            Rd: 30,
            Rm: 30,
            Rn: 31,
            size: 0
        }
    );
}
#[test]
fn test_frintm() {
    // frintm s2, s0
    assert_eq!(
        decode_root_unwrap(0x1E254002),
        InstructionKind::FRINTMSFloatdp1 { Rd: 2, Rn: 0 }
    );
    // frintm s1, s1
    assert_eq!(
        decode_root_unwrap(0x1E254021),
        InstructionKind::FRINTMSFloatdp1 { Rd: 1, Rn: 1 }
    );
    // frintm s20, s19
    assert_eq!(
        decode_root_unwrap(0x1E254274),
        InstructionKind::FRINTMSFloatdp1 { Rd: 20, Rn: 19 }
    );
    // frintm d0, d0
    assert_eq!(
        decode_root_unwrap(0x1E654000),
        InstructionKind::FRINTMDFloatdp1 { Rd: 0, Rn: 0 }
    );
    // frintm d1, d1
    assert_eq!(
        decode_root_unwrap(0x1E654021),
        InstructionKind::FRINTMDFloatdp1 { Rd: 1, Rn: 1 }
    );
    // frintm d3, d3
    assert_eq!(
        decode_root_unwrap(0x1E654063),
        InstructionKind::FRINTMDFloatdp1 { Rd: 3, Rn: 3 }
    );
}
#[test]
fn test_frintp() {
    // frintp s0, s0
    assert_eq!(
        decode_root_unwrap(0x1E24C000),
        InstructionKind::FRINTPSFloatdp1 { Rd: 0, Rn: 0 }
    );
    // frintp s3, s0
    assert_eq!(
        decode_root_unwrap(0x1E24C003),
        InstructionKind::FRINTPSFloatdp1 { Rd: 3, Rn: 0 }
    );
    // frintp s1, s1
    assert_eq!(
        decode_root_unwrap(0x1E24C021),
        InstructionKind::FRINTPSFloatdp1 { Rd: 1, Rn: 1 }
    );
    // frintp s3, s1
    assert_eq!(
        decode_root_unwrap(0x1E24C023),
        InstructionKind::FRINTPSFloatdp1 { Rd: 3, Rn: 1 }
    );
    // frintp s2, s2
    assert_eq!(
        decode_root_unwrap(0x1E24C042),
        InstructionKind::FRINTPSFloatdp1 { Rd: 2, Rn: 2 }
    );
    // frintp d0, d0
    assert_eq!(
        decode_root_unwrap(0x1E64C000),
        InstructionKind::FRINTPDFloatdp1 { Rd: 0, Rn: 0 }
    );
}
#[test]
fn test_frintx() {
    // frintx s0, s0
    assert_eq!(
        decode_root_unwrap(0x1E274000),
        InstructionKind::FRINTXSFloatdp1 { Rd: 0, Rn: 0 }
    );
    // frintx s2, s1
    assert_eq!(
        decode_root_unwrap(0x1E274022),
        InstructionKind::FRINTXSFloatdp1 { Rd: 2, Rn: 1 }
    );
    // frintx s8, s8
    assert_eq!(
        decode_root_unwrap(0x1E274108),
        InstructionKind::FRINTXSFloatdp1 { Rd: 8, Rn: 8 }
    );
    // frintx d0, d0
    assert_eq!(
        decode_root_unwrap(0x1E674000),
        InstructionKind::FRINTXDFloatdp1 { Rd: 0, Rn: 0 }
    );
    // frintx d2, d1
    assert_eq!(
        decode_root_unwrap(0x1E674022),
        InstructionKind::FRINTXDFloatdp1 { Rd: 2, Rn: 1 }
    );
    // frintx d8, d8
    assert_eq!(
        decode_root_unwrap(0x1E674108),
        InstructionKind::FRINTXDFloatdp1 { Rd: 8, Rn: 8 }
    );
}
#[test]
fn test_frsqrte() {
    // frsqrte v0.4s, v1.4s
    assert_eq!(
        decode_root_unwrap(0x6EA1D820),
        InstructionKind::FRSQRTEAsimdmiscR {
            Q: 1,
            Rd: 0,
            Rn: 1,
            size: 2
        }
    );
    // frsqrte v2.4s, v1.4s
    assert_eq!(
        decode_root_unwrap(0x6EA1D822),
        InstructionKind::FRSQRTEAsimdmiscR {
            Q: 1,
            Rd: 2,
            Rn: 1,
            size: 2
        }
    );
    // frsqrte v6.4s, v2.4s
    assert_eq!(
        decode_root_unwrap(0x6EA1D846),
        InstructionKind::FRSQRTEAsimdmiscR {
            Q: 1,
            Rd: 6,
            Rn: 2,
            size: 2
        }
    );
    // frsqrte v1.4s, v5.4s
    assert_eq!(
        decode_root_unwrap(0x6EA1D8A1),
        InstructionKind::FRSQRTEAsimdmiscR {
            Q: 1,
            Rd: 1,
            Rn: 5,
            size: 2
        }
    );
    // frsqrte v4.4s, v5.4s
    assert_eq!(
        decode_root_unwrap(0x6EA1D8A4),
        InstructionKind::FRSQRTEAsimdmiscR {
            Q: 1,
            Rd: 4,
            Rn: 5,
            size: 2
        }
    );
    // frsqrte v17.4s, v16.4s
    assert_eq!(
        decode_root_unwrap(0x6EA1DA11),
        InstructionKind::FRSQRTEAsimdmiscR {
            Q: 1,
            Rd: 17,
            Rn: 16,
            size: 2
        }
    );
}
#[test]
fn test_frsqrts() {
    // frsqrts v2.4s, v0.4s, v2.4s
    assert_eq!(
        decode_root_unwrap(0x4EA2FC02),
        InstructionKind::FRSQRTSAsimdsameOnly {
            Q: 1,
            Rd: 2,
            Rm: 2,
            Rn: 0,
            size: 2
        }
    );
    // frsqrts v3.4s, v4.4s, v3.4s
    assert_eq!(
        decode_root_unwrap(0x4EA3FC83),
        InstructionKind::FRSQRTSAsimdsameOnly {
            Q: 1,
            Rd: 3,
            Rm: 3,
            Rn: 4,
            size: 2
        }
    );
    // frsqrts v7.4s, v4.4s, v7.4s
    assert_eq!(
        decode_root_unwrap(0x4EA7FC87),
        InstructionKind::FRSQRTSAsimdsameOnly {
            Q: 1,
            Rd: 7,
            Rm: 7,
            Rn: 4,
            size: 2
        }
    );
    // frsqrts v7.4s, v6.4s, v7.4s
    assert_eq!(
        decode_root_unwrap(0x4EA7FCC7),
        InstructionKind::FRSQRTSAsimdsameOnly {
            Q: 1,
            Rd: 7,
            Rm: 7,
            Rn: 6,
            size: 2
        }
    );
    // frsqrts v16.4s, v7.4s, v16.4s
    assert_eq!(
        decode_root_unwrap(0x4EB0FCF0),
        InstructionKind::FRSQRTSAsimdsameOnly {
            Q: 1,
            Rd: 16,
            Rm: 16,
            Rn: 7,
            size: 2
        }
    );
    // frsqrts v31.4s, v30.4s, v31.4s
    assert_eq!(
        decode_root_unwrap(0x4EBFFFDF),
        InstructionKind::FRSQRTSAsimdsameOnly {
            Q: 1,
            Rd: 31,
            Rm: 31,
            Rn: 30,
            size: 2
        }
    );
}
#[test]
fn test_fsqrt() {
    // fsqrt s3, s0
    assert_eq!(
        decode_root_unwrap(0x1E21C003),
        InstructionKind::FSQRTSFloatdp1 { Rd: 3, Rn: 0 }
    );
    // fsqrt s8, s0
    assert_eq!(
        decode_root_unwrap(0x1E21C008),
        InstructionKind::FSQRTSFloatdp1 { Rd: 8, Rn: 0 }
    );
    // fsqrt s12, s0
    assert_eq!(
        decode_root_unwrap(0x1E21C00C),
        InstructionKind::FSQRTSFloatdp1 { Rd: 12, Rn: 0 }
    );
    // fsqrt s0, s1
    assert_eq!(
        decode_root_unwrap(0x1E21C020),
        InstructionKind::FSQRTSFloatdp1 { Rd: 0, Rn: 1 }
    );
    // fsqrt s10, s11
    assert_eq!(
        decode_root_unwrap(0x1E21C16A),
        InstructionKind::FSQRTSFloatdp1 { Rd: 10, Rn: 11 }
    );
    // fsqrt d0, d1
    assert_eq!(
        decode_root_unwrap(0x1E61C020),
        InstructionKind::FSQRTDFloatdp1 { Rd: 0, Rn: 1 }
    );
}
#[test]
fn test_fsub() {
    // fsub s4, s1, s0
    assert_eq!(
        decode_root_unwrap(0x1E203824),
        InstructionKind::FSUBSFloatdp2 {
            Rd: 4,
            Rm: 0,
            Rn: 1
        }
    );
    // fsub s2, s4, s8
    assert_eq!(
        decode_root_unwrap(0x1E283882),
        InstructionKind::FSUBSFloatdp2 {
            Rd: 2,
            Rm: 8,
            Rn: 4
        }
    );
    // fsub s3, s0, s11
    assert_eq!(
        decode_root_unwrap(0x1E2B3803),
        InstructionKind::FSUBSFloatdp2 {
            Rd: 3,
            Rm: 11,
            Rn: 0
        }
    );
    // fsub s3, s18, s17
    assert_eq!(
        decode_root_unwrap(0x1E313A43),
        InstructionKind::FSUBSFloatdp2 {
            Rd: 3,
            Rm: 17,
            Rn: 18
        }
    );
    // fsub v4.4s, v4.4s, v3.4s
    assert_eq!(
        decode_root_unwrap(0x4EA3D484),
        InstructionKind::FSUBAsimdsameOnly {
            Q: 1,
            Rd: 4,
            Rm: 3,
            Rn: 4,
            size: 2
        }
    );
    // fsub v21.4s, v26.4s, v30.4s
    assert_eq!(
        decode_root_unwrap(0x4EBED755),
        InstructionKind::FSUBAsimdsameOnly {
            Q: 1,
            Rd: 21,
            Rm: 30,
            Rn: 26,
            size: 2
        }
    );
}
#[test]
fn test_ld1() {
    // ld1 {v5.s}[0], [x0]
    assert_eq!(
        decode_root_unwrap(0xD408005),
        InstructionKind::LD1AsisdlsoS11S {
            Q: 0,
            Rn: 0,
            Rt: 5,
            S: 0
        }
    );
    // ld1 {v1.s}[0], [x11]
    assert_eq!(
        decode_root_unwrap(0xD408161),
        InstructionKind::LD1AsisdlsoS11S {
            Q: 0,
            Rn: 11,
            Rt: 1,
            S: 0
        }
    );
    // ld1 {v3.s}[0], [x18]
    assert_eq!(
        decode_root_unwrap(0xD408243),
        InstructionKind::LD1AsisdlsoS11S {
            Q: 0,
            Rn: 18,
            Rt: 3,
            S: 0
        }
    );
    // ld1 {v5.s}[1], [x11]
    assert_eq!(
        decode_root_unwrap(0xD409165),
        InstructionKind::LD1AsisdlsoS11S {
            Q: 0,
            Rn: 11,
            Rt: 5,
            S: 1
        }
    );
    // ld1 {v1.s}[0], [x1], #4
    assert_eq!(
        decode_root_unwrap(0xDDF8021),
        InstructionKind::LD1AsisdlsopS1I1S {
            Q: 0,
            Rn: 1,
            Rt: 1,
            S: 0
        }
    );
    // ld1 {v0.s}[0], [x8], #4
    assert_eq!(
        decode_root_unwrap(0xDDF8100),
        InstructionKind::LD1AsisdlsopS1I1S {
            Q: 0,
            Rn: 8,
            Rt: 0,
            S: 0
        }
    );
    // ld1 {v3.s}[1], [x12], #4
    assert_eq!(
        decode_root_unwrap(0xDDF9183),
        InstructionKind::LD1AsisdlsopS1I1S {
            Q: 0,
            Rn: 12,
            Rt: 3,
            S: 1
        }
    );
    // ld1 {v1.s}[3], [x17]
    assert_eq!(
        decode_root_unwrap(0x4D409221),
        InstructionKind::LD1AsisdlsoS11S {
            Q: 1,
            Rn: 17,
            Rt: 1,
            S: 1
        }
    );
    // ld1 {v1.s}[3], [x27], x25
    assert_eq!(
        decode_root_unwrap(0x4DD99361),
        InstructionKind::LD1AsisdlsopSx1R1S {
            Q: 1,
            Rm: 25,
            Rn: 27,
            Rt: 1,
            S: 1
        }
    );
    // ld1 {v0.s}[3], [x8], #4
    assert_eq!(
        decode_root_unwrap(0x4DDF9100),
        InstructionKind::LD1AsisdlsopS1I1S {
            Q: 1,
            Rn: 8,
            Rt: 0,
            S: 1
        }
    );
    // ld1 {v0.s}[3], [x17], #4
    assert_eq!(
        decode_root_unwrap(0x4DDF9220),
        InstructionKind::LD1AsisdlsopS1I1S {
            Q: 1,
            Rn: 17,
            Rt: 0,
            S: 1
        }
    );
    // ld1 {v16.s}[3], [x21], #4
    assert_eq!(
        decode_root_unwrap(0x4DDF92B0),
        InstructionKind::LD1AsisdlsopS1I1S {
            Q: 1,
            Rn: 21,
            Rt: 16,
            S: 1
        }
    );
}
#[test]
fn test_ld1r() {
    // ld1r {v5.2s}, [x1], #4
    assert_eq!(
        decode_root_unwrap(0xDDFC825),
        InstructionKind::LD1RAsisdlsopR1I {
            Q: 0,
            Rn: 1,
            Rt: 5,
            size: 2
        }
    );
    // ld1r {v7.16b}, [x8]
    assert_eq!(
        decode_root_unwrap(0x4D40C107),
        InstructionKind::LD1RAsisdlsoR1 {
            Q: 1,
            Rn: 8,
            Rt: 7,
            size: 0
        }
    );
    // ld1r {v6.4s}, [x8]
    assert_eq!(
        decode_root_unwrap(0x4D40C906),
        InstructionKind::LD1RAsisdlsoR1 {
            Q: 1,
            Rn: 8,
            Rt: 6,
            size: 2
        }
    );
    // ld1r {v19.4s}, [x10]
    assert_eq!(
        decode_root_unwrap(0x4D40C953),
        InstructionKind::LD1RAsisdlsoR1 {
            Q: 1,
            Rn: 10,
            Rt: 19,
            size: 2
        }
    );
    // ld1r {v25.4s}, [x10]
    assert_eq!(
        decode_root_unwrap(0x4D40C959),
        InstructionKind::LD1RAsisdlsoR1 {
            Q: 1,
            Rn: 10,
            Rt: 25,
            size: 2
        }
    );
    // ld1r {v0.4s}, [x11]
    assert_eq!(
        decode_root_unwrap(0x4D40C960),
        InstructionKind::LD1RAsisdlsoR1 {
            Q: 1,
            Rn: 11,
            Rt: 0,
            size: 2
        }
    );
    // ld1r {v5.4s}, [x11]
    assert_eq!(
        decode_root_unwrap(0x4D40C965),
        InstructionKind::LD1RAsisdlsoR1 {
            Q: 1,
            Rn: 11,
            Rt: 5,
            size: 2
        }
    );
    // ld1r {v0.16b}, [x8], #1
    assert_eq!(
        decode_root_unwrap(0x4DDFC100),
        InstructionKind::LD1RAsisdlsopR1I {
            Q: 1,
            Rn: 8,
            Rt: 0,
            size: 0
        }
    );
    // ld1r {v1.4s}, [x8], #4
    assert_eq!(
        decode_root_unwrap(0x4DDFC901),
        InstructionKind::LD1RAsisdlsopR1I {
            Q: 1,
            Rn: 8,
            Rt: 1,
            size: 2
        }
    );
    // ld1r {v27.4s}, [x8], #4
    assert_eq!(
        decode_root_unwrap(0x4DDFC91B),
        InstructionKind::LD1RAsisdlsopR1I {
            Q: 1,
            Rn: 8,
            Rt: 27,
            size: 2
        }
    );
    // ld1r {v9.4s}, [x12], #4
    assert_eq!(
        decode_root_unwrap(0x4DDFC989),
        InstructionKind::LD1RAsisdlsopR1I {
            Q: 1,
            Rn: 12,
            Rt: 9,
            size: 2
        }
    );
    // ld1r {v6.4s}, [x16], #4
    assert_eq!(
        decode_root_unwrap(0x4DDFCA06),
        InstructionKind::LD1RAsisdlsopR1I {
            Q: 1,
            Rn: 16,
            Rt: 6,
            size: 2
        }
    );
}
#[test]
fn test_ld2() {
    // ld2 {v3.2s, v4.2s}, [x15]
    assert_eq!(
        decode_root_unwrap(0xC4089E3),
        InstructionKind::LD2AsisdlseR2 {
            Q: 0,
            Rn: 15,
            Rt: 3,
            size: 2
        }
    );
    // ld2 {v6.2d, v7.2d}, [x13]
    assert_eq!(
        decode_root_unwrap(0x4C408DA6),
        InstructionKind::LD2AsisdlseR2 {
            Q: 1,
            Rn: 13,
            Rt: 6,
            size: 3
        }
    );
    // ld2 {v4.2d, v5.2d}, [x15]
    assert_eq!(
        decode_root_unwrap(0x4C408DE4),
        InstructionKind::LD2AsisdlseR2 {
            Q: 1,
            Rn: 15,
            Rt: 4,
            size: 3
        }
    );
}
#[test]
fn test_ldar() {
    // ldar w8, [x8]
    assert_eq!(
        decode_root_unwrap(0x88DFFD08),
        InstructionKind::LDARLr32Ldstexcl {
            Rn: 8,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
    // ldar w9, [x27]
    assert_eq!(
        decode_root_unwrap(0x88DFFF69),
        InstructionKind::LDARLr32Ldstexcl {
            Rn: 27,
            Rs: 31,
            Rt: 9,
            Rt2: 31
        }
    );
    // ldar x0, [x8]
    assert_eq!(
        decode_root_unwrap(0xC8DFFD00),
        InstructionKind::LDARLr64Ldstexcl {
            Rn: 8,
            Rs: 31,
            Rt: 0,
            Rt2: 31
        }
    );
    // ldar x9, [x8]
    assert_eq!(
        decode_root_unwrap(0xC8DFFD09),
        InstructionKind::LDARLr64Ldstexcl {
            Rn: 8,
            Rs: 31,
            Rt: 9,
            Rt2: 31
        }
    );
    // ldar x28, [x27]
    assert_eq!(
        decode_root_unwrap(0xC8DFFF7C),
        InstructionKind::LDARLr64Ldstexcl {
            Rn: 27,
            Rs: 31,
            Rt: 28,
            Rt2: 31
        }
    );
    // ldar x24, [x28]
    assert_eq!(
        decode_root_unwrap(0xC8DFFF98),
        InstructionKind::LDARLr64Ldstexcl {
            Rn: 28,
            Rs: 31,
            Rt: 24,
            Rt2: 31
        }
    );
}
#[test]
fn test_ldarb() {
    // ldarb w8, [x0]
    assert_eq!(
        decode_root_unwrap(0x8DFFC08),
        InstructionKind::LDARBLr32Ldstexcl {
            Rn: 0,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
    // ldarb w9, [x8]
    assert_eq!(
        decode_root_unwrap(0x8DFFD09),
        InstructionKind::LDARBLr32Ldstexcl {
            Rn: 8,
            Rs: 31,
            Rt: 9,
            Rt2: 31
        }
    );
    // ldarb w10, [x10]
    assert_eq!(
        decode_root_unwrap(0x8DFFD4A),
        InstructionKind::LDARBLr32Ldstexcl {
            Rn: 10,
            Rs: 31,
            Rt: 10,
            Rt2: 31
        }
    );
    // ldarb w8, [x22]
    assert_eq!(
        decode_root_unwrap(0x8DFFEC8),
        InstructionKind::LDARBLr32Ldstexcl {
            Rn: 22,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
    // ldarb w8, [x26]
    assert_eq!(
        decode_root_unwrap(0x8DFFF48),
        InstructionKind::LDARBLr32Ldstexcl {
            Rn: 26,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
    // ldarb w8, [x27]
    assert_eq!(
        decode_root_unwrap(0x8DFFF68),
        InstructionKind::LDARBLr32Ldstexcl {
            Rn: 27,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
}
#[test]
fn test_ldarh() {
    // ldarh w0, [x0]
    assert_eq!(
        decode_root_unwrap(0x48DFFC00),
        InstructionKind::LDARHLr32Ldstexcl {
            Rn: 0,
            Rs: 31,
            Rt: 0,
            Rt2: 31
        }
    );
    // ldarh w8, [x8]
    assert_eq!(
        decode_root_unwrap(0x48DFFD08),
        InstructionKind::LDARHLr32Ldstexcl {
            Rn: 8,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
    // ldarh w8, [x20]
    assert_eq!(
        decode_root_unwrap(0x48DFFE88),
        InstructionKind::LDARHLr32Ldstexcl {
            Rn: 20,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
}
#[test]
fn test_ldaxr() {
    // ldaxr w8, [x2]
    assert_eq!(
        decode_root_unwrap(0x885FFC48),
        InstructionKind::LDAXRLr32Ldstexcl {
            Rn: 2,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
    // ldaxr w0, [x8]
    assert_eq!(
        decode_root_unwrap(0x885FFD00),
        InstructionKind::LDAXRLr32Ldstexcl {
            Rn: 8,
            Rs: 31,
            Rt: 0,
            Rt2: 31
        }
    );
    // ldaxr w10, [x27]
    assert_eq!(
        decode_root_unwrap(0x885FFF6A),
        InstructionKind::LDAXRLr32Ldstexcl {
            Rn: 27,
            Rs: 31,
            Rt: 10,
            Rt2: 31
        }
    );
    // ldaxr w10, [x28]
    assert_eq!(
        decode_root_unwrap(0x885FFF8A),
        InstructionKind::LDAXRLr32Ldstexcl {
            Rn: 28,
            Rs: 31,
            Rt: 10,
            Rt2: 31
        }
    );
    // ldaxr x8, [x0]
    assert_eq!(
        decode_root_unwrap(0xC85FFC08),
        InstructionKind::LDAXRLr64Ldstexcl {
            Rn: 0,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
    // ldaxr x0, [x9]
    assert_eq!(
        decode_root_unwrap(0xC85FFD20),
        InstructionKind::LDAXRLr64Ldstexcl {
            Rn: 9,
            Rs: 31,
            Rt: 0,
            Rt2: 31
        }
    );
}
#[test]
fn test_ldaxrb() {
    // ldaxrb w8, [x0]
    assert_eq!(
        decode_root_unwrap(0x85FFC08),
        InstructionKind::LDAXRBLr32Ldstexcl {
            Rn: 0,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
    // ldaxrb w9, [x0]
    assert_eq!(
        decode_root_unwrap(0x85FFC09),
        InstructionKind::LDAXRBLr32Ldstexcl {
            Rn: 0,
            Rs: 31,
            Rt: 9,
            Rt2: 31
        }
    );
    // ldaxrb w10, [x0]
    assert_eq!(
        decode_root_unwrap(0x85FFC0A),
        InstructionKind::LDAXRBLr32Ldstexcl {
            Rn: 0,
            Rs: 31,
            Rt: 10,
            Rt2: 31
        }
    );
    // ldaxrb w10, [x8]
    assert_eq!(
        decode_root_unwrap(0x85FFD0A),
        InstructionKind::LDAXRBLr32Ldstexcl {
            Rn: 8,
            Rs: 31,
            Rt: 10,
            Rt2: 31
        }
    );
}
#[test]
fn test_ldaxrh() {
    // ldaxrh w8, [x0]
    assert_eq!(
        decode_root_unwrap(0x485FFC08),
        InstructionKind::LDAXRHLr32Ldstexcl {
            Rn: 0,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
    // ldaxrh w9, [x0]
    assert_eq!(
        decode_root_unwrap(0x485FFC09),
        InstructionKind::LDAXRHLr32Ldstexcl {
            Rn: 0,
            Rs: 31,
            Rt: 9,
            Rt2: 31
        }
    );
    // ldaxrh w10, [x0]
    assert_eq!(
        decode_root_unwrap(0x485FFC0A),
        InstructionKind::LDAXRHLr32Ldstexcl {
            Rn: 0,
            Rs: 31,
            Rt: 10,
            Rt2: 31
        }
    );
    // ldaxrh w9, [x20]
    assert_eq!(
        decode_root_unwrap(0x485FFE89),
        InstructionKind::LDAXRHLr32Ldstexcl {
            Rn: 20,
            Rs: 31,
            Rt: 9,
            Rt2: 31
        }
    );
    // ldaxrh w10, [x20]
    assert_eq!(
        decode_root_unwrap(0x485FFE8A),
        InstructionKind::LDAXRHLr32Ldstexcl {
            Rn: 20,
            Rs: 31,
            Rt: 10,
            Rt2: 31
        }
    );
}
#[test]
fn test_ldp() {
    // ldp w8, w9, [x1], #8
    assert_eq!(
        decode_root_unwrap(0x28C12428),
        InstructionKind::LDP32LdstpairPost {
            Rn: 1,
            Rt: 8,
            Rt2: 9,
            imm7: 2
        }
    );
    // ldp w11, w12, [x9], #8
    assert_eq!(
        decode_root_unwrap(0x28C1312B),
        InstructionKind::LDP32LdstpairPost {
            Rn: 9,
            Rt: 11,
            Rt2: 12,
            imm7: 2
        }
    );
    // ldp w11, w13, [x11, #4]
    assert_eq!(
        decode_root_unwrap(0x2940B56B),
        InstructionKind::LDP32LdstpairOff {
            Rn: 11,
            Rt: 11,
            Rt2: 13,
            imm7: 1
        }
    );
    // ldp w19, w17, [x17, #8]
    assert_eq!(
        decode_root_unwrap(0x29414633),
        InstructionKind::LDP32LdstpairOff {
            Rn: 17,
            Rt: 19,
            Rt2: 17,
            imm7: 2
        }
    );
    // ldp w9, w12, [sp, #0x44]
    assert_eq!(
        decode_root_unwrap(0x2948B3E9),
        InstructionKind::LDP32LdstpairOff {
            Rn: 31,
            Rt: 9,
            Rt2: 12,
            imm7: 17
        }
    );
    // ldp s16, s17, [x16], #0x10
    assert_eq!(
        decode_root_unwrap(0x2CC24610),
        InstructionKind::LDPSLdstpairPost {
            Rn: 16,
            Rt: 16,
            Rt2: 17,
            imm7: 4
        }
    );
    // ldp d13, d12, [sp], #0x40
    assert_eq!(
        decode_root_unwrap(0x6CC433ED),
        InstructionKind::LDPDLdstpairPost {
            Rn: 31,
            Rt: 13,
            Rt2: 12,
            imm7: 8
        }
    );
    // ldp x14, x15, [x2], #0x10
    assert_eq!(
        decode_root_unwrap(0xA8C13C4E),
        InstructionKind::LDP64LdstpairPost {
            Rn: 2,
            Rt: 14,
            Rt2: 15,
            imm7: 2
        }
    );
    // ldp x24, x23, [sp], #0x40
    assert_eq!(
        decode_root_unwrap(0xA8C45FF8),
        InstructionKind::LDP64LdstpairPost {
            Rn: 31,
            Rt: 24,
            Rt2: 23,
            imm7: 8
        }
    );
    // ldp x8, x2, [x0]
    assert_eq!(
        decode_root_unwrap(0xA9400808),
        InstructionKind::LDP64LdstpairOff {
            Rn: 0,
            Rt: 8,
            Rt2: 2,
            imm7: 0
        }
    );
    // ldp x14, x12, [sp, #0x60]
    assert_eq!(
        decode_root_unwrap(0xA94633EE),
        InstructionKind::LDP64LdstpairOff {
            Rn: 31,
            Rt: 14,
            Rt2: 12,
            imm7: 12
        }
    );
    // ldp x1, x3, [x29, #-0x60]
    assert_eq!(
        decode_root_unwrap(0xA97A0FA1),
        InstructionKind::LDP64LdstpairOff {
            Rn: 29,
            Rt: 1,
            Rt2: 3,
            imm7: 116
        }
    );
    // ldp x23, x22, [x27, #-0x10]
    assert_eq!(
        decode_root_unwrap(0xA97F5B77),
        InstructionKind::LDP64LdstpairOff {
            Rn: 27,
            Rt: 23,
            Rt2: 22,
            imm7: 126
        }
    );
    // ldp q1, q0, [x29, #-0x30]
    assert_eq!(
        decode_root_unwrap(0xAD7E83A1),
        InstructionKind::LDPQLdstpairOff {
            Rn: 29,
            Rt: 1,
            Rt2: 0,
            imm7: 125
        }
    );
}
#[test]
fn test_ldpsw() {
    // ldpsw x6, x5, [x5]
    assert_eq!(
        decode_root_unwrap(0x694014A6),
        InstructionKind::LDPSW64LdstpairOff {
            Rn: 5,
            Rt: 6,
            Rt2: 5,
            imm7: 0
        }
    );
    // ldpsw x18, x14, [x14]
    assert_eq!(
        decode_root_unwrap(0x694039D2),
        InstructionKind::LDPSW64LdstpairOff {
            Rn: 14,
            Rt: 18,
            Rt2: 14,
            imm7: 0
        }
    );
    // ldpsw x10, x8, [x19, #8]
    assert_eq!(
        decode_root_unwrap(0x6941226A),
        InstructionKind::LDPSW64LdstpairOff {
            Rn: 19,
            Rt: 10,
            Rt2: 8,
            imm7: 2
        }
    );
    // ldpsw x1, x14, [x12, #0x30]
    assert_eq!(
        decode_root_unwrap(0x69463981),
        InstructionKind::LDPSW64LdstpairOff {
            Rn: 12,
            Rt: 1,
            Rt2: 14,
            imm7: 12
        }
    );
    // ldpsw x10, x11, [x19, #0xac]
    assert_eq!(
        decode_root_unwrap(0x6955AE6A),
        InstructionKind::LDPSW64LdstpairOff {
            Rn: 19,
            Rt: 10,
            Rt2: 11,
            imm7: 43
        }
    );
    // ldpsw x14, x15, [x11, #-0x10]
    assert_eq!(
        decode_root_unwrap(0x697E3D6E),
        InstructionKind::LDPSW64LdstpairOff {
            Rn: 11,
            Rt: 14,
            Rt2: 15,
            imm7: 124
        }
    );
}
#[test]
fn test_ldr() {
    // ldr w5, [x0], #4
    assert_eq!(
        decode_root_unwrap(0xB8404405),
        InstructionKind::LDR32LdstImmpost {
            Rn: 0,
            Rt: 5,
            imm9: 4
        }
    );
    // ldr w6, [x0, #4]!
    assert_eq!(
        decode_root_unwrap(0xB8404C06),
        InstructionKind::LDR32LdstImmpre {
            Rn: 0,
            Rt: 6,
            imm9: 4
        }
    );
    // ldr w23, [x20], #0xfffffffffffffffc
    assert_eq!(
        decode_root_unwrap(0xB85FC697),
        InstructionKind::LDR32LdstImmpost {
            Rn: 20,
            Rt: 23,
            imm9: 508
        }
    );
    // ldr w15, [x16, w15, sxtw #2]
    assert_eq!(
        decode_root_unwrap(0xB86FDA0F),
        InstructionKind::LDR32LdstRegoff {
            Rm: 15,
            Rn: 16,
            Rt: 15,
            S: 1,
            option: 6
        }
    );
    // ldr w3, [x0]
    assert_eq!(
        decode_root_unwrap(0xB9400003),
        InstructionKind::LDR32LdstPos {
            Rn: 0,
            Rt: 3,
            imm12: 0
        }
    );
    // ldr w0, [x8, #0x73c]
    assert_eq!(
        decode_root_unwrap(0xB9473D00),
        InstructionKind::LDR32LdstPos {
            Rn: 8,
            Rt: 0,
            imm12: 463
        }
    );
    // ldr s2, [x3, #0x6c]
    assert_eq!(
        decode_root_unwrap(0xBD406C62),
        InstructionKind::LDRSLdstPos {
            Rn: 3,
            Rt: 2,
            imm12: 27
        }
    );
    // ldr s17, [sp, #0x14c]
    assert_eq!(
        decode_root_unwrap(0xBD414FF1),
        InstructionKind::LDRSLdstPos {
            Rn: 31,
            Rt: 17,
            imm12: 83
        }
    );
    // ldr s1, [x8, #0xd08]
    assert_eq!(
        decode_root_unwrap(0xBD4D0901),
        InstructionKind::LDRSLdstPos {
            Rn: 8,
            Rt: 1,
            imm12: 834
        }
    );
    // ldr x1, [x22], #8
    assert_eq!(
        decode_root_unwrap(0xF84086C1),
        InstructionKind::LDR64LdstImmpost {
            Rn: 22,
            Rt: 1,
            imm9: 8
        }
    );
    // ldr x0, [x8, #8]!
    assert_eq!(
        decode_root_unwrap(0xF8408D00),
        InstructionKind::LDR64LdstImmpre {
            Rn: 8,
            Rt: 0,
            imm9: 8
        }
    );
    // ldr x22, [x21, #0x10]!
    assert_eq!(
        decode_root_unwrap(0xF8410EB6),
        InstructionKind::LDR64LdstImmpre {
            Rn: 21,
            Rt: 22,
            imm9: 16
        }
    );
    // ldr x0, [x20], #0x30
    assert_eq!(
        decode_root_unwrap(0xF8430680),
        InstructionKind::LDR64LdstImmpost {
            Rn: 20,
            Rt: 0,
            imm9: 48
        }
    );
    // ldr x8, [x22], #0x30
    assert_eq!(
        decode_root_unwrap(0xF84306C8),
        InstructionKind::LDR64LdstImmpost {
            Rn: 22,
            Rt: 8,
            imm9: 48
        }
    );
    // ldr x25, [x22, #0xa0]!
    assert_eq!(
        decode_root_unwrap(0xF84A0ED9),
        InstructionKind::LDR64LdstImmpre {
            Rn: 22,
            Rt: 25,
            imm9: 160
        }
    );
    // ldr x10, [x20, #-0x10]!
    assert_eq!(
        decode_root_unwrap(0xF85F0E8A),
        InstructionKind::LDR64LdstImmpre {
            Rn: 20,
            Rt: 10,
            imm9: 496
        }
    );
    // ldr x8, [x21, #-0x10]!
    assert_eq!(
        decode_root_unwrap(0xF85F0EA8),
        InstructionKind::LDR64LdstImmpre {
            Rn: 21,
            Rt: 8,
            imm9: 496
        }
    );
    // ldr x0, [x8, w19, sxtw #3]
    assert_eq!(
        decode_root_unwrap(0xF873D900),
        InstructionKind::LDR64LdstRegoff {
            Rm: 19,
            Rn: 8,
            Rt: 0,
            S: 1,
            option: 6
        }
    );
    // ldr x0, [x0]
    assert_eq!(
        decode_root_unwrap(0xF9400000),
        InstructionKind::LDR64LdstPos {
            Rn: 0,
            Rt: 0,
            imm12: 0
        }
    );
    // ldr d2, [x9], #0xfffffffffffffff8
    assert_eq!(
        decode_root_unwrap(0xFC5F8522),
        InstructionKind::LDRDLdstImmpost {
            Rn: 9,
            Rt: 2,
            imm9: 504
        }
    );
}
#[test]
fn test_ldrb() {
    // ldrb w8, [x0], #1
    assert_eq!(
        decode_root_unwrap(0x38401408),
        InstructionKind::LDRB32LdstImmpost {
            Rn: 0,
            Rt: 8,
            imm9: 1
        }
    );
    // ldrb w0, [x17], #1
    assert_eq!(
        decode_root_unwrap(0x38401620),
        InstructionKind::LDRB32LdstImmpost {
            Rn: 17,
            Rt: 0,
            imm9: 1
        }
    );
    // ldrb w24, [x18], #1
    assert_eq!(
        decode_root_unwrap(0x38401658),
        InstructionKind::LDRB32LdstImmpost {
            Rn: 18,
            Rt: 24,
            imm9: 1
        }
    );
    // ldrb w4, [x5, #1]!
    assert_eq!(
        decode_root_unwrap(0x38401CA4),
        InstructionKind::LDRB32LdstImmpre {
            Rn: 5,
            Rt: 4,
            imm9: 1
        }
    );
    // ldrb w13, [x11], #2
    assert_eq!(
        decode_root_unwrap(0x3840256D),
        InstructionKind::LDRB32LdstImmpost {
            Rn: 11,
            Rt: 13,
            imm9: 2
        }
    );
    // ldrb w10, [x9, #8]!
    assert_eq!(
        decode_root_unwrap(0x38408D2A),
        InstructionKind::LDRB32LdstImmpre {
            Rn: 9,
            Rt: 10,
            imm9: 8
        }
    );
    // ldrb w12, [x8], #0x70
    assert_eq!(
        decode_root_unwrap(0x3847050C),
        InstructionKind::LDRB32LdstImmpost {
            Rn: 8,
            Rt: 12,
            imm9: 112
        }
    );
    // ldrb w11, [x10, #0x81]!
    assert_eq!(
        decode_root_unwrap(0x38481D4B),
        InstructionKind::LDRB32LdstImmpre {
            Rn: 10,
            Rt: 11,
            imm9: 129
        }
    );
    // ldrb w8, [x0, #0xb0]!
    assert_eq!(
        decode_root_unwrap(0x384B0C08),
        InstructionKind::LDRB32LdstImmpre {
            Rn: 0,
            Rt: 8,
            imm9: 176
        }
    );
    // ldrb w11, [x10, #-0x10]!
    assert_eq!(
        decode_root_unwrap(0x385F0D4B),
        InstructionKind::LDRB32LdstImmpre {
            Rn: 10,
            Rt: 11,
            imm9: 496
        }
    );
    // ldrb w18, [x11], #0xffffffffffffffff
    assert_eq!(
        decode_root_unwrap(0x385FF572),
        InstructionKind::LDRB32LdstImmpost {
            Rn: 11,
            Rt: 18,
            imm9: 511
        }
    );
    // ldrb w14, [x13, #-1]!
    assert_eq!(
        decode_root_unwrap(0x385FFDAE),
        InstructionKind::LDRB32LdstImmpre {
            Rn: 13,
            Rt: 14,
            imm9: 511
        }
    );
    // ldrb w14, [x13, w14, sxtw]
    assert_eq!(
        decode_root_unwrap(0x386EC9AE),
        InstructionKind::LDRB32BLdstRegoff {
            Rm: 14,
            Rn: 13,
            Rt: 14,
            S: 0,
            option: 6
        }
    );
    // ldrb w9, [x1]
    assert_eq!(
        decode_root_unwrap(0x39400029),
        InstructionKind::LDRB32LdstPos {
            Rn: 1,
            Rt: 9,
            imm12: 0
        }
    );
    // ldrb w9, [x0, #6]
    assert_eq!(
        decode_root_unwrap(0x39401809),
        InstructionKind::LDRB32LdstPos {
            Rn: 0,
            Rt: 9,
            imm12: 6
        }
    );
    // ldrb w9, [x2, #0x1c]
    assert_eq!(
        decode_root_unwrap(0x39407049),
        InstructionKind::LDRB32LdstPos {
            Rn: 2,
            Rt: 9,
            imm12: 28
        }
    );
    // ldrb w10, [sp, #0xc7]
    assert_eq!(
        decode_root_unwrap(0x39431FEA),
        InstructionKind::LDRB32LdstPos {
            Rn: 31,
            Rt: 10,
            imm12: 199
        }
    );
    // ldrb w8, [x19, #0xbea]
    assert_eq!(
        decode_root_unwrap(0x396FAA68),
        InstructionKind::LDRB32LdstPos {
            Rn: 19,
            Rt: 8,
            imm12: 3050
        }
    );
}
#[test]
fn test_ldrh() {
    // ldrh w10, [x2], #2
    assert_eq!(
        decode_root_unwrap(0x7840244A),
        InstructionKind::LDRH32LdstImmpost {
            Rn: 2,
            Rt: 10,
            imm9: 2
        }
    );
    // ldrh w5, [x6], #2
    assert_eq!(
        decode_root_unwrap(0x784024C5),
        InstructionKind::LDRH32LdstImmpost {
            Rn: 6,
            Rt: 5,
            imm9: 2
        }
    );
    // ldrh w0, [x8], #2
    assert_eq!(
        decode_root_unwrap(0x78402500),
        InstructionKind::LDRH32LdstImmpost {
            Rn: 8,
            Rt: 0,
            imm9: 2
        }
    );
    // ldrh w24, [x19], #2
    assert_eq!(
        decode_root_unwrap(0x78402678),
        InstructionKind::LDRH32LdstImmpost {
            Rn: 19,
            Rt: 24,
            imm9: 2
        }
    );
    // ldrh w8, [x20, #2]!
    assert_eq!(
        decode_root_unwrap(0x78402E88),
        InstructionKind::LDRH32LdstImmpre {
            Rn: 20,
            Rt: 8,
            imm9: 2
        }
    );
    // ldrh w8, [x24, #2]!
    assert_eq!(
        decode_root_unwrap(0x78402F08),
        InstructionKind::LDRH32LdstImmpre {
            Rn: 24,
            Rt: 8,
            imm9: 2
        }
    );
    // ldrh w9, [x8, #6]!
    assert_eq!(
        decode_root_unwrap(0x78406D09),
        InstructionKind::LDRH32LdstImmpre {
            Rn: 8,
            Rt: 9,
            imm9: 6
        }
    );
    // ldrh w3, [x18, #0x34]!
    assert_eq!(
        decode_root_unwrap(0x78434E43),
        InstructionKind::LDRH32LdstImmpre {
            Rn: 18,
            Rt: 3,
            imm9: 52
        }
    );
    // ldrh w8, [x24, #0x6c]!
    assert_eq!(
        decode_root_unwrap(0x7846CF08),
        InstructionKind::LDRH32LdstImmpre {
            Rn: 24,
            Rt: 8,
            imm9: 108
        }
    );
    // ldrh w13, [x10, #-0x20]!
    assert_eq!(
        decode_root_unwrap(0x785E0D4D),
        InstructionKind::LDRH32LdstImmpre {
            Rn: 10,
            Rt: 13,
            imm9: 480
        }
    );
    // ldrh w16, [x15], #0xfffffffffffffff0
    assert_eq!(
        decode_root_unwrap(0x785F05F0),
        InstructionKind::LDRH32LdstImmpost {
            Rn: 15,
            Rt: 16,
            imm9: 496
        }
    );
    // ldrh w13, [x11], #0xfffffffffffffffe
    assert_eq!(
        decode_root_unwrap(0x785FE56D),
        InstructionKind::LDRH32LdstImmpost {
            Rn: 11,
            Rt: 13,
            imm9: 510
        }
    );
    // ldrh w4, [x14, x2]
    assert_eq!(
        decode_root_unwrap(0x786269C4),
        InstructionKind::LDRH32LdstRegoff {
            Rm: 2,
            Rn: 14,
            Rt: 4,
            S: 0,
            option: 3
        }
    );
    // ldrh w10, [x11, w10, uxtw #1]
    assert_eq!(
        decode_root_unwrap(0x786A596A),
        InstructionKind::LDRH32LdstRegoff {
            Rm: 10,
            Rn: 11,
            Rt: 10,
            S: 1,
            option: 2
        }
    );
    // ldrh w7, [x1]
    assert_eq!(
        decode_root_unwrap(0x79400027),
        InstructionKind::LDRH32LdstPos {
            Rn: 1,
            Rt: 7,
            imm12: 0
        }
    );
    // ldrh w9, [x5, #2]
    assert_eq!(
        decode_root_unwrap(0x794004A9),
        InstructionKind::LDRH32LdstPos {
            Rn: 5,
            Rt: 9,
            imm12: 1
        }
    );
    // ldrh w11, [sp, #0x18]
    assert_eq!(
        decode_root_unwrap(0x794033EB),
        InstructionKind::LDRH32LdstPos {
            Rn: 31,
            Rt: 11,
            imm12: 12
        }
    );
    // ldrh w8, [x8, #0x128]
    assert_eq!(
        decode_root_unwrap(0x79425108),
        InstructionKind::LDRH32LdstPos {
            Rn: 8,
            Rt: 8,
            imm12: 148
        }
    );
}
#[test]
fn test_ldrsb() {
    // ldrsb x12, [x10], #1
    assert_eq!(
        decode_root_unwrap(0x3880154C),
        InstructionKind::LDRSB64LdstImmpost {
            Rn: 10,
            Rt: 12,
            imm9: 1
        }
    );
    // ldrsb x8, [x20], #1
    assert_eq!(
        decode_root_unwrap(0x38801688),
        InstructionKind::LDRSB64LdstImmpost {
            Rn: 20,
            Rt: 8,
            imm9: 1
        }
    );
    // ldrsb x11, [x10, #0x11]!
    assert_eq!(
        decode_root_unwrap(0x38811D4B),
        InstructionKind::LDRSB64LdstImmpre {
            Rn: 10,
            Rt: 11,
            imm9: 17
        }
    );
    // ldrsb w9, [x8], #1
    assert_eq!(
        decode_root_unwrap(0x38C01509),
        InstructionKind::LDRSB32LdstImmpost {
            Rn: 8,
            Rt: 9,
            imm9: 1
        }
    );
    // ldrsb w11, [x9], #1
    assert_eq!(
        decode_root_unwrap(0x38C0152B),
        InstructionKind::LDRSB32LdstImmpost {
            Rn: 9,
            Rt: 11,
            imm9: 1
        }
    );
    // ldrsb w26, [x20], #1
    assert_eq!(
        decode_root_unwrap(0x38C0169A),
        InstructionKind::LDRSB32LdstImmpost {
            Rn: 20,
            Rt: 26,
            imm9: 1
        }
    );
    // ldrsb w9, [x0, #1]!
    assert_eq!(
        decode_root_unwrap(0x38C01C09),
        InstructionKind::LDRSB32LdstImmpre {
            Rn: 0,
            Rt: 9,
            imm9: 1
        }
    );
    // ldrsb w10, [x9, #1]!
    assert_eq!(
        decode_root_unwrap(0x38C01D2A),
        InstructionKind::LDRSB32LdstImmpre {
            Rn: 9,
            Rt: 10,
            imm9: 1
        }
    );
    // ldrsb w8, [x21, #1]!
    assert_eq!(
        decode_root_unwrap(0x38C01EA8),
        InstructionKind::LDRSB32LdstImmpre {
            Rn: 21,
            Rt: 8,
            imm9: 1
        }
    );
    // ldrsb w27, [x11, #2]!
    assert_eq!(
        decode_root_unwrap(0x38C02D7B),
        InstructionKind::LDRSB32LdstImmpre {
            Rn: 11,
            Rt: 27,
            imm9: 2
        }
    );
    // ldrsb w11, [x8], #0xffffffffffffffff
    assert_eq!(
        decode_root_unwrap(0x38DFF50B),
        InstructionKind::LDRSB32LdstImmpost {
            Rn: 8,
            Rt: 11,
            imm9: 511
        }
    );
    // ldrsb w13, [x9, #-1]!
    assert_eq!(
        decode_root_unwrap(0x38DFFD2D),
        InstructionKind::LDRSB32LdstImmpre {
            Rn: 9,
            Rt: 13,
            imm9: 511
        }
    );
    // ldrsb w22, [x21, x9]
    assert_eq!(
        decode_root_unwrap(0x38E96AB6),
        InstructionKind::LDRSB32BlLdstRegoff {
            Rm: 9,
            Rn: 21,
            Rt: 22,
            S: 0
        }
    );
    // ldrsb w15, [x10, w15, sxtw]
    assert_eq!(
        decode_root_unwrap(0x38EFC94F),
        InstructionKind::LDRSB32BLdstRegoff {
            Rm: 15,
            Rn: 10,
            Rt: 15,
            S: 0,
            option: 6
        }
    );
    // ldrsb w9, [x25, x28]
    assert_eq!(
        decode_root_unwrap(0x38FC6B29),
        InstructionKind::LDRSB32BlLdstRegoff {
            Rm: 28,
            Rn: 25,
            Rt: 9,
            S: 0
        }
    );
    // ldrsb x8, [x8]
    assert_eq!(
        decode_root_unwrap(0x39800108),
        InstructionKind::LDRSB64LdstPos {
            Rn: 8,
            Rt: 8,
            imm12: 0
        }
    );
    // ldrsb w12, [x0, #3]
    assert_eq!(
        decode_root_unwrap(0x39C00C0C),
        InstructionKind::LDRSB32LdstPos {
            Rn: 0,
            Rt: 12,
            imm12: 3
        }
    );
    // ldrsb w8, [x0, #0x84]
    assert_eq!(
        decode_root_unwrap(0x39C21008),
        InstructionKind::LDRSB32LdstPos {
            Rn: 0,
            Rt: 8,
            imm12: 132
        }
    );
    // ldrsb w8, [x19, #0x19a]
    assert_eq!(
        decode_root_unwrap(0x39C66A68),
        InstructionKind::LDRSB32LdstPos {
            Rn: 19,
            Rt: 8,
            imm12: 410
        }
    );
}
#[test]
fn test_ldrsh() {
    // ldrsh x14, [x21, x13]
    assert_eq!(
        decode_root_unwrap(0x78AD6AAE),
        InstructionKind::LDRSH64LdstRegoff {
            Rm: 13,
            Rn: 21,
            Rt: 14,
            S: 0,
            option: 3
        }
    );
    // ldrsh w12, [x0], #2
    assert_eq!(
        decode_root_unwrap(0x78C0240C),
        InstructionKind::LDRSH32LdstImmpost {
            Rn: 0,
            Rt: 12,
            imm9: 2
        }
    );
    // ldrsh w2, [x1], #2
    assert_eq!(
        decode_root_unwrap(0x78C02422),
        InstructionKind::LDRSH32LdstImmpost {
            Rn: 1,
            Rt: 2,
            imm9: 2
        }
    );
    // ldrsh w9, [x1], #2
    assert_eq!(
        decode_root_unwrap(0x78C02429),
        InstructionKind::LDRSH32LdstImmpost {
            Rn: 1,
            Rt: 9,
            imm9: 2
        }
    );
    // ldrsh w11, [x10], #2
    assert_eq!(
        decode_root_unwrap(0x78C0254B),
        InstructionKind::LDRSH32LdstImmpost {
            Rn: 10,
            Rt: 11,
            imm9: 2
        }
    );
    // ldrsh w10, [x9, #2]!
    assert_eq!(
        decode_root_unwrap(0x78C02D2A),
        InstructionKind::LDRSH32LdstImmpre {
            Rn: 9,
            Rt: 10,
            imm9: 2
        }
    );
    // ldrsh w9, [x22, #4]!
    assert_eq!(
        decode_root_unwrap(0x78C04EC9),
        InstructionKind::LDRSH32LdstImmpre {
            Rn: 22,
            Rt: 9,
            imm9: 4
        }
    );
    // ldrsh w6, [x5, #8]!
    assert_eq!(
        decode_root_unwrap(0x78C08CA6),
        InstructionKind::LDRSH32LdstImmpre {
            Rn: 5,
            Rt: 6,
            imm9: 8
        }
    );
    // ldrsh w13, [x12, #-0x60]!
    assert_eq!(
        decode_root_unwrap(0x78DA0D8D),
        InstructionKind::LDRSH32LdstImmpre {
            Rn: 12,
            Rt: 13,
            imm9: 416
        }
    );
    // ldrsh w13, [x9], #0xfffffffffffffffe
    assert_eq!(
        decode_root_unwrap(0x78DFE52D),
        InstructionKind::LDRSH32LdstImmpost {
            Rn: 9,
            Rt: 13,
            imm9: 510
        }
    );
    // ldrsh w10, [x26], #0xfffffffffffffffe
    assert_eq!(
        decode_root_unwrap(0x78DFE74A),
        InstructionKind::LDRSH32LdstImmpost {
            Rn: 26,
            Rt: 10,
            imm9: 510
        }
    );
    // ldrsh w13, [x9, #-2]!
    assert_eq!(
        decode_root_unwrap(0x78DFED2D),
        InstructionKind::LDRSH32LdstImmpre {
            Rn: 9,
            Rt: 13,
            imm9: 510
        }
    );
    // ldrsh w9, [x11, #-2]!
    assert_eq!(
        decode_root_unwrap(0x78DFED69),
        InstructionKind::LDRSH32LdstImmpre {
            Rn: 11,
            Rt: 9,
            imm9: 510
        }
    );
    // ldrsh w15, [x20, w15, sxtw #1]
    assert_eq!(
        decode_root_unwrap(0x78EFDA8F),
        InstructionKind::LDRSH32LdstRegoff {
            Rm: 15,
            Rn: 20,
            Rt: 15,
            S: 1,
            option: 6
        }
    );
    // ldrsh w0, [x0]
    assert_eq!(
        decode_root_unwrap(0x79C00000),
        InstructionKind::LDRSH32LdstPos {
            Rn: 0,
            Rt: 0,
            imm12: 0
        }
    );
    // ldrsh w0, [x8, #4]
    assert_eq!(
        decode_root_unwrap(0x79C00900),
        InstructionKind::LDRSH32LdstPos {
            Rn: 8,
            Rt: 0,
            imm12: 2
        }
    );
    // ldrsh w8, [x8, #6]
    assert_eq!(
        decode_root_unwrap(0x79C00D08),
        InstructionKind::LDRSH32LdstPos {
            Rn: 8,
            Rt: 8,
            imm12: 3
        }
    );
    // ldrsh w9, [x17, #0x80]
    assert_eq!(
        decode_root_unwrap(0x79C10229),
        InstructionKind::LDRSH32LdstPos {
            Rn: 17,
            Rt: 9,
            imm12: 64
        }
    );
}
#[test]
fn test_ldrsw() {
    // ldrsw x7, [x2], #4
    assert_eq!(
        decode_root_unwrap(0xB8804447),
        InstructionKind::LDRSW64LdstImmpost {
            Rn: 2,
            Rt: 7,
            imm9: 4
        }
    );
    // ldrsw x14, [x10], #4
    assert_eq!(
        decode_root_unwrap(0xB880454E),
        InstructionKind::LDRSW64LdstImmpost {
            Rn: 10,
            Rt: 14,
            imm9: 4
        }
    );
    // ldrsw x19, [x18], #4
    assert_eq!(
        decode_root_unwrap(0xB8804653),
        InstructionKind::LDRSW64LdstImmpost {
            Rn: 18,
            Rt: 19,
            imm9: 4
        }
    );
    // ldrsw x23, [x26], #4
    assert_eq!(
        decode_root_unwrap(0xB8804757),
        InstructionKind::LDRSW64LdstImmpost {
            Rn: 26,
            Rt: 23,
            imm9: 4
        }
    );
    // ldrsw x4, [x0, #8]!
    assert_eq!(
        decode_root_unwrap(0xB8808C04),
        InstructionKind::LDRSW64LdstImmpre {
            Rn: 0,
            Rt: 4,
            imm9: 8
        }
    );
    // ldrsw x3, [x18, #8]!
    assert_eq!(
        decode_root_unwrap(0xB8808E43),
        InstructionKind::LDRSW64LdstImmpre {
            Rn: 18,
            Rt: 3,
            imm9: 8
        }
    );
    // ldrsw x8, [x21, #0x1c]!
    assert_eq!(
        decode_root_unwrap(0xB881CEA8),
        InstructionKind::LDRSW64LdstImmpre {
            Rn: 21,
            Rt: 8,
            imm9: 28
        }
    );
    // ldrsw x8, [x28, #0x2c]!
    assert_eq!(
        decode_root_unwrap(0xB882CF88),
        InstructionKind::LDRSW64LdstImmpre {
            Rn: 28,
            Rt: 8,
            imm9: 44
        }
    );
    // ldrsw x9, [x8, #0x68]!
    assert_eq!(
        decode_root_unwrap(0xB8868D09),
        InstructionKind::LDRSW64LdstImmpre {
            Rn: 8,
            Rt: 9,
            imm9: 104
        }
    );
    // ldrsw x8, [x23], #0x84
    assert_eq!(
        decode_root_unwrap(0xB88846E8),
        InstructionKind::LDRSW64LdstImmpost {
            Rn: 23,
            Rt: 8,
            imm9: 132
        }
    );
    // ldrsw x25, [x9, #-0x14]!
    assert_eq!(
        decode_root_unwrap(0xB89ECD39),
        InstructionKind::LDRSW64LdstImmpre {
            Rn: 9,
            Rt: 25,
            imm9: 492
        }
    );
    // ldrsw x15, [x16], #0xfffffffffffffffc
    assert_eq!(
        decode_root_unwrap(0xB89FC60F),
        InstructionKind::LDRSW64LdstImmpost {
            Rn: 16,
            Rt: 15,
            imm9: 508
        }
    );
    // ldrsw x26, [x8, x21, lsl #2]
    assert_eq!(
        decode_root_unwrap(0xB8B5791A),
        InstructionKind::LDRSW64LdstRegoff {
            Rm: 21,
            Rn: 8,
            Rt: 26,
            S: 1,
            option: 3
        }
    );
    // ldrsw x12, [x21, w28, sxtw #2]
    assert_eq!(
        decode_root_unwrap(0xB8BCDAAC),
        InstructionKind::LDRSW64LdstRegoff {
            Rm: 28,
            Rn: 21,
            Rt: 12,
            S: 1,
            option: 6
        }
    );
    // ldrsw x2, [x8]
    assert_eq!(
        decode_root_unwrap(0xB9800102),
        InstructionKind::LDRSW64LdstPos {
            Rn: 8,
            Rt: 2,
            imm12: 0
        }
    );
    // ldrsw x22, [x22]
    assert_eq!(
        decode_root_unwrap(0xB98002D6),
        InstructionKind::LDRSW64LdstPos {
            Rn: 22,
            Rt: 22,
            imm12: 0
        }
    );
    // ldrsw x13, [x0, #8]
    assert_eq!(
        decode_root_unwrap(0xB980080D),
        InstructionKind::LDRSW64LdstPos {
            Rn: 0,
            Rt: 13,
            imm12: 2
        }
    );
    // ldrsw x8, [x22, #0x1c]
    assert_eq!(
        decode_root_unwrap(0xB9801EC8),
        InstructionKind::LDRSW64LdstPos {
            Rn: 22,
            Rt: 8,
            imm12: 7
        }
    );
}
#[test]
fn test_ldtrsb() {
    // ldtrsb x1, [x3, #0xc4]
    assert_eq!(
        decode_root_unwrap(0x388C4861),
        InstructionKind::LDTRSB64LdstUnpriv {
            Rn: 3,
            Rt: 1,
            imm9: 196
        }
    );
}
#[test]
fn test_ldur() {
    // ldur q0, [x8, #0x4c]
    assert_eq!(
        decode_root_unwrap(0x3CC4C100),
        InstructionKind::LDURQLdstUnscaled {
            Rn: 8,
            Rt: 0,
            imm9: 76
        }
    );
    // ldur w11, [x29, #-0xf0]
    assert_eq!(
        decode_root_unwrap(0xB85103AB),
        InstructionKind::LDUR32LdstUnscaled {
            Rn: 29,
            Rt: 11,
            imm9: 272
        }
    );
    // ldur x7, [x4, #6]
    assert_eq!(
        decode_root_unwrap(0xF8406087),
        InstructionKind::LDUR64LdstUnscaled {
            Rn: 4,
            Rt: 7,
            imm9: 6
        }
    );
    // ldur x11, [sp, #0x26]
    assert_eq!(
        decode_root_unwrap(0xF84263EB),
        InstructionKind::LDUR64LdstUnscaled {
            Rn: 31,
            Rt: 11,
            imm9: 38
        }
    );
    // ldur x25, [x29, #-0xf0]
    assert_eq!(
        decode_root_unwrap(0xF85103B9),
        InstructionKind::LDUR64LdstUnscaled {
            Rn: 29,
            Rt: 25,
            imm9: 272
        }
    );
    // ldur x21, [x21, #-0x10]
    assert_eq!(
        decode_root_unwrap(0xF85F02B5),
        InstructionKind::LDUR64LdstUnscaled {
            Rn: 21,
            Rt: 21,
            imm9: 496
        }
    );
}
#[test]
fn test_ldurb() {
    // ldurb w8, [x24, #-0x3f]
    assert_eq!(
        decode_root_unwrap(0x385C1308),
        InstructionKind::LDURB32LdstUnscaled {
            Rn: 24,
            Rt: 8,
            imm9: 449
        }
    );
    // ldurb w22, [x5, #-0x1b]
    assert_eq!(
        decode_root_unwrap(0x385E50B6),
        InstructionKind::LDURB32LdstUnscaled {
            Rn: 5,
            Rt: 22,
            imm9: 485
        }
    );
    // ldurb w21, [x28, #-0x10]
    assert_eq!(
        decode_root_unwrap(0x385F0395),
        InstructionKind::LDURB32LdstUnscaled {
            Rn: 28,
            Rt: 21,
            imm9: 496
        }
    );
    // ldurb w0, [x15, #-5]
    assert_eq!(
        decode_root_unwrap(0x385FB1E0),
        InstructionKind::LDURB32LdstUnscaled {
            Rn: 15,
            Rt: 0,
            imm9: 507
        }
    );
    // ldurb w1, [x0, #-1]
    assert_eq!(
        decode_root_unwrap(0x385FF001),
        InstructionKind::LDURB32LdstUnscaled {
            Rn: 0,
            Rt: 1,
            imm9: 511
        }
    );
    // ldurb w6, [x5, #-1]
    assert_eq!(
        decode_root_unwrap(0x385FF0A6),
        InstructionKind::LDURB32LdstUnscaled {
            Rn: 5,
            Rt: 6,
            imm9: 511
        }
    );
}
#[test]
fn test_ldurh() {
    // ldurh w0, [x8, #1]
    assert_eq!(
        decode_root_unwrap(0x78401100),
        InstructionKind::LDURH32LdstUnscaled {
            Rn: 8,
            Rt: 0,
            imm9: 1
        }
    );
    // ldurh w18, [x15, #-0x10]
    assert_eq!(
        decode_root_unwrap(0x785F01F2),
        InstructionKind::LDURH32LdstUnscaled {
            Rn: 15,
            Rt: 18,
            imm9: 496
        }
    );
    // ldurh w8, [x22, #-7]
    assert_eq!(
        decode_root_unwrap(0x785F92C8),
        InstructionKind::LDURH32LdstUnscaled {
            Rn: 22,
            Rt: 8,
            imm9: 505
        }
    );
    // ldurh w20, [x8, #-2]
    assert_eq!(
        decode_root_unwrap(0x785FE114),
        InstructionKind::LDURH32LdstUnscaled {
            Rn: 8,
            Rt: 20,
            imm9: 510
        }
    );
    // ldurh w11, [x9, #-2]
    assert_eq!(
        decode_root_unwrap(0x785FE12B),
        InstructionKind::LDURH32LdstUnscaled {
            Rn: 9,
            Rt: 11,
            imm9: 510
        }
    );
    // ldurh w16, [x11, #-2]
    assert_eq!(
        decode_root_unwrap(0x785FE170),
        InstructionKind::LDURH32LdstUnscaled {
            Rn: 11,
            Rt: 16,
            imm9: 510
        }
    );
}
#[test]
fn test_ldursb() {
    // ldursb w18, [x15, #-0x30]
    assert_eq!(
        decode_root_unwrap(0x38DD01F2),
        InstructionKind::LDURSB32LdstUnscaled {
            Rn: 15,
            Rt: 18,
            imm9: 464
        }
    );
    // ldursb w1, [x29, #-0x1d]
    assert_eq!(
        decode_root_unwrap(0x38DE33A1),
        InstructionKind::LDURSB32LdstUnscaled {
            Rn: 29,
            Rt: 1,
            imm9: 483
        }
    );
    // ldursb w12, [x10, #-2]
    assert_eq!(
        decode_root_unwrap(0x38DFE14C),
        InstructionKind::LDURSB32LdstUnscaled {
            Rn: 10,
            Rt: 12,
            imm9: 510
        }
    );
    // ldursb w18, [x17, #-2]
    assert_eq!(
        decode_root_unwrap(0x38DFE232),
        InstructionKind::LDURSB32LdstUnscaled {
            Rn: 17,
            Rt: 18,
            imm9: 510
        }
    );
    // ldursb w9, [x1, #-1]
    assert_eq!(
        decode_root_unwrap(0x38DFF029),
        InstructionKind::LDURSB32LdstUnscaled {
            Rn: 1,
            Rt: 9,
            imm9: 511
        }
    );
    // ldursb w1, [x8, #-1]
    assert_eq!(
        decode_root_unwrap(0x38DFF101),
        InstructionKind::LDURSB32LdstUnscaled {
            Rn: 8,
            Rt: 1,
            imm9: 511
        }
    );
}
#[test]
fn test_ldursh() {
    // ldursh x8, [x9, #1]
    assert_eq!(
        decode_root_unwrap(0x78801128),
        InstructionKind::LDURSH64LdstUnscaled {
            Rn: 9,
            Rt: 8,
            imm9: 1
        }
    );
    // ldursh x24, [x14, #-2]
    assert_eq!(
        decode_root_unwrap(0x789FE1D8),
        InstructionKind::LDURSH64LdstUnscaled {
            Rn: 14,
            Rt: 24,
            imm9: 510
        }
    );
    // ldursh w6, [x15, #-0x1e]
    assert_eq!(
        decode_root_unwrap(0x78DE21E6),
        InstructionKind::LDURSH32LdstUnscaled {
            Rn: 15,
            Rt: 6,
            imm9: 482
        }
    );
    // ldursh w14, [x23, #-0x10]
    assert_eq!(
        decode_root_unwrap(0x78DF02EE),
        InstructionKind::LDURSH32LdstUnscaled {
            Rn: 23,
            Rt: 14,
            imm9: 496
        }
    );
    // ldursh w5, [x4, #-2]
    assert_eq!(
        decode_root_unwrap(0x78DFE085),
        InstructionKind::LDURSH32LdstUnscaled {
            Rn: 4,
            Rt: 5,
            imm9: 510
        }
    );
    // ldursh w12, [x11, #-2]
    assert_eq!(
        decode_root_unwrap(0x78DFE16C),
        InstructionKind::LDURSH32LdstUnscaled {
            Rn: 11,
            Rt: 12,
            imm9: 510
        }
    );
}
#[test]
fn test_ldursw() {
    // ldursw x8, [x9, #1]
    assert_eq!(
        decode_root_unwrap(0xB8801128),
        InstructionKind::LDURSW64LdstUnscaled {
            Rn: 9,
            Rt: 8,
            imm9: 1
        }
    );
    // ldursw x16, [x10, #-0xc0]
    assert_eq!(
        decode_root_unwrap(0xB8940150),
        InstructionKind::LDURSW64LdstUnscaled {
            Rn: 10,
            Rt: 16,
            imm9: 320
        }
    );
    // ldursw x8, [x29, #-0x3c]
    assert_eq!(
        decode_root_unwrap(0xB89C43A8),
        InstructionKind::LDURSW64LdstUnscaled {
            Rn: 29,
            Rt: 8,
            imm9: 452
        }
    );
    // ldursw x21, [x29, #-0x14]
    assert_eq!(
        decode_root_unwrap(0xB89EC3B5),
        InstructionKind::LDURSW64LdstUnscaled {
            Rn: 29,
            Rt: 21,
            imm9: 492
        }
    );
    // ldursw x10, [x23, #-0x10]
    assert_eq!(
        decode_root_unwrap(0xB89F02EA),
        InstructionKind::LDURSW64LdstUnscaled {
            Rn: 23,
            Rt: 10,
            imm9: 496
        }
    );
    // ldursw x13, [x11, #-4]
    assert_eq!(
        decode_root_unwrap(0xB89FC16D),
        InstructionKind::LDURSW64LdstUnscaled {
            Rn: 11,
            Rt: 13,
            imm9: 508
        }
    );
}
#[test]
fn test_ldxr() {
    // ldxr w8, [x0]
    assert_eq!(
        decode_root_unwrap(0x885F7C08),
        InstructionKind::LDXRLr32Ldstexcl {
            Rn: 0,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
    // ldxr w11, [x10]
    assert_eq!(
        decode_root_unwrap(0x885F7D4B),
        InstructionKind::LDXRLr32Ldstexcl {
            Rn: 10,
            Rs: 31,
            Rt: 11,
            Rt2: 31
        }
    );
    // ldxr x0, [x8]
    assert_eq!(
        decode_root_unwrap(0xC85F7D00),
        InstructionKind::LDXRLr64Ldstexcl {
            Rn: 8,
            Rs: 31,
            Rt: 0,
            Rt2: 31
        }
    );
    // ldxr x9, [x20]
    assert_eq!(
        decode_root_unwrap(0xC85F7E89),
        InstructionKind::LDXRLr64Ldstexcl {
            Rn: 20,
            Rs: 31,
            Rt: 9,
            Rt2: 31
        }
    );
    // ldxr x8, [x21]
    assert_eq!(
        decode_root_unwrap(0xC85F7EA8),
        InstructionKind::LDXRLr64Ldstexcl {
            Rn: 21,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
    // ldxr x26, [x27]
    assert_eq!(
        decode_root_unwrap(0xC85F7F7A),
        InstructionKind::LDXRLr64Ldstexcl {
            Rn: 27,
            Rs: 31,
            Rt: 26,
            Rt2: 31
        }
    );
}
#[test]
fn test_ldxrb() {
    // ldxrb w9, [x0]
    assert_eq!(
        decode_root_unwrap(0x85F7C09),
        InstructionKind::LDXRBLr32Ldstexcl {
            Rn: 0,
            Rs: 31,
            Rt: 9,
            Rt2: 31
        }
    );
    // ldxrb w8, [x0]
    assert_eq!(
        decode_root_unwrap(0x85F7C08),
        InstructionKind::LDXRBLr32Ldstexcl {
            Rn: 0,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
}
#[test]
fn test_ldxrh() {
    // ldxrh w8, [x0]
    assert_eq!(
        decode_root_unwrap(0x485F7C08),
        InstructionKind::LDXRHLr32Ldstexcl {
            Rn: 0,
            Rs: 31,
            Rt: 8,
            Rt2: 31
        }
    );
    // ldxrh w9, [x0]
    assert_eq!(
        decode_root_unwrap(0x485F7C09),
        InstructionKind::LDXRHLr32Ldstexcl {
            Rn: 0,
            Rs: 31,
            Rt: 9,
            Rt2: 31
        }
    );
    // ldxrh w9, [x20]
    assert_eq!(
        decode_root_unwrap(0x485F7E89),
        InstructionKind::LDXRHLr32Ldstexcl {
            Rn: 20,
            Rs: 31,
            Rt: 9,
            Rt2: 31
        }
    );
    // ldxrh w10, [x20]
    assert_eq!(
        decode_root_unwrap(0x485F7E8A),
        InstructionKind::LDXRHLr32Ldstexcl {
            Rn: 20,
            Rs: 31,
            Rt: 10,
            Rt2: 31
        }
    );
}
#[test]
fn test_lsl() {
    // lsl w27, w8, w26
    assert_eq!(
        decode_root_unwrap(0x1ADA211B),
        InstructionKind::LSLV32Dp2Src {
            Rd: 27,
            Rm: 26,
            Rn: 8
        }
    );
    // lsl w27, w8, w28
    assert_eq!(
        decode_root_unwrap(0x1ADC211B),
        InstructionKind::LSLV32Dp2Src {
            Rd: 27,
            Rm: 28,
            Rn: 8
        }
    );
    // lsl w30, w27, #0x10
    assert_eq!(
        decode_root_unwrap(0x53103F7E),
        InstructionKind::UBFM32MBitfield {
            Rd: 30,
            Rn: 27,
            immr: 16,
            imms: 15
        }
    );
    // lsl w25, w20, #1
    assert_eq!(
        decode_root_unwrap(0x531F7A99),
        InstructionKind::UBFM32MBitfield {
            Rd: 25,
            Rn: 20,
            immr: 31,
            imms: 30
        }
    );
    // lsl x9, x9, x0
    assert_eq!(
        decode_root_unwrap(0x9AC02129),
        InstructionKind::LSLV64Dp2Src {
            Rd: 9,
            Rm: 0,
            Rn: 9
        }
    );
    // lsl x9, x25, #3
    assert_eq!(
        decode_root_unwrap(0xD37DF329),
        InstructionKind::UBFM64MBitfield {
            Rd: 9,
            Rn: 25,
            immr: 61,
            imms: 60
        }
    );
}
#[test]
fn test_lsr() {
    // lsr w7, w2, w0
    assert_eq!(
        decode_root_unwrap(0x1AC02447),
        InstructionKind::LSRV32Dp2Src {
            Rd: 7,
            Rm: 0,
            Rn: 2
        }
    );
    // lsr w11, w9, #1
    assert_eq!(
        decode_root_unwrap(0x53017D2B),
        InstructionKind::UBFM32MBitfield {
            Rd: 11,
            Rn: 9,
            immr: 1,
            imms: 31
        }
    );
    // lsr w19, w19, #7
    assert_eq!(
        decode_root_unwrap(0x53077E73),
        InstructionKind::UBFM32MBitfield {
            Rd: 19,
            Rn: 19,
            immr: 7,
            imms: 31
        }
    );
    // lsr x19, x10, #0x16
    assert_eq!(
        decode_root_unwrap(0xD356FD53),
        InstructionKind::UBFM64MBitfield {
            Rd: 19,
            Rn: 10,
            immr: 22,
            imms: 63
        }
    );
    // lsr x26, x27, #0x1f
    assert_eq!(
        decode_root_unwrap(0xD35FFF7A),
        InstructionKind::UBFM64MBitfield {
            Rd: 26,
            Rn: 27,
            immr: 31,
            imms: 63
        }
    );
    // lsr x28, x19, #0x3f
    assert_eq!(
        decode_root_unwrap(0xD37FFE7C),
        InstructionKind::UBFM64MBitfield {
            Rd: 28,
            Rn: 19,
            immr: 63,
            imms: 63
        }
    );
}
#[test]
fn test_madd() {
    // madd w0, w1, w1, w0
    assert_eq!(
        decode_root_unwrap(0x1B010020),
        InstructionKind::MADD32ADp3Src {
            Ra: 0,
            Rd: 0,
            Rm: 1,
            Rn: 1
        }
    );
    // madd w6, w6, w10, w19
    assert_eq!(
        decode_root_unwrap(0x1B0A4CC6),
        InstructionKind::MADD32ADp3Src {
            Ra: 19,
            Rd: 6,
            Rm: 10,
            Rn: 6
        }
    );
    // madd w8, w14, w14, w8
    assert_eq!(
        decode_root_unwrap(0x1B0E21C8),
        InstructionKind::MADD32ADp3Src {
            Ra: 8,
            Rd: 8,
            Rm: 14,
            Rn: 14
        }
    );
    // madd w14, w14, w26, w16
    assert_eq!(
        decode_root_unwrap(0x1B1A41CE),
        InstructionKind::MADD32ADp3Src {
            Ra: 16,
            Rd: 14,
            Rm: 26,
            Rn: 14
        }
    );
    // madd x2, x23, x8, x0
    assert_eq!(
        decode_root_unwrap(0x9B0802E2),
        InstructionKind::MADD64ADp3Src {
            Ra: 0,
            Rd: 2,
            Rm: 8,
            Rn: 23
        }
    );
    // madd x16, x10, x16, x15
    assert_eq!(
        decode_root_unwrap(0x9B103D50),
        InstructionKind::MADD64ADp3Src {
            Ra: 15,
            Rd: 16,
            Rm: 16,
            Rn: 10
        }
    );
}
#[test]
fn test_mneg() {
    // mneg x8, x0, x8
    assert_eq!(
        decode_root_unwrap(0x9B08FC08),
        InstructionKind::MSUB64ADp3Src {
            Ra: 31,
            Rd: 8,
            Rm: 8,
            Rn: 0
        }
    );
    // mneg x14, x10, x12
    assert_eq!(
        decode_root_unwrap(0x9B0CFD4E),
        InstructionKind::MSUB64ADp3Src {
            Ra: 31,
            Rd: 14,
            Rm: 12,
            Rn: 10
        }
    );
    // mneg x15, x15, x12
    assert_eq!(
        decode_root_unwrap(0x9B0CFDEF),
        InstructionKind::MSUB64ADp3Src {
            Ra: 31,
            Rd: 15,
            Rm: 12,
            Rn: 15
        }
    );
    // mneg x15, x11, x13
    assert_eq!(
        decode_root_unwrap(0x9B0DFD6F),
        InstructionKind::MSUB64ADp3Src {
            Ra: 31,
            Rd: 15,
            Rm: 13,
            Rn: 11
        }
    );
    // mneg x0, x0, x17
    assert_eq!(
        decode_root_unwrap(0x9B11FC00),
        InstructionKind::MSUB64ADp3Src {
            Ra: 31,
            Rd: 0,
            Rm: 17,
            Rn: 0
        }
    );
    // mneg x0, x0, x18
    assert_eq!(
        decode_root_unwrap(0x9B12FC00),
        InstructionKind::MSUB64ADp3Src {
            Ra: 31,
            Rd: 0,
            Rm: 18,
            Rn: 0
        }
    );
}
#[test]
fn test_mov() {
    // mov w11, #-0x14
    assert_eq!(
        decode_root_unwrap(0x1280026B),
        InstructionKind::MOVN32Movewide {
            Rd: 11,
            hw: 0,
            imm16: 19
        }
    );
    // mov w9, #-0x1c
    assert_eq!(
        decode_root_unwrap(0x12800369),
        InstructionKind::MOVN32Movewide {
            Rd: 9,
            hw: 0,
            imm16: 27
        }
    );
    // mov w0, #-0xfde8
    assert_eq!(
        decode_root_unwrap(0x129FBCE0),
        InstructionKind::MOVN32Movewide {
            Rd: 0,
            hw: 0,
            imm16: 64999
        }
    );
    // mov w10, #0x10ffff
    assert_eq!(
        decode_root_unwrap(0x12BFFDEA),
        InstructionKind::MOVN32Movewide {
            Rd: 10,
            hw: 1,
            imm16: 65519
        }
    );
    // mov w14, w15
    assert_eq!(
        decode_root_unwrap(0x2A0F03EE),
        InstructionKind::ORR32LogShift {
            Rd: 14,
            Rm: 15,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // mov w1, w28
    assert_eq!(
        decode_root_unwrap(0x2A1C03E1),
        InstructionKind::ORR32LogShift {
            Rd: 1,
            Rm: 28,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // mov v0.s[0], w8
    assert_eq!(
        decode_root_unwrap(0x4E041D00),
        InstructionKind::INSAsimdinsIrR {
            Rd: 0,
            Rn: 8,
            imm5: 4
        }
    );
    // mov v24.16b, v31.16b
    assert_eq!(
        decode_root_unwrap(0x4EBF1FF8),
        InstructionKind::ORRAsimdsameOnly {
            Q: 1,
            Rd: 24,
            Rm: 31,
            Rn: 31
        }
    );
    // mov w27, #0x1a
    assert_eq!(
        decode_root_unwrap(0x5280035B),
        InstructionKind::MOVZ32Movewide {
            Rd: 27,
            hw: 0,
            imm16: 26
        }
    );
    // mov w3, #0x39e8
    assert_eq!(
        decode_root_unwrap(0x52873D03),
        InstructionKind::MOVZ32Movewide {
            Rd: 3,
            hw: 0,
            imm16: 14824
        }
    );
    // mov w8, #0x8a48
    assert_eq!(
        decode_root_unwrap(0x52914908),
        InstructionKind::MOVZ32Movewide {
            Rd: 8,
            hw: 0,
            imm16: 35400
        }
    );
    // mov w8, #0xebe4
    assert_eq!(
        decode_root_unwrap(0x529D7C88),
        InstructionKind::MOVZ32Movewide {
            Rd: 8,
            hw: 0,
            imm16: 60388
        }
    );
    // mov w10, #0x55550000
    assert_eq!(
        decode_root_unwrap(0x52AAAAAA),
        InstructionKind::MOVZ32Movewide {
            Rd: 10,
            hw: 1,
            imm16: 21845
        }
    );
    // mov v3.s[0], v2.s[0]
    assert_eq!(
        decode_root_unwrap(0x6E040443),
        InstructionKind::INSAsimdinsIvV {
            Rd: 3,
            Rn: 2,
            imm4: 0,
            imm5: 4
        }
    );
    // mov v16.s[0], v7.s[0]
    assert_eq!(
        decode_root_unwrap(0x6E0404F0),
        InstructionKind::INSAsimdinsIvV {
            Rd: 16,
            Rn: 7,
            imm4: 0,
            imm5: 4
        }
    );
    // mov v31.s[1], v24.s[1]
    assert_eq!(
        decode_root_unwrap(0x6E0C271F),
        InstructionKind::INSAsimdinsIvV {
            Rd: 31,
            Rn: 24,
            imm4: 4,
            imm5: 12
        }
    );
    // mov v4.d[1], v1.d[0]
    assert_eq!(
        decode_root_unwrap(0x6E180424),
        InstructionKind::INSAsimdinsIvV {
            Rd: 4,
            Rn: 1,
            imm4: 0,
            imm5: 24
        }
    );
    // mov v19.s[3], v18.s[3]
    assert_eq!(
        decode_root_unwrap(0x6E1C6653),
        InstructionKind::INSAsimdinsIvV {
            Rd: 19,
            Rn: 18,
            imm4: 12,
            imm5: 28
        }
    );
    // mov x3, sp
    assert_eq!(
        decode_root_unwrap(0x910003E3),
        InstructionKind::ADD64AddsubImm {
            Rd: 3,
            Rn: 31,
            imm12: 0,
            shift: 0
        }
    );
    // mov x1, #-1
    assert_eq!(
        decode_root_unwrap(0x92800001),
        InstructionKind::MOVN64Movewide {
            Rd: 1,
            hw: 0,
            imm16: 0
        }
    );
    // mov x10, #-0x2711
    assert_eq!(
        decode_root_unwrap(0x9284E20A),
        InstructionKind::MOVN64Movewide {
            Rd: 10,
            hw: 0,
            imm16: 10000
        }
    );
    // mov x2, x13
    assert_eq!(
        decode_root_unwrap(0xAA0D03E2),
        InstructionKind::ORR64LogShift {
            Rd: 2,
            Rm: 13,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // mov x18, x25
    assert_eq!(
        decode_root_unwrap(0xAA1903F2),
        InstructionKind::ORR64LogShift {
            Rd: 18,
            Rm: 25,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // mov x4, #0
    assert_eq!(
        decode_root_unwrap(0xD2800004),
        InstructionKind::MOVZ64Movewide {
            Rd: 4,
            hw: 0,
            imm16: 0
        }
    );
}
#[test]
fn test_movi() {
    // movi d29, #0000000000000000
    assert_eq!(
        decode_root_unwrap(0x2F00E41D),
        InstructionKind::MOVIAsimdimmDDs {
            Rd: 29,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            g: 0,
            h: 0
        }
    );
    // movi v0.4s, #1
    assert_eq!(
        decode_root_unwrap(0x4F000420),
        InstructionKind::MOVIAsimdimmLSl {
            Q: 1,
            Rd: 0,
            a: 0,
            b: 0,
            c: 0,
            cmode: 0,
            d: 0,
            e: 0,
            f: 0,
            g: 0,
            h: 1
        }
    );
    // movi v3.4s, #0x80, lsl #24
    assert_eq!(
        decode_root_unwrap(0x4F046403),
        InstructionKind::MOVIAsimdimmLSl {
            Q: 1,
            Rd: 3,
            a: 1,
            b: 0,
            c: 0,
            cmode: 6,
            d: 0,
            e: 0,
            f: 0,
            g: 0,
            h: 0
        }
    );
    // movi v1.4s, #0xbf, lsl #24
    assert_eq!(
        decode_root_unwrap(0x4F0567E1),
        InstructionKind::MOVIAsimdimmLSl {
            Q: 1,
            Rd: 1,
            a: 1,
            b: 0,
            c: 1,
            cmode: 6,
            d: 1,
            e: 1,
            f: 1,
            g: 1,
            h: 1
        }
    );
    // movi v7.2d, #0000000000000000
    assert_eq!(
        decode_root_unwrap(0x6F00E407),
        InstructionKind::MOVIAsimdimmD2D {
            Rd: 7,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            g: 0,
            h: 0
        }
    );
    // movi v1.2d, #0xffffffffffffffff
    assert_eq!(
        decode_root_unwrap(0x6F07E7E1),
        InstructionKind::MOVIAsimdimmD2D {
            Rd: 1,
            a: 1,
            b: 1,
            c: 1,
            d: 1,
            e: 1,
            f: 1,
            g: 1,
            h: 1
        }
    );
}
#[test]
fn test_movk() {
    // movk w25, #0x364
    assert_eq!(
        decode_root_unwrap(0x72806C99),
        InstructionKind::MOVK32Movewide {
            Rd: 25,
            hw: 0,
            imm16: 868
        }
    );
    // movk w19, #0x1006
    assert_eq!(
        decode_root_unwrap(0x728200D3),
        InstructionKind::MOVK32Movewide {
            Rd: 19,
            hw: 0,
            imm16: 4102
        }
    );
    // movk w8, #0x1201
    assert_eq!(
        decode_root_unwrap(0x72824028),
        InstructionKind::MOVK32Movewide {
            Rd: 8,
            hw: 0,
            imm16: 4609
        }
    );
    // movk w14, #0xd7d7
    assert_eq!(
        decode_root_unwrap(0x729AFAEE),
        InstructionKind::MOVK32Movewide {
            Rd: 14,
            hw: 0,
            imm16: 55255
        }
    );
    // movk x6, #0
    assert_eq!(
        decode_root_unwrap(0xF2800006),
        InstructionKind::MOVK64Movewide {
            Rd: 6,
            hw: 0,
            imm16: 0
        }
    );
    // movk x12, #0x3e00
    assert_eq!(
        decode_root_unwrap(0xF287C00C),
        InstructionKind::MOVK64Movewide {
            Rd: 12,
            hw: 0,
            imm16: 15872
        }
    );
    // movk x27, #0xffff, lsl #32
    assert_eq!(
        decode_root_unwrap(0xF2DFFFFB),
        InstructionKind::MOVK64Movewide {
            Rd: 27,
            hw: 2,
            imm16: 65535
        }
    );
}
#[test]
fn test_msub() {
    // msub w0, w2, w0, w1
    assert_eq!(
        decode_root_unwrap(0x1B008440),
        InstructionKind::MSUB32ADp3Src {
            Ra: 1,
            Rd: 0,
            Rm: 0,
            Rn: 2
        }
    );
    // msub w9, w12, w8, w9
    assert_eq!(
        decode_root_unwrap(0x1B08A589),
        InstructionKind::MSUB32ADp3Src {
            Ra: 9,
            Rd: 9,
            Rm: 8,
            Rn: 12
        }
    );
    // msub w10, w14, w10, w13
    assert_eq!(
        decode_root_unwrap(0x1B0AB5CA),
        InstructionKind::MSUB32ADp3Src {
            Ra: 13,
            Rd: 10,
            Rm: 10,
            Rn: 14
        }
    );
    // msub w14, w14, w13, w9
    assert_eq!(
        decode_root_unwrap(0x1B0DA5CE),
        InstructionKind::MSUB32ADp3Src {
            Ra: 9,
            Rd: 14,
            Rm: 13,
            Rn: 14
        }
    );
    // msub w15, w18, w15, w19
    assert_eq!(
        decode_root_unwrap(0x1B0FCE4F),
        InstructionKind::MSUB32ADp3Src {
            Ra: 19,
            Rd: 15,
            Rm: 15,
            Rn: 18
        }
    );
    // msub w9, w9, w26, w8
    assert_eq!(
        decode_root_unwrap(0x1B1AA129),
        InstructionKind::MSUB32ADp3Src {
            Ra: 8,
            Rd: 9,
            Rm: 26,
            Rn: 9
        }
    );
}
#[test]
fn test_mul() {
    // mul w3, w12, w3
    assert_eq!(
        decode_root_unwrap(0x1B037D83),
        InstructionKind::MADD32ADp3Src {
            Ra: 31,
            Rd: 3,
            Rm: 3,
            Rn: 12
        }
    );
    // mul w8, w12, w13
    assert_eq!(
        decode_root_unwrap(0x1B0D7D88),
        InstructionKind::MADD32ADp3Src {
            Ra: 31,
            Rd: 8,
            Rm: 13,
            Rn: 12
        }
    );
    // mul v0.4s, v0.4s, v0.s[1]
    assert_eq!(
        decode_root_unwrap(0x4FA08000),
        InstructionKind::MULAsimdelemR {
            H: 0,
            L: 1,
            M: 0,
            Q: 1,
            Rd: 0,
            Rm: 0,
            Rn: 0,
            size: 2
        }
    );
    // mul x0, x28, x0
    assert_eq!(
        decode_root_unwrap(0x9B007F80),
        InstructionKind::MADD64ADp3Src {
            Ra: 31,
            Rd: 0,
            Rm: 0,
            Rn: 28
        }
    );
    // mul x0, x0, x4
    assert_eq!(
        decode_root_unwrap(0x9B047C00),
        InstructionKind::MADD64ADp3Src {
            Ra: 31,
            Rd: 0,
            Rm: 4,
            Rn: 0
        }
    );
    // mul x19, x21, x9
    assert_eq!(
        decode_root_unwrap(0x9B097EB3),
        InstructionKind::MADD64ADp3Src {
            Ra: 31,
            Rd: 19,
            Rm: 9,
            Rn: 21
        }
    );
}
#[test]
fn test_mvn() {
    // mvn w17, w1
    assert_eq!(
        decode_root_unwrap(0x2A2103F1),
        InstructionKind::ORN32LogShift {
            Rd: 17,
            Rm: 1,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // mvn w0, w8
    assert_eq!(
        decode_root_unwrap(0x2A2803E0),
        InstructionKind::ORN32LogShift {
            Rd: 0,
            Rm: 8,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // mvn w0, w13
    assert_eq!(
        decode_root_unwrap(0x2A2D03E0),
        InstructionKind::ORN32LogShift {
            Rd: 0,
            Rm: 13,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // mvn w15, w13
    assert_eq!(
        decode_root_unwrap(0x2A2D03EF),
        InstructionKind::ORN32LogShift {
            Rd: 15,
            Rm: 13,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // mvn v16.16b, v16.16b
    assert_eq!(
        decode_root_unwrap(0x6E205A10),
        InstructionKind::NOTAsimdmiscR {
            Q: 1,
            Rd: 16,
            Rn: 16
        }
    );
    // mvn x11, x9
    assert_eq!(
        decode_root_unwrap(0xAA2903EB),
        InstructionKind::ORN64LogShift {
            Rd: 11,
            Rm: 9,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
}
#[test]
fn test_neg() {
    // neg w12, w3
    assert_eq!(
        decode_root_unwrap(0x4B0303EC),
        InstructionKind::SUB32AddsubShift {
            Rd: 12,
            Rm: 3,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // neg w9, w8, lsl #7
    assert_eq!(
        decode_root_unwrap(0x4B081FE9),
        InstructionKind::SUB32AddsubShift {
            Rd: 9,
            Rm: 8,
            Rn: 31,
            imm6: 7,
            shift: 0
        }
    );
    // neg w4, w19
    assert_eq!(
        decode_root_unwrap(0x4B1303E4),
        InstructionKind::SUB32AddsubShift {
            Rd: 4,
            Rm: 19,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // neg x0, x2
    assert_eq!(
        decode_root_unwrap(0xCB0203E0),
        InstructionKind::SUB64AddsubShift {
            Rd: 0,
            Rm: 2,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // neg x12, x10, lsl #29
    assert_eq!(
        decode_root_unwrap(0xCB0A77EC),
        InstructionKind::SUB64AddsubShift {
            Rd: 12,
            Rm: 10,
            Rn: 31,
            imm6: 29,
            shift: 0
        }
    );
    // neg x23, x22
    assert_eq!(
        decode_root_unwrap(0xCB1603F7),
        InstructionKind::SUB64AddsubShift {
            Rd: 23,
            Rm: 22,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
}
#[test]
fn test_negs() {
    // negs x0, x0
    assert_eq!(
        decode_root_unwrap(0xEB0003E0),
        InstructionKind::SUBS64AddsubShift {
            Rd: 0,
            Rm: 0,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // negs x2, x2
    assert_eq!(
        decode_root_unwrap(0xEB0203E2),
        InstructionKind::SUBS64AddsubShift {
            Rd: 2,
            Rm: 2,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // negs x4, x5
    assert_eq!(
        decode_root_unwrap(0xEB0503E4),
        InstructionKind::SUBS64AddsubShift {
            Rd: 4,
            Rm: 5,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // negs x15, x15
    assert_eq!(
        decode_root_unwrap(0xEB0F03EF),
        InstructionKind::SUBS64AddsubShift {
            Rd: 15,
            Rm: 15,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
    // negs x17, x17
    assert_eq!(
        decode_root_unwrap(0xEB1103F1),
        InstructionKind::SUBS64AddsubShift {
            Rd: 17,
            Rm: 17,
            Rn: 31,
            imm6: 0,
            shift: 0
        }
    );
}
#[test]
fn test_ngcs() {
    // ngcs x1, x1
    assert_eq!(
        decode_root_unwrap(0xFA0103E1),
        InstructionKind::SBCS64AddsubCarry {
            Rd: 1,
            Rm: 1,
            Rn: 31
        }
    );
    // ngcs x3, x3
    assert_eq!(
        decode_root_unwrap(0xFA0303E3),
        InstructionKind::SBCS64AddsubCarry {
            Rd: 3,
            Rm: 3,
            Rn: 31
        }
    );
    // ngcs x14, x14
    assert_eq!(
        decode_root_unwrap(0xFA0E03EE),
        InstructionKind::SBCS64AddsubCarry {
            Rd: 14,
            Rm: 14,
            Rn: 31
        }
    );
}
#[test]
fn test_orn() {
    // orn w8, w26, w0
    assert_eq!(
        decode_root_unwrap(0x2A200348),
        InstructionKind::ORN32LogShift {
            Rd: 8,
            Rm: 0,
            Rn: 26,
            imm6: 0,
            shift: 0
        }
    );
    // orn w3, w1, w2
    assert_eq!(
        decode_root_unwrap(0x2A220023),
        InstructionKind::ORN32LogShift {
            Rd: 3,
            Rm: 2,
            Rn: 1,
            imm6: 0,
            shift: 0
        }
    );
    // orn w10, w11, w10
    assert_eq!(
        decode_root_unwrap(0x2A2A016A),
        InstructionKind::ORN32LogShift {
            Rd: 10,
            Rm: 10,
            Rn: 11,
            imm6: 0,
            shift: 0
        }
    );
    // orn w18, w10, w11
    assert_eq!(
        decode_root_unwrap(0x2A2B0152),
        InstructionKind::ORN32LogShift {
            Rd: 18,
            Rm: 11,
            Rn: 10,
            imm6: 0,
            shift: 0
        }
    );
    // orn w8, w17, w13
    assert_eq!(
        decode_root_unwrap(0x2A2D0228),
        InstructionKind::ORN32LogShift {
            Rd: 8,
            Rm: 13,
            Rn: 17,
            imm6: 0,
            shift: 0
        }
    );
    // orn w1, w17, w18
    assert_eq!(
        decode_root_unwrap(0x2A320221),
        InstructionKind::ORN32LogShift {
            Rd: 1,
            Rm: 18,
            Rn: 17,
            imm6: 0,
            shift: 0
        }
    );
}
#[test]
fn test_orr() {
    // orr w11, w11, #0x80000
    assert_eq!(
        decode_root_unwrap(0x320D016B),
        InstructionKind::ORR32LogImm {
            Rd: 11,
            Rn: 11,
            immr: 13,
            imms: 0
        }
    );
    // orr x14, x4, x2
    assert_eq!(
        decode_root_unwrap(0xAA02008E),
        InstructionKind::ORR64LogShift {
            Rd: 14,
            Rm: 2,
            Rn: 4,
            imm6: 0,
            shift: 0
        }
    );
    // orr x21, x21, x7
    assert_eq!(
        decode_root_unwrap(0xAA0702B5),
        InstructionKind::ORR64LogShift {
            Rd: 21,
            Rm: 7,
            Rn: 21,
            imm6: 0,
            shift: 0
        }
    );
    // orr x1, x13, x10
    assert_eq!(
        decode_root_unwrap(0xAA0A01A1),
        InstructionKind::ORR64LogShift {
            Rd: 1,
            Rm: 10,
            Rn: 13,
            imm6: 0,
            shift: 0
        }
    );
    // orr x9, x9, #1
    assert_eq!(
        decode_root_unwrap(0xB2400129),
        InstructionKind::ORR64LogImm {
            N: 1,
            Rd: 9,
            Rn: 9,
            immr: 0,
            imms: 0
        }
    );
    // orr x22, xzr, #0xfffffffffffffffe
    assert_eq!(
        decode_root_unwrap(0xB27FFBF6),
        InstructionKind::ORR64LogImm {
            N: 1,
            Rd: 22,
            Rn: 31,
            immr: 63,
            imms: 62
        }
    );
}
#[test]
fn test_pmull() {
    // pmull v0.1q, v1.1d, v0.1d
    assert_eq!(
        decode_root_unwrap(0xEE0E020),
        InstructionKind::PMULLAsimddiffL {
            Q: 0,
            Rd: 0,
            Rm: 0,
            Rn: 1,
            size: 3
        }
    );
    // pmull v2.1q, v0.1d, v1.1d
    assert_eq!(
        decode_root_unwrap(0xEE1E002),
        InstructionKind::PMULLAsimddiffL {
            Q: 0,
            Rd: 2,
            Rm: 1,
            Rn: 0,
            size: 3
        }
    );
    // pmull v31.1q, v19.1d, v1.1d
    assert_eq!(
        decode_root_unwrap(0xEE1E27F),
        InstructionKind::PMULLAsimddiffL {
            Q: 0,
            Rd: 31,
            Rm: 1,
            Rn: 19,
            size: 3
        }
    );
    // pmull v21.1q, v17.1d, v2.1d
    assert_eq!(
        decode_root_unwrap(0xEE2E235),
        InstructionKind::PMULLAsimddiffL {
            Q: 0,
            Rd: 21,
            Rm: 2,
            Rn: 17,
            size: 3
        }
    );
    // pmull v20.1q, v16.1d, v3.1d
    assert_eq!(
        decode_root_unwrap(0xEE3E214),
        InstructionKind::PMULLAsimddiffL {
            Q: 0,
            Rd: 20,
            Rm: 3,
            Rn: 16,
            size: 3
        }
    );
    // pmull v22.1q, v18.1d, v8.1d
    assert_eq!(
        decode_root_unwrap(0xEE8E256),
        InstructionKind::PMULLAsimddiffL {
            Q: 0,
            Rd: 22,
            Rm: 8,
            Rn: 18,
            size: 3
        }
    );
}
#[test]
fn test_pmull2() {
    // pmull2 v0.1q, v0.2d, v1.2d
    assert_eq!(
        decode_root_unwrap(0x4EE1E000),
        InstructionKind::PMULLAsimddiffL {
            Q: 1,
            Rd: 0,
            Rm: 1,
            Rn: 0,
            size: 3
        }
    );
    // pmull2 v19.1q, v19.2d, v1.2d
    assert_eq!(
        decode_root_unwrap(0x4EE1E273),
        InstructionKind::PMULLAsimddiffL {
            Q: 1,
            Rd: 19,
            Rm: 1,
            Rn: 19,
            size: 3
        }
    );
    // pmull2 v17.1q, v17.2d, v2.2d
    assert_eq!(
        decode_root_unwrap(0x4EE2E231),
        InstructionKind::PMULLAsimddiffL {
            Q: 1,
            Rd: 17,
            Rm: 2,
            Rn: 17,
            size: 3
        }
    );
    // pmull2 v25.1q, v17.2d, v2.2d
    assert_eq!(
        decode_root_unwrap(0x4EE2E239),
        InstructionKind::PMULLAsimddiffL {
            Q: 1,
            Rd: 25,
            Rm: 2,
            Rn: 17,
            size: 3
        }
    );
    // pmull2 v16.1q, v16.2d, v3.2d
    assert_eq!(
        decode_root_unwrap(0x4EE3E210),
        InstructionKind::PMULLAsimddiffL {
            Q: 1,
            Rd: 16,
            Rm: 3,
            Rn: 16,
            size: 3
        }
    );
    // pmull2 v26.1q, v18.2d, v8.2d
    assert_eq!(
        decode_root_unwrap(0x4EE8E25A),
        InstructionKind::PMULLAsimddiffL {
            Q: 1,
            Rd: 26,
            Rm: 8,
            Rn: 18,
            size: 3
        }
    );
}
#[test]
fn test_prfm() {
    // prfm pldl2strm, [x8]
    assert_eq!(
        decode_root_unwrap(0xF9800103),
        InstructionKind::PRFMPLdstPos {
            Rn: 8,
            Rt: 3,
            imm12: 0
        }
    );
    // prfm pldl2strm, [x10]
    assert_eq!(
        decode_root_unwrap(0xF9800143),
        InstructionKind::PRFMPLdstPos {
            Rn: 10,
            Rt: 3,
            imm12: 0
        }
    );
    // prfm pstl2strm, [x10]
    assert_eq!(
        decode_root_unwrap(0xF9800153),
        InstructionKind::PRFMPLdstPos {
            Rn: 10,
            Rt: 19,
            imm12: 0
        }
    );
    // prfm pstl2strm, [x11]
    assert_eq!(
        decode_root_unwrap(0xF9800173),
        InstructionKind::PRFMPLdstPos {
            Rn: 11,
            Rt: 19,
            imm12: 0
        }
    );
    // prfm pstl1keep, [x8, #0x40]
    assert_eq!(
        decode_root_unwrap(0xF9802110),
        InstructionKind::PRFMPLdstPos {
            Rn: 8,
            Rt: 16,
            imm12: 8
        }
    );
    // prfm pstl1keep, [x9, #0x40]
    assert_eq!(
        decode_root_unwrap(0xF9802130),
        InstructionKind::PRFMPLdstPos {
            Rn: 9,
            Rt: 16,
            imm12: 8
        }
    );
}
#[test]
fn test_rbit() {
    // rbit w10, w10
    assert_eq!(
        decode_root_unwrap(0x5AC0014A),
        InstructionKind::RBIT32Dp1Src { Rd: 10, Rn: 10 }
    );
    // rbit w11, w15
    assert_eq!(
        decode_root_unwrap(0x5AC001EB),
        InstructionKind::RBIT32Dp1Src { Rd: 11, Rn: 15 }
    );
    // rbit x8, x0
    assert_eq!(
        decode_root_unwrap(0xDAC00008),
        InstructionKind::RBIT64Dp1Src { Rd: 8, Rn: 0 }
    );
    // rbit x9, x9
    assert_eq!(
        decode_root_unwrap(0xDAC00129),
        InstructionKind::RBIT64Dp1Src { Rd: 9, Rn: 9 }
    );
    // rbit x11, x11
    assert_eq!(
        decode_root_unwrap(0xDAC0016B),
        InstructionKind::RBIT64Dp1Src { Rd: 11, Rn: 11 }
    );
    // rbit x11, x12
    assert_eq!(
        decode_root_unwrap(0xDAC0018B),
        InstructionKind::RBIT64Dp1Src { Rd: 11, Rn: 12 }
    );
}
#[test]
fn test_rev() {
    // rev w4, w5
    assert_eq!(
        decode_root_unwrap(0x5AC008A4),
        InstructionKind::REV32Dp1Src { Rd: 4, Rn: 5 }
    );
    // rev w28, w30
    assert_eq!(
        decode_root_unwrap(0x5AC00BDC),
        InstructionKind::REV32Dp1Src { Rd: 28, Rn: 30 }
    );
    // rev w30, w30
    assert_eq!(
        decode_root_unwrap(0x5AC00BDE),
        InstructionKind::REV32Dp1Src { Rd: 30, Rn: 30 }
    );
    // rev x8, x8
    assert_eq!(
        decode_root_unwrap(0xDAC00D08),
        InstructionKind::REV64Dp1Src { Rd: 8, Rn: 8 }
    );
    // rev x11, x11
    assert_eq!(
        decode_root_unwrap(0xDAC00D6B),
        InstructionKind::REV64Dp1Src { Rd: 11, Rn: 11 }
    );
    // rev x12, x12
    assert_eq!(
        decode_root_unwrap(0xDAC00D8C),
        InstructionKind::REV64Dp1Src { Rd: 12, Rn: 12 }
    );
}
#[test]
fn test_rev16() {
    // rev16 w5, w3
    assert_eq!(
        decode_root_unwrap(0x5AC00465),
        InstructionKind::REV1632Dp1Src { Rd: 5, Rn: 3 }
    );
    // rev16 w10, w9
    assert_eq!(
        decode_root_unwrap(0x5AC0052A),
        InstructionKind::REV1632Dp1Src { Rd: 10, Rn: 9 }
    );
    // rev16 w3, w11
    assert_eq!(
        decode_root_unwrap(0x5AC00563),
        InstructionKind::REV1632Dp1Src { Rd: 3, Rn: 11 }
    );
    // rev16 w12, w12
    assert_eq!(
        decode_root_unwrap(0x5AC0058C),
        InstructionKind::REV1632Dp1Src { Rd: 12, Rn: 12 }
    );
    // rev16 w26, w24
    assert_eq!(
        decode_root_unwrap(0x5AC0071A),
        InstructionKind::REV1632Dp1Src { Rd: 26, Rn: 24 }
    );
    // rev16 w11, w28
    assert_eq!(
        decode_root_unwrap(0x5AC0078B),
        InstructionKind::REV1632Dp1Src { Rd: 11, Rn: 28 }
    );
}
#[test]
fn test_rev32() {
    // rev32 v0.16b, v0.16b
    assert_eq!(
        decode_root_unwrap(0x6E200800),
        InstructionKind::REV32AsimdmiscR {
            Q: 1,
            Rd: 0,
            Rn: 0,
            size: 0
        }
    );
    // rev32 v1.16b, v1.16b
    assert_eq!(
        decode_root_unwrap(0x6E200821),
        InstructionKind::REV32AsimdmiscR {
            Q: 1,
            Rd: 1,
            Rn: 1,
            size: 0
        }
    );
    // rev32 v3.16b, v3.16b
    assert_eq!(
        decode_root_unwrap(0x6E200863),
        InstructionKind::REV32AsimdmiscR {
            Q: 1,
            Rd: 3,
            Rn: 3,
            size: 0
        }
    );
    // rev32 v5.16b, v5.16b
    assert_eq!(
        decode_root_unwrap(0x6E2008A5),
        InstructionKind::REV32AsimdmiscR {
            Q: 1,
            Rd: 5,
            Rn: 5,
            size: 0
        }
    );
    // rev32 v7.16b, v7.16b
    assert_eq!(
        decode_root_unwrap(0x6E2008E7),
        InstructionKind::REV32AsimdmiscR {
            Q: 1,
            Rd: 7,
            Rn: 7,
            size: 0
        }
    );
    // rev32 v16.16b, v16.16b
    assert_eq!(
        decode_root_unwrap(0x6E200A10),
        InstructionKind::REV32AsimdmiscR {
            Q: 1,
            Rd: 16,
            Rn: 16,
            size: 0
        }
    );
}
#[test]
fn test_rev64() {
    // rev64 v0.4s, v0.4s
    assert_eq!(
        decode_root_unwrap(0x4EA00800),
        InstructionKind::REV64AsimdmiscR {
            Q: 1,
            Rd: 0,
            Rn: 0,
            size: 2
        }
    );
    // rev64 v1.4s, v1.4s
    assert_eq!(
        decode_root_unwrap(0x4EA00821),
        InstructionKind::REV64AsimdmiscR {
            Q: 1,
            Rd: 1,
            Rn: 1,
            size: 2
        }
    );
    // rev64 v2.4s, v2.4s
    assert_eq!(
        decode_root_unwrap(0x4EA00842),
        InstructionKind::REV64AsimdmiscR {
            Q: 1,
            Rd: 2,
            Rn: 2,
            size: 2
        }
    );
    // rev64 v3.4s, v3.4s
    assert_eq!(
        decode_root_unwrap(0x4EA00863),
        InstructionKind::REV64AsimdmiscR {
            Q: 1,
            Rd: 3,
            Rn: 3,
            size: 2
        }
    );
    // rev64 v4.4s, v4.4s
    assert_eq!(
        decode_root_unwrap(0x4EA00884),
        InstructionKind::REV64AsimdmiscR {
            Q: 1,
            Rd: 4,
            Rn: 4,
            size: 2
        }
    );
    // rev64 v7.4s, v7.4s
    assert_eq!(
        decode_root_unwrap(0x4EA008E7),
        InstructionKind::REV64AsimdmiscR {
            Q: 1,
            Rd: 7,
            Rn: 7,
            size: 2
        }
    );
}
#[test]
fn test_ror() {
    // ror w10, w10, #0x11
    assert_eq!(
        decode_root_unwrap(0x138A454A),
        InstructionKind::EXTR32Extract {
            Rd: 10,
            Rm: 10,
            Rn: 10,
            imms: 17
        }
    );
    // ror w10, w10, #0x19
    assert_eq!(
        decode_root_unwrap(0x138A654A),
        InstructionKind::EXTR32Extract {
            Rd: 10,
            Rm: 10,
            Rn: 10,
            imms: 25
        }
    );
    // ror w14, w14, #0xb
    assert_eq!(
        decode_root_unwrap(0x138E2DCE),
        InstructionKind::EXTR32Extract {
            Rd: 14,
            Rm: 14,
            Rn: 14,
            imms: 11
        }
    );
    // ror w17, w17, #0x1a
    assert_eq!(
        decode_root_unwrap(0x13916A31),
        InstructionKind::EXTR32Extract {
            Rd: 17,
            Rm: 17,
            Rn: 17,
            imms: 26
        }
    );
    // ror w18, w18, #0xb
    assert_eq!(
        decode_root_unwrap(0x13922E52),
        InstructionKind::EXTR32Extract {
            Rd: 18,
            Rm: 18,
            Rn: 18,
            imms: 11
        }
    );
    // ror w5, w1, w3
    assert_eq!(
        decode_root_unwrap(0x1AC32C25),
        InstructionKind::RORV32Dp2Src {
            Rd: 5,
            Rm: 3,
            Rn: 1
        }
    );
}
#[test]
fn test_sbcs() {
    // sbcs x1, x8, x3
    assert_eq!(
        decode_root_unwrap(0xFA030101),
        InstructionKind::SBCS64AddsubCarry {
            Rd: 1,
            Rm: 3,
            Rn: 8
        }
    );
    // sbcs x4, x4, x8
    assert_eq!(
        decode_root_unwrap(0xFA080084),
        InstructionKind::SBCS64AddsubCarry {
            Rd: 4,
            Rm: 8,
            Rn: 4
        }
    );
    // sbcs x21, x9, x8
    assert_eq!(
        decode_root_unwrap(0xFA080135),
        InstructionKind::SBCS64AddsubCarry {
            Rd: 21,
            Rm: 8,
            Rn: 9
        }
    );
    // sbcs x7, x7, x15
    assert_eq!(
        decode_root_unwrap(0xFA0F00E7),
        InstructionKind::SBCS64AddsubCarry {
            Rd: 7,
            Rm: 15,
            Rn: 7
        }
    );
    // sbcs x18, x10, xzr
    assert_eq!(
        decode_root_unwrap(0xFA1F0152),
        InstructionKind::SBCS64AddsubCarry {
            Rd: 18,
            Rm: 31,
            Rn: 10
        }
    );
    // sbcs x16, x18, xzr
    assert_eq!(
        decode_root_unwrap(0xFA1F0250),
        InstructionKind::SBCS64AddsubCarry {
            Rd: 16,
            Rm: 31,
            Rn: 18
        }
    );
}
#[test]
fn test_sbfiz() {
    // sbfiz x13, x13, #0x20, #0x20
    assert_eq!(
        decode_root_unwrap(0x93607DAD),
        InstructionKind::SBFM64MBitfield {
            Rd: 13,
            Rn: 13,
            immr: 32,
            imms: 31
        }
    );
    // sbfiz x12, x8, #3, #0x20
    assert_eq!(
        decode_root_unwrap(0x937D7D0C),
        InstructionKind::SBFM64MBitfield {
            Rd: 12,
            Rn: 8,
            immr: 61,
            imms: 31
        }
    );
    // sbfiz x14, x6, #2, #0x20
    assert_eq!(
        decode_root_unwrap(0x937E7CCE),
        InstructionKind::SBFM64MBitfield {
            Rd: 14,
            Rn: 6,
            immr: 62,
            imms: 31
        }
    );
    // sbfiz x7, x7, #2, #0x20
    assert_eq!(
        decode_root_unwrap(0x937E7CE7),
        InstructionKind::SBFM64MBitfield {
            Rd: 7,
            Rn: 7,
            immr: 62,
            imms: 31
        }
    );
    // sbfiz x2, x21, #2, #0x20
    assert_eq!(
        decode_root_unwrap(0x937E7EA2),
        InstructionKind::SBFM64MBitfield {
            Rd: 2,
            Rn: 21,
            immr: 62,
            imms: 31
        }
    );
    // sbfiz x8, x8, #1, #8
    assert_eq!(
        decode_root_unwrap(0x937F1D08),
        InstructionKind::SBFM64MBitfield {
            Rd: 8,
            Rn: 8,
            immr: 63,
            imms: 7
        }
    );
}
#[test]
fn test_sbfx() {
    // sbfx w17, w17, #1, #0xf
    assert_eq!(
        decode_root_unwrap(0x13013E31),
        InstructionKind::SBFM32MBitfield {
            Rd: 17,
            Rn: 17,
            immr: 1,
            imms: 15
        }
    );
    // sbfx w0, w8, #4, #1
    assert_eq!(
        decode_root_unwrap(0x13041100),
        InstructionKind::SBFM32MBitfield {
            Rd: 0,
            Rn: 8,
            immr: 4,
            imms: 4
        }
    );
    // sbfx x23, x2, #1, #0x1f
    assert_eq!(
        decode_root_unwrap(0x93417C57),
        InstructionKind::SBFM64MBitfield {
            Rd: 23,
            Rn: 2,
            immr: 1,
            imms: 31
        }
    );
    // sbfx x7, x3, #1, #0x1f
    assert_eq!(
        decode_root_unwrap(0x93417C67),
        InstructionKind::SBFM64MBitfield {
            Rd: 7,
            Rn: 3,
            immr: 1,
            imms: 31
        }
    );
    // sbfx x18, x10, #3, #0x1d
    assert_eq!(
        decode_root_unwrap(0x93437D52),
        InstructionKind::SBFM64MBitfield {
            Rd: 18,
            Rn: 10,
            immr: 3,
            imms: 31
        }
    );
    // sbfx x11, x11, #0x1e, #0x20
    assert_eq!(
        decode_root_unwrap(0x935EF56B),
        InstructionKind::SBFM64MBitfield {
            Rd: 11,
            Rn: 11,
            immr: 30,
            imms: 61
        }
    );
}
#[test]
fn test_scvtf() {
    // scvtf v27.2s, v27.2s
    assert_eq!(
        decode_root_unwrap(0xE21DB7B),
        InstructionKind::SCVTFAsimdmiscR {
            Q: 0,
            Rd: 27,
            Rn: 27,
            size: 0
        }
    );
    // scvtf s5, w8
    assert_eq!(
        decode_root_unwrap(0x1E220105),
        InstructionKind::SCVTFS32Float2Int { Rd: 5, Rn: 8 }
    );
    // scvtf s30, w10
    assert_eq!(
        decode_root_unwrap(0x1E22015E),
        InstructionKind::SCVTFS32Float2Int { Rd: 30, Rn: 10 }
    );
    // scvtf s21, w21
    assert_eq!(
        decode_root_unwrap(0x1E2202B5),
        InstructionKind::SCVTFS32Float2Int { Rd: 21, Rn: 21 }
    );
    // scvtf d0, w2
    assert_eq!(
        decode_root_unwrap(0x1E620040),
        InstructionKind::SCVTFD32Float2Int { Rd: 0, Rn: 2 }
    );
    // scvtf s1, s0
    assert_eq!(
        decode_root_unwrap(0x5E21D801),
        InstructionKind::SCVTFAsisdmiscR {
            Rd: 1,
            Rn: 0,
            size: 0
        }
    );
}
#[test]
fn test_sdiv() {
    // sdiv w8, w2, w1
    assert_eq!(
        decode_root_unwrap(0x1AC10C48),
        InstructionKind::SDIV32Dp2Src {
            Rd: 8,
            Rm: 1,
            Rn: 2
        }
    );
    // sdiv w12, w10, w8
    assert_eq!(
        decode_root_unwrap(0x1AC80D4C),
        InstructionKind::SDIV32Dp2Src {
            Rd: 12,
            Rm: 8,
            Rn: 10
        }
    );
    // sdiv w0, w8, w9
    assert_eq!(
        decode_root_unwrap(0x1AC90D00),
        InstructionKind::SDIV32Dp2Src {
            Rd: 0,
            Rm: 9,
            Rn: 8
        }
    );
    // sdiv w12, w13, w12
    assert_eq!(
        decode_root_unwrap(0x1ACC0DAC),
        InstructionKind::SDIV32Dp2Src {
            Rd: 12,
            Rm: 12,
            Rn: 13
        }
    );
    // sdiv w9, w10, w24
    assert_eq!(
        decode_root_unwrap(0x1AD80D49),
        InstructionKind::SDIV32Dp2Src {
            Rd: 9,
            Rm: 24,
            Rn: 10
        }
    );
    // sdiv x10, x11, x10
    assert_eq!(
        decode_root_unwrap(0x9ACA0D6A),
        InstructionKind::SDIV64Dp2Src {
            Rd: 10,
            Rm: 10,
            Rn: 11
        }
    );
}
#[test]
fn test_sha1su0() {
    // sha1su0 v5.4s, v6.4s, v7.4s
    assert_eq!(
        decode_root_unwrap(0x5E0730C5),
        InstructionKind::SHA1SU0VvvCryptosha3 {
            Rd: 5,
            Rm: 7,
            Rn: 6
        }
    );
    // sha1su0 v6.4s, v7.4s, v16.4s
    assert_eq!(
        decode_root_unwrap(0x5E1030E6),
        InstructionKind::SHA1SU0VvvCryptosha3 {
            Rd: 6,
            Rm: 16,
            Rn: 7
        }
    );
}
#[test]
fn test_shadd() {
    // shadd v1.8b, v3.8b, v5.8b
    assert_eq!(
        decode_root_unwrap(0xE250461),
        InstructionKind::SHADDAsimdsameOnly {
            Q: 0,
            Rd: 1,
            Rm: 5,
            Rn: 3,
            size: 0
        }
    );
}
#[test]
fn test_shl() {
    // shl v16.16b, v16.16b, #7
    assert_eq!(
        decode_root_unwrap(0x4F0F5610),
        InstructionKind::SHLAsimdshfR {
            Q: 1,
            Rd: 16,
            Rn: 16,
            immb: 7,
            immh: 1
        }
    );
    // shl v7.4s, v2.4s, #3
    assert_eq!(
        decode_root_unwrap(0x4F235447),
        InstructionKind::SHLAsimdshfR {
            Q: 1,
            Rd: 7,
            Rn: 2,
            immb: 3,
            immh: 4
        }
    );
    // shl v6.4s, v3.4s, #6
    assert_eq!(
        decode_root_unwrap(0x4F265466),
        InstructionKind::SHLAsimdshfR {
            Q: 1,
            Rd: 6,
            Rn: 3,
            immb: 6,
            immh: 4
        }
    );
    // shl v0.4s, v0.4s, #0x1f
    assert_eq!(
        decode_root_unwrap(0x4F3F5400),
        InstructionKind::SHLAsimdshfR {
            Q: 1,
            Rd: 0,
            Rn: 0,
            immb: 7,
            immh: 7
        }
    );
    // shl v6.4s, v6.4s, #0x1f
    assert_eq!(
        decode_root_unwrap(0x4F3F54C6),
        InstructionKind::SHLAsimdshfR {
            Q: 1,
            Rd: 6,
            Rn: 6,
            immb: 7,
            immh: 7
        }
    );
    // shl v21.4s, v21.4s, #0x1f
    assert_eq!(
        decode_root_unwrap(0x4F3F56B5),
        InstructionKind::SHLAsimdshfR {
            Q: 1,
            Rd: 21,
            Rn: 21,
            immb: 7,
            immh: 7
        }
    );
}
#[test]
fn test_shll2() {
    // shll2 v0.4s, v1.8h, #16
    assert_eq!(
        decode_root_unwrap(0x6E613820),
        InstructionKind::SHLLAsimdmiscS {
            Q: 1,
            Rd: 0,
            Rn: 1,
            size: 1
        }
    );
}
#[test]
fn test_shrn() {
    // shrn v2.2s, v0.2d, #0x20
    assert_eq!(
        decode_root_unwrap(0xF208402),
        InstructionKind::SHRNAsimdshfN {
            Q: 0,
            Rd: 2,
            Rn: 0,
            immb: 0,
            immh: 4
        }
    );
    // shrn v3.2s, v0.2d, #0x20
    assert_eq!(
        decode_root_unwrap(0xF208403),
        InstructionKind::SHRNAsimdshfN {
            Q: 0,
            Rd: 3,
            Rn: 0,
            immb: 0,
            immh: 4
        }
    );
    // shrn v5.2s, v1.2d, #0x20
    assert_eq!(
        decode_root_unwrap(0xF208425),
        InstructionKind::SHRNAsimdshfN {
            Q: 0,
            Rd: 5,
            Rn: 1,
            immb: 0,
            immh: 4
        }
    );
    // shrn v1.2s, v4.2d, #0x20
    assert_eq!(
        decode_root_unwrap(0xF208481),
        InstructionKind::SHRNAsimdshfN {
            Q: 0,
            Rd: 1,
            Rn: 4,
            immb: 0,
            immh: 4
        }
    );
    // shrn v17.2s, v6.2d, #0x20
    assert_eq!(
        decode_root_unwrap(0xF2084D1),
        InstructionKind::SHRNAsimdshfN {
            Q: 0,
            Rd: 17,
            Rn: 6,
            immb: 0,
            immh: 4
        }
    );
}
#[test]
fn test_smaddl() {
    // smaddl x8, w6, w3, x8
    assert_eq!(
        decode_root_unwrap(0x9B2320C8),
        InstructionKind::SMADDL64WaDp3Src {
            Ra: 8,
            Rd: 8,
            Rm: 3,
            Rn: 6
        }
    );
    // smaddl x8, w9, w8, x21
    assert_eq!(
        decode_root_unwrap(0x9B285528),
        InstructionKind::SMADDL64WaDp3Src {
            Ra: 21,
            Rd: 8,
            Rm: 8,
            Rn: 9
        }
    );
    // smaddl x8, w2, w9, x8
    assert_eq!(
        decode_root_unwrap(0x9B292048),
        InstructionKind::SMADDL64WaDp3Src {
            Ra: 8,
            Rd: 8,
            Rm: 9,
            Rn: 2
        }
    );
    // smaddl x10, w9, w10, x5
    assert_eq!(
        decode_root_unwrap(0x9B2A152A),
        InstructionKind::SMADDL64WaDp3Src {
            Ra: 5,
            Rd: 10,
            Rm: 10,
            Rn: 9
        }
    );
    // smaddl x9, w13, w27, x9
    assert_eq!(
        decode_root_unwrap(0x9B3B25A9),
        InstructionKind::SMADDL64WaDp3Src {
            Ra: 9,
            Rd: 9,
            Rm: 27,
            Rn: 13
        }
    );
    // smaddl x20, w26, w28, x19
    assert_eq!(
        decode_root_unwrap(0x9B3C4F54),
        InstructionKind::SMADDL64WaDp3Src {
            Ra: 19,
            Rd: 20,
            Rm: 28,
            Rn: 26
        }
    );
}
#[test]
fn test_smax() {
    // smax v0.4s, v0.4s, v1.4s
    assert_eq!(
        decode_root_unwrap(0x4EA16400),
        InstructionKind::SMAXAsimdsameOnly {
            Q: 1,
            Rd: 0,
            Rm: 1,
            Rn: 0,
            size: 2
        }
    );
    // smax v0.4s, v0.4s, v2.4s
    assert_eq!(
        decode_root_unwrap(0x4EA26400),
        InstructionKind::SMAXAsimdsameOnly {
            Q: 1,
            Rd: 0,
            Rm: 2,
            Rn: 0,
            size: 2
        }
    );
    // smax v0.4s, v0.4s, v3.4s
    assert_eq!(
        decode_root_unwrap(0x4EA36400),
        InstructionKind::SMAXAsimdsameOnly {
            Q: 1,
            Rd: 0,
            Rm: 3,
            Rn: 0,
            size: 2
        }
    );
    // smax v1.4s, v1.4s, v5.4s
    assert_eq!(
        decode_root_unwrap(0x4EA56421),
        InstructionKind::SMAXAsimdsameOnly {
            Q: 1,
            Rd: 1,
            Rm: 5,
            Rn: 1,
            size: 2
        }
    );
    // smax v2.4s, v2.4s, v6.4s
    assert_eq!(
        decode_root_unwrap(0x4EA66442),
        InstructionKind::SMAXAsimdsameOnly {
            Q: 1,
            Rd: 2,
            Rm: 6,
            Rn: 2,
            size: 2
        }
    );
    // smax v3.4s, v3.4s, v7.4s
    assert_eq!(
        decode_root_unwrap(0x4EA76463),
        InstructionKind::SMAXAsimdsameOnly {
            Q: 1,
            Rd: 3,
            Rm: 7,
            Rn: 3,
            size: 2
        }
    );
}
#[test]
fn test_smlal() {
    // smlal v2.2d, v0.2s, v5.2s
    assert_eq!(
        decode_root_unwrap(0xEA58002),
        InstructionKind::SMLALAsimddiffL {
            Q: 0,
            Rd: 2,
            Rm: 5,
            Rn: 0,
            size: 2
        }
    );
    // smlal v2.2d, v1.2s, v5.2s
    assert_eq!(
        decode_root_unwrap(0xEA58022),
        InstructionKind::SMLALAsimddiffL {
            Q: 0,
            Rd: 2,
            Rm: 5,
            Rn: 1,
            size: 2
        }
    );
    // smlal v2.2d, v3.2s, v6.s[0]
    assert_eq!(
        decode_root_unwrap(0xF862062),
        InstructionKind::SMLALAsimdelemL {
            H: 0,
            L: 0,
            M: 0,
            Q: 0,
            Rd: 2,
            Rm: 6,
            Rn: 3,
            size: 2
        }
    );
}
#[test]
fn test_smulh() {
    // smulh x8, x0, x8
    assert_eq!(
        decode_root_unwrap(0x9B487C08),
        InstructionKind::SMULH64Dp3Src {
            Ra: 31,
            Rd: 8,
            Rm: 8,
            Rn: 0
        }
    );
    // smulh x8, x22, x8
    assert_eq!(
        decode_root_unwrap(0x9B487EC8),
        InstructionKind::SMULH64Dp3Src {
            Ra: 31,
            Rd: 8,
            Rm: 8,
            Rn: 22
        }
    );
    // smulh x8, x8, x9
    assert_eq!(
        decode_root_unwrap(0x9B497D08),
        InstructionKind::SMULH64Dp3Src {
            Ra: 31,
            Rd: 8,
            Rm: 9,
            Rn: 8
        }
    );
    // smulh x9, x8, x9
    assert_eq!(
        decode_root_unwrap(0x9B497D09),
        InstructionKind::SMULH64Dp3Src {
            Ra: 31,
            Rd: 9,
            Rm: 9,
            Rn: 8
        }
    );
    // smulh x9, x19, x9
    assert_eq!(
        decode_root_unwrap(0x9B497E69),
        InstructionKind::SMULH64Dp3Src {
            Ra: 31,
            Rd: 9,
            Rm: 9,
            Rn: 19
        }
    );
    // smulh x8, x8, x11
    assert_eq!(
        decode_root_unwrap(0x9B4B7D08),
        InstructionKind::SMULH64Dp3Src {
            Ra: 31,
            Rd: 8,
            Rm: 11,
            Rn: 8
        }
    );
    // smulh x10, x10, x11
    assert_eq!(
        decode_root_unwrap(0x9B4B7D4A),
        InstructionKind::SMULH64Dp3Src {
            Ra: 31,
            Rd: 10,
            Rm: 11,
            Rn: 10
        }
    );
}
#[test]
fn test_smull() {
    // smull v2.2d, v2.2s, v0.2s
    assert_eq!(
        decode_root_unwrap(0xEA0C042),
        InstructionKind::SMULLAsimddiffL {
            Q: 0,
            Rd: 2,
            Rm: 0,
            Rn: 2,
            size: 2
        }
    );
    // smull x9, w9, w0
    assert_eq!(
        decode_root_unwrap(0x9B207D29),
        InstructionKind::SMADDL64WaDp3Src {
            Ra: 31,
            Rd: 9,
            Rm: 0,
            Rn: 9
        }
    );
    // smull x4, w4, w4
    assert_eq!(
        decode_root_unwrap(0x9B247C84),
        InstructionKind::SMADDL64WaDp3Src {
            Ra: 31,
            Rd: 4,
            Rm: 4,
            Rn: 4
        }
    );
    // smull x7, w1, w6
    assert_eq!(
        decode_root_unwrap(0x9B267C27),
        InstructionKind::SMADDL64WaDp3Src {
            Ra: 31,
            Rd: 7,
            Rm: 6,
            Rn: 1
        }
    );
    // smull x10, w10, w10
    assert_eq!(
        decode_root_unwrap(0x9B2A7D4A),
        InstructionKind::SMADDL64WaDp3Src {
            Ra: 31,
            Rd: 10,
            Rm: 10,
            Rn: 10
        }
    );
    // smull x11, w0, w11
    assert_eq!(
        decode_root_unwrap(0x9B2B7C0B),
        InstructionKind::SMADDL64WaDp3Src {
            Ra: 31,
            Rd: 11,
            Rm: 11,
            Rn: 0
        }
    );
    // smull x9, w1, w22
    assert_eq!(
        decode_root_unwrap(0x9B367C29),
        InstructionKind::SMADDL64WaDp3Src {
            Ra: 31,
            Rd: 9,
            Rm: 22,
            Rn: 1
        }
    );
}
#[test]
fn test_sqadd() {
    // sqadd v0.2s, v1.2s, v0.2s
    assert_eq!(
        decode_root_unwrap(0xEA00C20),
        InstructionKind::SQADDAsimdsameOnly {
            Q: 0,
            Rd: 0,
            Rm: 0,
            Rn: 1,
            size: 2
        }
    );
    // sqadd v1.2s, v1.2s, v0.2s
    assert_eq!(
        decode_root_unwrap(0xEA00C21),
        InstructionKind::SQADDAsimdsameOnly {
            Q: 0,
            Rd: 1,
            Rm: 0,
            Rn: 1,
            size: 2
        }
    );
    // sqadd v2.2s, v2.2s, v0.2s
    assert_eq!(
        decode_root_unwrap(0xEA00C42),
        InstructionKind::SQADDAsimdsameOnly {
            Q: 0,
            Rd: 2,
            Rm: 0,
            Rn: 2,
            size: 2
        }
    );
    // sqadd v1.2s, v2.2s, v1.2s
    assert_eq!(
        decode_root_unwrap(0xEA10C41),
        InstructionKind::SQADDAsimdsameOnly {
            Q: 0,
            Rd: 1,
            Rm: 1,
            Rn: 2,
            size: 2
        }
    );
    // sqadd v1.2s, v3.2s, v1.2s
    assert_eq!(
        decode_root_unwrap(0xEA10C61),
        InstructionKind::SQADDAsimdsameOnly {
            Q: 0,
            Rd: 1,
            Rm: 1,
            Rn: 3,
            size: 2
        }
    );
    // sqadd v2.2s, v3.2s, v2.2s
    assert_eq!(
        decode_root_unwrap(0xEA20C62),
        InstructionKind::SQADDAsimdsameOnly {
            Q: 0,
            Rd: 2,
            Rm: 2,
            Rn: 3,
            size: 2
        }
    );
}
#[test]
fn test_sqrshrn() {
    // sqrshrn v0.2s, v0.2d, #0x1e
    assert_eq!(
        decode_root_unwrap(0xF229C00),
        InstructionKind::SQRSHRNAsimdshfN {
            Q: 0,
            Rd: 0,
            Rn: 0,
            immb: 2,
            immh: 4
        }
    );
    // sqrshrn v0.2s, v0.2d, #0xf
    assert_eq!(
        decode_root_unwrap(0xF319C00),
        InstructionKind::SQRSHRNAsimdshfN {
            Q: 0,
            Rd: 0,
            Rn: 0,
            immb: 1,
            immh: 6
        }
    );
    // sqrshrn v1.2s, v1.2d, #0xf
    assert_eq!(
        decode_root_unwrap(0xF319C21),
        InstructionKind::SQRSHRNAsimdshfN {
            Q: 0,
            Rd: 1,
            Rn: 1,
            immb: 1,
            immh: 6
        }
    );
    // sqrshrn v2.2s, v2.2d, #0xf
    assert_eq!(
        decode_root_unwrap(0xF319C42),
        InstructionKind::SQRSHRNAsimdshfN {
            Q: 0,
            Rd: 2,
            Rn: 2,
            immb: 1,
            immh: 6
        }
    );
    // sqrshrn v6.2s, v2.2d, #0xe
    assert_eq!(
        decode_root_unwrap(0xF329C46),
        InstructionKind::SQRSHRNAsimdshfN {
            Q: 0,
            Rd: 6,
            Rn: 2,
            immb: 2,
            immh: 6
        }
    );
}
#[test]
fn test_sqxtn2() {
    // sqxtn2 v0.4s, v1.2d
    assert_eq!(
        decode_root_unwrap(0x4EA14820),
        InstructionKind::SQXTNAsimdmiscN {
            Q: 1,
            Rd: 0,
            Rn: 1,
            size: 2
        }
    );
}
#[test]
fn test_sshll() {
    // sshll v2.4s, v2.4h, #0
    assert_eq!(
        decode_root_unwrap(0xF10A442),
        InstructionKind::SSHLLAsimdshfL {
            Q: 0,
            Rd: 2,
            Rn: 2,
            immb: 0,
            immh: 2
        }
    );
    // sshll v3.4s, v3.4h, #0
    assert_eq!(
        decode_root_unwrap(0xF10A463),
        InstructionKind::SSHLLAsimdshfL {
            Q: 0,
            Rd: 3,
            Rn: 3,
            immb: 0,
            immh: 2
        }
    );
    // sshll v5.4s, v5.4h, #0
    assert_eq!(
        decode_root_unwrap(0xF10A4A5),
        InstructionKind::SSHLLAsimdshfL {
            Q: 0,
            Rd: 5,
            Rn: 5,
            immb: 0,
            immh: 2
        }
    );
    // sshll v2.2d, v2.2s, #0
    assert_eq!(
        decode_root_unwrap(0xF20A442),
        InstructionKind::SSHLLAsimdshfL {
            Q: 0,
            Rd: 2,
            Rn: 2,
            immb: 0,
            immh: 4
        }
    );
    // sshll v3.2d, v3.2s, #0
    assert_eq!(
        decode_root_unwrap(0xF20A463),
        InstructionKind::SSHLLAsimdshfL {
            Q: 0,
            Rd: 3,
            Rn: 3,
            immb: 0,
            immh: 4
        }
    );
    // sshll v4.2d, v4.2s, #0
    assert_eq!(
        decode_root_unwrap(0xF20A484),
        InstructionKind::SSHLLAsimdshfL {
            Q: 0,
            Rd: 4,
            Rn: 4,
            immb: 0,
            immh: 4
        }
    );
}
#[test]
fn test_sshr() {
    // sshr v0.16b, v1.16b, #7
    assert_eq!(
        decode_root_unwrap(0x4F090420),
        InstructionKind::SSHRAsimdshfR {
            Q: 1,
            Rd: 0,
            Rn: 1,
            immb: 1,
            immh: 1
        }
    );
    // sshr v0.4s, v0.4s, #0x1f
    assert_eq!(
        decode_root_unwrap(0x4F210400),
        InstructionKind::SSHRAsimdshfR {
            Q: 1,
            Rd: 0,
            Rn: 0,
            immb: 1,
            immh: 4
        }
    );
    // sshr v17.4s, v6.4s, #0x1f
    assert_eq!(
        decode_root_unwrap(0x4F2104D1),
        InstructionKind::SSHRAsimdshfR {
            Q: 1,
            Rd: 17,
            Rn: 6,
            immb: 1,
            immh: 4
        }
    );
    // sshr v7.4s, v7.4s, #0x1f
    assert_eq!(
        decode_root_unwrap(0x4F2104E7),
        InstructionKind::SSHRAsimdshfR {
            Q: 1,
            Rd: 7,
            Rn: 7,
            immb: 1,
            immh: 4
        }
    );
    // sshr v16.4s, v16.4s, #0x1f
    assert_eq!(
        decode_root_unwrap(0x4F210610),
        InstructionKind::SSHRAsimdshfR {
            Q: 1,
            Rd: 16,
            Rn: 16,
            immb: 1,
            immh: 4
        }
    );
    // sshr v21.4s, v21.4s, #0x1f
    assert_eq!(
        decode_root_unwrap(0x4F2106B5),
        InstructionKind::SSHRAsimdshfR {
            Q: 1,
            Rd: 21,
            Rn: 21,
            immb: 1,
            immh: 4
        }
    );
}
#[test]
fn test_stp() {
    // stp w0, w1, [x2]
    assert_eq!(
        decode_root_unwrap(0x29000440),
        InstructionKind::STP32LdstpairOff {
            Rn: 2,
            Rt: 0,
            Rt2: 1,
            imm7: 0
        }
    );
}
#[test]
fn test_stur() {
    // stur h0, [x0, #-0x14]
    assert_eq!(
        decode_root_unwrap(0x7C1EC000),
        InstructionKind::STURHLdstUnscaled {
            Rn: 0,
            Rt: 0,
            imm9: 492
        }
    );
}
#[test]
fn test_sub() {
    // sub w7, w7, w21
    assert_eq!(
        decode_root_unwrap(0x4B1500E7),
        InstructionKind::SUB32AddsubShift {
            Rd: 7,
            Rm: 21,
            Rn: 7,
            imm6: 0,
            shift: 0
        }
    );
    // sub w0, w8, w9, uxtb
    assert_eq!(
        decode_root_unwrap(0x4B290100),
        InstructionKind::SUB32AddsubExt {
            Rd: 0,
            Rm: 9,
            Rn: 8,
            imm3: 0,
            option: 0
        }
    );
    // sub w10, w22, #0x10, lsl #12
    assert_eq!(
        decode_root_unwrap(0x514042CA),
        InstructionKind::SUB32AddsubImm {
            Rd: 10,
            Rn: 22,
            imm12: 16,
            shift: 1
        }
    );
    // sub w10, w10, #0xa00, lsl #12
    assert_eq!(
        decode_root_unwrap(0x5168014A),
        InstructionKind::SUB32AddsubImm {
            Rd: 10,
            Rn: 10,
            imm12: 2560,
            shift: 1
        }
    );
    // sub x2, x9, x0
    assert_eq!(
        decode_root_unwrap(0xCB000122),
        InstructionKind::SUB64AddsubShift {
            Rd: 2,
            Rm: 0,
            Rn: 9,
            imm6: 0,
            shift: 0
        }
    );
    // sub x3, x29, #0x20
    assert_eq!(
        decode_root_unwrap(0xD10083A3),
        InstructionKind::SUB64AddsubImm {
            Rd: 3,
            Rn: 29,
            imm12: 32,
            shift: 0
        }
    );
    // sub x4, x29, #0x5c
    assert_eq!(
        decode_root_unwrap(0xD10173A4),
        InstructionKind::SUB64AddsubImm {
            Rd: 4,
            Rn: 29,
            imm12: 92,
            shift: 0
        }
    );
    // sub sp, sp, #0x70
    assert_eq!(
        decode_root_unwrap(0xD101C3FF),
        InstructionKind::SUB64AddsubImm {
            Rd: 31,
            Rn: 31,
            imm12: 112,
            shift: 0
        }
    );
}
#[test]
fn test_subhn2() {
    // subhn2 v1.16b, v3.8h, v5.8h
    assert_eq!(
        decode_root_unwrap(0x4E256061),
        InstructionKind::SUBHNAsimddiffN {
            Q: 1,
            Rd: 1,
            Rm: 5,
            Rn: 3,
            size: 0
        }
    );
}
#[test]
fn test_subs() {
    // subs w9, w26, w8
    assert_eq!(
        decode_root_unwrap(0x6B080349),
        InstructionKind::SUBS32AddsubShift {
            Rd: 9,
            Rm: 8,
            Rn: 26,
            imm6: 0,
            shift: 0
        }
    );
    // subs w8, w8, w10
    assert_eq!(
        decode_root_unwrap(0x6B0A0108),
        InstructionKind::SUBS32AddsubShift {
            Rd: 8,
            Rm: 10,
            Rn: 8,
            imm6: 0,
            shift: 0
        }
    );
    // subs w17, w17, w12
    assert_eq!(
        decode_root_unwrap(0x6B0C0231),
        InstructionKind::SUBS32AddsubShift {
            Rd: 17,
            Rm: 12,
            Rn: 17,
            imm6: 0,
            shift: 0
        }
    );
    // subs x25, x20, x21
    assert_eq!(
        decode_root_unwrap(0xEB150299),
        InstructionKind::SUBS64AddsubShift {
            Rd: 25,
            Rm: 21,
            Rn: 20,
            imm6: 0,
            shift: 0
        }
    );
    // subs x8, x8, #1
    assert_eq!(
        decode_root_unwrap(0xF1000508),
        InstructionKind::SUBS64SAddsubImm {
            Rd: 8,
            Rn: 8,
            imm12: 1,
            shift: 0
        }
    );
    // subs x21, x21, #0x600, lsl #12
    assert_eq!(
        decode_root_unwrap(0xF15802B5),
        InstructionKind::SUBS64SAddsubImm {
            Rd: 21,
            Rn: 21,
            imm12: 1536,
            shift: 1
        }
    );
}
#[test]
fn test_sxtb() {
    // sxtb w4, w6
    assert_eq!(
        decode_root_unwrap(0x13001CC4),
        InstructionKind::SBFM32MBitfield {
            Rd: 4,
            Rn: 6,
            immr: 0,
            imms: 7
        }
    );
    // sxtb w9, w8
    assert_eq!(
        decode_root_unwrap(0x13001D09),
        InstructionKind::SBFM32MBitfield {
            Rd: 9,
            Rn: 8,
            immr: 0,
            imms: 7
        }
    );
    // sxtb w17, w8
    assert_eq!(
        decode_root_unwrap(0x13001D11),
        InstructionKind::SBFM32MBitfield {
            Rd: 17,
            Rn: 8,
            immr: 0,
            imms: 7
        }
    );
    // sxtb w18, w15
    assert_eq!(
        decode_root_unwrap(0x13001DF2),
        InstructionKind::SBFM32MBitfield {
            Rd: 18,
            Rn: 15,
            immr: 0,
            imms: 7
        }
    );
    // sxtb x8, w16
    assert_eq!(
        decode_root_unwrap(0x93401E08),
        InstructionKind::SBFM64MBitfield {
            Rd: 8,
            Rn: 16,
            immr: 0,
            imms: 7
        }
    );
    // sxtb x20, w19
    assert_eq!(
        decode_root_unwrap(0x93401E74),
        InstructionKind::SBFM64MBitfield {
            Rd: 20,
            Rn: 19,
            immr: 0,
            imms: 7
        }
    );
}
#[test]
fn test_sxth() {
    // sxth w2, w0
    assert_eq!(
        decode_root_unwrap(0x13003C02),
        InstructionKind::SBFM32MBitfield {
            Rd: 2,
            Rn: 0,
            immr: 0,
            imms: 15
        }
    );
    // sxth w21, w0
    assert_eq!(
        decode_root_unwrap(0x13003C15),
        InstructionKind::SBFM32MBitfield {
            Rd: 21,
            Rn: 0,
            immr: 0,
            imms: 15
        }
    );
    // sxth w5, w3
    assert_eq!(
        decode_root_unwrap(0x13003C65),
        InstructionKind::SBFM32MBitfield {
            Rd: 5,
            Rn: 3,
            immr: 0,
            imms: 15
        }
    );
    // sxth w23, w26
    assert_eq!(
        decode_root_unwrap(0x13003F57),
        InstructionKind::SBFM32MBitfield {
            Rd: 23,
            Rn: 26,
            immr: 0,
            imms: 15
        }
    );
    // sxth w25, w30
    assert_eq!(
        decode_root_unwrap(0x13003FD9),
        InstructionKind::SBFM32MBitfield {
            Rd: 25,
            Rn: 30,
            immr: 0,
            imms: 15
        }
    );
    // sxth x4, w14
    assert_eq!(
        decode_root_unwrap(0x93403DC4),
        InstructionKind::SBFM64MBitfield {
            Rd: 4,
            Rn: 14,
            immr: 0,
            imms: 15
        }
    );
}
#[test]
fn test_sxtw() {
    // sxtw x1, w1
    assert_eq!(
        decode_root_unwrap(0x93407C21),
        InstructionKind::SBFM64MBitfield {
            Rd: 1,
            Rn: 1,
            immr: 0,
            imms: 31
        }
    );
    // sxtw x14, w4
    assert_eq!(
        decode_root_unwrap(0x93407C8E),
        InstructionKind::SBFM64MBitfield {
            Rd: 14,
            Rn: 4,
            immr: 0,
            imms: 31
        }
    );
    // sxtw x22, w8
    assert_eq!(
        decode_root_unwrap(0x93407D16),
        InstructionKind::SBFM64MBitfield {
            Rd: 22,
            Rn: 8,
            immr: 0,
            imms: 31
        }
    );
    // sxtw x26, w25
    assert_eq!(
        decode_root_unwrap(0x93407F3A),
        InstructionKind::SBFM64MBitfield {
            Rd: 26,
            Rn: 25,
            immr: 0,
            imms: 31
        }
    );
    // sxtw x1, w27
    assert_eq!(
        decode_root_unwrap(0x93407F61),
        InstructionKind::SBFM64MBitfield {
            Rd: 1,
            Rn: 27,
            immr: 0,
            imms: 31
        }
    );
    // sxtw x24, w29
    assert_eq!(
        decode_root_unwrap(0x93407FB8),
        InstructionKind::SBFM64MBitfield {
            Rd: 24,
            Rn: 29,
            immr: 0,
            imms: 31
        }
    );
}
#[test]
fn test_tbl() {
    // tbl v1.8b, {v5.16b}, v1.8b
    assert_eq!(
        decode_root_unwrap(0xE0100A1),
        InstructionKind::TBLAsimdtblL11 {
            Q: 0,
            Rd: 1,
            Rm: 1,
            Rn: 5
        }
    );
    // tbl v4.8b, {v7.16b}, v1.8b
    assert_eq!(
        decode_root_unwrap(0xE0100E4),
        InstructionKind::TBLAsimdtblL11 {
            Q: 0,
            Rd: 4,
            Rm: 1,
            Rn: 7
        }
    );
    // tbl v5.8b, {v23.16b}, v16.8b
    assert_eq!(
        decode_root_unwrap(0xE1002E5),
        InstructionKind::TBLAsimdtblL11 {
            Q: 0,
            Rd: 5,
            Rm: 16,
            Rn: 23
        }
    );
    // tbl v28.8b, {v27.16b}, v16.8b
    assert_eq!(
        decode_root_unwrap(0xE10037C),
        InstructionKind::TBLAsimdtblL11 {
            Q: 0,
            Rd: 28,
            Rm: 16,
            Rn: 27
        }
    );
    // tbl v12.8b, {v8.16b, v9.16b}, v18.8b
    assert_eq!(
        decode_root_unwrap(0xE12210C),
        InstructionKind::TBLAsimdtblL22 {
            Q: 0,
            Rd: 12,
            Rm: 18,
            Rn: 8
        }
    );
    // tbl v26.8b, {v26.16b, v27.16b}, v31.8b
    assert_eq!(
        decode_root_unwrap(0xE1F235A),
        InstructionKind::TBLAsimdtblL22 {
            Q: 0,
            Rd: 26,
            Rm: 31,
            Rn: 26
        }
    );
}
#[test]
fn test_tst() {
    // tst w5, w7
    assert_eq!(
        decode_root_unwrap(0x6A0700BF),
        InstructionKind::ANDS32LogShift {
            Rd: 31,
            Rm: 7,
            Rn: 5,
            imm6: 0,
            shift: 0
        }
    );
    // tst w13, w12
    assert_eq!(
        decode_root_unwrap(0x6A0C01BF),
        InstructionKind::ANDS32LogShift {
            Rd: 31,
            Rm: 12,
            Rn: 13,
            imm6: 0,
            shift: 0
        }
    );
    // tst w9, #0x1000000
    assert_eq!(
        decode_root_unwrap(0x7208013F),
        InstructionKind::ANDS32SLogImm {
            Rd: 31,
            Rn: 9,
            immr: 8,
            imms: 0
        }
    );
    // tst w17, #0x400
    assert_eq!(
        decode_root_unwrap(0x7216023F),
        InstructionKind::ANDS32SLogImm {
            Rd: 31,
            Rn: 17,
            immr: 22,
            imms: 0
        }
    );
    // tst w25, #4
    assert_eq!(
        decode_root_unwrap(0x721E033F),
        InstructionKind::ANDS32SLogImm {
            Rd: 31,
            Rn: 25,
            immr: 30,
            imms: 0
        }
    );
    // tst x10, #0x7fffffffffffffff
    assert_eq!(
        decode_root_unwrap(0xF240F95F),
        InstructionKind::ANDS64SLogImm {
            N: 1,
            Rd: 31,
            Rn: 10,
            immr: 0,
            imms: 62
        }
    );
}
#[test]
fn test_uabdl2() {
    // uabdl2 v4.2d, v1.4s, v3.4s
    assert_eq!(
        decode_root_unwrap(0x6EA37024),
        InstructionKind::UABDLAsimddiffL {
            Q: 1,
            Rd: 4,
            Rm: 3,
            Rn: 1,
            size: 2
        }
    );
}
#[test]
fn test_ubfiz() {
    // ubfiz w21, w0, #8, #6
    assert_eq!(
        decode_root_unwrap(0x53181415),
        InstructionKind::UBFM32MBitfield {
            Rd: 21,
            Rn: 0,
            immr: 24,
            imms: 5
        }
    );
    // ubfiz w10, w10, #8, #8
    assert_eq!(
        decode_root_unwrap(0x53181D4A),
        InstructionKind::UBFM32MBitfield {
            Rd: 10,
            Rn: 10,
            immr: 24,
            imms: 7
        }
    );
    // ubfiz w12, w12, #5, #2
    assert_eq!(
        decode_root_unwrap(0x531B058C),
        InstructionKind::UBFM32MBitfield {
            Rd: 12,
            Rn: 12,
            immr: 27,
            imms: 1
        }
    );
    // ubfiz w10, w8, #4, #2
    assert_eq!(
        decode_root_unwrap(0x531C050A),
        InstructionKind::UBFM32MBitfield {
            Rd: 10,
            Rn: 8,
            immr: 28,
            imms: 1
        }
    );
    // ubfiz w8, w8, #4, #4
    assert_eq!(
        decode_root_unwrap(0x531C0D08),
        InstructionKind::UBFM32MBitfield {
            Rd: 8,
            Rn: 8,
            immr: 28,
            imms: 3
        }
    );
    // ubfiz x12, x13, #0x28, #0x14
    assert_eq!(
        decode_root_unwrap(0xD3584DAC),
        InstructionKind::UBFM64MBitfield {
            Rd: 12,
            Rn: 13,
            immr: 24,
            imms: 19
        }
    );
}
#[test]
fn test_ubfx() {
    // ubfx w9, w0, #1, #1
    assert_eq!(
        decode_root_unwrap(0x53010409),
        InstructionKind::UBFM32MBitfield {
            Rd: 9,
            Rn: 0,
            immr: 1,
            imms: 1
        }
    );
    // ubfx w8, w20, #1, #1
    assert_eq!(
        decode_root_unwrap(0x53010688),
        InstructionKind::UBFM32MBitfield {
            Rd: 8,
            Rn: 20,
            immr: 1,
            imms: 1
        }
    );
    // ubfx w11, w9, #3, #1
    assert_eq!(
        decode_root_unwrap(0x53030D2B),
        InstructionKind::UBFM32MBitfield {
            Rd: 11,
            Rn: 9,
            immr: 3,
            imms: 3
        }
    );
    // ubfx w13, w13, #0xc, #1
    assert_eq!(
        decode_root_unwrap(0x530C31AD),
        InstructionKind::UBFM32MBitfield {
            Rd: 13,
            Rn: 13,
            immr: 12,
            imms: 12
        }
    );
    // ubfx w0, w8, #0xf, #5
    assert_eq!(
        decode_root_unwrap(0x530F4D00),
        InstructionKind::UBFM32MBitfield {
            Rd: 0,
            Rn: 8,
            immr: 15,
            imms: 19
        }
    );
    // ubfx x15, x15, #0x1f, #0x20
    assert_eq!(
        decode_root_unwrap(0xD35FF9EF),
        InstructionKind::UBFM64MBitfield {
            Rd: 15,
            Rn: 15,
            immr: 31,
            imms: 62
        }
    );
}
#[test]
fn test_ucvtf() {
    // ucvtf s5, w8
    assert_eq!(
        decode_root_unwrap(0x1E230105),
        InstructionKind::UCVTFS32Float2Int { Rd: 5, Rn: 8 }
    );
    // ucvtf s8, w24
    assert_eq!(
        decode_root_unwrap(0x1E230308),
        InstructionKind::UCVTFS32Float2Int { Rd: 8, Rn: 24 }
    );
    // ucvtf s8, w27
    assert_eq!(
        decode_root_unwrap(0x1E230368),
        InstructionKind::UCVTFS32Float2Int { Rd: 8, Rn: 27 }
    );
    // ucvtf s10, w27
    assert_eq!(
        decode_root_unwrap(0x1E23036A),
        InstructionKind::UCVTFS32Float2Int { Rd: 10, Rn: 27 }
    );
    // ucvtf s9, w28
    assert_eq!(
        decode_root_unwrap(0x1E230389),
        InstructionKind::UCVTFS32Float2Int { Rd: 9, Rn: 28 }
    );
    // ucvtf s4, s1
    assert_eq!(
        decode_root_unwrap(0x7E21D824),
        InstructionKind::UCVTFAsisdmiscR {
            Rd: 4,
            Rn: 1,
            size: 0
        }
    );
}
#[test]
fn test_udiv() {
    // udiv w5, w5, w7
    assert_eq!(
        decode_root_unwrap(0x1AC708A5),
        InstructionKind::UDIV32Dp2Src {
            Rd: 5,
            Rm: 7,
            Rn: 5
        }
    );
    // udiv w1, w1, w9
    assert_eq!(
        decode_root_unwrap(0x1AC90821),
        InstructionKind::UDIV32Dp2Src {
            Rd: 1,
            Rm: 9,
            Rn: 1
        }
    );
    // udiv w15, w19, w10
    assert_eq!(
        decode_root_unwrap(0x1ACA0A6F),
        InstructionKind::UDIV32Dp2Src {
            Rd: 15,
            Rm: 10,
            Rn: 19
        }
    );
    // udiv w12, w13, w12
    assert_eq!(
        decode_root_unwrap(0x1ACC09AC),
        InstructionKind::UDIV32Dp2Src {
            Rd: 12,
            Rm: 12,
            Rn: 13
        }
    );
    // udiv w18, w18, w17
    assert_eq!(
        decode_root_unwrap(0x1AD10A52),
        InstructionKind::UDIV32Dp2Src {
            Rd: 18,
            Rm: 17,
            Rn: 18
        }
    );
    // udiv w14, w12, w21
    assert_eq!(
        decode_root_unwrap(0x1AD5098E),
        InstructionKind::UDIV32Dp2Src {
            Rd: 14,
            Rm: 21,
            Rn: 12
        }
    );
    // udiv x10, x22, x9
    assert_eq!(
        decode_root_unwrap(0x9AC90ACA),
        InstructionKind::UDIV64Dp2Src {
            Rd: 10,
            Rm: 9,
            Rn: 22
        }
    );
}
#[test]
fn test_uhsub() {
    // uhsub v0.8b, v1.8b, v3.8b
    assert_eq!(
        decode_root_unwrap(0x2E232420),
        InstructionKind::UHSUBAsimdsameOnly {
            Q: 0,
            Rd: 0,
            Rm: 3,
            Rn: 1,
            size: 0
        }
    );
}
#[test]
fn test_umaddl() {
    // umaddl x8, w3, w1, x8
    assert_eq!(
        decode_root_unwrap(0x9BA12068),
        InstructionKind::UMADDL64WaDp3Src {
            Ra: 8,
            Rd: 8,
            Rm: 1,
            Rn: 3
        }
    );
    // umaddl x22, w22, w7, x19
    assert_eq!(
        decode_root_unwrap(0x9BA74ED6),
        InstructionKind::UMADDL64WaDp3Src {
            Ra: 19,
            Rd: 22,
            Rm: 7,
            Rn: 22
        }
    );
    // umaddl x9, w19, w9, x20
    assert_eq!(
        decode_root_unwrap(0x9BA95269),
        InstructionKind::UMADDL64WaDp3Src {
            Ra: 20,
            Rd: 9,
            Rm: 9,
            Rn: 19
        }
    );
    // umaddl x17, w17, w20, x9
    assert_eq!(
        decode_root_unwrap(0x9BB42631),
        InstructionKind::UMADDL64WaDp3Src {
            Ra: 9,
            Rd: 17,
            Rm: 20,
            Rn: 17
        }
    );
    // umaddl x22, w10, w21, x24
    assert_eq!(
        decode_root_unwrap(0x9BB56156),
        InstructionKind::UMADDL64WaDp3Src {
            Ra: 24,
            Rd: 22,
            Rm: 21,
            Rn: 10
        }
    );
    // umaddl x9, w26, w28, x19
    assert_eq!(
        decode_root_unwrap(0x9BBC4F49),
        InstructionKind::UMADDL64WaDp3Src {
            Ra: 19,
            Rd: 9,
            Rm: 28,
            Rn: 26
        }
    );
}
#[test]
fn test_uminp() {
    // uminp v0.8b, v1.8b, v3.8b
    assert_eq!(
        decode_root_unwrap(0x2E23AC20),
        InstructionKind::UMINPAsimdsameOnly {
            Q: 0,
            Rd: 0,
            Rm: 3,
            Rn: 1,
            size: 0
        }
    );
}
#[test]
fn test_umov() {
    // umov w8, v0.h[0]
    assert_eq!(
        decode_root_unwrap(0xE023C08),
        InstructionKind::UMOVAsimdinsWW {
            Rd: 8,
            Rn: 0,
            imm5: 2
        }
    );
    // umov w9, v0.h[0]
    assert_eq!(
        decode_root_unwrap(0xE023C09),
        InstructionKind::UMOVAsimdinsWW {
            Rd: 9,
            Rn: 0,
            imm5: 2
        }
    );
    // umov w9, v1.h[0]
    assert_eq!(
        decode_root_unwrap(0xE023C29),
        InstructionKind::UMOVAsimdinsWW {
            Rd: 9,
            Rn: 1,
            imm5: 2
        }
    );
    // umov w10, v2.h[0]
    assert_eq!(
        decode_root_unwrap(0xE023C4A),
        InstructionKind::UMOVAsimdinsWW {
            Rd: 10,
            Rn: 2,
            imm5: 2
        }
    );
    // umov w8, v5.h[0]
    assert_eq!(
        decode_root_unwrap(0xE023CA8),
        InstructionKind::UMOVAsimdinsWW {
            Rd: 8,
            Rn: 5,
            imm5: 2
        }
    );
    // umov w16, v16.h[0]
    assert_eq!(
        decode_root_unwrap(0xE023E10),
        InstructionKind::UMOVAsimdinsWW {
            Rd: 16,
            Rn: 16,
            imm5: 2
        }
    );
}
#[test]
fn test_umulh() {
    // umulh x8, x2, x0
    assert_eq!(
        decode_root_unwrap(0x9BC07C48),
        InstructionKind::UMULH64Dp3Src {
            Ra: 31,
            Rd: 8,
            Rm: 0,
            Rn: 2
        }
    );
    // umulh x9, x9, x11
    assert_eq!(
        decode_root_unwrap(0x9BCB7D29),
        InstructionKind::UMULH64Dp3Src {
            Ra: 31,
            Rd: 9,
            Rm: 11,
            Rn: 9
        }
    );
    // umulh x9, x27, x11
    assert_eq!(
        decode_root_unwrap(0x9BCB7F69),
        InstructionKind::UMULH64Dp3Src {
            Ra: 31,
            Rd: 9,
            Rm: 11,
            Rn: 27
        }
    );
    // umulh x3, x2, x12
    assert_eq!(
        decode_root_unwrap(0x9BCC7C43),
        InstructionKind::UMULH64Dp3Src {
            Ra: 31,
            Rd: 3,
            Rm: 12,
            Rn: 2
        }
    );
    // umulh x10, x10, x23
    assert_eq!(
        decode_root_unwrap(0x9BD77D4A),
        InstructionKind::UMULH64Dp3Src {
            Ra: 31,
            Rd: 10,
            Rm: 23,
            Rn: 10
        }
    );
    // umulh x8, x23, x24
    assert_eq!(
        decode_root_unwrap(0x9BD87EE8),
        InstructionKind::UMULH64Dp3Src {
            Ra: 31,
            Rd: 8,
            Rm: 24,
            Rn: 23
        }
    );
}
#[test]
fn test_umull() {
    // umull x1, w21, w8
    assert_eq!(
        decode_root_unwrap(0x9BA87EA1),
        InstructionKind::UMADDL64WaDp3Src {
            Ra: 31,
            Rd: 1,
            Rm: 8,
            Rn: 21
        }
    );
    // umull x4, w8, w9
    assert_eq!(
        decode_root_unwrap(0x9BA97D04),
        InstructionKind::UMADDL64WaDp3Src {
            Ra: 31,
            Rd: 4,
            Rm: 9,
            Rn: 8
        }
    );
    // umull x11, w11, w10
    assert_eq!(
        decode_root_unwrap(0x9BAA7D6B),
        InstructionKind::UMADDL64WaDp3Src {
            Ra: 31,
            Rd: 11,
            Rm: 10,
            Rn: 11
        }
    );
    // umull x15, w12, w13
    assert_eq!(
        decode_root_unwrap(0x9BAD7D8F),
        InstructionKind::UMADDL64WaDp3Src {
            Ra: 31,
            Rd: 15,
            Rm: 13,
            Rn: 12
        }
    );
    // umull x22, w23, w13
    assert_eq!(
        decode_root_unwrap(0x9BAD7EF6),
        InstructionKind::UMADDL64WaDp3Src {
            Ra: 31,
            Rd: 22,
            Rm: 13,
            Rn: 23
        }
    );
    // umull x23, w23, w13
    assert_eq!(
        decode_root_unwrap(0x9BAD7EF7),
        InstructionKind::UMADDL64WaDp3Src {
            Ra: 31,
            Rd: 23,
            Rm: 13,
            Rn: 23
        }
    );
}
#[test]
fn test_ushll() {
    // ushll v2.8h, v1.8b, #0
    assert_eq!(
        decode_root_unwrap(0x2F08A422),
        InstructionKind::USHLLAsimdshfL {
            Q: 0,
            Rd: 2,
            Rn: 1,
            immb: 0,
            immh: 1
        }
    );
    // ushll v0.4s, v0.4h, #0
    assert_eq!(
        decode_root_unwrap(0x2F10A400),
        InstructionKind::USHLLAsimdshfL {
            Q: 0,
            Rd: 0,
            Rn: 0,
            immb: 0,
            immh: 2
        }
    );
    // ushll v3.4s, v2.4h, #0
    assert_eq!(
        decode_root_unwrap(0x2F10A443),
        InstructionKind::USHLLAsimdshfL {
            Q: 0,
            Rd: 3,
            Rn: 2,
            immb: 0,
            immh: 2
        }
    );
    // ushll v6.4s, v6.4h, #0
    assert_eq!(
        decode_root_unwrap(0x2F10A4C6),
        InstructionKind::USHLLAsimdshfL {
            Q: 0,
            Rd: 6,
            Rn: 6,
            immb: 0,
            immh: 2
        }
    );
    // ushll v13.4s, v13.4h, #0
    assert_eq!(
        decode_root_unwrap(0x2F10A5AD),
        InstructionKind::USHLLAsimdshfL {
            Q: 0,
            Rd: 13,
            Rn: 13,
            immb: 0,
            immh: 2
        }
    );
    // ushll v21.4s, v21.4h, #0
    assert_eq!(
        decode_root_unwrap(0x2F10A6B5),
        InstructionKind::USHLLAsimdshfL {
            Q: 0,
            Rd: 21,
            Rn: 21,
            immb: 0,
            immh: 2
        }
    );
}
#[test]
fn test_ushll2() {
    // ushll2 v1.8h, v1.16b, #0
    assert_eq!(
        decode_root_unwrap(0x6F08A421),
        InstructionKind::USHLLAsimdshfL {
            Q: 1,
            Rd: 1,
            Rn: 1,
            immb: 0,
            immh: 1
        }
    );
    // ushll2 v1.4s, v1.8h, #0
    assert_eq!(
        decode_root_unwrap(0x6F10A421),
        InstructionKind::USHLLAsimdshfL {
            Q: 1,
            Rd: 1,
            Rn: 1,
            immb: 0,
            immh: 2
        }
    );
    // ushll2 v2.4s, v2.8h, #0
    assert_eq!(
        decode_root_unwrap(0x6F10A442),
        InstructionKind::USHLLAsimdshfL {
            Q: 1,
            Rd: 2,
            Rn: 2,
            immb: 0,
            immh: 2
        }
    );
}
#[test]
fn test_xtn() {
    // xtn v3.4h, v3.4s
    assert_eq!(
        decode_root_unwrap(0xE612863),
        InstructionKind::XTNAsimdmiscN {
            Q: 0,
            Rd: 3,
            Rn: 3,
            size: 1
        }
    );
    // xtn v21.4h, v7.4s
    assert_eq!(
        decode_root_unwrap(0xE6128F5),
        InstructionKind::XTNAsimdmiscN {
            Q: 0,
            Rd: 21,
            Rn: 7,
            size: 1
        }
    );
    // xtn v13.4h, v10.4s
    assert_eq!(
        decode_root_unwrap(0xE61294D),
        InstructionKind::XTNAsimdmiscN {
            Q: 0,
            Rd: 13,
            Rn: 10,
            size: 1
        }
    );
    // xtn v0.4h, v13.4s
    assert_eq!(
        decode_root_unwrap(0xE6129A0),
        InstructionKind::XTNAsimdmiscN {
            Q: 0,
            Rd: 0,
            Rn: 13,
            size: 1
        }
    );
    // xtn v1.2s, v0.2d
    assert_eq!(
        decode_root_unwrap(0xEA12801),
        InstructionKind::XTNAsimdmiscN {
            Q: 0,
            Rd: 1,
            Rn: 0,
            size: 2
        }
    );
    // xtn v4.2s, v1.2d
    assert_eq!(
        decode_root_unwrap(0xEA12824),
        InstructionKind::XTNAsimdmiscN {
            Q: 0,
            Rd: 4,
            Rn: 1,
            size: 2
        }
    );
}
#[test]
fn test_xtn2() {
    // xtn2 v4.16b, v3.8h
    assert_eq!(
        decode_root_unwrap(0x4E212864),
        InstructionKind::XTNAsimdmiscN {
            Q: 1,
            Rd: 4,
            Rn: 3,
            size: 0
        }
    );
    // xtn2 v6.16b, v5.8h
    assert_eq!(
        decode_root_unwrap(0x4E2128A6),
        InstructionKind::XTNAsimdmiscN {
            Q: 1,
            Rd: 6,
            Rn: 5,
            size: 0
        }
    );
    // xtn2 v7.16b, v17.8h
    assert_eq!(
        decode_root_unwrap(0x4E212A27),
        InstructionKind::XTNAsimdmiscN {
            Q: 1,
            Rd: 7,
            Rn: 17,
            size: 0
        }
    );
    // xtn2 v7.8h, v6.4s
    assert_eq!(
        decode_root_unwrap(0x4E6128C7),
        InstructionKind::XTNAsimdmiscN {
            Q: 1,
            Rd: 7,
            Rn: 6,
            size: 1
        }
    );
    // xtn2 v5.8h, v16.4s
    assert_eq!(
        decode_root_unwrap(0x4E612A05),
        InstructionKind::XTNAsimdmiscN {
            Q: 1,
            Rd: 5,
            Rn: 16,
            size: 1
        }
    );
    // xtn2 v7.8h, v16.4s
    assert_eq!(
        decode_root_unwrap(0x4E612A07),
        InstructionKind::XTNAsimdmiscN {
            Q: 1,
            Rd: 7,
            Rn: 16,
            size: 1
        }
    );
}
#[test]
fn test_zip1() {
    // zip1 v30.2s, v28.2s, v29.2s
    assert_eq!(
        decode_root_unwrap(0xE9D3B9E),
        InstructionKind::ZIP1AsimdpermOnly {
            Q: 0,
            Rd: 30,
            Rm: 29,
            Rn: 28,
            size: 2
        }
    );
    // zip1 v0.4s, v2.4s, v1.4s
    assert_eq!(
        decode_root_unwrap(0x4E813840),
        InstructionKind::ZIP1AsimdpermOnly {
            Q: 1,
            Rd: 0,
            Rm: 1,
            Rn: 2,
            size: 2
        }
    );
    // zip1 v5.4s, v0.4s, v2.4s
    assert_eq!(
        decode_root_unwrap(0x4E823805),
        InstructionKind::ZIP1AsimdpermOnly {
            Q: 1,
            Rd: 5,
            Rm: 2,
            Rn: 0,
            size: 2
        }
    );
    // zip1 v2.4s, v1.4s, v4.4s
    assert_eq!(
        decode_root_unwrap(0x4E843822),
        InstructionKind::ZIP1AsimdpermOnly {
            Q: 1,
            Rd: 2,
            Rm: 4,
            Rn: 1,
            size: 2
        }
    );
    // zip1 v3.4s, v9.4s, v11.4s
    assert_eq!(
        decode_root_unwrap(0x4E8B3923),
        InstructionKind::ZIP1AsimdpermOnly {
            Q: 1,
            Rd: 3,
            Rm: 11,
            Rn: 9,
            size: 2
        }
    );
    // zip1 v5.4s, v16.4s, v17.4s
    assert_eq!(
        decode_root_unwrap(0x4E913A05),
        InstructionKind::ZIP1AsimdpermOnly {
            Q: 1,
            Rd: 5,
            Rm: 17,
            Rn: 16,
            size: 2
        }
    );
}
#[test]
fn test_zip2() {
    // zip2 v0.4s, v0.4s, v1.4s
    assert_eq!(
        decode_root_unwrap(0x4E817800),
        InstructionKind::ZIP2AsimdpermOnly {
            Q: 1,
            Rd: 0,
            Rm: 1,
            Rn: 0,
            size: 2
        }
    );
    // zip2 v1.4s, v1.4s, v2.4s
    assert_eq!(
        decode_root_unwrap(0x4E827821),
        InstructionKind::ZIP2AsimdpermOnly {
            Q: 1,
            Rd: 1,
            Rm: 2,
            Rn: 1,
            size: 2
        }
    );
    // zip2 v2.4s, v4.4s, v2.4s
    assert_eq!(
        decode_root_unwrap(0x4E827882),
        InstructionKind::ZIP2AsimdpermOnly {
            Q: 1,
            Rd: 2,
            Rm: 2,
            Rn: 4,
            size: 2
        }
    );
    // zip2 v0.4s, v0.4s, v4.4s
    assert_eq!(
        decode_root_unwrap(0x4E847800),
        InstructionKind::ZIP2AsimdpermOnly {
            Q: 1,
            Rd: 0,
            Rm: 4,
            Rn: 0,
            size: 2
        }
    );
    // zip2 v3.4s, v3.4s, v4.4s
    assert_eq!(
        decode_root_unwrap(0x4E847863),
        InstructionKind::ZIP2AsimdpermOnly {
            Q: 1,
            Rd: 3,
            Rm: 4,
            Rn: 3,
            size: 2
        }
    );
    // zip2 v27.4s, v27.4s, v29.4s
    assert_eq!(
        decode_root_unwrap(0x4E9D7B7B),
        InstructionKind::ZIP2AsimdpermOnly {
            Q: 1,
            Rd: 27,
            Rm: 29,
            Rn: 27,
            size: 2
        }
    );
}
