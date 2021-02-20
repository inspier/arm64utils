#![allow(unused, non_snake_case)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::no_effect)]
#![no_std]

pub mod instruction;
use crate::instruction::InstructionKind;

pub const fn decode_root(d: u32) -> Option<InstructionKind> {
    let op0 = (d >> 25) & 0xF;
    if (d & 0x0e000000) == 0x0a000000 {
        return decode_dataproc_register(d);
    }
    if (d & 0x0e000000) == 0x0e000000 {
        return decode_dataproc_simd(d);
    }
    if (d & 0x1c000000) == 0x10000000 {
        return decode_dataproc_immediate(d);
    }
    if (d & 0x1c000000) == 0x14000000 {
        return decode_branch_and_sys(d);
    }
    if (d & 0x0a000000) == 0x08000000 {
        return decode_load_and_store(d);
    }
    None
}
pub const fn decode_dataproc_immediate(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1c000000) != 0x10000000) as usize];
    let op0 = (d >> 23) & 7;
    if (d & 0x1f800000) == 0x12000000 {
        return decode_log_imm(d);
    }
    if (d & 0x1f800000) == 0x12800000 {
        return decode_movewide(d);
    }
    if (d & 0x1f800000) == 0x13000000 {
        return decode_bitfield(d);
    }
    if (d & 0x1f800000) == 0x13800000 {
        return decode_extract(d);
    }
    if (d & 0x1f000000) == 0x10000000 {
        return decode_pcreladdr(d);
    }
    if (d & 0x1f000000) == 0x11000000 {
        return decode_addsub_imm(d);
    }
    None
}
pub const fn decode_branch_and_sys(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1c000000) != 0x14000000) as usize];
    let op0 = (d >> 29) & 7;
    let op1 = (d >> 22) & 0xF;
    if (d & 0xffc00000) == 0xd5000000 {
        return decode_system(d);
    }
    if (d & 0xff000000) == 0xd4000000 {
        return decode_exception(d);
    }
    if (d & 0xfe000000) == 0x54000000 {
        return decode_condbranch(d);
    }
    if (d & 0xfe000000) == 0xd6000000 {
        return decode_branch_reg(d);
    }
    if (d & 0x7e000000) == 0x34000000 {
        return decode_compbranch(d);
    }
    if (d & 0x7e000000) == 0x36000000 {
        return decode_testbranch(d);
    }
    if (d & 0x7c000000) == 0x14000000 {
        return decode_branch_imm(d);
    }
    None
}
pub const fn decode_load_and_store(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x0a000000) != 0x08000000) as usize];
    let op0 = (d >> 31) & 1;
    let op1 = (d >> 28) & 3;
    let op2 = (d >> 26) & 1;
    let op3 = (d >> 23) & 3;
    let op4 = (d >> 16) & 0x3F;
    let op5 = (d >> 10) & 3;
    if (d & 0xbfbf0000) == 0x0c000000 {
        return decode_asisdlse(d);
    }
    if (d & 0xbf9f0000) == 0x0d000000 {
        return decode_asisdlso(d);
    }
    if (d & 0xbfa00000) == 0x0c800000 {
        return decode_asisdlsep(d);
    }
    if (d & 0x3b200c00) == 0x38000000 {
        return decode_ldst_unscaled(d);
    }
    if (d & 0x3b200c00) == 0x38000400 {
        return decode_ldst_immpost(d);
    }
    if (d & 0x3b200c00) == 0x38000800 {
        return decode_ldst_unpriv(d);
    }
    if (d & 0x3b200c00) == 0x38000c00 {
        return decode_ldst_immpre(d);
    }
    if (d & 0x3b200c00) == 0x38200000 {
        return decode_memop(d);
    }
    if (d & 0x3b200c00) == 0x38200800 {
        return decode_ldst_regoff(d);
    }
    if (d & 0xbf800000) == 0x0d800000 {
        return decode_asisdlsop(d);
    }
    if (d & 0x3b200400) == 0x38200400 {
        return decode_ldst_pac(d);
    }
    if (d & 0x3b800000) == 0x28000000 {
        return decode_ldstnapair_offs(d);
    }
    if (d & 0x3b800000) == 0x28800000 {
        return decode_ldstpair_post(d);
    }
    if (d & 0x3b800000) == 0x29000000 {
        return decode_ldstpair_off(d);
    }
    if (d & 0x3b800000) == 0x29800000 {
        return decode_ldstpair_pre(d);
    }
    if (d & 0x3f000000) == 0x08000000 {
        return decode_ldstexcl(d);
    }
    if (d & 0x3b000000) == 0x18000000 {
        return decode_loadlit(d);
    }
    if (d & 0x3b000000) == 0x39000000 {
        return decode_ldst_pos(d);
    }
    None
}
pub const fn decode_dataproc_register(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x0e000000) != 0x0a000000) as usize];
    let op0 = (d >> 30) & 1;
    let op1 = (d >> 28) & 1;
    let op2 = (d >> 21) & 0xF;
    let op3 = (d >> 11) & 1;
    if (d & 0x1fe00800) == 0x1a400000 {
        return decode_condcmp_reg(d);
    }
    if (d & 0x1fe00800) == 0x1a400800 {
        return decode_condcmp_imm(d);
    }
    if (d & 0x5fe00000) == 0x1ac00000 {
        return decode_dp_2src(d);
    }
    if (d & 0x5fe00000) == 0x5ac00000 {
        return decode_dp_1src(d);
    }
    if (d & 0x1fe00000) == 0x1a000000 {
        return decode_addsub_carry(d);
    }
    if (d & 0x1fe00000) == 0x1a800000 {
        return decode_condsel(d);
    }
    if (d & 0x1f200000) == 0x0b000000 {
        return decode_addsub_shift(d);
    }
    if (d & 0x1f200000) == 0x0b200000 {
        return decode_addsub_ext(d);
    }
    if (d & 0x1f000000) == 0x0a000000 {
        return decode_log_shift(d);
    }
    if (d & 0x1f000000) == 0x1b000000 {
        return decode_dp_3src(d);
    }
    None
}
pub const fn decode_dataproc_simd(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x0e000000) != 0x0e000000) as usize];
    let op0 = (d >> 28) & 0xF;
    let op1 = (d >> 23) & 3;
    let op2 = (d >> 19) & 0xF;
    let op3 = (d >> 10) & 0x1FF;
    if (d & 0xfffff000) == 0xcec08000 {
        return decode_cryptosha512_2(d);
    }
    if (d & 0xdf7e0c00) == 0x5e780800 {
        return decode_asisdmiscfp16(d);
    }
    if (d & 0xff3e0c00) == 0x4e280800 {
        return decode_cryptoaes(d);
    }
    if (d & 0xff3e0c00) == 0x5e280800 {
        return decode_cryptosha2(d);
    }
    if (d & 0x9f7e0c00) == 0x0e780800 {
        return decode_asimdmiscfp16(d);
    }
    if (d & 0xdf3e0c00) == 0x5e200800 {
        return decode_asisdmisc(d);
    }
    if (d & 0xdf3e0c00) == 0x5e300800 {
        return decode_asisdpair(d);
    }
    if (d & 0xffe0b000) == 0xce608000 {
        return decode_cryptosha512_3(d);
    }
    if (d & 0x5f20fc00) == 0x1e200000 {
        return decode_float2int(d);
    }
    if (d & 0x9f3e0c00) == 0x0e200800 {
        return decode_asimdmisc(d);
    }
    if (d & 0x9f3e0c00) == 0x0e300800 {
        return decode_asimdall(d);
    }
    if (d & 0xffe0c000) == 0xce408000 {
        return decode_crypto3_imm2(d);
    }
    if (d & 0x5f207c00) == 0x1e204000 {
        return decode_floatdp1(d);
    }
    if (d & 0x9ff80400) == 0x0f000400 {
        return decode_asimdimm(d);
    }
    if (d & 0xdf60c400) == 0x5e400400 {
        return decode_asisdsamefp16(d);
    }
    if (d & 0xdfe08400) == 0x5e000400 {
        return decode_asisdone(d);
    }
    if (d & 0xff208c00) == 0x5e000000 {
        return decode_cryptosha3(d);
    }
    if (d & 0x5f203c00) == 0x1e202000 {
        return decode_floatcmp(d);
    }
    if (d & 0x9f60c400) == 0x0e400400 {
        return decode_asimdsamefp16(d);
    }
    if (d & 0x9fe08400) == 0x0e000400 {
        return decode_asimdins(d);
    }
    if (d & 0xbf208c00) == 0x0e000000 {
        return decode_asimdtbl(d);
    }
    if (d & 0xbf208c00) == 0x0e000800 {
        return decode_asimdperm(d);
    }
    if (d & 0xffe00000) == 0xce800000 {
        return decode_crypto3_imm6(d);
    }
    if (d & 0x5f201c00) == 0x1e201000 {
        return decode_floatimm(d);
    }
    if (d & 0xbf208400) == 0x2e000000 {
        return decode_asimdext(d);
    }
    if (d & 0xdf200c00) == 0x5e200000 {
        return decode_asisddiff(d);
    }
    if (d & 0xdf208400) == 0x5e008400 {
        return decode_asisdsame2(d);
    }
    if (d & 0xff808000) == 0xce000000 {
        return decode_crypto4(d);
    }
    if (d & 0x5f200c00) == 0x1e200400 {
        return decode_floatccmp(d);
    }
    if (d & 0x5f200c00) == 0x1e200800 {
        return decode_floatdp2(d);
    }
    if (d & 0x5f200c00) == 0x1e200c00 {
        return decode_floatsel(d);
    }
    if (d & 0x9f200c00) == 0x0e200000 {
        return decode_asimddiff(d);
    }
    if (d & 0x9f208400) == 0x0e008400 {
        return decode_asimdsame2(d);
    }
    if (d & 0xdf200400) == 0x5e200400 {
        return decode_asisdsame(d);
    }
    if (d & 0xdf800400) == 0x5f000400 {
        return decode_asisdshf(d);
    }
    if (d & 0x9f200400) == 0x0e200400 {
        return decode_asimdsame(d);
    }
    if (d & 0xdf000400) == 0x5f000000 {
        return decode_asisdelem(d);
    }
    if (d & 0x5f200000) == 0x1e000000 {
        return decode_float2fix(d);
    }
    if (d & 0x9f000400) == 0x0f000000 {
        return decode_asimdelem(d);
    }
    if (d & 0x5f000000) == 0x1f000000 {
        return decode_floatdp3(d);
    }
    if (d & 0x9f800400) == 0x0f000400 && (d & 0x780000) != 0x000000 {
        return decode_asimdshf(d);
    }
    None
}
pub const fn decode_compbranch(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x7e000000) != 0x34000000) as usize];
    let Rt = d & 0x1F;
    let imm19 = (d >> 5) & 0x7FFFF;
    let op = (d >> 24) & 1;
    let sf = (d >> 31) & 1;
    if (d & 0xff000000) == 0x34000000 {
        return Some(InstructionKind::CBZ32Compbranch {
            Rt: Rt as _,
            imm19: imm19 as _,
        });
    }
    if (d & 0xff000000) == 0x35000000 {
        return Some(InstructionKind::CBNZ32Compbranch {
            Rt: Rt as _,
            imm19: imm19 as _,
        });
    }
    if (d & 0xff000000) == 0xb4000000 {
        return Some(InstructionKind::CBZ64Compbranch {
            Rt: Rt as _,
            imm19: imm19 as _,
        });
    }
    if (d & 0xff000000) == 0xb5000000 {
        return Some(InstructionKind::CBNZ64Compbranch {
            Rt: Rt as _,
            imm19: imm19 as _,
        });
    }
    None
}
pub const fn decode_condbranch(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xfe000000) != 0x54000000) as usize];
    let cond = d & 0xF;
    let imm19 = (d >> 5) & 0x7FFFF;
    let o0 = (d >> 4) & 1;
    let o1 = (d >> 24) & 1;
    if (d & 0xff000010) == 0x54000000 {
        return Some(InstructionKind::BOnlyCondbranch {
            cond: cond as _,
            imm19: imm19 as _,
        });
    }
    None
}
pub const fn decode_exception(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xff000000) != 0xd4000000) as usize];
    let LL = d & 3;
    let imm16 = (d >> 5) & 0xFFFF;
    let op2 = (d >> 2) & 7;
    let opc = (d >> 21) & 7;
    if (d & 0xffe0001f) == 0xd4000001 {
        return Some(InstructionKind::SVCExException { imm16: imm16 as _ });
    }
    if (d & 0xffe0001f) == 0xd4000002 {
        return Some(InstructionKind::HVCExException { imm16: imm16 as _ });
    }
    if (d & 0xffe0001f) == 0xd4000003 {
        return Some(InstructionKind::SMCExException { imm16: imm16 as _ });
    }
    if (d & 0xffe0001f) == 0xd4200000 {
        return Some(InstructionKind::BRKExException { imm16: imm16 as _ });
    }
    if (d & 0xffe0001f) == 0xd4400000 {
        return Some(InstructionKind::HLTExException { imm16: imm16 as _ });
    }
    if (d & 0xffe0001f) == 0xd4a00001 {
        return Some(InstructionKind::DCPS1DcException { imm16: imm16 as _ });
    }
    if (d & 0xffe0001f) == 0xd4a00002 {
        return Some(InstructionKind::DCPS2DcException { imm16: imm16 as _ });
    }
    if (d & 0xffe0001f) == 0xd4a00003 {
        return Some(InstructionKind::DCPS3DcException { imm16: imm16 as _ });
    }
    None
}
pub const fn decode_system(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xffc00000) != 0xd5000000) as usize];
    let CRm = (d >> 8) & 0xF;
    let CRn = (d >> 12) & 0xF;
    let L = (d >> 21) & 1;
    let Rt = d & 0x1F;
    let op0 = (d >> 19) & 3;
    let op1 = (d >> 16) & 7;
    let op2 = (d >> 5) & 7;
    if d == 0xd503201f {
        return Some(InstructionKind::NOPHiSystem);
    }
    if d == 0xd503203f {
        return Some(InstructionKind::YIELDHiSystem);
    }
    if d == 0xd503205f {
        return Some(InstructionKind::WFEHiSystem);
    }
    if d == 0xd503207f {
        return Some(InstructionKind::WFIHiSystem);
    }
    if d == 0xd503209f {
        return Some(InstructionKind::SEVHiSystem);
    }
    if d == 0xd50320bf {
        return Some(InstructionKind::SEVLHiSystem);
    }
    if d == 0xd50320ff {
        return Some(InstructionKind::XPACLRIHiSystem);
    }
    if d == 0xd503211f {
        return Some(InstructionKind::PACIA1716HiSystem);
    }
    if d == 0xd503215f {
        return Some(InstructionKind::PACIB1716HiSystem);
    }
    if d == 0xd503219f {
        return Some(InstructionKind::AUTIA1716HiSystem);
    }
    if d == 0xd50321df {
        return Some(InstructionKind::AUTIB1716HiSystem);
    }
    if d == 0xd503221f {
        return Some(InstructionKind::ESBHiSystem);
    }
    if d == 0xd503223f {
        return Some(InstructionKind::PSBHcSystem);
    }
    if d == 0xd503231f {
        return Some(InstructionKind::PACIAZHiSystem);
    }
    if d == 0xd503233f {
        return Some(InstructionKind::PACIASPHiSystem);
    }
    if d == 0xd503235f {
        return Some(InstructionKind::PACIBZHiSystem);
    }
    if d == 0xd503237f {
        return Some(InstructionKind::PACIBSPHiSystem);
    }
    if d == 0xd503239f {
        return Some(InstructionKind::AUTIAZHiSystem);
    }
    if d == 0xd50323bf {
        return Some(InstructionKind::AUTIASPHiSystem);
    }
    if d == 0xd50323df {
        return Some(InstructionKind::AUTIBZHiSystem);
    }
    if d == 0xd50323ff {
        return Some(InstructionKind::AUTIBSPHiSystem);
    }
    if (d & 0xffffffdf) == 0xd50320df {
        return Some(InstructionKind::HINT1 { op2: op2 as _ });
    }
    if (d & 0xfffff0ff) == 0xd503305f {
        return Some(InstructionKind::CLREXBnSystem { CRm: CRm as _ });
    }
    if (d & 0xfffff0ff) == 0xd503309f {
        return Some(InstructionKind::DSBBoSystem { CRm: CRm as _ });
    }
    if (d & 0xfffff0ff) == 0xd50330bf {
        return Some(InstructionKind::DMBBoSystem { CRm: CRm as _ });
    }
    if (d & 0xfffff0ff) == 0xd50330df {
        return Some(InstructionKind::ISBBiSystem { CRm: CRm as _ });
    }
    if (d & 0xfff8f01f) == 0xd500401f {
        return Some(InstructionKind::MSRSiSystem {
            CRm: CRm as _,
            op1: op1 as _,
            op2: op2 as _,
        });
    }
    if (d & 0xfff80000) == 0xd5080000 {
        return Some(InstructionKind::SYSCrSystem {
            CRm: CRm as _,
            CRn: CRn as _,
            Rt: Rt as _,
            op1: op1 as _,
            op2: op2 as _,
        });
    }
    if (d & 0xfff80000) == 0xd5280000 {
        return Some(InstructionKind::SYSLRcSystem {
            CRm: CRm as _,
            CRn: CRn as _,
            Rt: Rt as _,
            op1: op1 as _,
            op2: op2 as _,
        });
    }
    if (d & 0xfff00000) == 0xd5100000 {
        return Some(InstructionKind::MSRSrSystem {
            CRm: CRm as _,
            CRn: CRn as _,
            Rt: Rt as _,
            op0: op0 as _,
            op1: op1 as _,
            op2: op2 as _,
        });
    }
    if (d & 0xfff00000) == 0xd5300000 {
        return Some(InstructionKind::MRSRsSystem {
            CRm: CRm as _,
            CRn: CRn as _,
            Rt: Rt as _,
            op0: op0 as _,
            op1: op1 as _,
            op2: op2 as _,
        });
    }
    if (d & 0xffffff1f) == 0xd503221f && (d & 0x0000c0) != 0x000000 {
        return Some(InstructionKind::HINT3 { op2: op2 as _ });
    }
    if (d & 0xfffff01f) == 0xd503201f && (d & 0x000d00) != 0x000000 {
        return Some(InstructionKind::HINT2 {
            CRm: CRm as _,
            op2: op2 as _,
        });
    }
    None
}
pub const fn decode_testbranch(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x7e000000) != 0x36000000) as usize];
    let Rt = d & 0x1F;
    let b40 = (d >> 19) & 0x1F;
    let b5 = (d >> 31) & 1;
    let imm14 = (d >> 5) & 0x3FFF;
    let op = (d >> 24) & 1;
    if (d & 0x7f000000) == 0x36000000 {
        return Some(InstructionKind::TBZOnlyTestbranch {
            Rt: Rt as _,
            b40: b40 as _,
            b5: b5 as _,
            imm14: imm14 as _,
        });
    }
    if (d & 0x7f000000) == 0x37000000 {
        return Some(InstructionKind::TBNZOnlyTestbranch {
            Rt: Rt as _,
            b40: b40 as _,
            b5: b5 as _,
            imm14: imm14 as _,
        });
    }
    None
}
pub const fn decode_branch_imm(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x7c000000) != 0x14000000) as usize];
    let imm26 = d & 0x3FFFFFF;
    let op = (d >> 31) & 1;
    if (d & 0xfc000000) == 0x14000000 {
        return Some(InstructionKind::BOnlyBranchImm { imm26: imm26 as _ });
    }
    if (d & 0xfc000000) == 0x94000000 {
        return Some(InstructionKind::BLOnlyBranchImm { imm26: imm26 as _ });
    }
    None
}
pub const fn decode_branch_reg(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xfe000000) != 0xd6000000) as usize];
    let Rn = (d >> 5) & 0x1F;
    let op2 = (d >> 16) & 0x1F;
    let op3 = (d >> 10) & 0x3F;
    let op4 = d & 0x1F;
    let opc = (d >> 21) & 0xF;
    if d == 0xd65f0bff {
        return Some(InstructionKind::RETAA64EBranchReg);
    }
    if d == 0xd65f0fff {
        return Some(InstructionKind::RETAB64EBranchReg);
    }
    if d == 0xd69f03e0 {
        return Some(InstructionKind::ERET64EBranchReg);
    }
    if d == 0xd69f0bff {
        return Some(InstructionKind::ERETAA64EBranchReg);
    }
    if d == 0xd69f0fff {
        return Some(InstructionKind::ERETAB64EBranchReg);
    }
    if d == 0xd6bf03e0 {
        return Some(InstructionKind::DRPS64EBranchReg);
    }
    if (d & 0xfffffc1f) == 0xd61f0000 {
        return Some(InstructionKind::BR64BranchReg { Rn: Rn as _ });
    }
    if (d & 0xfffffc1f) == 0xd61f081f {
        return Some(InstructionKind::BRAAZ64BranchReg { Rn: Rn as _ });
    }
    if (d & 0xfffffc1f) == 0xd61f0c1f {
        return Some(InstructionKind::BRABZ64BranchReg { Rn: Rn as _ });
    }
    if (d & 0xfffffc1f) == 0xd63f0000 {
        return Some(InstructionKind::BLR64BranchReg { Rn: Rn as _ });
    }
    if (d & 0xfffffc1f) == 0xd63f081f {
        return Some(InstructionKind::BLRAAZ64BranchReg { Rn: Rn as _ });
    }
    if (d & 0xfffffc1f) == 0xd63f0c1f {
        return Some(InstructionKind::BLRABZ64BranchReg { Rn: Rn as _ });
    }
    if (d & 0xfffffc1f) == 0xd65f0000 {
        return Some(InstructionKind::RET64RBranchReg { Rn: Rn as _ });
    }
    if (d & 0xfffffc00) == 0xd71f0800 {
        return Some(InstructionKind::BRAA64PBranchReg {
            Rn: Rn as _,
            op4: op4 as _,
        });
    }
    if (d & 0xfffffc00) == 0xd71f0c00 {
        return Some(InstructionKind::BRAB64PBranchReg {
            Rn: Rn as _,
            op4: op4 as _,
        });
    }
    if (d & 0xfffffc00) == 0xd73f0800 {
        return Some(InstructionKind::BLRAA64PBranchReg {
            Rn: Rn as _,
            op4: op4 as _,
        });
    }
    if (d & 0xfffffc00) == 0xd73f0c00 {
        return Some(InstructionKind::BLRAB64PBranchReg {
            Rn: Rn as _,
            op4: op4 as _,
        });
    }
    None
}
pub const fn decode_asisdlse(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xbfbf0000) != 0x0c000000) as usize];
    let L = (d >> 22) & 1;
    let Q = (d >> 30) & 1;
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let opcode = (d >> 12) & 0xF;
    let size = (d >> 10) & 3;
    if (d & 0xbffff000) == 0x0c000000 {
        return Some(InstructionKind::ST4AsisdlseR4 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c002000 {
        return Some(InstructionKind::ST1AsisdlseR44V {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c004000 {
        return Some(InstructionKind::ST3AsisdlseR3 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c006000 {
        return Some(InstructionKind::ST1AsisdlseR33V {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c007000 {
        return Some(InstructionKind::ST1AsisdlseR11V {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c008000 {
        return Some(InstructionKind::ST2AsisdlseR2 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c00a000 {
        return Some(InstructionKind::ST1AsisdlseR22V {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c400000 {
        return Some(InstructionKind::LD4AsisdlseR4 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c402000 {
        return Some(InstructionKind::LD1AsisdlseR44V {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c404000 {
        return Some(InstructionKind::LD3AsisdlseR3 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c406000 {
        return Some(InstructionKind::LD1AsisdlseR33V {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c407000 {
        return Some(InstructionKind::LD1AsisdlseR11V {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c408000 {
        return Some(InstructionKind::LD2AsisdlseR2 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c40a000 {
        return Some(InstructionKind::LD1AsisdlseR22V {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asisdlsep(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xbfa00000) != 0x0c800000) as usize];
    let L = (d >> 22) & 1;
    let Q = (d >> 30) & 1;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let opcode = (d >> 12) & 0xF;
    let size = (d >> 10) & 3;
    if (d & 0xbffff000) == 0x0c9f0000 {
        return Some(InstructionKind::ST4AsisdlsepI4I {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c9f2000 {
        return Some(InstructionKind::ST1AsisdlsepI4I4 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c9f4000 {
        return Some(InstructionKind::ST3AsisdlsepI3I {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c9f6000 {
        return Some(InstructionKind::ST1AsisdlsepI3I3 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c9f7000 {
        return Some(InstructionKind::ST1AsisdlsepI1I1 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c9f8000 {
        return Some(InstructionKind::ST2AsisdlsepI2I {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0c9fa000 {
        return Some(InstructionKind::ST1AsisdlsepI2I2 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0cdf0000 {
        return Some(InstructionKind::LD4AsisdlsepI4I {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0cdf2000 {
        return Some(InstructionKind::LD1AsisdlsepI4I4 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0cdf4000 {
        return Some(InstructionKind::LD3AsisdlsepI3I {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0cdf6000 {
        return Some(InstructionKind::LD1AsisdlsepI3I3 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0cdf7000 {
        return Some(InstructionKind::LD1AsisdlsepI1I1 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0cdf8000 {
        return Some(InstructionKind::LD2AsisdlsepI2I {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0cdfa000 {
        return Some(InstructionKind::LD1AsisdlsepI2I2 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0c800000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST4AsisdlsepR4R {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0c802000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST1AsisdlsepR4R4 {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0c804000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST3AsisdlsepR3R {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0c806000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST1AsisdlsepR3R3 {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0c807000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST1AsisdlsepR1R1 {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0c808000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST2AsisdlsepR2R {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0c80a000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST1AsisdlsepR2R2 {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0cc00000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD4AsisdlsepR4R {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0cc02000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD1AsisdlsepR4R4 {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0cc04000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD3AsisdlsepR3R {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0cc06000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD1AsisdlsepR3R3 {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0cc07000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD1AsisdlsepR1R1 {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0cc08000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD2AsisdlsepR2R {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0cc0a000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD1AsisdlsepR2R2 {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asisdlso(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xbf9f0000) != 0x0d000000) as usize];
    let L = (d >> 22) & 1;
    let Q = (d >> 30) & 1;
    let R = (d >> 21) & 1;
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let S = (d >> 12) & 1;
    let opcode = (d >> 13) & 7;
    let size = (d >> 10) & 3;
    if (d & 0xbffffc00) == 0x0d008400 {
        return Some(InstructionKind::ST1AsisdlsoD11D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbffffc00) == 0x0d00a400 {
        return Some(InstructionKind::ST3AsisdlsoD33D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbffffc00) == 0x0d208400 {
        return Some(InstructionKind::ST2AsisdlsoD22D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbffffc00) == 0x0d20a400 {
        return Some(InstructionKind::ST4AsisdlsoD44D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbffffc00) == 0x0d408400 {
        return Some(InstructionKind::LD1AsisdlsoD11D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbffffc00) == 0x0d40a400 {
        return Some(InstructionKind::LD3AsisdlsoD33D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbffffc00) == 0x0d608400 {
        return Some(InstructionKind::LD2AsisdlsoD22D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbffffc00) == 0x0d60a400 {
        return Some(InstructionKind::LD4AsisdlsoD44D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbfffec00) == 0x0d008000 {
        return Some(InstructionKind::ST1AsisdlsoS11S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffec00) == 0x0d00a000 {
        return Some(InstructionKind::ST3AsisdlsoS33S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffec00) == 0x0d208000 {
        return Some(InstructionKind::ST2AsisdlsoS22S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffec00) == 0x0d20a000 {
        return Some(InstructionKind::ST4AsisdlsoS44S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffec00) == 0x0d408000 {
        return Some(InstructionKind::LD1AsisdlsoS11S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffec00) == 0x0d40a000 {
        return Some(InstructionKind::LD3AsisdlsoS33S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffec00) == 0x0d608000 {
        return Some(InstructionKind::LD2AsisdlsoS22S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffec00) == 0x0d60a000 {
        return Some(InstructionKind::LD4AsisdlsoS44S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffe400) == 0x0d004000 {
        return Some(InstructionKind::ST1AsisdlsoH11H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe400) == 0x0d006000 {
        return Some(InstructionKind::ST3AsisdlsoH33H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe400) == 0x0d204000 {
        return Some(InstructionKind::ST2AsisdlsoH22H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe400) == 0x0d206000 {
        return Some(InstructionKind::ST4AsisdlsoH44H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe400) == 0x0d404000 {
        return Some(InstructionKind::LD1AsisdlsoH11H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe400) == 0x0d406000 {
        return Some(InstructionKind::LD3AsisdlsoH33H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe400) == 0x0d604000 {
        return Some(InstructionKind::LD2AsisdlsoH22H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe400) == 0x0d606000 {
        return Some(InstructionKind::LD4AsisdlsoH44H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0d40c000 {
        return Some(InstructionKind::LD1RAsisdlsoR1 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0d40e000 {
        return Some(InstructionKind::LD3RAsisdlsoR3 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0d60c000 {
        return Some(InstructionKind::LD2RAsisdlsoR2 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0d60e000 {
        return Some(InstructionKind::LD4RAsisdlsoR4 {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0d000000 {
        return Some(InstructionKind::ST1AsisdlsoB11B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0d002000 {
        return Some(InstructionKind::ST3AsisdlsoB33B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0d200000 {
        return Some(InstructionKind::ST2AsisdlsoB22B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0d202000 {
        return Some(InstructionKind::ST4AsisdlsoB44B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0d400000 {
        return Some(InstructionKind::LD1AsisdlsoB11B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0d402000 {
        return Some(InstructionKind::LD3AsisdlsoB33B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0d600000 {
        return Some(InstructionKind::LD2AsisdlsoB22B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0d602000 {
        return Some(InstructionKind::LD4AsisdlsoB44B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asisdlsop(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xbf800000) != 0x0d800000) as usize];
    let L = (d >> 22) & 1;
    let Q = (d >> 30) & 1;
    let R = (d >> 21) & 1;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let S = (d >> 12) & 1;
    let opcode = (d >> 13) & 7;
    let size = (d >> 10) & 3;
    if (d & 0xbffffc00) == 0x0d9f8400 {
        return Some(InstructionKind::ST1AsisdlsopD1I1D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbffffc00) == 0x0d9fa400 {
        return Some(InstructionKind::ST3AsisdlsopD3I3D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbffffc00) == 0x0dbf8400 {
        return Some(InstructionKind::ST2AsisdlsopD2I2D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbffffc00) == 0x0dbfa400 {
        return Some(InstructionKind::ST4AsisdlsopD4I4D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbffffc00) == 0x0ddf8400 {
        return Some(InstructionKind::LD1AsisdlsopD1I1D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbffffc00) == 0x0ddfa400 {
        return Some(InstructionKind::LD3AsisdlsopD3I3D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbffffc00) == 0x0dff8400 {
        return Some(InstructionKind::LD2AsisdlsopD2I2D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbffffc00) == 0x0dffa400 {
        return Some(InstructionKind::LD4AsisdlsopD4I4D {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbfffec00) == 0x0d9f8000 {
        return Some(InstructionKind::ST1AsisdlsopS1I1S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffec00) == 0x0d9fa000 {
        return Some(InstructionKind::ST3AsisdlsopS3I3S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffec00) == 0x0dbf8000 {
        return Some(InstructionKind::ST2AsisdlsopS2I2S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffec00) == 0x0dbfa000 {
        return Some(InstructionKind::ST4AsisdlsopS4I4S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffec00) == 0x0ddf8000 {
        return Some(InstructionKind::LD1AsisdlsopS1I1S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffec00) == 0x0ddfa000 {
        return Some(InstructionKind::LD3AsisdlsopS3I3S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffec00) == 0x0dff8000 {
        return Some(InstructionKind::LD2AsisdlsopS2I2S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffec00) == 0x0dffa000 {
        return Some(InstructionKind::LD4AsisdlsopS4I4S {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfffe400) == 0x0d9f4000 {
        return Some(InstructionKind::ST1AsisdlsopH1I1H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe400) == 0x0d9f6000 {
        return Some(InstructionKind::ST3AsisdlsopH3I3H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe400) == 0x0dbf4000 {
        return Some(InstructionKind::ST2AsisdlsopH2I2H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe400) == 0x0dbf6000 {
        return Some(InstructionKind::ST4AsisdlsopH4I4H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe400) == 0x0ddf4000 {
        return Some(InstructionKind::LD1AsisdlsopH1I1H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe400) == 0x0ddf6000 {
        return Some(InstructionKind::LD3AsisdlsopH3I3H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe400) == 0x0dff4000 {
        return Some(InstructionKind::LD2AsisdlsopH2I2H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe400) == 0x0dff6000 {
        return Some(InstructionKind::LD4AsisdlsopH4I4H {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0ddfc000 {
        return Some(InstructionKind::LD1RAsisdlsopR1I {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0ddfe000 {
        return Some(InstructionKind::LD3RAsisdlsopR3I {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0dffc000 {
        return Some(InstructionKind::LD2RAsisdlsopR2I {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbffff000) == 0x0dffe000 {
        return Some(InstructionKind::LD4RAsisdlsopR4I {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0d9f0000 {
        return Some(InstructionKind::ST1AsisdlsopB1I1B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0d9f2000 {
        return Some(InstructionKind::ST3AsisdlsopB3I3B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0dbf0000 {
        return Some(InstructionKind::ST2AsisdlsopB2I2B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0dbf2000 {
        return Some(InstructionKind::ST4AsisdlsopB4I4B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0ddf0000 {
        return Some(InstructionKind::LD1AsisdlsopB1I1B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0ddf2000 {
        return Some(InstructionKind::LD3AsisdlsopB3I3B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0dff0000 {
        return Some(InstructionKind::LD2AsisdlsopB2I2B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfffe000) == 0x0dff2000 {
        return Some(InstructionKind::LD4AsisdlsopB4I4B {
            Q: Q as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0d808400 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST1AsisdlsopDx1R1D {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0d80a400 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST3AsisdlsopDx3R3D {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0da08400 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST2AsisdlsopDx2R2D {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0da0a400 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST4AsisdlsopDx4R4D {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0dc08400 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD1AsisdlsopDx1R1D {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0dc0a400 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD3AsisdlsopDx3R3D {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0de08400 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD2AsisdlsopDx2R2D {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0de0a400 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD4AsisdlsopDx4R4D {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xbfe0ec00) == 0x0d808000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST1AsisdlsopSx1R1S {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfe0ec00) == 0x0d80a000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST3AsisdlsopSx3R3S {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfe0ec00) == 0x0da08000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST2AsisdlsopSx2R2S {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfe0ec00) == 0x0da0a000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST4AsisdlsopSx4R4S {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfe0ec00) == 0x0dc08000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD1AsisdlsopSx1R1S {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfe0ec00) == 0x0dc0a000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD3AsisdlsopSx3R3S {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfe0ec00) == 0x0de08000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD2AsisdlsopSx2R2S {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfe0ec00) == 0x0de0a000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD4AsisdlsopSx4R4S {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xbfe0e400) == 0x0d804000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST1AsisdlsopHx1R1H {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e400) == 0x0d806000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST3AsisdlsopHx3R3H {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e400) == 0x0da04000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST2AsisdlsopHx2R2H {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e400) == 0x0da06000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST4AsisdlsopHx4R4H {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e400) == 0x0dc04000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD1AsisdlsopHx1R1H {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e400) == 0x0dc06000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD3AsisdlsopHx3R3H {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e400) == 0x0de04000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD2AsisdlsopHx2R2H {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e400) == 0x0de06000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD4AsisdlsopHx4R4H {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0dc0c000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD1RAsisdlsopRx1R {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0dc0e000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD3RAsisdlsopRx3R {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0de0c000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD2RAsisdlsopRx2R {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0f000) == 0x0de0e000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD4RAsisdlsopRx4R {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e000) == 0x0d800000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST1AsisdlsopBx1R1B {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e000) == 0x0d802000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST3AsisdlsopBx3R3B {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e000) == 0x0da00000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST2AsisdlsopBx2R2B {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e000) == 0x0da02000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::ST4AsisdlsopBx4R4B {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e000) == 0x0dc00000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD1AsisdlsopBx1R1B {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e000) == 0x0dc02000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD3AsisdlsopBx3R3B {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e000) == 0x0de00000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD2AsisdlsopBx2R2B {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    if (d & 0xbfe0e000) == 0x0de02000 && (d & 0x1f0000) != 0x1f0000 {
        return Some(InstructionKind::LD4AsisdlsopBx4R4B {
            Q: Q as _,
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_memop(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x3b200c00) != 0x38200000) as usize];
    let A = (d >> 23) & 1;
    let R = (d >> 22) & 1;
    let Rn = (d >> 5) & 0x1F;
    let Rs = (d >> 16) & 0x1F;
    let Rt = d & 0x1F;
    let V = (d >> 26) & 1;
    let o3 = (d >> 15) & 1;
    let opc = (d >> 12) & 7;
    let size = (d >> 30) & 3;
    if (d & 0xffe0fc1f) == 0x3820001f {
        return Some(InstructionKind::STADDB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3820101f {
        return Some(InstructionKind::STCLRB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3820201f {
        return Some(InstructionKind::STEORB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3820301f {
        return Some(InstructionKind::STSETB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3820401f {
        return Some(InstructionKind::STSMAXB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3820501f {
        return Some(InstructionKind::STSMINB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3820601f {
        return Some(InstructionKind::STUMAXB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3820701f {
        return Some(InstructionKind::STUMINB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3860001f {
        return Some(InstructionKind::STADDLB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3860101f {
        return Some(InstructionKind::STCLRLB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3860201f {
        return Some(InstructionKind::STEORLB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3860301f {
        return Some(InstructionKind::STSETLB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3860401f {
        return Some(InstructionKind::STSMAXLB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3860501f {
        return Some(InstructionKind::STSMINLB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3860601f {
        return Some(InstructionKind::STUMAXLB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x3860701f {
        return Some(InstructionKind::STUMINLB32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7820001f {
        return Some(InstructionKind::STADDH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7820101f {
        return Some(InstructionKind::STCLRH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7820201f {
        return Some(InstructionKind::STEORH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7820301f {
        return Some(InstructionKind::STSETH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7820401f {
        return Some(InstructionKind::STSMAXH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7820501f {
        return Some(InstructionKind::STSMINH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7820601f {
        return Some(InstructionKind::STUMAXH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7820701f {
        return Some(InstructionKind::STUMINH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7860001f {
        return Some(InstructionKind::STADDLH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7860101f {
        return Some(InstructionKind::STCLRLH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7860201f {
        return Some(InstructionKind::STEORLH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7860301f {
        return Some(InstructionKind::STSETLH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7860401f {
        return Some(InstructionKind::STSMAXLH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7860501f {
        return Some(InstructionKind::STSMINLH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7860601f {
        return Some(InstructionKind::STUMAXLH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x7860701f {
        return Some(InstructionKind::STUMINLH32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb820001f {
        return Some(InstructionKind::STADD32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb820101f {
        return Some(InstructionKind::STCLR32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb820201f {
        return Some(InstructionKind::STEOR32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb820301f {
        return Some(InstructionKind::STSET32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb820401f {
        return Some(InstructionKind::STSMAX32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb820501f {
        return Some(InstructionKind::STSMIN32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb820601f {
        return Some(InstructionKind::STUMAX32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb820701f {
        return Some(InstructionKind::STUMIN32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb860001f {
        return Some(InstructionKind::STADDL32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb860101f {
        return Some(InstructionKind::STCLRL32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb860201f {
        return Some(InstructionKind::STEORL32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb860301f {
        return Some(InstructionKind::STSETL32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb860401f {
        return Some(InstructionKind::STSMAXL32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb860501f {
        return Some(InstructionKind::STSMINL32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb860601f {
        return Some(InstructionKind::STUMAXL32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xb860701f {
        return Some(InstructionKind::STUMINL32SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf820001f {
        return Some(InstructionKind::STADD64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf820101f {
        return Some(InstructionKind::STCLR64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf820201f {
        return Some(InstructionKind::STEOR64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf820301f {
        return Some(InstructionKind::STSET64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf820401f {
        return Some(InstructionKind::STSMAX64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf820501f {
        return Some(InstructionKind::STSMIN64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf820601f {
        return Some(InstructionKind::STUMAX64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf820701f {
        return Some(InstructionKind::STUMIN64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf860001f {
        return Some(InstructionKind::STADDL64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf860101f {
        return Some(InstructionKind::STCLRL64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf860201f {
        return Some(InstructionKind::STEORL64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf860301f {
        return Some(InstructionKind::STSETL64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf860401f {
        return Some(InstructionKind::STSMAXL64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf860501f {
        return Some(InstructionKind::STSMINL64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf860601f {
        return Some(InstructionKind::STUMAXL64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc1f) == 0xf860701f {
        return Some(InstructionKind::STUMINL64SMemop {
            Rn: Rn as _,
            Rs: Rs as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38208000 {
        return Some(InstructionKind::SWPB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38608000 {
        return Some(InstructionKind::SWPLB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38a00000 {
        return Some(InstructionKind::LDADDAB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38a01000 {
        return Some(InstructionKind::LDCLRAB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38a02000 {
        return Some(InstructionKind::LDEORAB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38a03000 {
        return Some(InstructionKind::LDSETAB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38a04000 {
        return Some(InstructionKind::LDSMAXAB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38a05000 {
        return Some(InstructionKind::LDSMINAB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38a06000 {
        return Some(InstructionKind::LDUMAXAB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38a07000 {
        return Some(InstructionKind::LDUMINAB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38a08000 {
        return Some(InstructionKind::SWPAB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38a0c000 {
        return Some(InstructionKind::LDAPRB32LMemop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38e00000 {
        return Some(InstructionKind::LDADDALB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38e01000 {
        return Some(InstructionKind::LDCLRALB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38e02000 {
        return Some(InstructionKind::LDEORALB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38e03000 {
        return Some(InstructionKind::LDSETALB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38e04000 {
        return Some(InstructionKind::LDSMAXALB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38e05000 {
        return Some(InstructionKind::LDSMINALB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38e06000 {
        return Some(InstructionKind::LDUMAXALB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38e07000 {
        return Some(InstructionKind::LDUMINALB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38e08000 {
        return Some(InstructionKind::SWPALB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78208000 {
        return Some(InstructionKind::SWPH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78608000 {
        return Some(InstructionKind::SWPLH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78a00000 {
        return Some(InstructionKind::LDADDAH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78a01000 {
        return Some(InstructionKind::LDCLRAH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78a02000 {
        return Some(InstructionKind::LDEORAH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78a03000 {
        return Some(InstructionKind::LDSETAH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78a04000 {
        return Some(InstructionKind::LDSMAXAH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78a05000 {
        return Some(InstructionKind::LDSMINAH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78a06000 {
        return Some(InstructionKind::LDUMAXAH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78a07000 {
        return Some(InstructionKind::LDUMINAH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78a08000 {
        return Some(InstructionKind::SWPAH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78a0c000 {
        return Some(InstructionKind::LDAPRH32LMemop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78e00000 {
        return Some(InstructionKind::LDADDALH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78e01000 {
        return Some(InstructionKind::LDCLRALH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78e02000 {
        return Some(InstructionKind::LDEORALH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78e03000 {
        return Some(InstructionKind::LDSETALH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78e04000 {
        return Some(InstructionKind::LDSMAXALH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78e05000 {
        return Some(InstructionKind::LDSMINALH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78e06000 {
        return Some(InstructionKind::LDUMAXALH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78e07000 {
        return Some(InstructionKind::LDUMINALH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78e08000 {
        return Some(InstructionKind::SWPALH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8208000 {
        return Some(InstructionKind::SWP32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8608000 {
        return Some(InstructionKind::SWPL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8a00000 {
        return Some(InstructionKind::LDADDA32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8a01000 {
        return Some(InstructionKind::LDCLRA32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8a02000 {
        return Some(InstructionKind::LDEORA32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8a03000 {
        return Some(InstructionKind::LDSETA32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8a04000 {
        return Some(InstructionKind::LDSMAXA32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8a05000 {
        return Some(InstructionKind::LDSMINA32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8a06000 {
        return Some(InstructionKind::LDUMAXA32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8a07000 {
        return Some(InstructionKind::LDUMINA32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8a08000 {
        return Some(InstructionKind::SWPA32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8a0c000 {
        return Some(InstructionKind::LDAPR32LMemop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8e00000 {
        return Some(InstructionKind::LDADDAL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8e01000 {
        return Some(InstructionKind::LDCLRAL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8e02000 {
        return Some(InstructionKind::LDEORAL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8e03000 {
        return Some(InstructionKind::LDSETAL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8e04000 {
        return Some(InstructionKind::LDSMAXAL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8e05000 {
        return Some(InstructionKind::LDSMINAL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8e06000 {
        return Some(InstructionKind::LDUMAXAL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8e07000 {
        return Some(InstructionKind::LDUMINAL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8e08000 {
        return Some(InstructionKind::SWPAL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8208000 {
        return Some(InstructionKind::SWP64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8608000 {
        return Some(InstructionKind::SWPL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8a00000 {
        return Some(InstructionKind::LDADDA64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8a01000 {
        return Some(InstructionKind::LDCLRA64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8a02000 {
        return Some(InstructionKind::LDEORA64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8a03000 {
        return Some(InstructionKind::LDSETA64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8a04000 {
        return Some(InstructionKind::LDSMAXA64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8a05000 {
        return Some(InstructionKind::LDSMINA64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8a06000 {
        return Some(InstructionKind::LDUMAXA64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8a07000 {
        return Some(InstructionKind::LDUMINA64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8a08000 {
        return Some(InstructionKind::SWPA64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8a0c000 {
        return Some(InstructionKind::LDAPR64LMemop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8e00000 {
        return Some(InstructionKind::LDADDAL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8e01000 {
        return Some(InstructionKind::LDCLRAL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8e02000 {
        return Some(InstructionKind::LDEORAL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8e03000 {
        return Some(InstructionKind::LDSETAL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8e04000 {
        return Some(InstructionKind::LDSMAXAL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8e05000 {
        return Some(InstructionKind::LDSMINAL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8e06000 {
        return Some(InstructionKind::LDUMAXAL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8e07000 {
        return Some(InstructionKind::LDUMINAL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8e08000 {
        return Some(InstructionKind::SWPAL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38200000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDADDB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38201000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDCLRB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38202000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDEORB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38203000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSETB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38204000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMAXB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38205000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMINB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38206000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMAXB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38207000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMINB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38600000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDADDLB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38601000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDCLRLB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38602000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDEORLB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38603000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSETLB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38604000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMAXLB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38605000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMINLB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38606000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMAXLB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x38607000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMINLB32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78200000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDADDH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78201000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDCLRH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78202000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDEORH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78203000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSETH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78204000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMAXH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78205000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMINH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78206000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMAXH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78207000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMINH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78600000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDADDLH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78601000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDCLRLH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78602000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDEORLH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78603000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSETLH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78604000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMAXLH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78605000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMINLH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78606000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMAXLH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x78607000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMINLH32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8200000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDADD32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8201000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDCLR32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8202000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDEOR32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8203000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSET32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8204000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMAX32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8205000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMIN32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8206000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMAX32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8207000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMIN32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8600000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDADDL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8601000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDCLRL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8602000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDEORL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8603000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSETL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8604000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMAXL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8605000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMINL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8606000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMAXL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xb8607000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMINL32Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8200000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDADD64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8201000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDCLR64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8202000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDEOR64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8203000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSET64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8204000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMAX64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8205000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMIN64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8206000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMAX64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8207000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMIN64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8600000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDADDL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8601000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDCLRL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8602000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDEORL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8603000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSETL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8604000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMAXL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8605000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDSMINL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8606000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMAXL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xf8607000 && (d & 0x00001f) != 0x00001f {
        return Some(InstructionKind::LDUMINL64Memop {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    None
}
pub const fn decode_loadlit(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x3b000000) != 0x18000000) as usize];
    let Rt = d & 0x1F;
    let V = (d >> 26) & 1;
    let imm19 = (d >> 5) & 0x7FFFF;
    let opc = (d >> 30) & 3;
    if (d & 0xff000000) == 0x18000000 {
        return Some(InstructionKind::LDR32Loadlit {
            Rt: Rt as _,
            imm19: imm19 as _,
        });
    }
    if (d & 0xff000000) == 0x1c000000 {
        return Some(InstructionKind::LDRSLoadlit {
            Rt: Rt as _,
            imm19: imm19 as _,
        });
    }
    if (d & 0xff000000) == 0x58000000 {
        return Some(InstructionKind::LDR64Loadlit {
            Rt: Rt as _,
            imm19: imm19 as _,
        });
    }
    if (d & 0xff000000) == 0x5c000000 {
        return Some(InstructionKind::LDRDLoadlit {
            Rt: Rt as _,
            imm19: imm19 as _,
        });
    }
    if (d & 0xff000000) == 0x98000000 {
        return Some(InstructionKind::LDRSW64Loadlit {
            Rt: Rt as _,
            imm19: imm19 as _,
        });
    }
    if (d & 0xff000000) == 0x9c000000 {
        return Some(InstructionKind::LDRQLoadlit {
            Rt: Rt as _,
            imm19: imm19 as _,
        });
    }
    if (d & 0xff000000) == 0xd8000000 {
        return Some(InstructionKind::PRFMPLoadlit {
            Rt: Rt as _,
            imm19: imm19 as _,
        });
    }
    None
}
pub const fn decode_ldstexcl(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x3f000000) != 0x08000000) as usize];
    let L = (d >> 22) & 1;
    let Rn = (d >> 5) & 0x1F;
    let Rs = (d >> 16) & 0x1F;
    let Rt = d & 0x1F;
    let Rt2 = (d >> 10) & 0x1F;
    let o0 = (d >> 15) & 1;
    let o1 = (d >> 21) & 1;
    let o2 = (d >> 23) & 1;
    let size = (d >> 30) & 3;
    if (d & 0xffe0fc00) == 0x08207c00 {
        return Some(InstructionKind::CASPCp32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x0820fc00 {
        return Some(InstructionKind::CASPLCp32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x08607c00 {
        return Some(InstructionKind::CASPACp32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x0860fc00 {
        return Some(InstructionKind::CASPALCp32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x08a07c00 {
        return Some(InstructionKind::CASBC32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x08a0fc00 {
        return Some(InstructionKind::CASLBC32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x08e07c00 {
        return Some(InstructionKind::CASABC32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x08e0fc00 {
        return Some(InstructionKind::CASALBC32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x48207c00 {
        return Some(InstructionKind::CASPCp64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x4820fc00 {
        return Some(InstructionKind::CASPLCp64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x48607c00 {
        return Some(InstructionKind::CASPACp64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x4860fc00 {
        return Some(InstructionKind::CASPALCp64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x48a07c00 {
        return Some(InstructionKind::CASHC32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x48a0fc00 {
        return Some(InstructionKind::CASLHC32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x48e07c00 {
        return Some(InstructionKind::CASAHC32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x48e0fc00 {
        return Some(InstructionKind::CASALHC32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x88a07c00 {
        return Some(InstructionKind::CASC32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x88a0fc00 {
        return Some(InstructionKind::CASLC32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x88e07c00 {
        return Some(InstructionKind::CASAC32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0x88e0fc00 {
        return Some(InstructionKind::CASALC32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xc8a07c00 {
        return Some(InstructionKind::CASC64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xc8a0fc00 {
        return Some(InstructionKind::CASLC64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xc8e07c00 {
        return Some(InstructionKind::CASAC64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe0fc00) == 0xc8e0fc00 {
        return Some(InstructionKind::CASALC64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
        });
    }
    if (d & 0xffe08000) == 0x08000000 {
        return Some(InstructionKind::STXRBSr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x08008000 {
        return Some(InstructionKind::STLXRBSr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x08400000 {
        return Some(InstructionKind::LDXRBLr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x08408000 {
        return Some(InstructionKind::LDAXRBLr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x08800000 {
        return Some(InstructionKind::STLLRBSl32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x08808000 {
        return Some(InstructionKind::STLRBSl32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x08c00000 {
        return Some(InstructionKind::LDLARBLr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x08c08000 {
        return Some(InstructionKind::LDARBLr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x48000000 {
        return Some(InstructionKind::STXRHSr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x48008000 {
        return Some(InstructionKind::STLXRHSr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x48400000 {
        return Some(InstructionKind::LDXRHLr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x48408000 {
        return Some(InstructionKind::LDAXRHLr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x48800000 {
        return Some(InstructionKind::STLLRHSl32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x48808000 {
        return Some(InstructionKind::STLRHSl32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x48c00000 {
        return Some(InstructionKind::LDLARHLr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x48c08000 {
        return Some(InstructionKind::LDARHLr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x88000000 {
        return Some(InstructionKind::STXRSr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x88008000 {
        return Some(InstructionKind::STLXRSr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x88200000 {
        return Some(InstructionKind::STXPSp32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x88208000 {
        return Some(InstructionKind::STLXPSp32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x88400000 {
        return Some(InstructionKind::LDXRLr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x88408000 {
        return Some(InstructionKind::LDAXRLr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x88600000 {
        return Some(InstructionKind::LDXPLp32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x88608000 {
        return Some(InstructionKind::LDAXPLp32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x88800000 {
        return Some(InstructionKind::STLLRSl32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x88808000 {
        return Some(InstructionKind::STLRSl32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x88c00000 {
        return Some(InstructionKind::LDLARLr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0x88c08000 {
        return Some(InstructionKind::LDARLr32Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0xc8000000 {
        return Some(InstructionKind::STXRSr64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0xc8008000 {
        return Some(InstructionKind::STLXRSr64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0xc8200000 {
        return Some(InstructionKind::STXPSp64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0xc8208000 {
        return Some(InstructionKind::STLXPSp64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0xc8400000 {
        return Some(InstructionKind::LDXRLr64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0xc8408000 {
        return Some(InstructionKind::LDAXRLr64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0xc8600000 {
        return Some(InstructionKind::LDXPLp64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0xc8608000 {
        return Some(InstructionKind::LDAXPLp64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0xc8800000 {
        return Some(InstructionKind::STLLRSl64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0xc8808000 {
        return Some(InstructionKind::STLRSl64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0xc8c00000 {
        return Some(InstructionKind::LDLARLr64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    if (d & 0xffe08000) == 0xc8c08000 {
        return Some(InstructionKind::LDARLr64Ldstexcl {
            Rn: Rn as _,
            Rs: Rs as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
        });
    }
    None
}
pub const fn decode_ldstnapair_offs(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x3b800000) != 0x28000000) as usize];
    let L = (d >> 22) & 1;
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let Rt2 = (d >> 10) & 0x1F;
    let V = (d >> 26) & 1;
    let imm7 = (d >> 15) & 0x7F;
    let opc = (d >> 30) & 3;
    if (d & 0xffc00000) == 0x28000000 {
        return Some(InstructionKind::STNP32LdstnapairOffs {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x28400000 {
        return Some(InstructionKind::LDNP32LdstnapairOffs {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x2c000000 {
        return Some(InstructionKind::STNPSLdstnapairOffs {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x2c400000 {
        return Some(InstructionKind::LDNPSLdstnapairOffs {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x6c000000 {
        return Some(InstructionKind::STNPDLdstnapairOffs {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x6c400000 {
        return Some(InstructionKind::LDNPDLdstnapairOffs {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xa8000000 {
        return Some(InstructionKind::STNP64LdstnapairOffs {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xa8400000 {
        return Some(InstructionKind::LDNP64LdstnapairOffs {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xac000000 {
        return Some(InstructionKind::STNPQLdstnapairOffs {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xac400000 {
        return Some(InstructionKind::LDNPQLdstnapairOffs {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    None
}
pub const fn decode_ldst_immpost(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x3b200c00) != 0x38000400) as usize];
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let V = (d >> 26) & 1;
    let imm9 = (d >> 12) & 0x1FF;
    let opc = (d >> 22) & 3;
    let size = (d >> 30) & 3;
    if (d & 0xffe00c00) == 0x38000400 {
        return Some(InstructionKind::STRB32LdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x38400400 {
        return Some(InstructionKind::LDRB32LdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x38800400 {
        return Some(InstructionKind::LDRSB64LdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x38c00400 {
        return Some(InstructionKind::LDRSB32LdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x3c000400 {
        return Some(InstructionKind::STRBLdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x3c400400 {
        return Some(InstructionKind::LDRBLdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x3c800400 {
        return Some(InstructionKind::STRQLdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x3cc00400 {
        return Some(InstructionKind::LDRQLdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78000400 {
        return Some(InstructionKind::STRH32LdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78400400 {
        return Some(InstructionKind::LDRH32LdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78800400 {
        return Some(InstructionKind::LDRSH64LdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78c00400 {
        return Some(InstructionKind::LDRSH32LdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x7c000400 {
        return Some(InstructionKind::STRHLdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x7c400400 {
        return Some(InstructionKind::LDRHLdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8000400 {
        return Some(InstructionKind::STR32LdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8400400 {
        return Some(InstructionKind::LDR32LdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8800400 {
        return Some(InstructionKind::LDRSW64LdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xbc000400 {
        return Some(InstructionKind::STRSLdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xbc400400 {
        return Some(InstructionKind::LDRSLdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xf8000400 {
        return Some(InstructionKind::STR64LdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xf8400400 {
        return Some(InstructionKind::LDR64LdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xfc000400 {
        return Some(InstructionKind::STRDLdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xfc400400 {
        return Some(InstructionKind::LDRDLdstImmpost {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    None
}
pub const fn decode_ldst_immpre(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x3b200c00) != 0x38000c00) as usize];
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let V = (d >> 26) & 1;
    let imm9 = (d >> 12) & 0x1FF;
    let opc = (d >> 22) & 3;
    let size = (d >> 30) & 3;
    if (d & 0xffe00c00) == 0x38000c00 {
        return Some(InstructionKind::STRB32LdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x38400c00 {
        return Some(InstructionKind::LDRB32LdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x38800c00 {
        return Some(InstructionKind::LDRSB64LdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x38c00c00 {
        return Some(InstructionKind::LDRSB32LdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x3c000c00 {
        return Some(InstructionKind::STRBLdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x3c400c00 {
        return Some(InstructionKind::LDRBLdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x3c800c00 {
        return Some(InstructionKind::STRQLdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x3cc00c00 {
        return Some(InstructionKind::LDRQLdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78000c00 {
        return Some(InstructionKind::STRH32LdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78400c00 {
        return Some(InstructionKind::LDRH32LdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78800c00 {
        return Some(InstructionKind::LDRSH64LdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78c00c00 {
        return Some(InstructionKind::LDRSH32LdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x7c000c00 {
        return Some(InstructionKind::STRHLdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x7c400c00 {
        return Some(InstructionKind::LDRHLdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8000c00 {
        return Some(InstructionKind::STR32LdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8400c00 {
        return Some(InstructionKind::LDR32LdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8800c00 {
        return Some(InstructionKind::LDRSW64LdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xbc000c00 {
        return Some(InstructionKind::STRSLdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xbc400c00 {
        return Some(InstructionKind::LDRSLdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xf8000c00 {
        return Some(InstructionKind::STR64LdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xf8400c00 {
        return Some(InstructionKind::LDR64LdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xfc000c00 {
        return Some(InstructionKind::STRDLdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xfc400c00 {
        return Some(InstructionKind::LDRDLdstImmpre {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    None
}
pub const fn decode_ldst_pac(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x3b200400) != 0x38200400) as usize];
    let M = (d >> 23) & 1;
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let S = (d >> 22) & 1;
    let V = (d >> 26) & 1;
    let W = (d >> 11) & 1;
    let imm9 = (d >> 12) & 0x1FF;
    let size = (d >> 30) & 3;
    if (d & 0xfba00c00) == 0xf8200400 {
        return Some(InstructionKind::LDRAA64LdstPac {
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            V: V as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xfba00c00) == 0xf8200c00 {
        return Some(InstructionKind::LDRAA64WLdstPac {
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            V: V as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xfba00c00) == 0xf8a00400 {
        return Some(InstructionKind::LDRAB64LdstPac {
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            V: V as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xfba00c00) == 0xf8a00c00 {
        return Some(InstructionKind::LDRAB64WLdstPac {
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            V: V as _,
            imm9: imm9 as _,
        });
    }
    None
}
pub const fn decode_ldst_regoff(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x3b200c00) != 0x38200800) as usize];
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let S = (d >> 12) & 1;
    let V = (d >> 26) & 1;
    let opc = (d >> 22) & 3;
    let option = (d >> 13) & 7;
    let size = (d >> 30) & 3;
    if (d & 0xffe0ec00) == 0x38206800 {
        return Some(InstructionKind::STRB32BlLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xffe0ec00) == 0x38606800 {
        return Some(InstructionKind::LDRB32BlLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xffe0ec00) == 0x38a06800 {
        return Some(InstructionKind::LDRSB64BlLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xffe0ec00) == 0x38e06800 {
        return Some(InstructionKind::LDRSB32BlLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xffe0ec00) == 0x3c206800 {
        return Some(InstructionKind::STRBlLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xffe0ec00) == 0x3c606800 {
        return Some(InstructionKind::LDRBlLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
        });
    }
    if (d & 0xffe00c00) == 0x3ca00800 {
        return Some(InstructionKind::STRQLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0x3ce00800 {
        return Some(InstructionKind::LDRQLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0x78200800 {
        return Some(InstructionKind::STRH32LdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0x78600800 {
        return Some(InstructionKind::LDRH32LdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0x78a00800 {
        return Some(InstructionKind::LDRSH64LdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0x78e00800 {
        return Some(InstructionKind::LDRSH32LdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0x7c200800 {
        return Some(InstructionKind::STRHLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0x7c600800 {
        return Some(InstructionKind::LDRHLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8200800 {
        return Some(InstructionKind::STR32LdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8600800 {
        return Some(InstructionKind::LDR32LdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8a00800 {
        return Some(InstructionKind::LDRSW64LdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0xbc200800 {
        return Some(InstructionKind::STRSLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0xbc600800 {
        return Some(InstructionKind::LDRSLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0xf8200800 {
        return Some(InstructionKind::STR64LdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0xf8600800 {
        return Some(InstructionKind::LDR64LdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0xf8a00800 {
        return Some(InstructionKind::PRFMPLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0xfc200800 {
        return Some(InstructionKind::STRDLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0xfc600800 {
        return Some(InstructionKind::LDRDLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0x38200800 && (d & 0x00e000) != 0x006000 {
        return Some(InstructionKind::STRB32BLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0x38600800 && (d & 0x00e000) != 0x006000 {
        return Some(InstructionKind::LDRB32BLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0x38a00800 && (d & 0x00e000) != 0x006000 {
        return Some(InstructionKind::LDRSB64BLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0x38e00800 && (d & 0x00e000) != 0x006000 {
        return Some(InstructionKind::LDRSB32BLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0x3c200800 && (d & 0x00e000) != 0x006000 {
        return Some(InstructionKind::STRBLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    if (d & 0xffe00c00) == 0x3c600800 && (d & 0x00e000) != 0x006000 {
        return Some(InstructionKind::LDRBLdstRegoff {
            Rm: Rm as _,
            Rn: Rn as _,
            Rt: Rt as _,
            S: S as _,
            option: option as _,
        });
    }
    None
}
pub const fn decode_ldst_unpriv(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x3b200c00) != 0x38000800) as usize];
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let V = (d >> 26) & 1;
    let imm9 = (d >> 12) & 0x1FF;
    let opc = (d >> 22) & 3;
    let size = (d >> 30) & 3;
    if (d & 0xffe00c00) == 0x38000800 {
        return Some(InstructionKind::STTRB32LdstUnpriv {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x38400800 {
        return Some(InstructionKind::LDTRB32LdstUnpriv {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x38800800 {
        return Some(InstructionKind::LDTRSB64LdstUnpriv {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x38c00800 {
        return Some(InstructionKind::LDTRSB32LdstUnpriv {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78000800 {
        return Some(InstructionKind::STTRH32LdstUnpriv {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78400800 {
        return Some(InstructionKind::LDTRH32LdstUnpriv {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78800800 {
        return Some(InstructionKind::LDTRSH64LdstUnpriv {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78c00800 {
        return Some(InstructionKind::LDTRSH32LdstUnpriv {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8000800 {
        return Some(InstructionKind::STTR32LdstUnpriv {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8400800 {
        return Some(InstructionKind::LDTR32LdstUnpriv {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8800800 {
        return Some(InstructionKind::LDTRSW64LdstUnpriv {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xf8000800 {
        return Some(InstructionKind::STTR64LdstUnpriv {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xf8400800 {
        return Some(InstructionKind::LDTR64LdstUnpriv {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    None
}
pub const fn decode_ldst_unscaled(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x3b200c00) != 0x38000000) as usize];
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let V = (d >> 26) & 1;
    let imm9 = (d >> 12) & 0x1FF;
    let opc = (d >> 22) & 3;
    let size = (d >> 30) & 3;
    if (d & 0xffe00c00) == 0x38000000 {
        return Some(InstructionKind::STURB32LdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x38400000 {
        return Some(InstructionKind::LDURB32LdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x38800000 {
        return Some(InstructionKind::LDURSB64LdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x38c00000 {
        return Some(InstructionKind::LDURSB32LdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x3c000000 {
        return Some(InstructionKind::STURBLdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x3c400000 {
        return Some(InstructionKind::LDURBLdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x3c800000 {
        return Some(InstructionKind::STURQLdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x3cc00000 {
        return Some(InstructionKind::LDURQLdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78000000 {
        return Some(InstructionKind::STURH32LdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78400000 {
        return Some(InstructionKind::LDURH32LdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78800000 {
        return Some(InstructionKind::LDURSH64LdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x78c00000 {
        return Some(InstructionKind::LDURSH32LdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x7c000000 {
        return Some(InstructionKind::STURHLdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0x7c400000 {
        return Some(InstructionKind::LDURHLdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8000000 {
        return Some(InstructionKind::STUR32LdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8400000 {
        return Some(InstructionKind::LDUR32LdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xb8800000 {
        return Some(InstructionKind::LDURSW64LdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xbc000000 {
        return Some(InstructionKind::STURSLdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xbc400000 {
        return Some(InstructionKind::LDURSLdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xf8000000 {
        return Some(InstructionKind::STUR64LdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xf8400000 {
        return Some(InstructionKind::LDUR64LdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xf8800000 {
        return Some(InstructionKind::PRFUMPLdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xfc000000 {
        return Some(InstructionKind::STURDLdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    if (d & 0xffe00c00) == 0xfc400000 {
        return Some(InstructionKind::LDURDLdstUnscaled {
            Rn: Rn as _,
            Rt: Rt as _,
            imm9: imm9 as _,
        });
    }
    None
}
pub const fn decode_ldst_pos(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x3b000000) != 0x39000000) as usize];
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let V = (d >> 26) & 1;
    let imm12 = (d >> 10) & 0xFFF;
    let opc = (d >> 22) & 3;
    let size = (d >> 30) & 3;
    if (d & 0xffc00000) == 0x39000000 {
        return Some(InstructionKind::STRB32LdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0x39400000 {
        return Some(InstructionKind::LDRB32LdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0x39800000 {
        return Some(InstructionKind::LDRSB64LdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0x39c00000 {
        return Some(InstructionKind::LDRSB32LdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0x3d000000 {
        return Some(InstructionKind::STRBLdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0x3d400000 {
        return Some(InstructionKind::LDRBLdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0x3d800000 {
        return Some(InstructionKind::STRQLdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0x3dc00000 {
        return Some(InstructionKind::LDRQLdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0x79000000 {
        return Some(InstructionKind::STRH32LdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0x79400000 {
        return Some(InstructionKind::LDRH32LdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0x79800000 {
        return Some(InstructionKind::LDRSH64LdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0x79c00000 {
        return Some(InstructionKind::LDRSH32LdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0x7d000000 {
        return Some(InstructionKind::STRHLdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0x7d400000 {
        return Some(InstructionKind::LDRHLdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0xb9000000 {
        return Some(InstructionKind::STR32LdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0xb9400000 {
        return Some(InstructionKind::LDR32LdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0xb9800000 {
        return Some(InstructionKind::LDRSW64LdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0xbd000000 {
        return Some(InstructionKind::STRSLdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0xbd400000 {
        return Some(InstructionKind::LDRSLdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0xf9000000 {
        return Some(InstructionKind::STR64LdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0xf9400000 {
        return Some(InstructionKind::LDR64LdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0xf9800000 {
        return Some(InstructionKind::PRFMPLdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0xfd000000 {
        return Some(InstructionKind::STRDLdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    if (d & 0xffc00000) == 0xfd400000 {
        return Some(InstructionKind::LDRDLdstPos {
            Rn: Rn as _,
            Rt: Rt as _,
            imm12: imm12 as _,
        });
    }
    None
}
pub const fn decode_ldstpair_off(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x3b800000) != 0x29000000) as usize];
    let L = (d >> 22) & 1;
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let Rt2 = (d >> 10) & 0x1F;
    let V = (d >> 26) & 1;
    let imm7 = (d >> 15) & 0x7F;
    let opc = (d >> 30) & 3;
    if (d & 0xffc00000) == 0x29000000 {
        return Some(InstructionKind::STP32LdstpairOff {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x29400000 {
        return Some(InstructionKind::LDP32LdstpairOff {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x2d000000 {
        return Some(InstructionKind::STPSLdstpairOff {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x2d400000 {
        return Some(InstructionKind::LDPSLdstpairOff {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x69400000 {
        return Some(InstructionKind::LDPSW64LdstpairOff {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x6d000000 {
        return Some(InstructionKind::STPDLdstpairOff {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x6d400000 {
        return Some(InstructionKind::LDPDLdstpairOff {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xa9000000 {
        return Some(InstructionKind::STP64LdstpairOff {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xa9400000 {
        return Some(InstructionKind::LDP64LdstpairOff {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xad000000 {
        return Some(InstructionKind::STPQLdstpairOff {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xad400000 {
        return Some(InstructionKind::LDPQLdstpairOff {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    None
}
pub const fn decode_ldstpair_post(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x3b800000) != 0x28800000) as usize];
    let L = (d >> 22) & 1;
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let Rt2 = (d >> 10) & 0x1F;
    let V = (d >> 26) & 1;
    let imm7 = (d >> 15) & 0x7F;
    let opc = (d >> 30) & 3;
    if (d & 0xffc00000) == 0x28800000 {
        return Some(InstructionKind::STP32LdstpairPost {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x28c00000 {
        return Some(InstructionKind::LDP32LdstpairPost {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x2c800000 {
        return Some(InstructionKind::STPSLdstpairPost {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x2cc00000 {
        return Some(InstructionKind::LDPSLdstpairPost {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x68c00000 {
        return Some(InstructionKind::LDPSW64LdstpairPost {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x6c800000 {
        return Some(InstructionKind::STPDLdstpairPost {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x6cc00000 {
        return Some(InstructionKind::LDPDLdstpairPost {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xa8800000 {
        return Some(InstructionKind::STP64LdstpairPost {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xa8c00000 {
        return Some(InstructionKind::LDP64LdstpairPost {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xac800000 {
        return Some(InstructionKind::STPQLdstpairPost {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xacc00000 {
        return Some(InstructionKind::LDPQLdstpairPost {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    None
}
pub const fn decode_ldstpair_pre(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x3b800000) != 0x29800000) as usize];
    let L = (d >> 22) & 1;
    let Rn = (d >> 5) & 0x1F;
    let Rt = d & 0x1F;
    let Rt2 = (d >> 10) & 0x1F;
    let V = (d >> 26) & 1;
    let imm7 = (d >> 15) & 0x7F;
    let opc = (d >> 30) & 3;
    if (d & 0xffc00000) == 0x29800000 {
        return Some(InstructionKind::STP32LdstpairPre {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x29c00000 {
        return Some(InstructionKind::LDP32LdstpairPre {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x2d800000 {
        return Some(InstructionKind::STPSLdstpairPre {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x2dc00000 {
        return Some(InstructionKind::LDPSLdstpairPre {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x69c00000 {
        return Some(InstructionKind::LDPSW64LdstpairPre {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x6d800000 {
        return Some(InstructionKind::STPDLdstpairPre {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0x6dc00000 {
        return Some(InstructionKind::LDPDLdstpairPre {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xa9800000 {
        return Some(InstructionKind::STP64LdstpairPre {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xa9c00000 {
        return Some(InstructionKind::LDP64LdstpairPre {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xad800000 {
        return Some(InstructionKind::STPQLdstpairPre {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    if (d & 0xffc00000) == 0xadc00000 {
        return Some(InstructionKind::LDPQLdstpairPre {
            Rn: Rn as _,
            Rt: Rt as _,
            Rt2: Rt2 as _,
            imm7: imm7 as _,
        });
    }
    None
}
pub const fn decode_addsub_imm(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1f000000) != 0x11000000) as usize];
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let imm12 = (d >> 10) & 0xFFF;
    let op = (d >> 30) & 1;
    let sf = (d >> 31) & 1;
    let shift = (d >> 22) & 3;
    if (d & 0xff000000) == 0x11000000 {
        return Some(InstructionKind::ADD32AddsubImm {
            Rd: Rd as _,
            Rn: Rn as _,
            imm12: imm12 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff000000) == 0x31000000 {
        return Some(InstructionKind::ADDS32SAddsubImm {
            Rd: Rd as _,
            Rn: Rn as _,
            imm12: imm12 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff000000) == 0x51000000 {
        return Some(InstructionKind::SUB32AddsubImm {
            Rd: Rd as _,
            Rn: Rn as _,
            imm12: imm12 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff000000) == 0x71000000 {
        return Some(InstructionKind::SUBS32SAddsubImm {
            Rd: Rd as _,
            Rn: Rn as _,
            imm12: imm12 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff000000) == 0x91000000 {
        return Some(InstructionKind::ADD64AddsubImm {
            Rd: Rd as _,
            Rn: Rn as _,
            imm12: imm12 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff000000) == 0xb1000000 {
        return Some(InstructionKind::ADDS64SAddsubImm {
            Rd: Rd as _,
            Rn: Rn as _,
            imm12: imm12 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff000000) == 0xd1000000 {
        return Some(InstructionKind::SUB64AddsubImm {
            Rd: Rd as _,
            Rn: Rn as _,
            imm12: imm12 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff000000) == 0xf1000000 {
        return Some(InstructionKind::SUBS64SAddsubImm {
            Rd: Rd as _,
            Rn: Rn as _,
            imm12: imm12 as _,
            shift: shift as _,
        });
    }
    None
}
pub const fn decode_bitfield(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1f800000) != 0x13000000) as usize];
    let N = (d >> 22) & 1;
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let immr = (d >> 16) & 0x3F;
    let imms = (d >> 10) & 0x3F;
    let opc = (d >> 29) & 3;
    let sf = (d >> 31) & 1;
    if (d & 0xffc00000) == 0x13000000 {
        return Some(InstructionKind::SBFM32MBitfield {
            Rd: Rd as _,
            Rn: Rn as _,
            immr: immr as _,
            imms: imms as _,
        });
    }
    if (d & 0xffc00000) == 0x33000000 {
        return Some(InstructionKind::BFM32MBitfield {
            Rd: Rd as _,
            Rn: Rn as _,
            immr: immr as _,
            imms: imms as _,
        });
    }
    if (d & 0xffc00000) == 0x53000000 {
        return Some(InstructionKind::UBFM32MBitfield {
            Rd: Rd as _,
            Rn: Rn as _,
            immr: immr as _,
            imms: imms as _,
        });
    }
    if (d & 0xffc00000) == 0x93400000 {
        return Some(InstructionKind::SBFM64MBitfield {
            Rd: Rd as _,
            Rn: Rn as _,
            immr: immr as _,
            imms: imms as _,
        });
    }
    if (d & 0xffc00000) == 0xb3400000 {
        return Some(InstructionKind::BFM64MBitfield {
            Rd: Rd as _,
            Rn: Rn as _,
            immr: immr as _,
            imms: imms as _,
        });
    }
    if (d & 0xffc00000) == 0xd3400000 {
        return Some(InstructionKind::UBFM64MBitfield {
            Rd: Rd as _,
            Rn: Rn as _,
            immr: immr as _,
            imms: imms as _,
        });
    }
    None
}
pub const fn decode_extract(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1f800000) != 0x13800000) as usize];
    let N = (d >> 22) & 1;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let imms = (d >> 10) & 0x3F;
    let o0 = (d >> 21) & 1;
    let op21 = (d >> 29) & 3;
    let sf = (d >> 31) & 1;
    if (d & 0xffe08000) == 0x13800000 {
        return Some(InstructionKind::EXTR32Extract {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imms: imms as _,
        });
    }
    if (d & 0xffe00000) == 0x93c00000 {
        return Some(InstructionKind::EXTR64Extract {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imms: imms as _,
        });
    }
    None
}
pub const fn decode_log_imm(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1f800000) != 0x12000000) as usize];
    let N = (d >> 22) & 1;
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let immr = (d >> 16) & 0x3F;
    let imms = (d >> 10) & 0x3F;
    let opc = (d >> 29) & 3;
    let sf = (d >> 31) & 1;
    if (d & 0xffc00000) == 0x12000000 {
        return Some(InstructionKind::AND32LogImm {
            Rd: Rd as _,
            Rn: Rn as _,
            immr: immr as _,
            imms: imms as _,
        });
    }
    if (d & 0xffc00000) == 0x32000000 {
        return Some(InstructionKind::ORR32LogImm {
            Rd: Rd as _,
            Rn: Rn as _,
            immr: immr as _,
            imms: imms as _,
        });
    }
    if (d & 0xffc00000) == 0x52000000 {
        return Some(InstructionKind::EOR32LogImm {
            Rd: Rd as _,
            Rn: Rn as _,
            immr: immr as _,
            imms: imms as _,
        });
    }
    if (d & 0xffc00000) == 0x72000000 {
        return Some(InstructionKind::ANDS32SLogImm {
            Rd: Rd as _,
            Rn: Rn as _,
            immr: immr as _,
            imms: imms as _,
        });
    }
    if (d & 0xff800000) == 0x92000000 {
        return Some(InstructionKind::AND64LogImm {
            N: N as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immr: immr as _,
            imms: imms as _,
        });
    }
    if (d & 0xff800000) == 0xb2000000 {
        return Some(InstructionKind::ORR64LogImm {
            N: N as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immr: immr as _,
            imms: imms as _,
        });
    }
    if (d & 0xff800000) == 0xd2000000 {
        return Some(InstructionKind::EOR64LogImm {
            N: N as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immr: immr as _,
            imms: imms as _,
        });
    }
    if (d & 0xff800000) == 0xf2000000 {
        return Some(InstructionKind::ANDS64SLogImm {
            N: N as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immr: immr as _,
            imms: imms as _,
        });
    }
    None
}
pub const fn decode_movewide(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1f800000) != 0x12800000) as usize];
    let Rd = d & 0x1F;
    let hw = (d >> 21) & 3;
    let imm16 = (d >> 5) & 0xFFFF;
    let opc = (d >> 29) & 3;
    let sf = (d >> 31) & 1;
    if (d & 0xff800000) == 0x12800000 {
        return Some(InstructionKind::MOVN32Movewide {
            Rd: Rd as _,
            hw: hw as _,
            imm16: imm16 as _,
        });
    }
    if (d & 0xff800000) == 0x52800000 {
        return Some(InstructionKind::MOVZ32Movewide {
            Rd: Rd as _,
            hw: hw as _,
            imm16: imm16 as _,
        });
    }
    if (d & 0xff800000) == 0x72800000 {
        return Some(InstructionKind::MOVK32Movewide {
            Rd: Rd as _,
            hw: hw as _,
            imm16: imm16 as _,
        });
    }
    if (d & 0xff800000) == 0x92800000 {
        return Some(InstructionKind::MOVN64Movewide {
            Rd: Rd as _,
            hw: hw as _,
            imm16: imm16 as _,
        });
    }
    if (d & 0xff800000) == 0xd2800000 {
        return Some(InstructionKind::MOVZ64Movewide {
            Rd: Rd as _,
            hw: hw as _,
            imm16: imm16 as _,
        });
    }
    if (d & 0xff800000) == 0xf2800000 {
        return Some(InstructionKind::MOVK64Movewide {
            Rd: Rd as _,
            hw: hw as _,
            imm16: imm16 as _,
        });
    }
    None
}
pub const fn decode_pcreladdr(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1f000000) != 0x10000000) as usize];
    let Rd = d & 0x1F;
    let immhi = (d >> 5) & 0x7FFFF;
    let immlo = (d >> 29) & 3;
    let op = (d >> 31) & 1;
    if (d & 0x9f000000) == 0x10000000 {
        return Some(InstructionKind::ADROnlyPcreladdr {
            Rd: Rd as _,
            immhi: immhi as _,
            immlo: immlo as _,
        });
    }
    if (d & 0x9f000000) == 0x90000000 {
        return Some(InstructionKind::ADRPOnlyPcreladdr {
            Rd: Rd as _,
            immhi: immhi as _,
            immlo: immlo as _,
        });
    }
    None
}
pub const fn decode_addsub_ext(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1f200000) != 0x0b200000) as usize];
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let imm3 = (d >> 10) & 7;
    let op = (d >> 30) & 1;
    let opt = (d >> 22) & 3;
    let option = (d >> 13) & 7;
    let sf = (d >> 31) & 1;
    if (d & 0xffe00000) == 0x0b200000 {
        return Some(InstructionKind::ADD32AddsubExt {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm3: imm3 as _,
            option: option as _,
        });
    }
    if (d & 0xffe00000) == 0x2b200000 {
        return Some(InstructionKind::ADDS32SAddsubExt {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm3: imm3 as _,
            option: option as _,
        });
    }
    if (d & 0xffe00000) == 0x4b200000 {
        return Some(InstructionKind::SUB32AddsubExt {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm3: imm3 as _,
            option: option as _,
        });
    }
    if (d & 0xffe00000) == 0x6b200000 {
        return Some(InstructionKind::SUBS32SAddsubExt {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm3: imm3 as _,
            option: option as _,
        });
    }
    if (d & 0xffe00000) == 0x8b200000 {
        return Some(InstructionKind::ADD64AddsubExt {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm3: imm3 as _,
            option: option as _,
        });
    }
    if (d & 0xffe00000) == 0xab200000 {
        return Some(InstructionKind::ADDS64SAddsubExt {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm3: imm3 as _,
            option: option as _,
        });
    }
    if (d & 0xffe00000) == 0xcb200000 {
        return Some(InstructionKind::SUB64AddsubExt {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm3: imm3 as _,
            option: option as _,
        });
    }
    if (d & 0xffe00000) == 0xeb200000 {
        return Some(InstructionKind::SUBS64SAddsubExt {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm3: imm3 as _,
            option: option as _,
        });
    }
    None
}
pub const fn decode_addsub_shift(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1f200000) != 0x0b000000) as usize];
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let imm6 = (d >> 10) & 0x3F;
    let op = (d >> 30) & 1;
    let sf = (d >> 31) & 1;
    let shift = (d >> 22) & 3;
    if (d & 0xff200000) == 0x0b000000 {
        return Some(InstructionKind::ADD32AddsubShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0x2b000000 {
        return Some(InstructionKind::ADDS32AddsubShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0x4b000000 {
        return Some(InstructionKind::SUB32AddsubShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0x6b000000 {
        return Some(InstructionKind::SUBS32AddsubShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0x8b000000 {
        return Some(InstructionKind::ADD64AddsubShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0xab000000 {
        return Some(InstructionKind::ADDS64AddsubShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0xcb000000 {
        return Some(InstructionKind::SUB64AddsubShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0xeb000000 {
        return Some(InstructionKind::SUBS64AddsubShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    None
}
pub const fn decode_addsub_carry(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1fe00000) != 0x1a000000) as usize];
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let op = (d >> 30) & 1;
    let opcode2 = (d >> 10) & 0x3F;
    let sf = (d >> 31) & 1;
    if (d & 0xffe0fc00) == 0x1a000000 {
        return Some(InstructionKind::ADC32AddsubCarry {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x3a000000 {
        return Some(InstructionKind::ADCS32AddsubCarry {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x5a000000 {
        return Some(InstructionKind::SBC32AddsubCarry {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x7a000000 {
        return Some(InstructionKind::SBCS32AddsubCarry {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x9a000000 {
        return Some(InstructionKind::ADC64AddsubCarry {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0xba000000 {
        return Some(InstructionKind::ADCS64AddsubCarry {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0xda000000 {
        return Some(InstructionKind::SBC64AddsubCarry {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0xfa000000 {
        return Some(InstructionKind::SBCS64AddsubCarry {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_condcmp_imm(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1fe00800) != 0x1a400800) as usize];
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let cond = (d >> 12) & 0xF;
    let imm5 = (d >> 16) & 0x1F;
    let nzcv = d & 0xF;
    let o2 = (d >> 10) & 1;
    let o3 = (d >> 4) & 1;
    let op = (d >> 30) & 1;
    let sf = (d >> 31) & 1;
    if (d & 0xffe00c10) == 0x3a400800 {
        return Some(InstructionKind::CCMN32CondcmpImm {
            Rn: Rn as _,
            cond: cond as _,
            imm5: imm5 as _,
            nzcv: nzcv as _,
        });
    }
    if (d & 0xffe00c10) == 0x7a400800 {
        return Some(InstructionKind::CCMP32CondcmpImm {
            Rn: Rn as _,
            cond: cond as _,
            imm5: imm5 as _,
            nzcv: nzcv as _,
        });
    }
    if (d & 0xffe00c10) == 0xba400800 {
        return Some(InstructionKind::CCMN64CondcmpImm {
            Rn: Rn as _,
            cond: cond as _,
            imm5: imm5 as _,
            nzcv: nzcv as _,
        });
    }
    if (d & 0xffe00c10) == 0xfa400800 {
        return Some(InstructionKind::CCMP64CondcmpImm {
            Rn: Rn as _,
            cond: cond as _,
            imm5: imm5 as _,
            nzcv: nzcv as _,
        });
    }
    None
}
pub const fn decode_condcmp_reg(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1fe00800) != 0x1a400000) as usize];
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let cond = (d >> 12) & 0xF;
    let nzcv = d & 0xF;
    let o2 = (d >> 10) & 1;
    let o3 = (d >> 4) & 1;
    let op = (d >> 30) & 1;
    let sf = (d >> 31) & 1;
    if (d & 0xffe00c10) == 0x3a400000 {
        return Some(InstructionKind::CCMN32CondcmpReg {
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
            nzcv: nzcv as _,
        });
    }
    if (d & 0xffe00c10) == 0x7a400000 {
        return Some(InstructionKind::CCMP32CondcmpReg {
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
            nzcv: nzcv as _,
        });
    }
    if (d & 0xffe00c10) == 0xba400000 {
        return Some(InstructionKind::CCMN64CondcmpReg {
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
            nzcv: nzcv as _,
        });
    }
    if (d & 0xffe00c10) == 0xfa400000 {
        return Some(InstructionKind::CCMP64CondcmpReg {
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
            nzcv: nzcv as _,
        });
    }
    None
}
pub const fn decode_condsel(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1fe00000) != 0x1a800000) as usize];
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let cond = (d >> 12) & 0xF;
    let op = (d >> 30) & 1;
    let op2 = (d >> 10) & 3;
    let sf = (d >> 31) & 1;
    if (d & 0xffe00c00) == 0x1a800000 {
        return Some(InstructionKind::CSEL32Condsel {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
        });
    }
    if (d & 0xffe00c00) == 0x1a800400 {
        return Some(InstructionKind::CSINC32Condsel {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
        });
    }
    if (d & 0xffe00c00) == 0x5a800000 {
        return Some(InstructionKind::CSINV32Condsel {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
        });
    }
    if (d & 0xffe00c00) == 0x5a800400 {
        return Some(InstructionKind::CSNEG32Condsel {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
        });
    }
    if (d & 0xffe00c00) == 0x9a800000 {
        return Some(InstructionKind::CSEL64Condsel {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
        });
    }
    if (d & 0xffe00c00) == 0x9a800400 {
        return Some(InstructionKind::CSINC64Condsel {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
        });
    }
    if (d & 0xffe00c00) == 0xda800000 {
        return Some(InstructionKind::CSINV64Condsel {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
        });
    }
    if (d & 0xffe00c00) == 0xda800400 {
        return Some(InstructionKind::CSNEG64Condsel {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
        });
    }
    None
}
pub const fn decode_dp_1src(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x5fe00000) != 0x5ac00000) as usize];
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let opcode = (d >> 10) & 0x3F;
    let opcode2 = (d >> 16) & 0x1F;
    let sf = (d >> 31) & 1;
    if (d & 0xffffffe0) == 0xdac123e0 {
        return Some(InstructionKind::PACIZA64ZDp1Src { Rd: Rd as _ });
    }
    if (d & 0xffffffe0) == 0xdac127e0 {
        return Some(InstructionKind::PACIZB64ZDp1Src { Rd: Rd as _ });
    }
    if (d & 0xffffffe0) == 0xdac12be0 {
        return Some(InstructionKind::PACDZA64ZDp1Src { Rd: Rd as _ });
    }
    if (d & 0xffffffe0) == 0xdac12fe0 {
        return Some(InstructionKind::PACDZB64ZDp1Src { Rd: Rd as _ });
    }
    if (d & 0xffffffe0) == 0xdac133e0 {
        return Some(InstructionKind::AUTIZA64ZDp1Src { Rd: Rd as _ });
    }
    if (d & 0xffffffe0) == 0xdac137e0 {
        return Some(InstructionKind::AUTIZB64ZDp1Src { Rd: Rd as _ });
    }
    if (d & 0xffffffe0) == 0xdac13be0 {
        return Some(InstructionKind::AUTDZA64ZDp1Src { Rd: Rd as _ });
    }
    if (d & 0xffffffe0) == 0xdac13fe0 {
        return Some(InstructionKind::AUTDZB64ZDp1Src { Rd: Rd as _ });
    }
    if (d & 0xffffffe0) == 0xdac143e0 {
        return Some(InstructionKind::XPACI64ZDp1Src { Rd: Rd as _ });
    }
    if (d & 0xffffffe0) == 0xdac147e0 {
        return Some(InstructionKind::XPACD64ZDp1Src { Rd: Rd as _ });
    }
    if (d & 0xfffffc00) == 0x5ac00000 {
        return Some(InstructionKind::RBIT32Dp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5ac00400 {
        return Some(InstructionKind::REV1632Dp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5ac00800 {
        return Some(InstructionKind::REV32Dp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5ac01000 {
        return Some(InstructionKind::CLZ32Dp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5ac01400 {
        return Some(InstructionKind::CLS32Dp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xdac00000 {
        return Some(InstructionKind::RBIT64Dp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xdac00400 {
        return Some(InstructionKind::REV1664Dp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xdac00800 {
        return Some(InstructionKind::REV3264Dp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xdac00c00 {
        return Some(InstructionKind::REV64Dp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xdac01000 {
        return Some(InstructionKind::CLZ64Dp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xdac01400 {
        return Some(InstructionKind::CLS64Dp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xdac10000 {
        return Some(InstructionKind::PACIA64PDp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xdac10400 {
        return Some(InstructionKind::PACIB64PDp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xdac10800 {
        return Some(InstructionKind::PACDA64PDp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xdac10c00 {
        return Some(InstructionKind::PACDB64PDp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xdac11000 {
        return Some(InstructionKind::AUTIA64PDp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xdac11400 {
        return Some(InstructionKind::AUTIB64PDp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xdac11800 {
        return Some(InstructionKind::AUTDA64PDp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xdac11c00 {
        return Some(InstructionKind::AUTDB64PDp1Src {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_dp_2src(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x5fe00000) != 0x1ac00000) as usize];
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let opcode = (d >> 10) & 0x3F;
    let sf = (d >> 31) & 1;
    if (d & 0xffe0fc00) == 0x1ac00800 {
        return Some(InstructionKind::UDIV32Dp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ac00c00 {
        return Some(InstructionKind::SDIV32Dp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ac02000 {
        return Some(InstructionKind::LSLV32Dp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ac02400 {
        return Some(InstructionKind::LSRV32Dp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ac02800 {
        return Some(InstructionKind::ASRV32Dp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ac02c00 {
        return Some(InstructionKind::RORV32Dp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ac04000 {
        return Some(InstructionKind::CRC32B32CDp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ac04400 {
        return Some(InstructionKind::CRC32H32CDp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ac04800 {
        return Some(InstructionKind::CRC32W32CDp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ac05000 {
        return Some(InstructionKind::CRC32CB32CDp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ac05400 {
        return Some(InstructionKind::CRC32CH32CDp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ac05800 {
        return Some(InstructionKind::CRC32CW32CDp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x9ac00800 {
        return Some(InstructionKind::UDIV64Dp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x9ac00c00 {
        return Some(InstructionKind::SDIV64Dp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x9ac02000 {
        return Some(InstructionKind::LSLV64Dp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x9ac02400 {
        return Some(InstructionKind::LSRV64Dp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x9ac02800 {
        return Some(InstructionKind::ASRV64Dp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x9ac02c00 {
        return Some(InstructionKind::RORV64Dp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x9ac03000 {
        return Some(InstructionKind::PACGA64PDp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x9ac04c00 {
        return Some(InstructionKind::CRC32X64CDp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x9ac05c00 {
        return Some(InstructionKind::CRC32CX64CDp2Src {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_dp_3src(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1f000000) != 0x1b000000) as usize];
    let Ra = (d >> 10) & 0x1F;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let o0 = (d >> 15) & 1;
    let op31 = (d >> 21) & 7;
    let op54 = (d >> 29) & 3;
    let sf = (d >> 31) & 1;
    if (d & 0xffe08000) == 0x1b000000 {
        return Some(InstructionKind::MADD32ADp3Src {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x1b008000 {
        return Some(InstructionKind::MSUB32ADp3Src {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x9b000000 {
        return Some(InstructionKind::MADD64ADp3Src {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x9b008000 {
        return Some(InstructionKind::MSUB64ADp3Src {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x9b200000 {
        return Some(InstructionKind::SMADDL64WaDp3Src {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x9b208000 {
        return Some(InstructionKind::SMSUBL64WaDp3Src {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x9b400000 {
        return Some(InstructionKind::SMULH64Dp3Src {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x9ba00000 {
        return Some(InstructionKind::UMADDL64WaDp3Src {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x9ba08000 {
        return Some(InstructionKind::UMSUBL64WaDp3Src {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x9bc00000 {
        return Some(InstructionKind::UMULH64Dp3Src {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_log_shift(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x1f000000) != 0x0a000000) as usize];
    let N = (d >> 21) & 1;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let imm6 = (d >> 10) & 0x3F;
    let opc = (d >> 29) & 3;
    let sf = (d >> 31) & 1;
    let shift = (d >> 22) & 3;
    if (d & 0xff200000) == 0x0a000000 {
        return Some(InstructionKind::AND32LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0x0a200000 {
        return Some(InstructionKind::BIC32LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0x2a000000 {
        return Some(InstructionKind::ORR32LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0x2a200000 {
        return Some(InstructionKind::ORN32LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0x4a000000 {
        return Some(InstructionKind::EOR32LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0x4a200000 {
        return Some(InstructionKind::EON32LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0x6a000000 {
        return Some(InstructionKind::ANDS32LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0x6a200000 {
        return Some(InstructionKind::BICS32LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0x8a000000 {
        return Some(InstructionKind::AND64LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0x8a200000 {
        return Some(InstructionKind::BIC64LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0xaa000000 {
        return Some(InstructionKind::ORR64LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0xaa200000 {
        return Some(InstructionKind::ORN64LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0xca000000 {
        return Some(InstructionKind::EOR64LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0xca200000 {
        return Some(InstructionKind::EON64LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0xea000000 {
        return Some(InstructionKind::ANDS64LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    if (d & 0xff200000) == 0xea200000 {
        return Some(InstructionKind::BICS64LogShift {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
            shift: shift as _,
        });
    }
    None
}
pub const fn decode_asimdall(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x9f3e0c00) != 0x0e300800) as usize];
    let Q = (d >> 30) & 1;
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let opcode = (d >> 12) & 0x1F;
    let size = (d >> 22) & 3;
    if (d & 0xbffffc00) == 0x0e30c800 {
        return Some(InstructionKind::FMAXNMVAsimdallOnlyH {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0e30f800 {
        return Some(InstructionKind::FMAXVAsimdallOnlyH {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0eb0c800 {
        return Some(InstructionKind::FMINNMVAsimdallOnlyH {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0eb0f800 {
        return Some(InstructionKind::FMINVAsimdallOnlyH {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2e30c800 {
        return Some(InstructionKind::FMAXNMVAsimdallOnlySd {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2e30f800 {
        return Some(InstructionKind::FMAXVAsimdallOnlySd {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2eb0c800 {
        return Some(InstructionKind::FMINNMVAsimdallOnlySd {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2eb0f800 {
        return Some(InstructionKind::FMINVAsimdallOnlySd {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e303800 {
        return Some(InstructionKind::SADDLVAsimdallOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e30a800 {
        return Some(InstructionKind::SMAXVAsimdallOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e31a800 {
        return Some(InstructionKind::SMINVAsimdallOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e31b800 {
        return Some(InstructionKind::ADDVAsimdallOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e303800 {
        return Some(InstructionKind::UADDLVAsimdallOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e30a800 {
        return Some(InstructionKind::UMAXVAsimdallOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e31a800 {
        return Some(InstructionKind::UMINVAsimdallOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asimdins(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x9fe08400) != 0x0e000400) as usize];
    let Q = (d >> 30) & 1;
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let imm4 = (d >> 11) & 0xF;
    let imm5 = (d >> 16) & 0x1F;
    let op = (d >> 29) & 1;
    if (d & 0xffeffc00) == 0x4e083c00 {
        return Some(InstructionKind::UMOVAsimdinsXX {
            Rd: Rd as _,
            Rn: Rn as _,
            imm5: imm5 as _,
        });
    }
    if (d & 0xffe0fc00) == 0x0e002c00 {
        return Some(InstructionKind::SMOVAsimdinsWW {
            Rd: Rd as _,
            Rn: Rn as _,
            imm5: imm5 as _,
        });
    }
    if (d & 0xffe0fc00) == 0x0e003c00 {
        return Some(InstructionKind::UMOVAsimdinsWW {
            Rd: Rd as _,
            Rn: Rn as _,
            imm5: imm5 as _,
        });
    }
    if (d & 0xffe0fc00) == 0x4e001c00 {
        return Some(InstructionKind::INSAsimdinsIrR {
            Rd: Rd as _,
            Rn: Rn as _,
            imm5: imm5 as _,
        });
    }
    if (d & 0xffe0fc00) == 0x4e002c00 {
        return Some(InstructionKind::SMOVAsimdinsXX {
            Rd: Rd as _,
            Rn: Rn as _,
            imm5: imm5 as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e000400 {
        return Some(InstructionKind::DUPAsimdinsDvV {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            imm5: imm5 as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e000c00 {
        return Some(InstructionKind::DUPAsimdinsDrR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            imm5: imm5 as _,
        });
    }
    if (d & 0xffe08400) == 0x6e000400 {
        return Some(InstructionKind::INSAsimdinsIvV {
            Rd: Rd as _,
            Rn: Rn as _,
            imm4: imm4 as _,
            imm5: imm5 as _,
        });
    }
    None
}
pub const fn decode_asimdext(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xbf208400) != 0x2e000000) as usize];
    let Q = (d >> 30) & 1;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let imm4 = (d >> 11) & 0xF;
    let op2 = (d >> 22) & 3;
    if (d & 0xbfe08400) == 0x2e000000 {
        return Some(InstructionKind::EXTAsimdextOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm4: imm4 as _,
        });
    }
    None
}
pub const fn decode_asimdimm(v: u32) -> Option<InstructionKind> {
    ["Could not decode."][((v & 0x9ff80400) != 0x0f000400) as usize];
    let Q = (v >> 30) & 1;
    let Rd = v & 0x1F;
    let a = (v >> 18) & 1;
    let b = (v >> 17) & 1;
    let c = (v >> 16) & 1;
    let cmode = (v >> 12) & 0xF;
    let d = (v >> 9) & 1;
    let e = (v >> 8) & 1;
    let f = (v >> 7) & 1;
    let g = (v >> 6) & 1;
    let h = (v >> 5) & 1;
    let o2 = (v >> 11) & 1;
    let op = (v >> 29) & 1;
    if (v & 0xfff8fc00) == 0x2f00e400 {
        return Some(InstructionKind::MOVIAsimdimmDDs {
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xfff8fc00) == 0x6f00e400 {
        return Some(InstructionKind::MOVIAsimdimmD2D {
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xfff8fc00) == 0x6f00f400 {
        return Some(InstructionKind::FMOVAsimdimmD2D {
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xbff8fc00) == 0x0f00e400 {
        return Some(InstructionKind::MOVIAsimdimmNB {
            Q: Q as _,
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xbff8fc00) == 0x0f00f400 {
        return Some(InstructionKind::FMOVAsimdimmSS {
            Q: Q as _,
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xbff8fc00) == 0x0f00fc00 {
        return Some(InstructionKind::FMOVAsimdimmHH {
            Q: Q as _,
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xbff8dc00) == 0x0f008400 {
        return Some(InstructionKind::MOVIAsimdimmLHl {
            Q: Q as _,
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            cmode: cmode as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xbff8dc00) == 0x0f009400 {
        return Some(InstructionKind::ORRAsimdimmLHl {
            Q: Q as _,
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            cmode: cmode as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xbff8dc00) == 0x2f008400 {
        return Some(InstructionKind::MVNIAsimdimmLHl {
            Q: Q as _,
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            cmode: cmode as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xbff8dc00) == 0x2f009400 {
        return Some(InstructionKind::BICAsimdimmLHl {
            Q: Q as _,
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            cmode: cmode as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xbff8ec00) == 0x0f00c400 {
        return Some(InstructionKind::MOVIAsimdimmMSm {
            Q: Q as _,
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            cmode: cmode as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xbff8ec00) == 0x2f00c400 {
        return Some(InstructionKind::MVNIAsimdimmMSm {
            Q: Q as _,
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            cmode: cmode as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xbff89c00) == 0x0f000400 {
        return Some(InstructionKind::MOVIAsimdimmLSl {
            Q: Q as _,
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            cmode: cmode as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xbff89c00) == 0x0f001400 {
        return Some(InstructionKind::ORRAsimdimmLSl {
            Q: Q as _,
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            cmode: cmode as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xbff89c00) == 0x2f000400 {
        return Some(InstructionKind::MVNIAsimdimmLSl {
            Q: Q as _,
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            cmode: cmode as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    if (v & 0xbff89c00) == 0x2f001400 {
        return Some(InstructionKind::BICAsimdimmLSl {
            Q: Q as _,
            Rd: Rd as _,
            a: a as _,
            b: b as _,
            c: c as _,
            cmode: cmode as _,
            d: d as _,
            e: e as _,
            f: f as _,
            g: g as _,
            h: h as _,
        });
    }
    None
}
pub const fn decode_asimdperm(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xbf208c00) != 0x0e000800) as usize];
    let Q = (d >> 30) & 1;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let opcode = (d >> 12) & 7;
    let size = (d >> 22) & 3;
    if (d & 0xbf20fc00) == 0x0e001800 {
        return Some(InstructionKind::UZP1AsimdpermOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e002800 {
        return Some(InstructionKind::TRN1AsimdpermOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e003800 {
        return Some(InstructionKind::ZIP1AsimdpermOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e005800 {
        return Some(InstructionKind::UZP2AsimdpermOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e006800 {
        return Some(InstructionKind::TRN2AsimdpermOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e007800 {
        return Some(InstructionKind::ZIP2AsimdpermOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asisdone(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xdfe08400) != 0x5e000400) as usize];
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let imm4 = (d >> 11) & 0xF;
    let imm5 = (d >> 16) & 0x1F;
    let op = (d >> 29) & 1;
    if (d & 0xffe0fc00) == 0x5e000400 {
        return Some(InstructionKind::DUPAsisdoneOnly {
            Rd: Rd as _,
            Rn: Rn as _,
            imm5: imm5 as _,
        });
    }
    None
}
pub const fn decode_asisdpair(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xdf3e0c00) != 0x5e300800) as usize];
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let opcode = (d >> 12) & 0x1F;
    let size = (d >> 22) & 3;
    if (d & 0xfffffc00) == 0x5e30c800 {
        return Some(InstructionKind::FMAXNMPAsisdpairOnlyH {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5e30d800 {
        return Some(InstructionKind::FADDPAsisdpairOnlyH {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5e30f800 {
        return Some(InstructionKind::FMAXPAsisdpairOnlyH {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5eb0c800 {
        return Some(InstructionKind::FMINNMPAsisdpairOnlyH {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5eb0f800 {
        return Some(InstructionKind::FMINPAsisdpairOnlyH {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffbffc00) == 0x7e30c800 {
        return Some(InstructionKind::FMAXNMPAsisdpairOnlySd {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x7e30d800 {
        return Some(InstructionKind::FADDPAsisdpairOnlySd {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x7e30f800 {
        return Some(InstructionKind::FMAXPAsisdpairOnlySd {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x7eb0c800 {
        return Some(InstructionKind::FMINNMPAsisdpairOnlySd {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x7eb0f800 {
        return Some(InstructionKind::FMINPAsisdpairOnlySd {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x5e31b800 {
        return Some(InstructionKind::ADDPAsisdpairOnly {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asisdshf(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xdf800400) != 0x5f000400) as usize];
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let immb = (d >> 16) & 7;
    let immh = (d >> 19) & 0xF;
    let opcode = (d >> 11) & 0x1F;
    if (d & 0xff80fc00) == 0x5f000400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SSHRAsisdshfR {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x5f001400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SSRAAsisdshfR {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x5f002400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SRSHRAsisdshfR {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x5f003400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SRSRAAsisdshfR {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x5f005400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SHLAsisdshfR {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x5f007400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SQSHLAsisdshfR {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x5f009400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SQSHRNAsisdshfN {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x5f009c00 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SQRSHRNAsisdshfN {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x5f00e400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SCVTFAsisdshfC {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x5f00fc00 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::FCVTZSAsisdshfC {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x7f000400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::USHRAsisdshfR {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x7f001400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::USRAAsisdshfR {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x7f002400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::URSHRAsisdshfR {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x7f003400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::URSRAAsisdshfR {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x7f004400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SRIAsisdshfR {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x7f005400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SLIAsisdshfR {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x7f006400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SQSHLUAsisdshfR {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x7f007400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::UQSHLAsisdshfR {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x7f008400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SQSHRUNAsisdshfN {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x7f008c00 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SQRSHRUNAsisdshfN {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x7f009400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::UQSHRNAsisdshfN {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x7f009c00 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::UQRSHRNAsisdshfN {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x7f00e400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::UCVTFAsisdshfC {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xff80fc00) == 0x7f00fc00 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::FCVTZUAsisdshfC {
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    None
}
pub const fn decode_asisddiff(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xdf200c00) != 0x5e200000) as usize];
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let opcode = (d >> 12) & 0xF;
    let size = (d >> 22) & 3;
    if (d & 0xff20fc00) == 0x5e209000 {
        return Some(InstructionKind::SQDMLALAsisddiffOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x5e20b000 {
        return Some(InstructionKind::SQDMLSLAsisddiffOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x5e20d000 {
        return Some(InstructionKind::SQDMULLAsisddiffOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asisdsame(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xdf200400) != 0x5e200400) as usize];
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let opcode = (d >> 11) & 0x1F;
    let size = (d >> 22) & 3;
    if (d & 0xffa0fc00) == 0x5e20dc00 {
        return Some(InstructionKind::FMULXAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffa0fc00) == 0x5e20e400 {
        return Some(InstructionKind::FCMEQAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffa0fc00) == 0x5e20fc00 {
        return Some(InstructionKind::FRECPSAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffa0fc00) == 0x5ea0fc00 {
        return Some(InstructionKind::FRSQRTSAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffa0fc00) == 0x7e20e400 {
        return Some(InstructionKind::FCMGEAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffa0fc00) == 0x7e20ec00 {
        return Some(InstructionKind::FACGEAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffa0fc00) == 0x7ea0d400 {
        return Some(InstructionKind::FABDAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffa0fc00) == 0x7ea0e400 {
        return Some(InstructionKind::FCMGTAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffa0fc00) == 0x7ea0ec00 {
        return Some(InstructionKind::FACGTAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x5e200c00 {
        return Some(InstructionKind::SQADDAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x5e202c00 {
        return Some(InstructionKind::SQSUBAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x5e203400 {
        return Some(InstructionKind::CMGTAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x5e203c00 {
        return Some(InstructionKind::CMGEAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x5e204400 {
        return Some(InstructionKind::SSHLAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x5e204c00 {
        return Some(InstructionKind::SQSHLAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x5e205400 {
        return Some(InstructionKind::SRSHLAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x5e205c00 {
        return Some(InstructionKind::SQRSHLAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x5e208400 {
        return Some(InstructionKind::ADDAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x5e208c00 {
        return Some(InstructionKind::CMTSTAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x5e20b400 {
        return Some(InstructionKind::SQDMULHAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x7e200c00 {
        return Some(InstructionKind::UQADDAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x7e202c00 {
        return Some(InstructionKind::UQSUBAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x7e203400 {
        return Some(InstructionKind::CMHIAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x7e203c00 {
        return Some(InstructionKind::CMHSAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x7e204400 {
        return Some(InstructionKind::USHLAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x7e204c00 {
        return Some(InstructionKind::UQSHLAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x7e205400 {
        return Some(InstructionKind::URSHLAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x7e205c00 {
        return Some(InstructionKind::UQRSHLAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x7e208400 {
        return Some(InstructionKind::SUBAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x7e208c00 {
        return Some(InstructionKind::CMEQAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x7e20b400 {
        return Some(InstructionKind::SQRDMULHAsisdsameOnly {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asisdsamefp16(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xdf60c400) != 0x5e400400) as usize];
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let a = (d >> 23) & 1;
    let opcode = (d >> 11) & 7;
    if (d & 0xffe0fc00) == 0x5e401c00 {
        return Some(InstructionKind::FMULXAsisdsamefp16Only {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x5e402400 {
        return Some(InstructionKind::FCMEQAsisdsamefp16Only {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x5e403c00 {
        return Some(InstructionKind::FRECPSAsisdsamefp16Only {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x5ec03c00 {
        return Some(InstructionKind::FRSQRTSAsisdsamefp16Only {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x7e402400 {
        return Some(InstructionKind::FCMGEAsisdsamefp16Only {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x7e402c00 {
        return Some(InstructionKind::FACGEAsisdsamefp16Only {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x7ec01400 {
        return Some(InstructionKind::FABDAsisdsamefp16Only {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x7ec02400 {
        return Some(InstructionKind::FCMGTAsisdsamefp16Only {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x7ec02c00 {
        return Some(InstructionKind::FACGTAsisdsamefp16Only {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_asisdsame2(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xdf208400) != 0x5e008400) as usize];
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let opcode = (d >> 11) & 0xF;
    let size = (d >> 22) & 3;
    if (d & 0xff20fc00) == 0x7e008400 {
        return Some(InstructionKind::SQRDMLAHAsisdsame2Only {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff20fc00) == 0x7e008c00 {
        return Some(InstructionKind::SQRDMLSHAsisdsame2Only {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asisdmisc(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xdf3e0c00) != 0x5e200800) as usize];
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let opcode = (d >> 12) & 0x1F;
    let size = (d >> 22) & 3;
    if (d & 0xffbffc00) == 0x5e21a800 {
        return Some(InstructionKind::FCVTNSAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x5e21b800 {
        return Some(InstructionKind::FCVTMSAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x5e21c800 {
        return Some(InstructionKind::FCVTASAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x5e21d800 {
        return Some(InstructionKind::SCVTFAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x5ea0c800 {
        return Some(InstructionKind::FCMGTAsisdmiscFz {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x5ea0d800 {
        return Some(InstructionKind::FCMEQAsisdmiscFz {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x5ea0e800 {
        return Some(InstructionKind::FCMLTAsisdmiscFz {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x5ea1a800 {
        return Some(InstructionKind::FCVTPSAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x5ea1b800 {
        return Some(InstructionKind::FCVTZSAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x5ea1d800 {
        return Some(InstructionKind::FRECPEAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x5ea1f800 {
        return Some(InstructionKind::FRECPXAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x7e216800 {
        return Some(InstructionKind::FCVTXNAsisdmiscN {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x7e21a800 {
        return Some(InstructionKind::FCVTNUAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x7e21b800 {
        return Some(InstructionKind::FCVTMUAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x7e21c800 {
        return Some(InstructionKind::FCVTAUAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x7e21d800 {
        return Some(InstructionKind::UCVTFAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x7ea0c800 {
        return Some(InstructionKind::FCMGEAsisdmiscFz {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x7ea0d800 {
        return Some(InstructionKind::FCMLEAsisdmiscFz {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x7ea1a800 {
        return Some(InstructionKind::FCVTPUAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x7ea1b800 {
        return Some(InstructionKind::FCVTZUAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xffbffc00) == 0x7ea1d800 {
        return Some(InstructionKind::FRSQRTEAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x5e203800 {
        return Some(InstructionKind::SUQADDAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x5e207800 {
        return Some(InstructionKind::SQABSAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x5e208800 {
        return Some(InstructionKind::CMGTAsisdmiscZ {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x5e209800 {
        return Some(InstructionKind::CMEQAsisdmiscZ {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x5e20a800 {
        return Some(InstructionKind::CMLTAsisdmiscZ {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x5e20b800 {
        return Some(InstructionKind::ABSAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x5e214800 {
        return Some(InstructionKind::SQXTNAsisdmiscN {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x7e203800 {
        return Some(InstructionKind::USQADDAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x7e207800 {
        return Some(InstructionKind::SQNEGAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x7e208800 {
        return Some(InstructionKind::CMGEAsisdmiscZ {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x7e209800 {
        return Some(InstructionKind::CMLEAsisdmiscZ {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x7e20b800 {
        return Some(InstructionKind::NEGAsisdmiscR {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x7e212800 {
        return Some(InstructionKind::SQXTUNAsisdmiscN {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff3ffc00) == 0x7e214800 {
        return Some(InstructionKind::UQXTNAsisdmiscN {
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asisdmiscfp16(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xdf7e0c00) != 0x5e780800) as usize];
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let a = (d >> 23) & 1;
    let opcode = (d >> 12) & 0x1F;
    if (d & 0xfffffc00) == 0x5e79a800 {
        return Some(InstructionKind::FCVTNSAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5e79b800 {
        return Some(InstructionKind::FCVTMSAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5e79c800 {
        return Some(InstructionKind::FCVTASAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5e79d800 {
        return Some(InstructionKind::SCVTFAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5ef8c800 {
        return Some(InstructionKind::FCMGTAsisdmiscfp16Fz {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5ef8d800 {
        return Some(InstructionKind::FCMEQAsisdmiscfp16Fz {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5ef8e800 {
        return Some(InstructionKind::FCMLTAsisdmiscfp16Fz {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5ef9a800 {
        return Some(InstructionKind::FCVTPSAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5ef9b800 {
        return Some(InstructionKind::FCVTZSAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5ef9d800 {
        return Some(InstructionKind::FRECPEAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5ef9f800 {
        return Some(InstructionKind::FRECPXAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x7e79a800 {
        return Some(InstructionKind::FCVTNUAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x7e79b800 {
        return Some(InstructionKind::FCVTMUAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x7e79c800 {
        return Some(InstructionKind::FCVTAUAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x7e79d800 {
        return Some(InstructionKind::UCVTFAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x7ef8c800 {
        return Some(InstructionKind::FCMGEAsisdmiscfp16Fz {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x7ef8d800 {
        return Some(InstructionKind::FCMLEAsisdmiscfp16Fz {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x7ef9a800 {
        return Some(InstructionKind::FCVTPUAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x7ef9b800 {
        return Some(InstructionKind::FCVTZUAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x7ef9d800 {
        return Some(InstructionKind::FRSQRTEAsisdmiscfp16R {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_asisdelem(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xdf000400) != 0x5f000000) as usize];
    let H = (d >> 11) & 1;
    let L = (d >> 21) & 1;
    let M = (d >> 20) & 1;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0xF;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let opcode = (d >> 12) & 0xF;
    let size = (d >> 22) & 3;
    if (d & 0xffc0f400) == 0x5f001000 {
        return Some(InstructionKind::FMLAAsisdelemRhH {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffc0f400) == 0x5f005000 {
        return Some(InstructionKind::FMLSAsisdelemRhH {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffc0f400) == 0x5f009000 {
        return Some(InstructionKind::FMULAsisdelemRhH {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffc0f400) == 0x7f009000 {
        return Some(InstructionKind::FMULXAsisdelemRhH {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xff80f400) == 0x5f801000 {
        return Some(InstructionKind::FMLAAsisdelemRSd {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff80f400) == 0x5f805000 {
        return Some(InstructionKind::FMLSAsisdelemRSd {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff80f400) == 0x5f809000 {
        return Some(InstructionKind::FMULAsisdelemRSd {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff80f400) == 0x7f809000 {
        return Some(InstructionKind::FMULXAsisdelemRSd {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff00f400) == 0x5f003000 {
        return Some(InstructionKind::SQDMLALAsisdelemL {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff00f400) == 0x5f007000 {
        return Some(InstructionKind::SQDMLSLAsisdelemL {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff00f400) == 0x5f00b000 {
        return Some(InstructionKind::SQDMULLAsisdelemL {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff00f400) == 0x5f00c000 {
        return Some(InstructionKind::SQDMULHAsisdelemR {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff00f400) == 0x5f00d000 {
        return Some(InstructionKind::SQRDMULHAsisdelemR {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff00f400) == 0x7f00d000 {
        return Some(InstructionKind::SQRDMLAHAsisdelemR {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xff00f400) == 0x7f00f000 {
        return Some(InstructionKind::SQRDMLSHAsisdelemR {
            H: H as _,
            L: L as _,
            M: M as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asimdshf(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x9f800400) != 0x0f000400 && (d & 0x780000) != 0x000000) as usize];
    let Q = (d >> 30) & 1;
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let immb = (d >> 16) & 7;
    let immh = (d >> 19) & 0xF;
    let opcode = (d >> 11) & 0x1F;
    if (d & 0xbf80fc00) == 0x0f000400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SSHRAsimdshfR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x0f001400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SSRAAsimdshfR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x0f002400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SRSHRAsimdshfR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x0f003400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SRSRAAsimdshfR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x0f005400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SHLAsimdshfR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x0f007400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SQSHLAsimdshfR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x0f008400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SHRNAsimdshfN {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x0f008c00 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::RSHRNAsimdshfN {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x0f009400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SQSHRNAsimdshfN {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x0f009c00 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SQRSHRNAsimdshfN {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x0f00a400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SSHLLAsimdshfL {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x0f00e400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SCVTFAsimdshfC {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x0f00fc00 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::FCVTZSAsimdshfC {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f000400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::USHRAsimdshfR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f001400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::USRAAsimdshfR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f002400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::URSHRAsimdshfR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f003400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::URSRAAsimdshfR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f004400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SRIAsimdshfR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f005400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SLIAsimdshfR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f006400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SQSHLUAsimdshfR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f007400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::UQSHLAsimdshfR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f008400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SQSHRUNAsimdshfN {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f008c00 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::SQRSHRUNAsimdshfN {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f009400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::UQSHRNAsimdshfN {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f009c00 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::UQRSHRNAsimdshfN {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f00a400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::USHLLAsimdshfL {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f00e400 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::UCVTFAsimdshfC {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    if (d & 0xbf80fc00) == 0x2f00fc00 && (d & 0x780000) != 0x000000 {
        return Some(InstructionKind::FCVTZUAsimdshfC {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            immb: immb as _,
            immh: immh as _,
        });
    }
    None
}
pub const fn decode_asimdtbl(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xbf208c00) != 0x0e000000) as usize];
    let Q = (d >> 30) & 1;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let len = (d >> 13) & 3;
    let op = (d >> 12) & 1;
    let op2 = (d >> 22) & 3;
    if (d & 0xbfe0fc00) == 0x0e000000 {
        return Some(InstructionKind::TBLAsimdtblL11 {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e001000 {
        return Some(InstructionKind::TBXAsimdtblL11 {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e002000 {
        return Some(InstructionKind::TBLAsimdtblL22 {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e003000 {
        return Some(InstructionKind::TBXAsimdtblL22 {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e004000 {
        return Some(InstructionKind::TBLAsimdtblL33 {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e005000 {
        return Some(InstructionKind::TBXAsimdtblL33 {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e006000 {
        return Some(InstructionKind::TBLAsimdtblL44 {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e007000 {
        return Some(InstructionKind::TBXAsimdtblL44 {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_asimddiff(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x9f200c00) != 0x0e200000) as usize];
    let Q = (d >> 30) & 1;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let opcode = (d >> 12) & 0xF;
    let size = (d >> 22) & 3;
    if (d & 0xbf20fc00) == 0x0e200000 {
        return Some(InstructionKind::SADDLAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e201000 {
        return Some(InstructionKind::SADDWAsimddiffW {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e202000 {
        return Some(InstructionKind::SSUBLAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e203000 {
        return Some(InstructionKind::SSUBWAsimddiffW {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e204000 {
        return Some(InstructionKind::ADDHNAsimddiffN {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e205000 {
        return Some(InstructionKind::SABALAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e206000 {
        return Some(InstructionKind::SUBHNAsimddiffN {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e207000 {
        return Some(InstructionKind::SABDLAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e208000 {
        return Some(InstructionKind::SMLALAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e209000 {
        return Some(InstructionKind::SQDMLALAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e20a000 {
        return Some(InstructionKind::SMLSLAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e20b000 {
        return Some(InstructionKind::SQDMLSLAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e20c000 {
        return Some(InstructionKind::SMULLAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e20d000 {
        return Some(InstructionKind::SQDMULLAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e20e000 {
        return Some(InstructionKind::PMULLAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e200000 {
        return Some(InstructionKind::UADDLAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e201000 {
        return Some(InstructionKind::UADDWAsimddiffW {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e202000 {
        return Some(InstructionKind::USUBLAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e203000 {
        return Some(InstructionKind::USUBWAsimddiffW {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e204000 {
        return Some(InstructionKind::RADDHNAsimddiffN {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e205000 {
        return Some(InstructionKind::UABALAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e206000 {
        return Some(InstructionKind::RSUBHNAsimddiffN {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e207000 {
        return Some(InstructionKind::UABDLAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e208000 {
        return Some(InstructionKind::UMLALAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e20a000 {
        return Some(InstructionKind::UMLSLAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e20c000 {
        return Some(InstructionKind::UMULLAsimddiffL {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asimdsame(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x9f200400) != 0x0e200400) as usize];
    let Q = (d >> 30) & 1;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let opcode = (d >> 11) & 0x1F;
    let size = (d >> 22) & 3;
    if (d & 0xbfe0fc00) == 0x0e201c00 {
        return Some(InstructionKind::ANDAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e601c00 {
        return Some(InstructionKind::BICAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0ea01c00 {
        return Some(InstructionKind::ORRAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0ee01c00 {
        return Some(InstructionKind::ORNAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2e201c00 {
        return Some(InstructionKind::EORAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2e601c00 {
        return Some(InstructionKind::BSLAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2ea01c00 {
        return Some(InstructionKind::BITAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2ee01c00 {
        return Some(InstructionKind::BIFAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x0e20c400 {
        return Some(InstructionKind::FMAXNMAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x0e20cc00 {
        return Some(InstructionKind::FMLAAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x0e20d400 {
        return Some(InstructionKind::FADDAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x0e20dc00 {
        return Some(InstructionKind::FMULXAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x0e20e400 {
        return Some(InstructionKind::FCMEQAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x0e20f400 {
        return Some(InstructionKind::FMAXAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x0e20fc00 {
        return Some(InstructionKind::FRECPSAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x0ea0c400 {
        return Some(InstructionKind::FMINNMAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x0ea0cc00 {
        return Some(InstructionKind::FMLSAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x0ea0d400 {
        return Some(InstructionKind::FSUBAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x0ea0f400 {
        return Some(InstructionKind::FMINAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x0ea0fc00 {
        return Some(InstructionKind::FRSQRTSAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x2e20c400 {
        return Some(InstructionKind::FMAXNMPAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x2e20d400 {
        return Some(InstructionKind::FADDPAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x2e20dc00 {
        return Some(InstructionKind::FMULAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x2e20e400 {
        return Some(InstructionKind::FCMGEAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x2e20ec00 {
        return Some(InstructionKind::FACGEAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x2e20f400 {
        return Some(InstructionKind::FMAXPAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x2e20fc00 {
        return Some(InstructionKind::FDIVAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x2ea0c400 {
        return Some(InstructionKind::FMINNMPAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x2ea0d400 {
        return Some(InstructionKind::FABDAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x2ea0e400 {
        return Some(InstructionKind::FCMGTAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x2ea0ec00 {
        return Some(InstructionKind::FACGTAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfa0fc00) == 0x2ea0f400 {
        return Some(InstructionKind::FMINPAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e200400 {
        return Some(InstructionKind::SHADDAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e200c00 {
        return Some(InstructionKind::SQADDAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e201400 {
        return Some(InstructionKind::SRHADDAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e202400 {
        return Some(InstructionKind::SHSUBAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e202c00 {
        return Some(InstructionKind::SQSUBAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e203400 {
        return Some(InstructionKind::CMGTAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e203c00 {
        return Some(InstructionKind::CMGEAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e204400 {
        return Some(InstructionKind::SSHLAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e204c00 {
        return Some(InstructionKind::SQSHLAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e205400 {
        return Some(InstructionKind::SRSHLAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e205c00 {
        return Some(InstructionKind::SQRSHLAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e206400 {
        return Some(InstructionKind::SMAXAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e206c00 {
        return Some(InstructionKind::SMINAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e207400 {
        return Some(InstructionKind::SABDAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e207c00 {
        return Some(InstructionKind::SABAAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e208400 {
        return Some(InstructionKind::ADDAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e208c00 {
        return Some(InstructionKind::CMTSTAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e209400 {
        return Some(InstructionKind::MLAAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e209c00 {
        return Some(InstructionKind::MULAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e20a400 {
        return Some(InstructionKind::SMAXPAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e20ac00 {
        return Some(InstructionKind::SMINPAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e20b400 {
        return Some(InstructionKind::SQDMULHAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x0e20bc00 {
        return Some(InstructionKind::ADDPAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e200400 {
        return Some(InstructionKind::UHADDAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e200c00 {
        return Some(InstructionKind::UQADDAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e201400 {
        return Some(InstructionKind::URHADDAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e202400 {
        return Some(InstructionKind::UHSUBAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e202c00 {
        return Some(InstructionKind::UQSUBAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e203400 {
        return Some(InstructionKind::CMHIAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e203c00 {
        return Some(InstructionKind::CMHSAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e204400 {
        return Some(InstructionKind::USHLAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e204c00 {
        return Some(InstructionKind::UQSHLAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e205400 {
        return Some(InstructionKind::URSHLAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e205c00 {
        return Some(InstructionKind::UQRSHLAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e206400 {
        return Some(InstructionKind::UMAXAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e206c00 {
        return Some(InstructionKind::UMINAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e207400 {
        return Some(InstructionKind::UABDAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e207c00 {
        return Some(InstructionKind::UABAAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e208400 {
        return Some(InstructionKind::SUBAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e208c00 {
        return Some(InstructionKind::CMEQAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e209400 {
        return Some(InstructionKind::MLSAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e209c00 {
        return Some(InstructionKind::PMULAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e20a400 {
        return Some(InstructionKind::UMAXPAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e20ac00 {
        return Some(InstructionKind::UMINPAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e20b400 {
        return Some(InstructionKind::SQRDMULHAsimdsameOnly {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asimdsamefp16(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x9f60c400) != 0x0e400400) as usize];
    let Q = (d >> 30) & 1;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let a = (d >> 23) & 1;
    let opcode = (d >> 11) & 7;
    if (d & 0xbfe0fc00) == 0x0e400400 {
        return Some(InstructionKind::FMAXNMAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e400c00 {
        return Some(InstructionKind::FMLAAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e401400 {
        return Some(InstructionKind::FADDAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e401c00 {
        return Some(InstructionKind::FMULXAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e402400 {
        return Some(InstructionKind::FCMEQAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e403400 {
        return Some(InstructionKind::FMAXAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0e403c00 {
        return Some(InstructionKind::FRECPSAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0ec00400 {
        return Some(InstructionKind::FMINNMAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0ec00c00 {
        return Some(InstructionKind::FMLSAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0ec01400 {
        return Some(InstructionKind::FSUBAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0ec03400 {
        return Some(InstructionKind::FMINAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x0ec03c00 {
        return Some(InstructionKind::FRSQRTSAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2e400400 {
        return Some(InstructionKind::FMAXNMPAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2e401400 {
        return Some(InstructionKind::FADDPAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2e401c00 {
        return Some(InstructionKind::FMULAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2e402400 {
        return Some(InstructionKind::FCMGEAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2e402c00 {
        return Some(InstructionKind::FACGEAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2e403400 {
        return Some(InstructionKind::FMAXPAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2e403c00 {
        return Some(InstructionKind::FDIVAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2ec00400 {
        return Some(InstructionKind::FMINNMPAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2ec01400 {
        return Some(InstructionKind::FABDAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2ec02400 {
        return Some(InstructionKind::FCMGTAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2ec02c00 {
        return Some(InstructionKind::FACGTAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfe0fc00) == 0x2ec03400 {
        return Some(InstructionKind::FMINPAsimdsamefp16Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_asimdsame2(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x9f208400) != 0x0e008400) as usize];
    let Q = (d >> 30) & 1;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let opcode = (d >> 11) & 0xF;
    let size = (d >> 22) & 3;
    if (d & 0xbf20fc00) == 0x0e009400 {
        return Some(InstructionKind::SDOTAsimdsame2D {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e008400 {
        return Some(InstructionKind::SQRDMLAHAsimdsame2Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e008c00 {
        return Some(InstructionKind::SQRDMLSHAsimdsame2Only {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20fc00) == 0x2e009400 {
        return Some(InstructionKind::UDOTAsimdsame2D {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf20ec00) == 0x2e00e400 {
        return Some(InstructionKind::FCADDAsimdsame2C {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            opcode: opcode as _,
            size: size as _,
        });
    }
    if (d & 0xbf20e400) == 0x2e00c400 {
        return Some(InstructionKind::FCMLAAsimdsame2C {
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            opcode: opcode as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asimdmisc(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x9f3e0c00) != 0x0e200800) as usize];
    let Q = (d >> 30) & 1;
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let opcode = (d >> 12) & 0x1F;
    let size = (d >> 22) & 3;
    if (d & 0xbffffc00) == 0x2e205800 {
        return Some(InstructionKind::NOTAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2e605800 {
        return Some(InstructionKind::RBITAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0e216800 {
        return Some(InstructionKind::FCVTNAsimdmiscN {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0e217800 {
        return Some(InstructionKind::FCVTLAsimdmiscL {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0e218800 {
        return Some(InstructionKind::FRINTNAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0e219800 {
        return Some(InstructionKind::FRINTMAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0e21a800 {
        return Some(InstructionKind::FCVTNSAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0e21b800 {
        return Some(InstructionKind::FCVTMSAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0e21c800 {
        return Some(InstructionKind::FCVTASAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0e21d800 {
        return Some(InstructionKind::SCVTFAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0ea0c800 {
        return Some(InstructionKind::FCMGTAsimdmiscFz {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0ea0d800 {
        return Some(InstructionKind::FCMEQAsimdmiscFz {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0ea0e800 {
        return Some(InstructionKind::FCMLTAsimdmiscFz {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0ea0f800 {
        return Some(InstructionKind::FABSAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0ea18800 {
        return Some(InstructionKind::FRINTPAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0ea19800 {
        return Some(InstructionKind::FRINTZAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0ea1a800 {
        return Some(InstructionKind::FCVTPSAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0ea1b800 {
        return Some(InstructionKind::FCVTZSAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0ea1c800 {
        return Some(InstructionKind::URECPEAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x0ea1d800 {
        return Some(InstructionKind::FRECPEAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2e216800 {
        return Some(InstructionKind::FCVTXNAsimdmiscN {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2e218800 {
        return Some(InstructionKind::FRINTAAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2e219800 {
        return Some(InstructionKind::FRINTXAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2e21a800 {
        return Some(InstructionKind::FCVTNUAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2e21b800 {
        return Some(InstructionKind::FCVTMUAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2e21c800 {
        return Some(InstructionKind::FCVTAUAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2e21d800 {
        return Some(InstructionKind::UCVTFAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2ea0c800 {
        return Some(InstructionKind::FCMGEAsimdmiscFz {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2ea0d800 {
        return Some(InstructionKind::FCMLEAsimdmiscFz {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2ea0f800 {
        return Some(InstructionKind::FNEGAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2ea19800 {
        return Some(InstructionKind::FRINTIAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2ea1a800 {
        return Some(InstructionKind::FCVTPUAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2ea1b800 {
        return Some(InstructionKind::FCVTZUAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2ea1c800 {
        return Some(InstructionKind::URSQRTEAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2ea1d800 {
        return Some(InstructionKind::FRSQRTEAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfbffc00) == 0x2ea1f800 {
        return Some(InstructionKind::FSQRTAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e200800 {
        return Some(InstructionKind::REV64AsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e201800 {
        return Some(InstructionKind::REV16AsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e202800 {
        return Some(InstructionKind::SADDLPAsimdmiscP {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e203800 {
        return Some(InstructionKind::SUQADDAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e204800 {
        return Some(InstructionKind::CLSAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e205800 {
        return Some(InstructionKind::CNTAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e206800 {
        return Some(InstructionKind::SADALPAsimdmiscP {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e207800 {
        return Some(InstructionKind::SQABSAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e208800 {
        return Some(InstructionKind::CMGTAsimdmiscZ {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e209800 {
        return Some(InstructionKind::CMEQAsimdmiscZ {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e20a800 {
        return Some(InstructionKind::CMLTAsimdmiscZ {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e20b800 {
        return Some(InstructionKind::ABSAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e212800 {
        return Some(InstructionKind::XTNAsimdmiscN {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x0e214800 {
        return Some(InstructionKind::SQXTNAsimdmiscN {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e200800 {
        return Some(InstructionKind::REV32AsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e202800 {
        return Some(InstructionKind::UADDLPAsimdmiscP {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e203800 {
        return Some(InstructionKind::USQADDAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e204800 {
        return Some(InstructionKind::CLZAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e206800 {
        return Some(InstructionKind::UADALPAsimdmiscP {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e207800 {
        return Some(InstructionKind::SQNEGAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e208800 {
        return Some(InstructionKind::CMGEAsimdmiscZ {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e209800 {
        return Some(InstructionKind::CMLEAsimdmiscZ {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e20b800 {
        return Some(InstructionKind::NEGAsimdmiscR {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e212800 {
        return Some(InstructionKind::SQXTUNAsimdmiscN {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e213800 {
        return Some(InstructionKind::SHLLAsimdmiscS {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf3ffc00) == 0x2e214800 {
        return Some(InstructionKind::UQXTNAsimdmiscN {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    None
}
pub const fn decode_asimdmiscfp16(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x9f7e0c00) != 0x0e780800) as usize];
    let Q = (d >> 30) & 1;
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let a = (d >> 23) & 1;
    let opcode = (d >> 12) & 0x1F;
    if (d & 0xbffffc00) == 0x0e798800 {
        return Some(InstructionKind::FRINTNAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0e799800 {
        return Some(InstructionKind::FRINTMAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0e79a800 {
        return Some(InstructionKind::FCVTNSAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0e79b800 {
        return Some(InstructionKind::FCVTMSAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0e79c800 {
        return Some(InstructionKind::FCVTASAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0e79d800 {
        return Some(InstructionKind::SCVTFAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0ef8c800 {
        return Some(InstructionKind::FCMGTAsimdmiscfp16Fz {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0ef8d800 {
        return Some(InstructionKind::FCMEQAsimdmiscfp16Fz {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0ef8e800 {
        return Some(InstructionKind::FCMLTAsimdmiscfp16Fz {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0ef8f800 {
        return Some(InstructionKind::FABSAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0ef98800 {
        return Some(InstructionKind::FRINTPAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0ef99800 {
        return Some(InstructionKind::FRINTZAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0ef9a800 {
        return Some(InstructionKind::FCVTPSAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0ef9b800 {
        return Some(InstructionKind::FCVTZSAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x0ef9d800 {
        return Some(InstructionKind::FRECPEAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2e798800 {
        return Some(InstructionKind::FRINTAAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2e799800 {
        return Some(InstructionKind::FRINTXAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2e79a800 {
        return Some(InstructionKind::FCVTNUAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2e79b800 {
        return Some(InstructionKind::FCVTMUAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2e79c800 {
        return Some(InstructionKind::FCVTAUAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2e79d800 {
        return Some(InstructionKind::UCVTFAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2ef8c800 {
        return Some(InstructionKind::FCMGEAsimdmiscfp16Fz {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2ef8d800 {
        return Some(InstructionKind::FCMLEAsimdmiscfp16Fz {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2ef8f800 {
        return Some(InstructionKind::FNEGAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2ef99800 {
        return Some(InstructionKind::FRINTIAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2ef9a800 {
        return Some(InstructionKind::FCVTPUAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2ef9b800 {
        return Some(InstructionKind::FCVTZUAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2ef9d800 {
        return Some(InstructionKind::FRSQRTEAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbffffc00) == 0x2ef9f800 {
        return Some(InstructionKind::FSQRTAsimdmiscfp16R {
            Q: Q as _,
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_asimdelem(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x9f000400) != 0x0f000000) as usize];
    let H = (d >> 11) & 1;
    let L = (d >> 21) & 1;
    let M = (d >> 20) & 1;
    let Q = (d >> 30) & 1;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0xF;
    let Rn = (d >> 5) & 0x1F;
    let U = (d >> 29) & 1;
    let opcode = (d >> 12) & 0xF;
    let size = (d >> 22) & 3;
    if (d & 0xbfc0f400) == 0x0f001000 {
        return Some(InstructionKind::FMLAAsimdelemRhH {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfc0f400) == 0x0f005000 {
        return Some(InstructionKind::FMLSAsimdelemRhH {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfc0f400) == 0x0f009000 {
        return Some(InstructionKind::FMULAsimdelemRhH {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbfc0f400) == 0x2f009000 {
        return Some(InstructionKind::FMULXAsimdelemRhH {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xbf80f400) == 0x0f801000 {
        return Some(InstructionKind::FMLAAsimdelemRSd {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf80f400) == 0x0f805000 {
        return Some(InstructionKind::FMLSAsimdelemRSd {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf80f400) == 0x0f809000 {
        return Some(InstructionKind::FMULAsimdelemRSd {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf80f400) == 0x2f809000 {
        return Some(InstructionKind::FMULXAsimdelemRSd {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x0f002000 {
        return Some(InstructionKind::SMLALAsimdelemL {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x0f003000 {
        return Some(InstructionKind::SQDMLALAsimdelemL {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x0f006000 {
        return Some(InstructionKind::SMLSLAsimdelemL {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x0f007000 {
        return Some(InstructionKind::SQDMLSLAsimdelemL {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x0f008000 {
        return Some(InstructionKind::MULAsimdelemR {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x0f00a000 {
        return Some(InstructionKind::SMULLAsimdelemL {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x0f00b000 {
        return Some(InstructionKind::SQDMULLAsimdelemL {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x0f00c000 {
        return Some(InstructionKind::SQDMULHAsimdelemR {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x0f00d000 {
        return Some(InstructionKind::SQRDMULHAsimdelemR {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x0f00e000 {
        return Some(InstructionKind::SDOTAsimdelemD {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x2f000000 {
        return Some(InstructionKind::MLAAsimdelemR {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x2f002000 {
        return Some(InstructionKind::UMLALAsimdelemL {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x2f004000 {
        return Some(InstructionKind::MLSAsimdelemR {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x2f006000 {
        return Some(InstructionKind::UMLSLAsimdelemL {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x2f00a000 {
        return Some(InstructionKind::UMULLAsimdelemL {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x2f00d000 {
        return Some(InstructionKind::SQRDMLAHAsimdelemR {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x2f00e000 {
        return Some(InstructionKind::UDOTAsimdelemD {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbf00f400) == 0x2f00f000 {
        return Some(InstructionKind::SQRDMLSHAsimdelemR {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            size: size as _,
        });
    }
    if (d & 0xbfc09400) == 0x2f401000 {
        return Some(InstructionKind::FCMLAAsimdelemCH {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            opcode: opcode as _,
        });
    }
    if (d & 0xbfc09400) == 0x2f801000 {
        return Some(InstructionKind::FCMLAAsimdelemCS {
            H: H as _,
            L: L as _,
            M: M as _,
            Q: Q as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            opcode: opcode as _,
        });
    }
    None
}
pub const fn decode_float2fix(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x5f200000) != 0x1e000000) as usize];
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let opcode = (d >> 16) & 7;
    let rmode = (d >> 19) & 3;
    let scale = (d >> 10) & 0x3F;
    let sf = (d >> 31) & 1;
    let kind = (d >> 22) & 3;
    if (d & 0xffff0000) == 0x1e020000 {
        return Some(InstructionKind::SCVTFS32Float2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x1e030000 {
        return Some(InstructionKind::UCVTFS32Float2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x1e180000 {
        return Some(InstructionKind::FCVTZS32SFloat2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x1e190000 {
        return Some(InstructionKind::FCVTZU32SFloat2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x1e420000 {
        return Some(InstructionKind::SCVTFD32Float2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x1e430000 {
        return Some(InstructionKind::UCVTFD32Float2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x1e580000 {
        return Some(InstructionKind::FCVTZS32DFloat2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x1e590000 {
        return Some(InstructionKind::FCVTZU32DFloat2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x1ec20000 {
        return Some(InstructionKind::SCVTFH32Float2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x1ec30000 {
        return Some(InstructionKind::UCVTFH32Float2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x1ed80000 {
        return Some(InstructionKind::FCVTZS32HFloat2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x1ed90000 {
        return Some(InstructionKind::FCVTZU32HFloat2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x9e020000 {
        return Some(InstructionKind::SCVTFS64Float2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x9e030000 {
        return Some(InstructionKind::UCVTFS64Float2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x9e180000 {
        return Some(InstructionKind::FCVTZS64SFloat2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x9e190000 {
        return Some(InstructionKind::FCVTZU64SFloat2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x9e420000 {
        return Some(InstructionKind::SCVTFD64Float2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x9e430000 {
        return Some(InstructionKind::UCVTFD64Float2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x9e580000 {
        return Some(InstructionKind::FCVTZS64DFloat2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x9e590000 {
        return Some(InstructionKind::FCVTZU64DFloat2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x9ec20000 {
        return Some(InstructionKind::SCVTFH64Float2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x9ec30000 {
        return Some(InstructionKind::UCVTFH64Float2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x9ed80000 {
        return Some(InstructionKind::FCVTZS64HFloat2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    if (d & 0xffff0000) == 0x9ed90000 {
        return Some(InstructionKind::FCVTZU64HFloat2Fix {
            Rd: Rd as _,
            Rn: Rn as _,
            scale: scale as _,
        });
    }
    None
}
pub const fn decode_float2int(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x5f20fc00) != 0x1e200000) as usize];
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let opcode = (d >> 16) & 7;
    let rmode = (d >> 19) & 3;
    let sf = (d >> 31) & 1;
    let kind = (d >> 22) & 3;
    if (d & 0xfffffc00) == 0x1e200000 {
        return Some(InstructionKind::FCVTNS32SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e210000 {
        return Some(InstructionKind::FCVTNU32SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e220000 {
        return Some(InstructionKind::SCVTFS32Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e230000 {
        return Some(InstructionKind::UCVTFS32Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e240000 {
        return Some(InstructionKind::FCVTAS32SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e250000 {
        return Some(InstructionKind::FCVTAU32SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e260000 {
        return Some(InstructionKind::FMOV32SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e270000 {
        return Some(InstructionKind::FMOVS32Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e280000 {
        return Some(InstructionKind::FCVTPS32SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e290000 {
        return Some(InstructionKind::FCVTPU32SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e300000 {
        return Some(InstructionKind::FCVTMS32SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e310000 {
        return Some(InstructionKind::FCVTMU32SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e380000 {
        return Some(InstructionKind::FCVTZS32SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e390000 {
        return Some(InstructionKind::FCVTZU32SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e600000 {
        return Some(InstructionKind::FCVTNS32DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e610000 {
        return Some(InstructionKind::FCVTNU32DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e620000 {
        return Some(InstructionKind::SCVTFD32Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e630000 {
        return Some(InstructionKind::UCVTFD32Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e640000 {
        return Some(InstructionKind::FCVTAS32DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e650000 {
        return Some(InstructionKind::FCVTAU32DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e680000 {
        return Some(InstructionKind::FCVTPS32DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e690000 {
        return Some(InstructionKind::FCVTPU32DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e700000 {
        return Some(InstructionKind::FCVTMS32DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e710000 {
        return Some(InstructionKind::FCVTMU32DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e780000 {
        return Some(InstructionKind::FCVTZS32DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e790000 {
        return Some(InstructionKind::FCVTZU32DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e7e0000 {
        return Some(InstructionKind::FJCVTZS32DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee00000 {
        return Some(InstructionKind::FCVTNS32HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee10000 {
        return Some(InstructionKind::FCVTNU32HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee20000 {
        return Some(InstructionKind::SCVTFH32Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee30000 {
        return Some(InstructionKind::UCVTFH32Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee40000 {
        return Some(InstructionKind::FCVTAS32HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee50000 {
        return Some(InstructionKind::FCVTAU32HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee60000 {
        return Some(InstructionKind::FMOV32HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee70000 {
        return Some(InstructionKind::FMOVH32Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee80000 {
        return Some(InstructionKind::FCVTPS32HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee90000 {
        return Some(InstructionKind::FCVTPU32HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ef00000 {
        return Some(InstructionKind::FCVTMS32HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ef10000 {
        return Some(InstructionKind::FCVTMU32HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ef80000 {
        return Some(InstructionKind::FCVTZS32HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ef90000 {
        return Some(InstructionKind::FCVTZU32HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e200000 {
        return Some(InstructionKind::FCVTNS64SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e210000 {
        return Some(InstructionKind::FCVTNU64SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e220000 {
        return Some(InstructionKind::SCVTFS64Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e230000 {
        return Some(InstructionKind::UCVTFS64Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e240000 {
        return Some(InstructionKind::FCVTAS64SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e250000 {
        return Some(InstructionKind::FCVTAU64SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e280000 {
        return Some(InstructionKind::FCVTPS64SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e290000 {
        return Some(InstructionKind::FCVTPU64SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e300000 {
        return Some(InstructionKind::FCVTMS64SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e310000 {
        return Some(InstructionKind::FCVTMU64SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e380000 {
        return Some(InstructionKind::FCVTZS64SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e390000 {
        return Some(InstructionKind::FCVTZU64SFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e600000 {
        return Some(InstructionKind::FCVTNS64DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e610000 {
        return Some(InstructionKind::FCVTNU64DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e620000 {
        return Some(InstructionKind::SCVTFD64Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e630000 {
        return Some(InstructionKind::UCVTFD64Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e640000 {
        return Some(InstructionKind::FCVTAS64DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e650000 {
        return Some(InstructionKind::FCVTAU64DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e660000 {
        return Some(InstructionKind::FMOV64DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e670000 {
        return Some(InstructionKind::FMOVD64Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e680000 {
        return Some(InstructionKind::FCVTPS64DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e690000 {
        return Some(InstructionKind::FCVTPU64DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e700000 {
        return Some(InstructionKind::FCVTMS64DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e710000 {
        return Some(InstructionKind::FCVTMU64DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e780000 {
        return Some(InstructionKind::FCVTZS64DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9e790000 {
        return Some(InstructionKind::FCVTZU64DFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9eae0000 {
        return Some(InstructionKind::FMOV64VxFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9eaf0000 {
        return Some(InstructionKind::FMOVV64IFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9ee00000 {
        return Some(InstructionKind::FCVTNS64HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9ee10000 {
        return Some(InstructionKind::FCVTNU64HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9ee20000 {
        return Some(InstructionKind::SCVTFH64Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9ee30000 {
        return Some(InstructionKind::UCVTFH64Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9ee40000 {
        return Some(InstructionKind::FCVTAS64HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9ee50000 {
        return Some(InstructionKind::FCVTAU64HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9ee60000 {
        return Some(InstructionKind::FMOV64HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9ee70000 {
        return Some(InstructionKind::FMOVH64Float2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9ee80000 {
        return Some(InstructionKind::FCVTPS64HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9ee90000 {
        return Some(InstructionKind::FCVTPU64HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9ef00000 {
        return Some(InstructionKind::FCVTMS64HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9ef10000 {
        return Some(InstructionKind::FCVTMU64HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9ef80000 {
        return Some(InstructionKind::FCVTZS64HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x9ef90000 {
        return Some(InstructionKind::FCVTZU64HFloat2Int {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_cryptoaes(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xff3e0c00) != 0x4e280800) as usize];
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let opcode = (d >> 12) & 0x1F;
    let size = (d >> 22) & 3;
    if (d & 0xfffffc00) == 0x4e284800 {
        return Some(InstructionKind::AESEBCryptoaes {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x4e285800 {
        return Some(InstructionKind::AESDBCryptoaes {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x4e286800 {
        return Some(InstructionKind::AESMCBCryptoaes {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x4e287800 {
        return Some(InstructionKind::AESIMCBCryptoaes {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_crypto4(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xff808000) != 0xce000000) as usize];
    let Op0 = (d >> 21) & 3;
    let Ra = (d >> 10) & 0x1F;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    if (d & 0xffe08000) == 0xce000000 {
        return Some(InstructionKind::EOR3Vvv16Crypto4 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0xce200000 {
        return Some(InstructionKind::BCAXVvv16Crypto4 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0xce400000 {
        return Some(InstructionKind::SM3SS1Vvv4Crypto4 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_cryptosha3(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xff208c00) != 0x5e000000) as usize];
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let opcode = (d >> 12) & 7;
    let size = (d >> 22) & 3;
    if (d & 0xffe0fc00) == 0x5e000000 {
        return Some(InstructionKind::SHA1CQsvCryptosha3 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x5e001000 {
        return Some(InstructionKind::SHA1PQsvCryptosha3 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x5e002000 {
        return Some(InstructionKind::SHA1MQsvCryptosha3 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x5e003000 {
        return Some(InstructionKind::SHA1SU0VvvCryptosha3 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x5e004000 {
        return Some(InstructionKind::SHA256HQqvCryptosha3 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x5e005000 {
        return Some(InstructionKind::SHA256H2QqvCryptosha3 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x5e006000 {
        return Some(InstructionKind::SHA256SU1VvvCryptosha3 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_cryptosha512_3(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xffe0b000) != 0xce608000) as usize];
    let O = (d >> 14) & 1;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let opcode = (d >> 10) & 3;
    if (d & 0xffe0fc00) == 0xce608000 {
        return Some(InstructionKind::SHA512HQqvCryptosha5123 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0xce608400 {
        return Some(InstructionKind::SHA512H2QqvCryptosha5123 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0xce608800 {
        return Some(InstructionKind::SHA512SU1Vvv2Cryptosha5123 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0xce608c00 {
        return Some(InstructionKind::RAX1Vvv2Cryptosha5123 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0xce60c000 {
        return Some(InstructionKind::SM3PARTW1Vvv4Cryptosha5123 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0xce60c400 {
        return Some(InstructionKind::SM3PARTW2Vvv4Cryptosha5123 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0xce60c800 {
        return Some(InstructionKind::SM4EKEYVvv4Cryptosha5123 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_crypto3_imm2(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xffe0c000) != 0xce408000) as usize];
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let imm2 = (d >> 12) & 3;
    let opcode = (d >> 10) & 3;
    if (d & 0xffe0cc00) == 0xce408000 {
        return Some(InstructionKind::SM3TT1AVvv4Crypto3Imm2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm2: imm2 as _,
        });
    }
    if (d & 0xffe0cc00) == 0xce408400 {
        return Some(InstructionKind::SM3TT1BVvv4Crypto3Imm2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm2: imm2 as _,
        });
    }
    if (d & 0xffe0cc00) == 0xce408800 {
        return Some(InstructionKind::SM3TT2AVvv4Crypto3Imm2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm2: imm2 as _,
        });
    }
    if (d & 0xffe0cc00) == 0xce408c00 {
        return Some(InstructionKind::SM3TT2BVvvCrypto3Imm2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm2: imm2 as _,
        });
    }
    None
}
pub const fn decode_crypto3_imm6(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xffe00000) != 0xce800000) as usize];
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let imm6 = (d >> 10) & 0x3F;
    if (d & 0xffe00000) == 0xce800000 {
        return Some(InstructionKind::XARVvv2Crypto3Imm6 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            imm6: imm6 as _,
        });
    }
    None
}
pub const fn decode_cryptosha2(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xff3e0c00) != 0x5e280800) as usize];
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let opcode = (d >> 12) & 0x1F;
    let size = (d >> 22) & 3;
    if (d & 0xfffffc00) == 0x5e280800 {
        return Some(InstructionKind::SHA1HSsCryptosha2 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5e281800 {
        return Some(InstructionKind::SHA1SU1VvCryptosha2 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x5e282800 {
        return Some(InstructionKind::SHA256SU0VvCryptosha2 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_cryptosha512_2(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0xfffff000) != 0xcec08000) as usize];
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let opcode = (d >> 10) & 3;
    if (d & 0xfffffc00) == 0xcec08000 {
        return Some(InstructionKind::SHA512SU0Vv2Cryptosha5122 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0xcec08400 {
        return Some(InstructionKind::SM4EVv4Cryptosha5122 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_floatcmp(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x5f203c00) != 0x1e202000) as usize];
    let M = (d >> 31) & 1;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let op = (d >> 14) & 3;
    let opcode2 = d & 0x1F;
    let kind = (d >> 22) & 3;
    if (d & 0xffe0fc1f) == 0x1e202000 {
        return Some(InstructionKind::FCMPSFloatcmp {
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x1e202008 {
        return Some(InstructionKind::FCMPSzFloatcmp {
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x1e202010 {
        return Some(InstructionKind::FCMPESFloatcmp {
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x1e202018 {
        return Some(InstructionKind::FCMPESzFloatcmp {
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x1e602000 {
        return Some(InstructionKind::FCMPDFloatcmp {
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x1e602008 {
        return Some(InstructionKind::FCMPDzFloatcmp {
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x1e602010 {
        return Some(InstructionKind::FCMPEDFloatcmp {
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x1e602018 {
        return Some(InstructionKind::FCMPEDzFloatcmp {
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x1ee02000 {
        return Some(InstructionKind::FCMPHFloatcmp {
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x1ee02008 {
        return Some(InstructionKind::FCMPHzFloatcmp {
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x1ee02010 {
        return Some(InstructionKind::FCMPEHFloatcmp {
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc1f) == 0x1ee02018 {
        return Some(InstructionKind::FCMPEHzFloatcmp {
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_floatccmp(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x5f200c00) != 0x1e200400) as usize];
    let M = (d >> 31) & 1;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let cond = (d >> 12) & 0xF;
    let nzcv = d & 0xF;
    let op = (d >> 4) & 1;
    let kind = (d >> 22) & 3;
    if (d & 0xffe00c10) == 0x1e200400 {
        return Some(InstructionKind::FCCMPSFloatccmp {
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
            nzcv: nzcv as _,
        });
    }
    if (d & 0xffe00c10) == 0x1e200410 {
        return Some(InstructionKind::FCCMPESFloatccmp {
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
            nzcv: nzcv as _,
        });
    }
    if (d & 0xffe00c10) == 0x1e600400 {
        return Some(InstructionKind::FCCMPDFloatccmp {
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
            nzcv: nzcv as _,
        });
    }
    if (d & 0xffe00c10) == 0x1e600410 {
        return Some(InstructionKind::FCCMPEDFloatccmp {
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
            nzcv: nzcv as _,
        });
    }
    if (d & 0xffe00c10) == 0x1ee00400 {
        return Some(InstructionKind::FCCMPHFloatccmp {
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
            nzcv: nzcv as _,
        });
    }
    if (d & 0xffe00c10) == 0x1ee00410 {
        return Some(InstructionKind::FCCMPEHFloatccmp {
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
            nzcv: nzcv as _,
        });
    }
    None
}
pub const fn decode_floatsel(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x5f200c00) != 0x1e200c00) as usize];
    let M = (d >> 31) & 1;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let cond = (d >> 12) & 0xF;
    let kind = (d >> 22) & 3;
    if (d & 0xffe00c00) == 0x1e200c00 {
        return Some(InstructionKind::FCSELSFloatsel {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
        });
    }
    if (d & 0xffe00c00) == 0x1e600c00 {
        return Some(InstructionKind::FCSELDFloatsel {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
        });
    }
    if (d & 0xffe00c00) == 0x1ee00c00 {
        return Some(InstructionKind::FCSELHFloatsel {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
            cond: cond as _,
        });
    }
    None
}
pub const fn decode_floatdp1(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x5f207c00) != 0x1e204000) as usize];
    let M = (d >> 31) & 1;
    let Rd = d & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let opcode = (d >> 15) & 0x3F;
    let kind = (d >> 22) & 3;
    if (d & 0xfffffc00) == 0x1e204000 {
        return Some(InstructionKind::FMOVSFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e20c000 {
        return Some(InstructionKind::FABSSFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e214000 {
        return Some(InstructionKind::FNEGSFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e21c000 {
        return Some(InstructionKind::FSQRTSFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e22c000 {
        return Some(InstructionKind::FCVTDsFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e23c000 {
        return Some(InstructionKind::FCVTHsFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e244000 {
        return Some(InstructionKind::FRINTNSFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e24c000 {
        return Some(InstructionKind::FRINTPSFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e254000 {
        return Some(InstructionKind::FRINTMSFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e25c000 {
        return Some(InstructionKind::FRINTZSFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e264000 {
        return Some(InstructionKind::FRINTASFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e274000 {
        return Some(InstructionKind::FRINTXSFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e27c000 {
        return Some(InstructionKind::FRINTISFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e604000 {
        return Some(InstructionKind::FMOVDFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e60c000 {
        return Some(InstructionKind::FABSDFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e614000 {
        return Some(InstructionKind::FNEGDFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e61c000 {
        return Some(InstructionKind::FSQRTDFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e624000 {
        return Some(InstructionKind::FCVTSdFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e63c000 {
        return Some(InstructionKind::FCVTHdFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e644000 {
        return Some(InstructionKind::FRINTNDFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e64c000 {
        return Some(InstructionKind::FRINTPDFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e654000 {
        return Some(InstructionKind::FRINTMDFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e65c000 {
        return Some(InstructionKind::FRINTZDFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e664000 {
        return Some(InstructionKind::FRINTADFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e674000 {
        return Some(InstructionKind::FRINTXDFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1e67c000 {
        return Some(InstructionKind::FRINTIDFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee04000 {
        return Some(InstructionKind::FMOVHFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee0c000 {
        return Some(InstructionKind::FABSHFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee14000 {
        return Some(InstructionKind::FNEGHFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee1c000 {
        return Some(InstructionKind::FSQRTHFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee24000 {
        return Some(InstructionKind::FCVTShFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee2c000 {
        return Some(InstructionKind::FCVTDhFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee44000 {
        return Some(InstructionKind::FRINTNHFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee4c000 {
        return Some(InstructionKind::FRINTPHFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee54000 {
        return Some(InstructionKind::FRINTMHFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee5c000 {
        return Some(InstructionKind::FRINTZHFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee64000 {
        return Some(InstructionKind::FRINTAHFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee74000 {
        return Some(InstructionKind::FRINTXHFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xfffffc00) == 0x1ee7c000 {
        return Some(InstructionKind::FRINTIHFloatdp1 {
            Rd: Rd as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_floatdp2(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x5f200c00) != 0x1e200800) as usize];
    let M = (d >> 31) & 1;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let opcode = (d >> 12) & 0xF;
    let kind = (d >> 22) & 3;
    if (d & 0xffe0fc00) == 0x1e200800 {
        return Some(InstructionKind::FMULSFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e201800 {
        return Some(InstructionKind::FDIVSFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e202800 {
        return Some(InstructionKind::FADDSFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e203800 {
        return Some(InstructionKind::FSUBSFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e204800 {
        return Some(InstructionKind::FMAXSFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e205800 {
        return Some(InstructionKind::FMINSFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e206800 {
        return Some(InstructionKind::FMAXNMSFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e207800 {
        return Some(InstructionKind::FMINNMSFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e208800 {
        return Some(InstructionKind::FNMULSFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e600800 {
        return Some(InstructionKind::FMULDFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e601800 {
        return Some(InstructionKind::FDIVDFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e602800 {
        return Some(InstructionKind::FADDDFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e603800 {
        return Some(InstructionKind::FSUBDFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e604800 {
        return Some(InstructionKind::FMAXDFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e605800 {
        return Some(InstructionKind::FMINDFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e606800 {
        return Some(InstructionKind::FMAXNMDFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e607800 {
        return Some(InstructionKind::FMINNMDFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1e608800 {
        return Some(InstructionKind::FNMULDFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ee00800 {
        return Some(InstructionKind::FMULHFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ee01800 {
        return Some(InstructionKind::FDIVHFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ee02800 {
        return Some(InstructionKind::FADDHFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ee03800 {
        return Some(InstructionKind::FSUBHFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ee04800 {
        return Some(InstructionKind::FMAXHFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ee05800 {
        return Some(InstructionKind::FMINHFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ee06800 {
        return Some(InstructionKind::FMAXNMHFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ee07800 {
        return Some(InstructionKind::FMINNMHFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe0fc00) == 0x1ee08800 {
        return Some(InstructionKind::FNMULHFloatdp2 {
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_floatdp3(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x5f000000) != 0x1f000000) as usize];
    let M = (d >> 31) & 1;
    let Ra = (d >> 10) & 0x1F;
    let Rd = d & 0x1F;
    let Rm = (d >> 16) & 0x1F;
    let Rn = (d >> 5) & 0x1F;
    let S = (d >> 29) & 1;
    let o0 = (d >> 15) & 1;
    let o1 = (d >> 21) & 1;
    let kind = (d >> 22) & 3;
    if (d & 0xffe08000) == 0x1f000000 {
        return Some(InstructionKind::FMADDSFloatdp3 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x1f008000 {
        return Some(InstructionKind::FMSUBSFloatdp3 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x1f200000 {
        return Some(InstructionKind::FNMADDSFloatdp3 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x1f208000 {
        return Some(InstructionKind::FNMSUBSFloatdp3 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x1f400000 {
        return Some(InstructionKind::FMADDDFloatdp3 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x1f408000 {
        return Some(InstructionKind::FMSUBDFloatdp3 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x1f600000 {
        return Some(InstructionKind::FNMADDDFloatdp3 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x1f608000 {
        return Some(InstructionKind::FNMSUBDFloatdp3 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x1fc00000 {
        return Some(InstructionKind::FMADDHFloatdp3 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x1fc08000 {
        return Some(InstructionKind::FMSUBHFloatdp3 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x1fe00000 {
        return Some(InstructionKind::FNMADDHFloatdp3 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    if (d & 0xffe08000) == 0x1fe08000 {
        return Some(InstructionKind::FNMSUBHFloatdp3 {
            Ra: Ra as _,
            Rd: Rd as _,
            Rm: Rm as _,
            Rn: Rn as _,
        });
    }
    None
}
pub const fn decode_floatimm(d: u32) -> Option<InstructionKind> {
    ["Could not decode."][((d & 0x5f201c00) != 0x1e201000) as usize];
    let M = (d >> 31) & 1;
    let Rd = d & 0x1F;
    let S = (d >> 29) & 1;
    let imm5 = (d >> 5) & 0x1F;
    let imm8 = (d >> 13) & 0xFF;
    let kind = (d >> 22) & 3;
    if (d & 0xffe01fe0) == 0x1e201000 {
        return Some(InstructionKind::FMOVSFloatimm {
            Rd: Rd as _,
            imm8: imm8 as _,
        });
    }
    if (d & 0xffe01fe0) == 0x1e601000 {
        return Some(InstructionKind::FMOVDFloatimm {
            Rd: Rd as _,
            imm8: imm8 as _,
        });
    }
    if (d & 0xffe01fe0) == 0x1ee01000 {
        return Some(InstructionKind::FMOVHFloatimm {
            Rd: Rd as _,
            imm8: imm8 as _,
        });
    }
    None
}

pub const fn is_ldst_immpre(d: u32) -> bool {
    (d & 0x3b200c00) == 0x38000c00
}
pub const fn is_memop(d: u32) -> bool {
    (d & 0x3b200c00) == 0x38200000
}
pub const fn is_ldst_regoff(d: u32) -> bool {
    (d & 0x3b200c00) == 0x38200800
}
pub const fn is_ldst_pac(d: u32) -> bool {
    (d & 0x3b200400) == 0x38200400
}
pub const fn is_ldst_pos(d: u32) -> bool {
    (d & 0x3b000000) == 0x39000000
}
pub const fn is_dp_2src(d: u32) -> bool {
    (d & 0x5fe00000) == 0x1ac00000
}
pub const fn is_dp_1src(d: u32) -> bool {
    (d & 0x5fe00000) == 0x5ac00000
}
pub const fn is_log_shift(d: u32) -> bool {
    (d & 0x1f000000) == 0x0a000000
}
pub const fn is_addsub_shift(d: u32) -> bool {
    (d & 0x1f200000) == 0x0b000000
}
pub const fn is_addsub_ext(d: u32) -> bool {
    (d & 0x1f200000) == 0x0b200000
}
pub const fn is_addsub_carry(d: u32) -> bool {
    (d & 0x1fe00000) == 0x1a000000
}
pub const fn is_condcmp_reg(d: u32) -> bool {
    (d & 0x1fe00800) == 0x1a400000
}
pub const fn is_condcmp_imm(d: u32) -> bool {
    (d & 0x1fe00800) == 0x1a400800
}
pub const fn is_condsel(d: u32) -> bool {
    (d & 0x1fe00000) == 0x1a800000
}
pub const fn is_dp_3src(d: u32) -> bool {
    (d & 0x1f000000) == 0x1b000000
}
pub const fn is_cryptoaes(d: u32) -> bool {
    (d & 0xff3e0c00) == 0x4e280800
}
pub const fn is_cryptosha3(d: u32) -> bool {
    (d & 0xff208c00) == 0x5e000000
}
pub const fn is_cryptosha2(d: u32) -> bool {
    (d & 0xff3e0c00) == 0x5e280800
}
pub const fn is_asisdone(d: u32) -> bool {
    (d & 0xdfe08400) == 0x5e000400
}
pub const fn is_asisdsamefp16(d: u32) -> bool {
    (d & 0xdf60c400) == 0x5e400400
}
pub const fn is_asisdmiscfp16(d: u32) -> bool {
    (d & 0xdf7e0c00) == 0x5e780800
}
pub const fn is_asisdsame2(d: u32) -> bool {
    (d & 0xdf208400) == 0x5e008400
}
pub const fn is_asisdmisc(d: u32) -> bool {
    (d & 0xdf3e0c00) == 0x5e200800
}
pub const fn is_asisdpair(d: u32) -> bool {
    (d & 0xdf3e0c00) == 0x5e300800
}
pub const fn is_asisddiff(d: u32) -> bool {
    (d & 0xdf200c00) == 0x5e200000
}
pub const fn is_asisdsame(d: u32) -> bool {
    (d & 0xdf200400) == 0x5e200400
}
pub const fn is_asisdshf(d: u32) -> bool {
    (d & 0xdf800400) == 0x5f000400
}
pub const fn is_asisdelem(d: u32) -> bool {
    (d & 0xdf000400) == 0x5f000000
}
pub const fn is_asimdtbl(d: u32) -> bool {
    (d & 0xbf208c00) == 0x0e000000
}
pub const fn is_asimdperm(d: u32) -> bool {
    (d & 0xbf208c00) == 0x0e000800
}
pub const fn is_asimdext(d: u32) -> bool {
    (d & 0xbf208400) == 0x2e000000
}
pub const fn is_asimdins(d: u32) -> bool {
    (d & 0x9fe08400) == 0x0e000400
}
pub const fn is_asimdsamefp16(d: u32) -> bool {
    (d & 0x9f60c400) == 0x0e400400
}
pub const fn is_asimdmiscfp16(d: u32) -> bool {
    (d & 0x9f7e0c00) == 0x0e780800
}
pub const fn is_asimdsame2(d: u32) -> bool {
    (d & 0x9f208400) == 0x0e008400
}
pub const fn is_asimdmisc(d: u32) -> bool {
    (d & 0x9f3e0c00) == 0x0e200800
}
pub const fn is_asimdall(d: u32) -> bool {
    (d & 0x9f3e0c00) == 0x0e300800
}
pub const fn is_asimddiff(d: u32) -> bool {
    (d & 0x9f200c00) == 0x0e200000
}
pub const fn is_asimdsame(d: u32) -> bool {
    (d & 0x9f200400) == 0x0e200400
}
pub const fn is_asimdimm(d: u32) -> bool {
    (d & 0x9ff80400) == 0x0f000400
}
pub const fn is_asimdshf(d: u32) -> bool {
    (d & 0x9f800400) == 0x0f000400 && (d & 0x780000) != 0x000000
}
pub const fn is_asimdelem(d: u32) -> bool {
    (d & 0x9f000400) == 0x0f000000
}
pub const fn is_crypto3_imm2(d: u32) -> bool {
    (d & 0xffe0c000) == 0xce408000
}
pub const fn is_cryptosha512_3(d: u32) -> bool {
    (d & 0xffe0b000) == 0xce608000
}
pub const fn is_crypto4(d: u32) -> bool {
    (d & 0xff808000) == 0xce000000
}
pub const fn is_crypto3_imm6(d: u32) -> bool {
    (d & 0xffe00000) == 0xce800000
}
pub const fn is_cryptosha512_2(d: u32) -> bool {
    (d & 0xfffff000) == 0xcec08000
}
pub const fn is_float2fix(d: u32) -> bool {
    (d & 0x5f200000) == 0x1e000000
}
pub const fn is_float2int(d: u32) -> bool {
    (d & 0x5f20fc00) == 0x1e200000
}
pub const fn is_floatdp1(d: u32) -> bool {
    (d & 0x5f207c00) == 0x1e204000
}
pub const fn is_floatcmp(d: u32) -> bool {
    (d & 0x5f203c00) == 0x1e202000
}
pub const fn is_floatimm(d: u32) -> bool {
    (d & 0x5f201c00) == 0x1e201000
}
pub const fn is_floatccmp(d: u32) -> bool {
    (d & 0x5f200c00) == 0x1e200400
}
pub const fn is_floatdp2(d: u32) -> bool {
    (d & 0x5f200c00) == 0x1e200800
}
pub const fn is_floatsel(d: u32) -> bool {
    (d & 0x5f200c00) == 0x1e200c00
}
pub const fn is_floatdp3(d: u32) -> bool {
    (d & 0x5f000000) == 0x1f000000
}
pub const fn is_CBZ_32_compbranch(d: u32) -> bool {
    (d & 0xff000000) == 0x34000000
}
pub const fn is_CBNZ_32_compbranch(d: u32) -> bool {
    (d & 0xff000000) == 0x35000000
}
pub const fn is_CBZ_64_compbranch(d: u32) -> bool {
    (d & 0xff000000) == 0xb4000000
}
pub const fn is_CBNZ_64_compbranch(d: u32) -> bool {
    (d & 0xff000000) == 0xb5000000
}
pub const fn is_B_only_condbranch(d: u32) -> bool {
    (d & 0xff000010) == 0x54000000
}
pub const fn is_SVC_EX_exception(d: u32) -> bool {
    (d & 0xffe0001f) == 0xd4000001
}
pub const fn is_HVC_EX_exception(d: u32) -> bool {
    (d & 0xffe0001f) == 0xd4000002
}
pub const fn is_SMC_EX_exception(d: u32) -> bool {
    (d & 0xffe0001f) == 0xd4000003
}
pub const fn is_BRK_EX_exception(d: u32) -> bool {
    (d & 0xffe0001f) == 0xd4200000
}
pub const fn is_HLT_EX_exception(d: u32) -> bool {
    (d & 0xffe0001f) == 0xd4400000
}
pub const fn is_DCPS1_DC_exception(d: u32) -> bool {
    (d & 0xffe0001f) == 0xd4a00001
}
pub const fn is_DCPS2_DC_exception(d: u32) -> bool {
    (d & 0xffe0001f) == 0xd4a00002
}
pub const fn is_DCPS3_DC_exception(d: u32) -> bool {
    (d & 0xffe0001f) == 0xd4a00003
}
pub const fn is_MSR_SI_system(d: u32) -> bool {
    (d & 0xfff8f01f) == 0xd500401f
}
pub const fn is_HINT_2(d: u32) -> bool {
    (d & 0xfffff01f) == 0xd503201f && (d & 0x000d00) != 0x000000
}
pub const fn is_NOP_HI_system(d: u32) -> bool {
    d == 0xd503201f
}
pub const fn is_YIELD_HI_system(d: u32) -> bool {
    d == 0xd503203f
}
pub const fn is_WFE_HI_system(d: u32) -> bool {
    d == 0xd503205f
}
pub const fn is_WFI_HI_system(d: u32) -> bool {
    d == 0xd503207f
}
pub const fn is_SEV_HI_system(d: u32) -> bool {
    d == 0xd503209f
}
pub const fn is_SEVL_HI_system(d: u32) -> bool {
    d == 0xd50320bf
}
pub const fn is_HINT_1(d: u32) -> bool {
    (d & 0xffffffdf) == 0xd50320df
}
pub const fn is_XPACLRI_HI_system(d: u32) -> bool {
    d == 0xd50320ff
}
pub const fn is_PACIA1716_HI_system(d: u32) -> bool {
    d == 0xd503211f
}
pub const fn is_PACIB1716_HI_system(d: u32) -> bool {
    d == 0xd503215f
}
pub const fn is_AUTIA1716_HI_system(d: u32) -> bool {
    d == 0xd503219f
}
pub const fn is_AUTIB1716_HI_system(d: u32) -> bool {
    d == 0xd50321df
}
pub const fn is_HINT_3(d: u32) -> bool {
    (d & 0xffffff1f) == 0xd503221f && (d & 0x0000c0) != 0x000000
}
pub const fn is_ESB_HI_system(d: u32) -> bool {
    d == 0xd503221f
}
pub const fn is_PSB_HC_system(d: u32) -> bool {
    d == 0xd503223f
}
pub const fn is_PACIAZ_HI_system(d: u32) -> bool {
    d == 0xd503231f
}
pub const fn is_PACIASP_HI_system(d: u32) -> bool {
    d == 0xd503233f
}
pub const fn is_PACIBZ_HI_system(d: u32) -> bool {
    d == 0xd503235f
}
pub const fn is_PACIBSP_HI_system(d: u32) -> bool {
    d == 0xd503237f
}
pub const fn is_AUTIAZ_HI_system(d: u32) -> bool {
    d == 0xd503239f
}
pub const fn is_AUTIASP_HI_system(d: u32) -> bool {
    d == 0xd50323bf
}
pub const fn is_AUTIBZ_HI_system(d: u32) -> bool {
    d == 0xd50323df
}
pub const fn is_AUTIBSP_HI_system(d: u32) -> bool {
    d == 0xd50323ff
}
pub const fn is_CLREX_BN_system(d: u32) -> bool {
    (d & 0xfffff0ff) == 0xd503305f
}
pub const fn is_DSB_BO_system(d: u32) -> bool {
    (d & 0xfffff0ff) == 0xd503309f
}
pub const fn is_DMB_BO_system(d: u32) -> bool {
    (d & 0xfffff0ff) == 0xd50330bf
}
pub const fn is_ISB_BI_system(d: u32) -> bool {
    (d & 0xfffff0ff) == 0xd50330df
}
pub const fn is_SYS_CR_system(d: u32) -> bool {
    (d & 0xfff80000) == 0xd5080000
}
pub const fn is_MSR_SR_system(d: u32) -> bool {
    (d & 0xfff00000) == 0xd5100000
}
pub const fn is_SYSL_RC_system(d: u32) -> bool {
    (d & 0xfff80000) == 0xd5280000
}
pub const fn is_MRS_RS_system(d: u32) -> bool {
    (d & 0xfff00000) == 0xd5300000
}
pub const fn is_TBZ_only_testbranch(d: u32) -> bool {
    (d & 0x7f000000) == 0x36000000
}
pub const fn is_TBNZ_only_testbranch(d: u32) -> bool {
    (d & 0x7f000000) == 0x37000000
}
pub const fn is_B_only_branch_imm(d: u32) -> bool {
    (d & 0xfc000000) == 0x14000000
}
pub const fn is_BL_only_branch_imm(d: u32) -> bool {
    (d & 0xfc000000) == 0x94000000
}
pub const fn is_BR_64_branch_reg(d: u32) -> bool {
    (d & 0xfffffc1f) == 0xd61f0000
}
pub const fn is_BRAAZ_64_branch_reg(d: u32) -> bool {
    (d & 0xfffffc1f) == 0xd61f081f
}
pub const fn is_BRABZ_64_branch_reg(d: u32) -> bool {
    (d & 0xfffffc1f) == 0xd61f0c1f
}
pub const fn is_BLR_64_branch_reg(d: u32) -> bool {
    (d & 0xfffffc1f) == 0xd63f0000
}
pub const fn is_BLRAAZ_64_branch_reg(d: u32) -> bool {
    (d & 0xfffffc1f) == 0xd63f081f
}
pub const fn is_BLRABZ_64_branch_reg(d: u32) -> bool {
    (d & 0xfffffc1f) == 0xd63f0c1f
}
pub const fn is_RET_64R_branch_reg(d: u32) -> bool {
    (d & 0xfffffc1f) == 0xd65f0000
}
pub const fn is_RETAA_64E_branch_reg(d: u32) -> bool {
    d == 0xd65f0bff
}
pub const fn is_RETAB_64E_branch_reg(d: u32) -> bool {
    d == 0xd65f0fff
}
pub const fn is_ERET_64E_branch_reg(d: u32) -> bool {
    d == 0xd69f03e0
}
pub const fn is_ERETAA_64E_branch_reg(d: u32) -> bool {
    d == 0xd69f0bff
}
pub const fn is_ERETAB_64E_branch_reg(d: u32) -> bool {
    d == 0xd69f0fff
}
pub const fn is_DRPS_64E_branch_reg(d: u32) -> bool {
    d == 0xd6bf03e0
}
pub const fn is_BRAA_64P_branch_reg(d: u32) -> bool {
    (d & 0xfffffc00) == 0xd71f0800
}
pub const fn is_BRAB_64P_branch_reg(d: u32) -> bool {
    (d & 0xfffffc00) == 0xd71f0c00
}
pub const fn is_BLRAA_64P_branch_reg(d: u32) -> bool {
    (d & 0xfffffc00) == 0xd73f0800
}
pub const fn is_BLRAB_64P_branch_reg(d: u32) -> bool {
    (d & 0xfffffc00) == 0xd73f0c00
}
pub const fn is_ST4_asisdlse_R4(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c000000
}
pub const fn is_ST1_asisdlse_R4_4v(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c002000
}
pub const fn is_ST3_asisdlse_R3(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c004000
}
pub const fn is_ST1_asisdlse_R3_3v(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c006000
}
pub const fn is_ST1_asisdlse_R1_1v(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c007000
}
pub const fn is_ST2_asisdlse_R2(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c008000
}
pub const fn is_ST1_asisdlse_R2_2v(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c00a000
}
pub const fn is_LD4_asisdlse_R4(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c400000
}
pub const fn is_LD1_asisdlse_R4_4v(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c402000
}
pub const fn is_LD3_asisdlse_R3(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c404000
}
pub const fn is_LD1_asisdlse_R3_3v(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c406000
}
pub const fn is_LD1_asisdlse_R1_1v(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c407000
}
pub const fn is_LD2_asisdlse_R2(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c408000
}
pub const fn is_LD1_asisdlse_R2_2v(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c40a000
}
pub const fn is_ST4_asisdlsep_R4_r(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0c800000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST1_asisdlsep_R4_r4(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0c802000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST3_asisdlsep_R3_r(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0c804000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST1_asisdlsep_R3_r3(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0c806000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST1_asisdlsep_R1_r1(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0c807000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST2_asisdlsep_R2_r(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0c808000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST1_asisdlsep_R2_r2(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0c80a000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST4_asisdlsep_I4_i(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c9f0000
}
pub const fn is_ST1_asisdlsep_I4_i4(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c9f2000
}
pub const fn is_ST3_asisdlsep_I3_i(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c9f4000
}
pub const fn is_ST1_asisdlsep_I3_i3(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c9f6000
}
pub const fn is_ST1_asisdlsep_I1_i1(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c9f7000
}
pub const fn is_ST2_asisdlsep_I2_i(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c9f8000
}
pub const fn is_ST1_asisdlsep_I2_i2(d: u32) -> bool {
    (d & 0xbffff000) == 0x0c9fa000
}
pub const fn is_LD4_asisdlsep_R4_r(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0cc00000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD1_asisdlsep_R4_r4(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0cc02000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD3_asisdlsep_R3_r(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0cc04000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD1_asisdlsep_R3_r3(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0cc06000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD1_asisdlsep_R1_r1(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0cc07000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD2_asisdlsep_R2_r(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0cc08000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD1_asisdlsep_R2_r2(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0cc0a000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD4_asisdlsep_I4_i(d: u32) -> bool {
    (d & 0xbffff000) == 0x0cdf0000
}
pub const fn is_LD1_asisdlsep_I4_i4(d: u32) -> bool {
    (d & 0xbffff000) == 0x0cdf2000
}
pub const fn is_LD3_asisdlsep_I3_i(d: u32) -> bool {
    (d & 0xbffff000) == 0x0cdf4000
}
pub const fn is_LD1_asisdlsep_I3_i3(d: u32) -> bool {
    (d & 0xbffff000) == 0x0cdf6000
}
pub const fn is_LD1_asisdlsep_I1_i1(d: u32) -> bool {
    (d & 0xbffff000) == 0x0cdf7000
}
pub const fn is_LD2_asisdlsep_I2_i(d: u32) -> bool {
    (d & 0xbffff000) == 0x0cdf8000
}
pub const fn is_LD1_asisdlsep_I2_i2(d: u32) -> bool {
    (d & 0xbffff000) == 0x0cdfa000
}
pub const fn is_ST1_asisdlso_B1_1b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0d000000
}
pub const fn is_ST3_asisdlso_B3_3b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0d002000
}
pub const fn is_ST1_asisdlso_H1_1h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0d004000
}
pub const fn is_ST3_asisdlso_H3_3h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0d006000
}
pub const fn is_ST1_asisdlso_S1_1s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0d008000
}
pub const fn is_ST1_asisdlso_D1_1d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0d008400
}
pub const fn is_ST3_asisdlso_S3_3s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0d00a000
}
pub const fn is_ST3_asisdlso_D3_3d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0d00a400
}
pub const fn is_ST2_asisdlso_B2_2b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0d200000
}
pub const fn is_ST4_asisdlso_B4_4b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0d202000
}
pub const fn is_ST2_asisdlso_H2_2h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0d204000
}
pub const fn is_ST4_asisdlso_H4_4h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0d206000
}
pub const fn is_ST2_asisdlso_S2_2s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0d208000
}
pub const fn is_ST2_asisdlso_D2_2d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0d208400
}
pub const fn is_ST4_asisdlso_S4_4s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0d20a000
}
pub const fn is_ST4_asisdlso_D4_4d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0d20a400
}
pub const fn is_LD1_asisdlso_B1_1b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0d400000
}
pub const fn is_LD3_asisdlso_B3_3b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0d402000
}
pub const fn is_LD1_asisdlso_H1_1h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0d404000
}
pub const fn is_LD3_asisdlso_H3_3h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0d406000
}
pub const fn is_LD1_asisdlso_S1_1s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0d408000
}
pub const fn is_LD1_asisdlso_D1_1d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0d408400
}
pub const fn is_LD3_asisdlso_S3_3s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0d40a000
}
pub const fn is_LD3_asisdlso_D3_3d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0d40a400
}
pub const fn is_LD1R_asisdlso_R1(d: u32) -> bool {
    (d & 0xbffff000) == 0x0d40c000
}
pub const fn is_LD3R_asisdlso_R3(d: u32) -> bool {
    (d & 0xbffff000) == 0x0d40e000
}
pub const fn is_LD2_asisdlso_B2_2b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0d600000
}
pub const fn is_LD4_asisdlso_B4_4b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0d602000
}
pub const fn is_LD2_asisdlso_H2_2h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0d604000
}
pub const fn is_LD4_asisdlso_H4_4h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0d606000
}
pub const fn is_LD2_asisdlso_S2_2s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0d608000
}
pub const fn is_LD2_asisdlso_D2_2d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0d608400
}
pub const fn is_LD4_asisdlso_S4_4s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0d60a000
}
pub const fn is_LD4_asisdlso_D4_4d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0d60a400
}
pub const fn is_LD2R_asisdlso_R2(d: u32) -> bool {
    (d & 0xbffff000) == 0x0d60c000
}
pub const fn is_LD4R_asisdlso_R4(d: u32) -> bool {
    (d & 0xbffff000) == 0x0d60e000
}
pub const fn is_ST1_asisdlsop_BX1_r1b(d: u32) -> bool {
    (d & 0xbfe0e000) == 0x0d800000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST3_asisdlsop_BX3_r3b(d: u32) -> bool {
    (d & 0xbfe0e000) == 0x0d802000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST1_asisdlsop_HX1_r1h(d: u32) -> bool {
    (d & 0xbfe0e400) == 0x0d804000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST3_asisdlsop_HX3_r3h(d: u32) -> bool {
    (d & 0xbfe0e400) == 0x0d806000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST1_asisdlsop_SX1_r1s(d: u32) -> bool {
    (d & 0xbfe0ec00) == 0x0d808000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST1_asisdlsop_DX1_r1d(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0d808400 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST3_asisdlsop_SX3_r3s(d: u32) -> bool {
    (d & 0xbfe0ec00) == 0x0d80a000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST3_asisdlsop_DX3_r3d(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0d80a400 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST1_asisdlsop_B1_i1b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0d9f0000
}
pub const fn is_ST3_asisdlsop_B3_i3b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0d9f2000
}
pub const fn is_ST1_asisdlsop_H1_i1h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0d9f4000
}
pub const fn is_ST3_asisdlsop_H3_i3h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0d9f6000
}
pub const fn is_ST1_asisdlsop_S1_i1s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0d9f8000
}
pub const fn is_ST1_asisdlsop_D1_i1d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0d9f8400
}
pub const fn is_ST3_asisdlsop_S3_i3s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0d9fa000
}
pub const fn is_ST3_asisdlsop_D3_i3d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0d9fa400
}
pub const fn is_ST2_asisdlsop_BX2_r2b(d: u32) -> bool {
    (d & 0xbfe0e000) == 0x0da00000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST4_asisdlsop_BX4_r4b(d: u32) -> bool {
    (d & 0xbfe0e000) == 0x0da02000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST2_asisdlsop_HX2_r2h(d: u32) -> bool {
    (d & 0xbfe0e400) == 0x0da04000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST4_asisdlsop_HX4_r4h(d: u32) -> bool {
    (d & 0xbfe0e400) == 0x0da06000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST2_asisdlsop_SX2_r2s(d: u32) -> bool {
    (d & 0xbfe0ec00) == 0x0da08000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST2_asisdlsop_DX2_r2d(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0da08400 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST4_asisdlsop_SX4_r4s(d: u32) -> bool {
    (d & 0xbfe0ec00) == 0x0da0a000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST4_asisdlsop_DX4_r4d(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0da0a400 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_ST2_asisdlsop_B2_i2b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0dbf0000
}
pub const fn is_ST4_asisdlsop_B4_i4b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0dbf2000
}
pub const fn is_ST2_asisdlsop_H2_i2h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0dbf4000
}
pub const fn is_ST4_asisdlsop_H4_i4h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0dbf6000
}
pub const fn is_ST2_asisdlsop_S2_i2s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0dbf8000
}
pub const fn is_ST2_asisdlsop_D2_i2d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0dbf8400
}
pub const fn is_ST4_asisdlsop_S4_i4s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0dbfa000
}
pub const fn is_ST4_asisdlsop_D4_i4d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0dbfa400
}
pub const fn is_LD1_asisdlsop_BX1_r1b(d: u32) -> bool {
    (d & 0xbfe0e000) == 0x0dc00000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD3_asisdlsop_BX3_r3b(d: u32) -> bool {
    (d & 0xbfe0e000) == 0x0dc02000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD1_asisdlsop_HX1_r1h(d: u32) -> bool {
    (d & 0xbfe0e400) == 0x0dc04000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD3_asisdlsop_HX3_r3h(d: u32) -> bool {
    (d & 0xbfe0e400) == 0x0dc06000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD1_asisdlsop_SX1_r1s(d: u32) -> bool {
    (d & 0xbfe0ec00) == 0x0dc08000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD1_asisdlsop_DX1_r1d(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0dc08400 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD3_asisdlsop_SX3_r3s(d: u32) -> bool {
    (d & 0xbfe0ec00) == 0x0dc0a000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD3_asisdlsop_DX3_r3d(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0dc0a400 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD1R_asisdlsop_RX1_r(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0dc0c000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD3R_asisdlsop_RX3_r(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0dc0e000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD1_asisdlsop_B1_i1b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0ddf0000
}
pub const fn is_LD3_asisdlsop_B3_i3b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0ddf2000
}
pub const fn is_LD1_asisdlsop_H1_i1h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0ddf4000
}
pub const fn is_LD3_asisdlsop_H3_i3h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0ddf6000
}
pub const fn is_LD1_asisdlsop_S1_i1s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0ddf8000
}
pub const fn is_LD1_asisdlsop_D1_i1d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0ddf8400
}
pub const fn is_LD3_asisdlsop_S3_i3s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0ddfa000
}
pub const fn is_LD3_asisdlsop_D3_i3d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0ddfa400
}
pub const fn is_LD1R_asisdlsop_R1_i(d: u32) -> bool {
    (d & 0xbffff000) == 0x0ddfc000
}
pub const fn is_LD3R_asisdlsop_R3_i(d: u32) -> bool {
    (d & 0xbffff000) == 0x0ddfe000
}
pub const fn is_LD2_asisdlsop_BX2_r2b(d: u32) -> bool {
    (d & 0xbfe0e000) == 0x0de00000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD4_asisdlsop_BX4_r4b(d: u32) -> bool {
    (d & 0xbfe0e000) == 0x0de02000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD2_asisdlsop_HX2_r2h(d: u32) -> bool {
    (d & 0xbfe0e400) == 0x0de04000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD4_asisdlsop_HX4_r4h(d: u32) -> bool {
    (d & 0xbfe0e400) == 0x0de06000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD2_asisdlsop_SX2_r2s(d: u32) -> bool {
    (d & 0xbfe0ec00) == 0x0de08000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD2_asisdlsop_DX2_r2d(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0de08400 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD4_asisdlsop_SX4_r4s(d: u32) -> bool {
    (d & 0xbfe0ec00) == 0x0de0a000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD4_asisdlsop_DX4_r4d(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0de0a400 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD2R_asisdlsop_RX2_r(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0de0c000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD4R_asisdlsop_RX4_r(d: u32) -> bool {
    (d & 0xbfe0f000) == 0x0de0e000 && (d & 0x1f0000) != 0x1f0000
}
pub const fn is_LD2_asisdlsop_B2_i2b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0dff0000
}
pub const fn is_LD4_asisdlsop_B4_i4b(d: u32) -> bool {
    (d & 0xbfffe000) == 0x0dff2000
}
pub const fn is_LD2_asisdlsop_H2_i2h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0dff4000
}
pub const fn is_LD4_asisdlsop_H4_i4h(d: u32) -> bool {
    (d & 0xbfffe400) == 0x0dff6000
}
pub const fn is_LD2_asisdlsop_S2_i2s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0dff8000
}
pub const fn is_LD2_asisdlsop_D2_i2d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0dff8400
}
pub const fn is_LD4_asisdlsop_S4_i4s(d: u32) -> bool {
    (d & 0xbfffec00) == 0x0dffa000
}
pub const fn is_LD4_asisdlsop_D4_i4d(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0dffa400
}
pub const fn is_LD2R_asisdlsop_R2_i(d: u32) -> bool {
    (d & 0xbffff000) == 0x0dffc000
}
pub const fn is_LD4R_asisdlsop_R4_i(d: u32) -> bool {
    (d & 0xbffff000) == 0x0dffe000
}
pub const fn is_LDADDB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38200000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STADDB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3820001f
}
pub const fn is_LDCLRB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38201000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STCLRB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3820101f
}
pub const fn is_LDEORB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38202000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STEORB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3820201f
}
pub const fn is_LDSETB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38203000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSETB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3820301f
}
pub const fn is_LDSMAXB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38204000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMAXB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3820401f
}
pub const fn is_LDSMINB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38205000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMINB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3820501f
}
pub const fn is_LDUMAXB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38206000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMAXB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3820601f
}
pub const fn is_LDUMINB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38207000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMINB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3820701f
}
pub const fn is_SWPB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38208000
}
pub const fn is_LDADDLB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38600000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STADDLB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3860001f
}
pub const fn is_LDCLRLB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38601000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STCLRLB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3860101f
}
pub const fn is_LDEORLB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38602000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STEORLB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3860201f
}
pub const fn is_LDSETLB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38603000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSETLB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3860301f
}
pub const fn is_LDSMAXLB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38604000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMAXLB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3860401f
}
pub const fn is_LDSMINLB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38605000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMINLB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3860501f
}
pub const fn is_LDUMAXLB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38606000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMAXLB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3860601f
}
pub const fn is_LDUMINLB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38607000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMINLB_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x3860701f
}
pub const fn is_SWPLB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38608000
}
pub const fn is_LDADDAB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38a00000
}
pub const fn is_LDCLRAB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38a01000
}
pub const fn is_LDEORAB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38a02000
}
pub const fn is_LDSETAB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38a03000
}
pub const fn is_LDSMAXAB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38a04000
}
pub const fn is_LDSMINAB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38a05000
}
pub const fn is_LDUMAXAB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38a06000
}
pub const fn is_LDUMINAB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38a07000
}
pub const fn is_SWPAB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38a08000
}
pub const fn is_LDAPRB_32L_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38a0c000
}
pub const fn is_LDADDALB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38e00000
}
pub const fn is_LDCLRALB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38e01000
}
pub const fn is_LDEORALB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38e02000
}
pub const fn is_LDSETALB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38e03000
}
pub const fn is_LDSMAXALB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38e04000
}
pub const fn is_LDSMINALB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38e05000
}
pub const fn is_LDUMAXALB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38e06000
}
pub const fn is_LDUMINALB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38e07000
}
pub const fn is_SWPALB_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x38e08000
}
pub const fn is_LDADDH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78200000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STADDH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7820001f
}
pub const fn is_LDCLRH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78201000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STCLRH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7820101f
}
pub const fn is_LDEORH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78202000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STEORH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7820201f
}
pub const fn is_LDSETH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78203000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSETH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7820301f
}
pub const fn is_LDSMAXH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78204000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMAXH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7820401f
}
pub const fn is_LDSMINH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78205000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMINH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7820501f
}
pub const fn is_LDUMAXH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78206000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMAXH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7820601f
}
pub const fn is_LDUMINH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78207000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMINH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7820701f
}
pub const fn is_SWPH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78208000
}
pub const fn is_LDADDLH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78600000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STADDLH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7860001f
}
pub const fn is_LDCLRLH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78601000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STCLRLH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7860101f
}
pub const fn is_LDEORLH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78602000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STEORLH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7860201f
}
pub const fn is_LDSETLH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78603000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSETLH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7860301f
}
pub const fn is_LDSMAXLH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78604000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMAXLH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7860401f
}
pub const fn is_LDSMINLH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78605000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMINLH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7860501f
}
pub const fn is_LDUMAXLH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78606000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMAXLH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7860601f
}
pub const fn is_LDUMINLH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78607000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMINLH_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x7860701f
}
pub const fn is_SWPLH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78608000
}
pub const fn is_LDADDAH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78a00000
}
pub const fn is_LDCLRAH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78a01000
}
pub const fn is_LDEORAH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78a02000
}
pub const fn is_LDSETAH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78a03000
}
pub const fn is_LDSMAXAH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78a04000
}
pub const fn is_LDSMINAH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78a05000
}
pub const fn is_LDUMAXAH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78a06000
}
pub const fn is_LDUMINAH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78a07000
}
pub const fn is_SWPAH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78a08000
}
pub const fn is_LDAPRH_32L_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78a0c000
}
pub const fn is_LDADDALH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78e00000
}
pub const fn is_LDCLRALH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78e01000
}
pub const fn is_LDEORALH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78e02000
}
pub const fn is_LDSETALH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78e03000
}
pub const fn is_LDSMAXALH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78e04000
}
pub const fn is_LDSMINALH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78e05000
}
pub const fn is_LDUMAXALH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78e06000
}
pub const fn is_LDUMINALH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78e07000
}
pub const fn is_SWPALH_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x78e08000
}
pub const fn is_LDADD_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8200000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STADD_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb820001f
}
pub const fn is_LDCLR_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8201000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STCLR_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb820101f
}
pub const fn is_LDEOR_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8202000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STEOR_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb820201f
}
pub const fn is_LDSET_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8203000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSET_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb820301f
}
pub const fn is_LDSMAX_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8204000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMAX_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb820401f
}
pub const fn is_LDSMIN_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8205000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMIN_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb820501f
}
pub const fn is_LDUMAX_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8206000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMAX_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb820601f
}
pub const fn is_LDUMIN_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8207000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMIN_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb820701f
}
pub const fn is_SWP_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8208000
}
pub const fn is_LDADDL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8600000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STADDL_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb860001f
}
pub const fn is_LDCLRL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8601000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STCLRL_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb860101f
}
pub const fn is_LDEORL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8602000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STEORL_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb860201f
}
pub const fn is_LDSETL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8603000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSETL_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb860301f
}
pub const fn is_LDSMAXL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8604000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMAXL_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb860401f
}
pub const fn is_LDSMINL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8605000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMINL_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb860501f
}
pub const fn is_LDUMAXL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8606000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMAXL_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb860601f
}
pub const fn is_LDUMINL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8607000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMINL_32S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xb860701f
}
pub const fn is_SWPL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8608000
}
pub const fn is_LDADDA_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8a00000
}
pub const fn is_LDCLRA_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8a01000
}
pub const fn is_LDEORA_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8a02000
}
pub const fn is_LDSETA_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8a03000
}
pub const fn is_LDSMAXA_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8a04000
}
pub const fn is_LDSMINA_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8a05000
}
pub const fn is_LDUMAXA_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8a06000
}
pub const fn is_LDUMINA_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8a07000
}
pub const fn is_SWPA_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8a08000
}
pub const fn is_LDAPR_32L_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8a0c000
}
pub const fn is_LDADDAL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8e00000
}
pub const fn is_LDCLRAL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8e01000
}
pub const fn is_LDEORAL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8e02000
}
pub const fn is_LDSETAL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8e03000
}
pub const fn is_LDSMAXAL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8e04000
}
pub const fn is_LDSMINAL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8e05000
}
pub const fn is_LDUMAXAL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8e06000
}
pub const fn is_LDUMINAL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8e07000
}
pub const fn is_SWPAL_32_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xb8e08000
}
pub const fn is_LDADD_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8200000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STADD_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf820001f
}
pub const fn is_LDCLR_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8201000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STCLR_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf820101f
}
pub const fn is_LDEOR_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8202000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STEOR_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf820201f
}
pub const fn is_LDSET_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8203000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSET_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf820301f
}
pub const fn is_LDSMAX_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8204000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMAX_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf820401f
}
pub const fn is_LDSMIN_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8205000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMIN_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf820501f
}
pub const fn is_LDUMAX_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8206000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMAX_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf820601f
}
pub const fn is_LDUMIN_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8207000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMIN_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf820701f
}
pub const fn is_SWP_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8208000
}
pub const fn is_LDADDL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8600000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STADDL_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf860001f
}
pub const fn is_LDCLRL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8601000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STCLRL_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf860101f
}
pub const fn is_LDEORL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8602000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STEORL_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf860201f
}
pub const fn is_LDSETL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8603000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSETL_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf860301f
}
pub const fn is_LDSMAXL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8604000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMAXL_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf860401f
}
pub const fn is_LDSMINL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8605000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STSMINL_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf860501f
}
pub const fn is_LDUMAXL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8606000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMAXL_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf860601f
}
pub const fn is_LDUMINL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8607000 && (d & 0x00001f) != 0x00001f
}
pub const fn is_STUMINL_64S_memop(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0xf860701f
}
pub const fn is_SWPL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8608000
}
pub const fn is_LDADDA_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8a00000
}
pub const fn is_LDCLRA_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8a01000
}
pub const fn is_LDEORA_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8a02000
}
pub const fn is_LDSETA_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8a03000
}
pub const fn is_LDSMAXA_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8a04000
}
pub const fn is_LDSMINA_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8a05000
}
pub const fn is_LDUMAXA_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8a06000
}
pub const fn is_LDUMINA_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8a07000
}
pub const fn is_SWPA_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8a08000
}
pub const fn is_LDAPR_64L_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8a0c000
}
pub const fn is_LDADDAL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8e00000
}
pub const fn is_LDCLRAL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8e01000
}
pub const fn is_LDEORAL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8e02000
}
pub const fn is_LDSETAL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8e03000
}
pub const fn is_LDSMAXAL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8e04000
}
pub const fn is_LDSMINAL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8e05000
}
pub const fn is_LDUMAXAL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8e06000
}
pub const fn is_LDUMINAL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8e07000
}
pub const fn is_SWPAL_64_memop(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xf8e08000
}
pub const fn is_LDR_32_loadlit(d: u32) -> bool {
    (d & 0xff000000) == 0x18000000
}
pub const fn is_LDR_S_loadlit(d: u32) -> bool {
    (d & 0xff000000) == 0x1c000000
}
pub const fn is_LDR_64_loadlit(d: u32) -> bool {
    (d & 0xff000000) == 0x58000000
}
pub const fn is_LDR_D_loadlit(d: u32) -> bool {
    (d & 0xff000000) == 0x5c000000
}
pub const fn is_LDRSW_64_loadlit(d: u32) -> bool {
    (d & 0xff000000) == 0x98000000
}
pub const fn is_LDR_Q_loadlit(d: u32) -> bool {
    (d & 0xff000000) == 0x9c000000
}
pub const fn is_PRFM_P_loadlit(d: u32) -> bool {
    (d & 0xff000000) == 0xd8000000
}
pub const fn is_STXRB_SR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x08000000
}
pub const fn is_STLXRB_SR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x08008000
}
pub const fn is_CASP_CP32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x08207c00
}
pub const fn is_CASPL_CP32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x0820fc00
}
pub const fn is_LDXRB_LR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x08400000
}
pub const fn is_LDAXRB_LR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x08408000
}
pub const fn is_CASPA_CP32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x08607c00
}
pub const fn is_CASPAL_CP32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x0860fc00
}
pub const fn is_STLLRB_SL32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x08800000
}
pub const fn is_STLRB_SL32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x08808000
}
pub const fn is_CASB_C32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x08a07c00
}
pub const fn is_CASLB_C32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x08a0fc00
}
pub const fn is_LDLARB_LR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x08c00000
}
pub const fn is_LDARB_LR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x08c08000
}
pub const fn is_CASAB_C32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x08e07c00
}
pub const fn is_CASALB_C32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x08e0fc00
}
pub const fn is_STXRH_SR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x48000000
}
pub const fn is_STLXRH_SR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x48008000
}
pub const fn is_CASP_CP64_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x48207c00
}
pub const fn is_CASPL_CP64_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x4820fc00
}
pub const fn is_LDXRH_LR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x48400000
}
pub const fn is_LDAXRH_LR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x48408000
}
pub const fn is_CASPA_CP64_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x48607c00
}
pub const fn is_CASPAL_CP64_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x4860fc00
}
pub const fn is_STLLRH_SL32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x48800000
}
pub const fn is_STLRH_SL32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x48808000
}
pub const fn is_CASH_C32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x48a07c00
}
pub const fn is_CASLH_C32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x48a0fc00
}
pub const fn is_LDLARH_LR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x48c00000
}
pub const fn is_LDARH_LR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x48c08000
}
pub const fn is_CASAH_C32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x48e07c00
}
pub const fn is_CASALH_C32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x48e0fc00
}
pub const fn is_STXR_SR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x88000000
}
pub const fn is_STLXR_SR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x88008000
}
pub const fn is_STXP_SP32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x88200000
}
pub const fn is_STLXP_SP32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x88208000
}
pub const fn is_LDXR_LR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x88400000
}
pub const fn is_LDAXR_LR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x88408000
}
pub const fn is_LDXP_LP32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x88600000
}
pub const fn is_LDAXP_LP32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x88608000
}
pub const fn is_STLLR_SL32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x88800000
}
pub const fn is_STLR_SL32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x88808000
}
pub const fn is_CAS_C32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x88a07c00
}
pub const fn is_CASL_C32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x88a0fc00
}
pub const fn is_LDLAR_LR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x88c00000
}
pub const fn is_LDAR_LR32_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0x88c08000
}
pub const fn is_CASA_C32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x88e07c00
}
pub const fn is_CASAL_C32_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x88e0fc00
}
pub const fn is_STXR_SR64_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0xc8000000
}
pub const fn is_STLXR_SR64_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0xc8008000
}
pub const fn is_STXP_SP64_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0xc8200000
}
pub const fn is_STLXP_SP64_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0xc8208000
}
pub const fn is_LDXR_LR64_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0xc8400000
}
pub const fn is_LDAXR_LR64_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0xc8408000
}
pub const fn is_LDXP_LP64_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0xc8600000
}
pub const fn is_LDAXP_LP64_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0xc8608000
}
pub const fn is_STLLR_SL64_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0xc8800000
}
pub const fn is_STLR_SL64_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0xc8808000
}
pub const fn is_CAS_C64_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xc8a07c00
}
pub const fn is_CASL_C64_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xc8a0fc00
}
pub const fn is_LDLAR_LR64_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0xc8c00000
}
pub const fn is_LDAR_LR64_ldstexcl(d: u32) -> bool {
    (d & 0xffe08000) == 0xc8c08000
}
pub const fn is_CASA_C64_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xc8e07c00
}
pub const fn is_CASAL_C64_ldstexcl(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xc8e0fc00
}
pub const fn is_STNP_32_ldstnapair_offs(d: u32) -> bool {
    (d & 0xffc00000) == 0x28000000
}
pub const fn is_LDNP_32_ldstnapair_offs(d: u32) -> bool {
    (d & 0xffc00000) == 0x28400000
}
pub const fn is_STNP_S_ldstnapair_offs(d: u32) -> bool {
    (d & 0xffc00000) == 0x2c000000
}
pub const fn is_LDNP_S_ldstnapair_offs(d: u32) -> bool {
    (d & 0xffc00000) == 0x2c400000
}
pub const fn is_STNP_D_ldstnapair_offs(d: u32) -> bool {
    (d & 0xffc00000) == 0x6c000000
}
pub const fn is_LDNP_D_ldstnapair_offs(d: u32) -> bool {
    (d & 0xffc00000) == 0x6c400000
}
pub const fn is_STNP_64_ldstnapair_offs(d: u32) -> bool {
    (d & 0xffc00000) == 0xa8000000
}
pub const fn is_LDNP_64_ldstnapair_offs(d: u32) -> bool {
    (d & 0xffc00000) == 0xa8400000
}
pub const fn is_STNP_Q_ldstnapair_offs(d: u32) -> bool {
    (d & 0xffc00000) == 0xac000000
}
pub const fn is_LDNP_Q_ldstnapair_offs(d: u32) -> bool {
    (d & 0xffc00000) == 0xac400000
}
pub const fn is_STRB_32_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38000400
}
pub const fn is_LDRB_32_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38400400
}
pub const fn is_LDRSB_64_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38800400
}
pub const fn is_LDRSB_32_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38c00400
}
pub const fn is_STR_B_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3c000400
}
pub const fn is_LDR_B_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3c400400
}
pub const fn is_STR_Q_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3c800400
}
pub const fn is_LDR_Q_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3cc00400
}
pub const fn is_STRH_32_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78000400
}
pub const fn is_LDRH_32_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78400400
}
pub const fn is_LDRSH_64_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78800400
}
pub const fn is_LDRSH_32_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78c00400
}
pub const fn is_STR_H_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0x7c000400
}
pub const fn is_LDR_H_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0x7c400400
}
pub const fn is_STR_32_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8000400
}
pub const fn is_LDR_32_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8400400
}
pub const fn is_LDRSW_64_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8800400
}
pub const fn is_STR_S_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0xbc000400
}
pub const fn is_LDR_S_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0xbc400400
}
pub const fn is_STR_64_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0xf8000400
}
pub const fn is_LDR_64_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0xf8400400
}
pub const fn is_STR_D_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0xfc000400
}
pub const fn is_LDR_D_ldst_immpost(d: u32) -> bool {
    (d & 0xffe00c00) == 0xfc400400
}
pub const fn is_STRB_32_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38000c00
}
pub const fn is_LDRB_32_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38400c00
}
pub const fn is_LDRSB_64_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38800c00
}
pub const fn is_LDRSB_32_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38c00c00
}
pub const fn is_STR_B_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3c000c00
}
pub const fn is_LDR_B_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3c400c00
}
pub const fn is_STR_Q_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3c800c00
}
pub const fn is_LDR_Q_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3cc00c00
}
pub const fn is_STRH_32_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78000c00
}
pub const fn is_LDRH_32_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78400c00
}
pub const fn is_LDRSH_64_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78800c00
}
pub const fn is_LDRSH_32_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78c00c00
}
pub const fn is_STR_H_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0x7c000c00
}
pub const fn is_LDR_H_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0x7c400c00
}
pub const fn is_STR_32_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8000c00
}
pub const fn is_LDR_32_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8400c00
}
pub const fn is_LDRSW_64_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8800c00
}
pub const fn is_STR_S_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0xbc000c00
}
pub const fn is_LDR_S_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0xbc400c00
}
pub const fn is_STR_64_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0xf8000c00
}
pub const fn is_LDR_64_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0xf8400c00
}
pub const fn is_STR_D_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0xfc000c00
}
pub const fn is_LDR_D_ldst_immpre(d: u32) -> bool {
    (d & 0xffe00c00) == 0xfc400c00
}
pub const fn is_LDRAA_64_ldst_pac(d: u32) -> bool {
    (d & 0xfba00c00) == 0xf8200400
}
pub const fn is_LDRAA_64W_ldst_pac(d: u32) -> bool {
    (d & 0xfba00c00) == 0xf8200c00
}
pub const fn is_LDRAB_64_ldst_pac(d: u32) -> bool {
    (d & 0xfba00c00) == 0xf8a00400
}
pub const fn is_LDRAB_64W_ldst_pac(d: u32) -> bool {
    (d & 0xfba00c00) == 0xf8a00c00
}
pub const fn is_STRB_32B_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38200800 && (d & 0x00e000) != 0x006000
}
pub const fn is_STRB_32BL_ldst_regoff(d: u32) -> bool {
    (d & 0xffe0ec00) == 0x38206800
}
pub const fn is_LDRB_32B_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38600800 && (d & 0x00e000) != 0x006000
}
pub const fn is_LDRB_32BL_ldst_regoff(d: u32) -> bool {
    (d & 0xffe0ec00) == 0x38606800
}
pub const fn is_LDRSB_64B_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38a00800 && (d & 0x00e000) != 0x006000
}
pub const fn is_LDRSB_64BL_ldst_regoff(d: u32) -> bool {
    (d & 0xffe0ec00) == 0x38a06800
}
pub const fn is_LDRSB_32B_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38e00800 && (d & 0x00e000) != 0x006000
}
pub const fn is_LDRSB_32BL_ldst_regoff(d: u32) -> bool {
    (d & 0xffe0ec00) == 0x38e06800
}
pub const fn is_STR_B_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3c200800 && (d & 0x00e000) != 0x006000
}
pub const fn is_STR_BL_ldst_regoff(d: u32) -> bool {
    (d & 0xffe0ec00) == 0x3c206800
}
pub const fn is_LDR_B_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3c600800 && (d & 0x00e000) != 0x006000
}
pub const fn is_LDR_BL_ldst_regoff(d: u32) -> bool {
    (d & 0xffe0ec00) == 0x3c606800
}
pub const fn is_STR_Q_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3ca00800
}
pub const fn is_LDR_Q_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3ce00800
}
pub const fn is_STRH_32_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78200800
}
pub const fn is_LDRH_32_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78600800
}
pub const fn is_LDRSH_64_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78a00800
}
pub const fn is_LDRSH_32_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78e00800
}
pub const fn is_STR_H_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0x7c200800
}
pub const fn is_LDR_H_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0x7c600800
}
pub const fn is_STR_32_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8200800
}
pub const fn is_LDR_32_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8600800
}
pub const fn is_LDRSW_64_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8a00800
}
pub const fn is_STR_S_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0xbc200800
}
pub const fn is_LDR_S_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0xbc600800
}
pub const fn is_STR_64_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0xf8200800
}
pub const fn is_LDR_64_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0xf8600800
}
pub const fn is_PRFM_P_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0xf8a00800
}
pub const fn is_STR_D_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0xfc200800
}
pub const fn is_LDR_D_ldst_regoff(d: u32) -> bool {
    (d & 0xffe00c00) == 0xfc600800
}
pub const fn is_STTRB_32_ldst_unpriv(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38000800
}
pub const fn is_LDTRB_32_ldst_unpriv(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38400800
}
pub const fn is_LDTRSB_64_ldst_unpriv(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38800800
}
pub const fn is_LDTRSB_32_ldst_unpriv(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38c00800
}
pub const fn is_STTRH_32_ldst_unpriv(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78000800
}
pub const fn is_LDTRH_32_ldst_unpriv(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78400800
}
pub const fn is_LDTRSH_64_ldst_unpriv(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78800800
}
pub const fn is_LDTRSH_32_ldst_unpriv(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78c00800
}
pub const fn is_STTR_32_ldst_unpriv(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8000800
}
pub const fn is_LDTR_32_ldst_unpriv(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8400800
}
pub const fn is_LDTRSW_64_ldst_unpriv(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8800800
}
pub const fn is_STTR_64_ldst_unpriv(d: u32) -> bool {
    (d & 0xffe00c00) == 0xf8000800
}
pub const fn is_LDTR_64_ldst_unpriv(d: u32) -> bool {
    (d & 0xffe00c00) == 0xf8400800
}
pub const fn is_STURB_32_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38000000
}
pub const fn is_LDURB_32_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38400000
}
pub const fn is_LDURSB_64_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38800000
}
pub const fn is_LDURSB_32_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0x38c00000
}
pub const fn is_STUR_B_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3c000000
}
pub const fn is_LDUR_B_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3c400000
}
pub const fn is_STUR_Q_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3c800000
}
pub const fn is_LDUR_Q_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0x3cc00000
}
pub const fn is_STURH_32_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78000000
}
pub const fn is_LDURH_32_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78400000
}
pub const fn is_LDURSH_64_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78800000
}
pub const fn is_LDURSH_32_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0x78c00000
}
pub const fn is_STUR_H_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0x7c000000
}
pub const fn is_LDUR_H_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0x7c400000
}
pub const fn is_STUR_32_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8000000
}
pub const fn is_LDUR_32_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8400000
}
pub const fn is_LDURSW_64_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0xb8800000
}
pub const fn is_STUR_S_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0xbc000000
}
pub const fn is_LDUR_S_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0xbc400000
}
pub const fn is_STUR_64_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0xf8000000
}
pub const fn is_LDUR_64_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0xf8400000
}
pub const fn is_PRFUM_P_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0xf8800000
}
pub const fn is_STUR_D_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0xfc000000
}
pub const fn is_LDUR_D_ldst_unscaled(d: u32) -> bool {
    (d & 0xffe00c00) == 0xfc400000
}
pub const fn is_STRB_32_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0x39000000
}
pub const fn is_LDRB_32_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0x39400000
}
pub const fn is_LDRSB_64_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0x39800000
}
pub const fn is_LDRSB_32_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0x39c00000
}
pub const fn is_STR_B_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0x3d000000
}
pub const fn is_LDR_B_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0x3d400000
}
pub const fn is_STR_Q_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0x3d800000
}
pub const fn is_LDR_Q_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0x3dc00000
}
pub const fn is_STRH_32_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0x79000000
}
pub const fn is_LDRH_32_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0x79400000
}
pub const fn is_LDRSH_64_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0x79800000
}
pub const fn is_LDRSH_32_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0x79c00000
}
pub const fn is_STR_H_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0x7d000000
}
pub const fn is_LDR_H_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0x7d400000
}
pub const fn is_STR_32_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0xb9000000
}
pub const fn is_LDR_32_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0xb9400000
}
pub const fn is_LDRSW_64_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0xb9800000
}
pub const fn is_STR_S_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0xbd000000
}
pub const fn is_LDR_S_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0xbd400000
}
pub const fn is_STR_64_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0xf9000000
}
pub const fn is_LDR_64_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0xf9400000
}
pub const fn is_PRFM_P_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0xf9800000
}
pub const fn is_STR_D_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0xfd000000
}
pub const fn is_LDR_D_ldst_pos(d: u32) -> bool {
    (d & 0xffc00000) == 0xfd400000
}
pub const fn is_STP_32_ldstpair_off(d: u32) -> bool {
    (d & 0xffc00000) == 0x29000000
}
pub const fn is_LDP_32_ldstpair_off(d: u32) -> bool {
    (d & 0xffc00000) == 0x29400000
}
pub const fn is_STP_S_ldstpair_off(d: u32) -> bool {
    (d & 0xffc00000) == 0x2d000000
}
pub const fn is_LDP_S_ldstpair_off(d: u32) -> bool {
    (d & 0xffc00000) == 0x2d400000
}
pub const fn is_LDPSW_64_ldstpair_off(d: u32) -> bool {
    (d & 0xffc00000) == 0x69400000
}
pub const fn is_STP_D_ldstpair_off(d: u32) -> bool {
    (d & 0xffc00000) == 0x6d000000
}
pub const fn is_LDP_D_ldstpair_off(d: u32) -> bool {
    (d & 0xffc00000) == 0x6d400000
}
pub const fn is_STP_64_ldstpair_off(d: u32) -> bool {
    (d & 0xffc00000) == 0xa9000000
}
pub const fn is_LDP_64_ldstpair_off(d: u32) -> bool {
    (d & 0xffc00000) == 0xa9400000
}
pub const fn is_STP_Q_ldstpair_off(d: u32) -> bool {
    (d & 0xffc00000) == 0xad000000
}
pub const fn is_LDP_Q_ldstpair_off(d: u32) -> bool {
    (d & 0xffc00000) == 0xad400000
}
pub const fn is_STP_32_ldstpair_post(d: u32) -> bool {
    (d & 0xffc00000) == 0x28800000
}
pub const fn is_LDP_32_ldstpair_post(d: u32) -> bool {
    (d & 0xffc00000) == 0x28c00000
}
pub const fn is_STP_S_ldstpair_post(d: u32) -> bool {
    (d & 0xffc00000) == 0x2c800000
}
pub const fn is_LDP_S_ldstpair_post(d: u32) -> bool {
    (d & 0xffc00000) == 0x2cc00000
}
pub const fn is_LDPSW_64_ldstpair_post(d: u32) -> bool {
    (d & 0xffc00000) == 0x68c00000
}
pub const fn is_STP_D_ldstpair_post(d: u32) -> bool {
    (d & 0xffc00000) == 0x6c800000
}
pub const fn is_LDP_D_ldstpair_post(d: u32) -> bool {
    (d & 0xffc00000) == 0x6cc00000
}
pub const fn is_STP_64_ldstpair_post(d: u32) -> bool {
    (d & 0xffc00000) == 0xa8800000
}
pub const fn is_LDP_64_ldstpair_post(d: u32) -> bool {
    (d & 0xffc00000) == 0xa8c00000
}
pub const fn is_STP_Q_ldstpair_post(d: u32) -> bool {
    (d & 0xffc00000) == 0xac800000
}
pub const fn is_LDP_Q_ldstpair_post(d: u32) -> bool {
    (d & 0xffc00000) == 0xacc00000
}
pub const fn is_STP_32_ldstpair_pre(d: u32) -> bool {
    (d & 0xffc00000) == 0x29800000
}
pub const fn is_LDP_32_ldstpair_pre(d: u32) -> bool {
    (d & 0xffc00000) == 0x29c00000
}
pub const fn is_STP_S_ldstpair_pre(d: u32) -> bool {
    (d & 0xffc00000) == 0x2d800000
}
pub const fn is_LDP_S_ldstpair_pre(d: u32) -> bool {
    (d & 0xffc00000) == 0x2dc00000
}
pub const fn is_LDPSW_64_ldstpair_pre(d: u32) -> bool {
    (d & 0xffc00000) == 0x69c00000
}
pub const fn is_STP_D_ldstpair_pre(d: u32) -> bool {
    (d & 0xffc00000) == 0x6d800000
}
pub const fn is_LDP_D_ldstpair_pre(d: u32) -> bool {
    (d & 0xffc00000) == 0x6dc00000
}
pub const fn is_STP_64_ldstpair_pre(d: u32) -> bool {
    (d & 0xffc00000) == 0xa9800000
}
pub const fn is_LDP_64_ldstpair_pre(d: u32) -> bool {
    (d & 0xffc00000) == 0xa9c00000
}
pub const fn is_STP_Q_ldstpair_pre(d: u32) -> bool {
    (d & 0xffc00000) == 0xad800000
}
pub const fn is_LDP_Q_ldstpair_pre(d: u32) -> bool {
    (d & 0xffc00000) == 0xadc00000
}
pub const fn is_ADD_32_addsub_imm(d: u32) -> bool {
    (d & 0xff000000) == 0x11000000
}
pub const fn is_ADDS_32S_addsub_imm(d: u32) -> bool {
    (d & 0xff000000) == 0x31000000
}
pub const fn is_SUB_32_addsub_imm(d: u32) -> bool {
    (d & 0xff000000) == 0x51000000
}
pub const fn is_SUBS_32S_addsub_imm(d: u32) -> bool {
    (d & 0xff000000) == 0x71000000
}
pub const fn is_ADD_64_addsub_imm(d: u32) -> bool {
    (d & 0xff000000) == 0x91000000
}
pub const fn is_ADDS_64S_addsub_imm(d: u32) -> bool {
    (d & 0xff000000) == 0xb1000000
}
pub const fn is_SUB_64_addsub_imm(d: u32) -> bool {
    (d & 0xff000000) == 0xd1000000
}
pub const fn is_SUBS_64S_addsub_imm(d: u32) -> bool {
    (d & 0xff000000) == 0xf1000000
}
pub const fn is_SBFM_32M_bitfield(d: u32) -> bool {
    (d & 0xffc00000) == 0x13000000
}
pub const fn is_BFM_32M_bitfield(d: u32) -> bool {
    (d & 0xffc00000) == 0x33000000
}
pub const fn is_UBFM_32M_bitfield(d: u32) -> bool {
    (d & 0xffc00000) == 0x53000000
}
pub const fn is_SBFM_64M_bitfield(d: u32) -> bool {
    (d & 0xffc00000) == 0x93400000
}
pub const fn is_BFM_64M_bitfield(d: u32) -> bool {
    (d & 0xffc00000) == 0xb3400000
}
pub const fn is_UBFM_64M_bitfield(d: u32) -> bool {
    (d & 0xffc00000) == 0xd3400000
}
pub const fn is_EXTR_32_extract(d: u32) -> bool {
    (d & 0xffe08000) == 0x13800000
}
pub const fn is_EXTR_64_extract(d: u32) -> bool {
    (d & 0xffe00000) == 0x93c00000
}
pub const fn is_AND_32_log_imm(d: u32) -> bool {
    (d & 0xffc00000) == 0x12000000
}
pub const fn is_ORR_32_log_imm(d: u32) -> bool {
    (d & 0xffc00000) == 0x32000000
}
pub const fn is_EOR_32_log_imm(d: u32) -> bool {
    (d & 0xffc00000) == 0x52000000
}
pub const fn is_ANDS_32S_log_imm(d: u32) -> bool {
    (d & 0xffc00000) == 0x72000000
}
pub const fn is_AND_64_log_imm(d: u32) -> bool {
    (d & 0xff800000) == 0x92000000
}
pub const fn is_ORR_64_log_imm(d: u32) -> bool {
    (d & 0xff800000) == 0xb2000000
}
pub const fn is_EOR_64_log_imm(d: u32) -> bool {
    (d & 0xff800000) == 0xd2000000
}
pub const fn is_ANDS_64S_log_imm(d: u32) -> bool {
    (d & 0xff800000) == 0xf2000000
}
pub const fn is_MOVN_32_movewide(d: u32) -> bool {
    (d & 0xff800000) == 0x12800000
}
pub const fn is_MOVZ_32_movewide(d: u32) -> bool {
    (d & 0xff800000) == 0x52800000
}
pub const fn is_MOVK_32_movewide(d: u32) -> bool {
    (d & 0xff800000) == 0x72800000
}
pub const fn is_MOVN_64_movewide(d: u32) -> bool {
    (d & 0xff800000) == 0x92800000
}
pub const fn is_MOVZ_64_movewide(d: u32) -> bool {
    (d & 0xff800000) == 0xd2800000
}
pub const fn is_MOVK_64_movewide(d: u32) -> bool {
    (d & 0xff800000) == 0xf2800000
}
pub const fn is_ADR_only_pcreladdr(d: u32) -> bool {
    (d & 0x9f000000) == 0x10000000
}
pub const fn is_ADRP_only_pcreladdr(d: u32) -> bool {
    (d & 0x9f000000) == 0x90000000
}
pub const fn is_ADD_32_addsub_ext(d: u32) -> bool {
    (d & 0xffe00000) == 0x0b200000
}
pub const fn is_ADDS_32S_addsub_ext(d: u32) -> bool {
    (d & 0xffe00000) == 0x2b200000
}
pub const fn is_SUB_32_addsub_ext(d: u32) -> bool {
    (d & 0xffe00000) == 0x4b200000
}
pub const fn is_SUBS_32S_addsub_ext(d: u32) -> bool {
    (d & 0xffe00000) == 0x6b200000
}
pub const fn is_ADD_64_addsub_ext(d: u32) -> bool {
    (d & 0xffe00000) == 0x8b200000
}
pub const fn is_ADDS_64S_addsub_ext(d: u32) -> bool {
    (d & 0xffe00000) == 0xab200000
}
pub const fn is_SUB_64_addsub_ext(d: u32) -> bool {
    (d & 0xffe00000) == 0xcb200000
}
pub const fn is_SUBS_64S_addsub_ext(d: u32) -> bool {
    (d & 0xffe00000) == 0xeb200000
}
pub const fn is_ADD_32_addsub_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x0b000000
}
pub const fn is_ADDS_32_addsub_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x2b000000
}
pub const fn is_SUB_32_addsub_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x4b000000
}
pub const fn is_SUBS_32_addsub_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x6b000000
}
pub const fn is_ADD_64_addsub_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x8b000000
}
pub const fn is_ADDS_64_addsub_shift(d: u32) -> bool {
    (d & 0xff200000) == 0xab000000
}
pub const fn is_SUB_64_addsub_shift(d: u32) -> bool {
    (d & 0xff200000) == 0xcb000000
}
pub const fn is_SUBS_64_addsub_shift(d: u32) -> bool {
    (d & 0xff200000) == 0xeb000000
}
pub const fn is_ADC_32_addsub_carry(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1a000000
}
pub const fn is_ADCS_32_addsub_carry(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x3a000000
}
pub const fn is_SBC_32_addsub_carry(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x5a000000
}
pub const fn is_SBCS_32_addsub_carry(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x7a000000
}
pub const fn is_ADC_64_addsub_carry(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x9a000000
}
pub const fn is_ADCS_64_addsub_carry(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xba000000
}
pub const fn is_SBC_64_addsub_carry(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xda000000
}
pub const fn is_SBCS_64_addsub_carry(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xfa000000
}
pub const fn is_CCMN_32_condcmp_imm(d: u32) -> bool {
    (d & 0xffe00c10) == 0x3a400800
}
pub const fn is_CCMP_32_condcmp_imm(d: u32) -> bool {
    (d & 0xffe00c10) == 0x7a400800
}
pub const fn is_CCMN_64_condcmp_imm(d: u32) -> bool {
    (d & 0xffe00c10) == 0xba400800
}
pub const fn is_CCMP_64_condcmp_imm(d: u32) -> bool {
    (d & 0xffe00c10) == 0xfa400800
}
pub const fn is_CCMN_32_condcmp_reg(d: u32) -> bool {
    (d & 0xffe00c10) == 0x3a400000
}
pub const fn is_CCMP_32_condcmp_reg(d: u32) -> bool {
    (d & 0xffe00c10) == 0x7a400000
}
pub const fn is_CCMN_64_condcmp_reg(d: u32) -> bool {
    (d & 0xffe00c10) == 0xba400000
}
pub const fn is_CCMP_64_condcmp_reg(d: u32) -> bool {
    (d & 0xffe00c10) == 0xfa400000
}
pub const fn is_CSEL_32_condsel(d: u32) -> bool {
    (d & 0xffe00c00) == 0x1a800000
}
pub const fn is_CSINC_32_condsel(d: u32) -> bool {
    (d & 0xffe00c00) == 0x1a800400
}
pub const fn is_CSINV_32_condsel(d: u32) -> bool {
    (d & 0xffe00c00) == 0x5a800000
}
pub const fn is_CSNEG_32_condsel(d: u32) -> bool {
    (d & 0xffe00c00) == 0x5a800400
}
pub const fn is_CSEL_64_condsel(d: u32) -> bool {
    (d & 0xffe00c00) == 0x9a800000
}
pub const fn is_CSINC_64_condsel(d: u32) -> bool {
    (d & 0xffe00c00) == 0x9a800400
}
pub const fn is_CSINV_64_condsel(d: u32) -> bool {
    (d & 0xffe00c00) == 0xda800000
}
pub const fn is_CSNEG_64_condsel(d: u32) -> bool {
    (d & 0xffe00c00) == 0xda800400
}
pub const fn is_RBIT_32_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5ac00000
}
pub const fn is_REV16_32_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5ac00400
}
pub const fn is_REV_32_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5ac00800
}
pub const fn is_CLZ_32_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5ac01000
}
pub const fn is_CLS_32_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5ac01400
}
pub const fn is_RBIT_64_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0xdac00000
}
pub const fn is_REV16_64_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0xdac00400
}
pub const fn is_REV32_64_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0xdac00800
}
pub const fn is_REV_64_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0xdac00c00
}
pub const fn is_CLZ_64_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0xdac01000
}
pub const fn is_CLS_64_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0xdac01400
}
pub const fn is_PACIA_64P_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0xdac10000
}
pub const fn is_PACIB_64P_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0xdac10400
}
pub const fn is_PACDA_64P_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0xdac10800
}
pub const fn is_PACDB_64P_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0xdac10c00
}
pub const fn is_AUTIA_64P_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0xdac11000
}
pub const fn is_AUTIB_64P_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0xdac11400
}
pub const fn is_AUTDA_64P_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0xdac11800
}
pub const fn is_AUTDB_64P_dp_1src(d: u32) -> bool {
    (d & 0xfffffc00) == 0xdac11c00
}
pub const fn is_PACIZA_64Z_dp_1src(d: u32) -> bool {
    (d & 0xffffffe0) == 0xdac123e0
}
pub const fn is_PACIZB_64Z_dp_1src(d: u32) -> bool {
    (d & 0xffffffe0) == 0xdac127e0
}
pub const fn is_PACDZA_64Z_dp_1src(d: u32) -> bool {
    (d & 0xffffffe0) == 0xdac12be0
}
pub const fn is_PACDZB_64Z_dp_1src(d: u32) -> bool {
    (d & 0xffffffe0) == 0xdac12fe0
}
pub const fn is_AUTIZA_64Z_dp_1src(d: u32) -> bool {
    (d & 0xffffffe0) == 0xdac133e0
}
pub const fn is_AUTIZB_64Z_dp_1src(d: u32) -> bool {
    (d & 0xffffffe0) == 0xdac137e0
}
pub const fn is_AUTDZA_64Z_dp_1src(d: u32) -> bool {
    (d & 0xffffffe0) == 0xdac13be0
}
pub const fn is_AUTDZB_64Z_dp_1src(d: u32) -> bool {
    (d & 0xffffffe0) == 0xdac13fe0
}
pub const fn is_XPACI_64Z_dp_1src(d: u32) -> bool {
    (d & 0xffffffe0) == 0xdac143e0
}
pub const fn is_XPACD_64Z_dp_1src(d: u32) -> bool {
    (d & 0xffffffe0) == 0xdac147e0
}
pub const fn is_UDIV_32_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ac00800
}
pub const fn is_SDIV_32_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ac00c00
}
pub const fn is_LSLV_32_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ac02000
}
pub const fn is_LSRV_32_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ac02400
}
pub const fn is_ASRV_32_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ac02800
}
pub const fn is_RORV_32_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ac02c00
}
pub const fn is_CRC32B_32C_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ac04000
}
pub const fn is_CRC32H_32C_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ac04400
}
pub const fn is_CRC32W_32C_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ac04800
}
pub const fn is_CRC32CB_32C_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ac05000
}
pub const fn is_CRC32CH_32C_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ac05400
}
pub const fn is_CRC32CW_32C_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ac05800
}
pub const fn is_UDIV_64_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x9ac00800
}
pub const fn is_SDIV_64_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x9ac00c00
}
pub const fn is_LSLV_64_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x9ac02000
}
pub const fn is_LSRV_64_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x9ac02400
}
pub const fn is_ASRV_64_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x9ac02800
}
pub const fn is_RORV_64_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x9ac02c00
}
pub const fn is_PACGA_64P_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x9ac03000
}
pub const fn is_CRC32X_64C_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x9ac04c00
}
pub const fn is_CRC32CX_64C_dp_2src(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x9ac05c00
}
pub const fn is_MADD_32A_dp_3src(d: u32) -> bool {
    (d & 0xffe08000) == 0x1b000000
}
pub const fn is_MSUB_32A_dp_3src(d: u32) -> bool {
    (d & 0xffe08000) == 0x1b008000
}
pub const fn is_MADD_64A_dp_3src(d: u32) -> bool {
    (d & 0xffe08000) == 0x9b000000
}
pub const fn is_MSUB_64A_dp_3src(d: u32) -> bool {
    (d & 0xffe08000) == 0x9b008000
}
pub const fn is_SMADDL_64WA_dp_3src(d: u32) -> bool {
    (d & 0xffe08000) == 0x9b200000
}
pub const fn is_SMSUBL_64WA_dp_3src(d: u32) -> bool {
    (d & 0xffe08000) == 0x9b208000
}
pub const fn is_SMULH_64_dp_3src(d: u32) -> bool {
    (d & 0xffe08000) == 0x9b400000
}
pub const fn is_UMADDL_64WA_dp_3src(d: u32) -> bool {
    (d & 0xffe08000) == 0x9ba00000
}
pub const fn is_UMSUBL_64WA_dp_3src(d: u32) -> bool {
    (d & 0xffe08000) == 0x9ba08000
}
pub const fn is_UMULH_64_dp_3src(d: u32) -> bool {
    (d & 0xffe08000) == 0x9bc00000
}
pub const fn is_AND_32_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x0a000000
}
pub const fn is_BIC_32_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x0a200000
}
pub const fn is_ORR_32_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x2a000000
}
pub const fn is_ORN_32_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x2a200000
}
pub const fn is_EOR_32_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x4a000000
}
pub const fn is_EON_32_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x4a200000
}
pub const fn is_ANDS_32_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x6a000000
}
pub const fn is_BICS_32_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x6a200000
}
pub const fn is_AND_64_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x8a000000
}
pub const fn is_BIC_64_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0x8a200000
}
pub const fn is_ORR_64_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0xaa000000
}
pub const fn is_ORN_64_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0xaa200000
}
pub const fn is_EOR_64_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0xca000000
}
pub const fn is_EON_64_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0xca200000
}
pub const fn is_ANDS_64_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0xea000000
}
pub const fn is_BICS_64_log_shift(d: u32) -> bool {
    (d & 0xff200000) == 0xea200000
}
pub const fn is_SADDLV_asimdall_only(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e303800
}
pub const fn is_SMAXV_asimdall_only(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e30a800
}
pub const fn is_SMINV_asimdall_only(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e31a800
}
pub const fn is_ADDV_asimdall_only(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e31b800
}
pub const fn is_FMAXNMV_asimdall_only_H(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0e30c800
}
pub const fn is_FMAXV_asimdall_only_H(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0e30f800
}
pub const fn is_FMINNMV_asimdall_only_H(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0eb0c800
}
pub const fn is_FMINV_asimdall_only_H(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0eb0f800
}
pub const fn is_UADDLV_asimdall_only(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e303800
}
pub const fn is_UMAXV_asimdall_only(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e30a800
}
pub const fn is_UMINV_asimdall_only(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e31a800
}
pub const fn is_FMAXNMV_asimdall_only_SD(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2e30c800
}
pub const fn is_FMAXV_asimdall_only_SD(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2e30f800
}
pub const fn is_FMINNMV_asimdall_only_SD(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2eb0c800
}
pub const fn is_FMINV_asimdall_only_SD(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2eb0f800
}
pub const fn is_DUP_asimdins_DV_v(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e000400
}
pub const fn is_DUP_asimdins_DR_r(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e000c00
}
pub const fn is_SMOV_asimdins_W_w(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x0e002c00
}
pub const fn is_UMOV_asimdins_W_w(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x0e003c00
}
pub const fn is_INS_asimdins_IR_r(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x4e001c00
}
pub const fn is_SMOV_asimdins_X_x(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x4e002c00
}
pub const fn is_UMOV_asimdins_X_x(d: u32) -> bool {
    (d & 0xffeffc00) == 0x4e083c00
}
pub const fn is_INS_asimdins_IV_v(d: u32) -> bool {
    (d & 0xffe08400) == 0x6e000400
}
pub const fn is_EXT_asimdext_only(d: u32) -> bool {
    (d & 0xbfe08400) == 0x2e000000
}
pub const fn is_MOVI_asimdimm_L_sl(d: u32) -> bool {
    (d & 0xbff89c00) == 0x0f000400
}
pub const fn is_ORR_asimdimm_L_sl(d: u32) -> bool {
    (d & 0xbff89c00) == 0x0f001400
}
pub const fn is_MOVI_asimdimm_L_hl(d: u32) -> bool {
    (d & 0xbff8dc00) == 0x0f008400
}
pub const fn is_ORR_asimdimm_L_hl(d: u32) -> bool {
    (d & 0xbff8dc00) == 0x0f009400
}
pub const fn is_MOVI_asimdimm_M_sm(d: u32) -> bool {
    (d & 0xbff8ec00) == 0x0f00c400
}
pub const fn is_MOVI_asimdimm_N_b(d: u32) -> bool {
    (d & 0xbff8fc00) == 0x0f00e400
}
pub const fn is_FMOV_asimdimm_S_s(d: u32) -> bool {
    (d & 0xbff8fc00) == 0x0f00f400
}
pub const fn is_FMOV_asimdimm_H_h(d: u32) -> bool {
    (d & 0xbff8fc00) == 0x0f00fc00
}
pub const fn is_MVNI_asimdimm_L_sl(d: u32) -> bool {
    (d & 0xbff89c00) == 0x2f000400
}
pub const fn is_BIC_asimdimm_L_sl(d: u32) -> bool {
    (d & 0xbff89c00) == 0x2f001400
}
pub const fn is_MVNI_asimdimm_L_hl(d: u32) -> bool {
    (d & 0xbff8dc00) == 0x2f008400
}
pub const fn is_BIC_asimdimm_L_hl(d: u32) -> bool {
    (d & 0xbff8dc00) == 0x2f009400
}
pub const fn is_MVNI_asimdimm_M_sm(d: u32) -> bool {
    (d & 0xbff8ec00) == 0x2f00c400
}
pub const fn is_MOVI_asimdimm_D_ds(d: u32) -> bool {
    (d & 0xfff8fc00) == 0x2f00e400
}
pub const fn is_MOVI_asimdimm_D2_d(d: u32) -> bool {
    (d & 0xfff8fc00) == 0x6f00e400
}
pub const fn is_FMOV_asimdimm_D2_d(d: u32) -> bool {
    (d & 0xfff8fc00) == 0x6f00f400
}
pub const fn is_UZP1_asimdperm_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e001800
}
pub const fn is_TRN1_asimdperm_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e002800
}
pub const fn is_ZIP1_asimdperm_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e003800
}
pub const fn is_UZP2_asimdperm_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e005800
}
pub const fn is_TRN2_asimdperm_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e006800
}
pub const fn is_ZIP2_asimdperm_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e007800
}
pub const fn is_DUP_asisdone_only(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x5e000400
}
pub const fn is_ADDP_asisdpair_only(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x5e31b800
}
pub const fn is_FMAXNMP_asisdpair_only_H(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5e30c800
}
pub const fn is_FADDP_asisdpair_only_H(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5e30d800
}
pub const fn is_FMAXP_asisdpair_only_H(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5e30f800
}
pub const fn is_FMINNMP_asisdpair_only_H(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5eb0c800
}
pub const fn is_FMINP_asisdpair_only_H(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5eb0f800
}
pub const fn is_FMAXNMP_asisdpair_only_SD(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7e30c800
}
pub const fn is_FADDP_asisdpair_only_SD(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7e30d800
}
pub const fn is_FMAXP_asisdpair_only_SD(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7e30f800
}
pub const fn is_FMINNMP_asisdpair_only_SD(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7eb0c800
}
pub const fn is_FMINP_asisdpair_only_SD(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7eb0f800
}
pub const fn is_SSHR_asisdshf_R(d: u32) -> bool {
    (d & 0xff80fc00) == 0x5f000400 && (d & 0x780000) != 0x000000
}
pub const fn is_SSRA_asisdshf_R(d: u32) -> bool {
    (d & 0xff80fc00) == 0x5f001400 && (d & 0x780000) != 0x000000
}
pub const fn is_SRSHR_asisdshf_R(d: u32) -> bool {
    (d & 0xff80fc00) == 0x5f002400 && (d & 0x780000) != 0x000000
}
pub const fn is_SRSRA_asisdshf_R(d: u32) -> bool {
    (d & 0xff80fc00) == 0x5f003400 && (d & 0x780000) != 0x000000
}
pub const fn is_SHL_asisdshf_R(d: u32) -> bool {
    (d & 0xff80fc00) == 0x5f005400 && (d & 0x780000) != 0x000000
}
pub const fn is_SQSHL_asisdshf_R(d: u32) -> bool {
    (d & 0xff80fc00) == 0x5f007400 && (d & 0x780000) != 0x000000
}
pub const fn is_SQSHRN_asisdshf_N(d: u32) -> bool {
    (d & 0xff80fc00) == 0x5f009400 && (d & 0x780000) != 0x000000
}
pub const fn is_SQRSHRN_asisdshf_N(d: u32) -> bool {
    (d & 0xff80fc00) == 0x5f009c00 && (d & 0x780000) != 0x000000
}
pub const fn is_SCVTF_asisdshf_C(d: u32) -> bool {
    (d & 0xff80fc00) == 0x5f00e400 && (d & 0x780000) != 0x000000
}
pub const fn is_FCVTZS_asisdshf_C(d: u32) -> bool {
    (d & 0xff80fc00) == 0x5f00fc00 && (d & 0x780000) != 0x000000
}
pub const fn is_USHR_asisdshf_R(d: u32) -> bool {
    (d & 0xff80fc00) == 0x7f000400 && (d & 0x780000) != 0x000000
}
pub const fn is_USRA_asisdshf_R(d: u32) -> bool {
    (d & 0xff80fc00) == 0x7f001400 && (d & 0x780000) != 0x000000
}
pub const fn is_URSHR_asisdshf_R(d: u32) -> bool {
    (d & 0xff80fc00) == 0x7f002400 && (d & 0x780000) != 0x000000
}
pub const fn is_URSRA_asisdshf_R(d: u32) -> bool {
    (d & 0xff80fc00) == 0x7f003400 && (d & 0x780000) != 0x000000
}
pub const fn is_SRI_asisdshf_R(d: u32) -> bool {
    (d & 0xff80fc00) == 0x7f004400 && (d & 0x780000) != 0x000000
}
pub const fn is_SLI_asisdshf_R(d: u32) -> bool {
    (d & 0xff80fc00) == 0x7f005400 && (d & 0x780000) != 0x000000
}
pub const fn is_SQSHLU_asisdshf_R(d: u32) -> bool {
    (d & 0xff80fc00) == 0x7f006400 && (d & 0x780000) != 0x000000
}
pub const fn is_UQSHL_asisdshf_R(d: u32) -> bool {
    (d & 0xff80fc00) == 0x7f007400 && (d & 0x780000) != 0x000000
}
pub const fn is_SQSHRUN_asisdshf_N(d: u32) -> bool {
    (d & 0xff80fc00) == 0x7f008400 && (d & 0x780000) != 0x000000
}
pub const fn is_SQRSHRUN_asisdshf_N(d: u32) -> bool {
    (d & 0xff80fc00) == 0x7f008c00 && (d & 0x780000) != 0x000000
}
pub const fn is_UQSHRN_asisdshf_N(d: u32) -> bool {
    (d & 0xff80fc00) == 0x7f009400 && (d & 0x780000) != 0x000000
}
pub const fn is_UQRSHRN_asisdshf_N(d: u32) -> bool {
    (d & 0xff80fc00) == 0x7f009c00 && (d & 0x780000) != 0x000000
}
pub const fn is_UCVTF_asisdshf_C(d: u32) -> bool {
    (d & 0xff80fc00) == 0x7f00e400 && (d & 0x780000) != 0x000000
}
pub const fn is_FCVTZU_asisdshf_C(d: u32) -> bool {
    (d & 0xff80fc00) == 0x7f00fc00 && (d & 0x780000) != 0x000000
}
pub const fn is_SQDMLAL_asisddiff_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x5e209000
}
pub const fn is_SQDMLSL_asisddiff_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x5e20b000
}
pub const fn is_SQDMULL_asisddiff_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x5e20d000
}
pub const fn is_SQADD_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x5e200c00
}
pub const fn is_SQSUB_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x5e202c00
}
pub const fn is_CMGT_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x5e203400
}
pub const fn is_CMGE_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x5e203c00
}
pub const fn is_SSHL_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x5e204400
}
pub const fn is_SQSHL_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x5e204c00
}
pub const fn is_SRSHL_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x5e205400
}
pub const fn is_SQRSHL_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x5e205c00
}
pub const fn is_ADD_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x5e208400
}
pub const fn is_CMTST_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x5e208c00
}
pub const fn is_SQDMULH_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x5e20b400
}
pub const fn is_FMULX_asisdsame_only(d: u32) -> bool {
    (d & 0xffa0fc00) == 0x5e20dc00
}
pub const fn is_FCMEQ_asisdsame_only(d: u32) -> bool {
    (d & 0xffa0fc00) == 0x5e20e400
}
pub const fn is_FRECPS_asisdsame_only(d: u32) -> bool {
    (d & 0xffa0fc00) == 0x5e20fc00
}
pub const fn is_FRSQRTS_asisdsame_only(d: u32) -> bool {
    (d & 0xffa0fc00) == 0x5ea0fc00
}
pub const fn is_UQADD_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x7e200c00
}
pub const fn is_UQSUB_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x7e202c00
}
pub const fn is_CMHI_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x7e203400
}
pub const fn is_CMHS_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x7e203c00
}
pub const fn is_USHL_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x7e204400
}
pub const fn is_UQSHL_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x7e204c00
}
pub const fn is_URSHL_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x7e205400
}
pub const fn is_UQRSHL_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x7e205c00
}
pub const fn is_SUB_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x7e208400
}
pub const fn is_CMEQ_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x7e208c00
}
pub const fn is_SQRDMULH_asisdsame_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x7e20b400
}
pub const fn is_FCMGE_asisdsame_only(d: u32) -> bool {
    (d & 0xffa0fc00) == 0x7e20e400
}
pub const fn is_FACGE_asisdsame_only(d: u32) -> bool {
    (d & 0xffa0fc00) == 0x7e20ec00
}
pub const fn is_FABD_asisdsame_only(d: u32) -> bool {
    (d & 0xffa0fc00) == 0x7ea0d400
}
pub const fn is_FCMGT_asisdsame_only(d: u32) -> bool {
    (d & 0xffa0fc00) == 0x7ea0e400
}
pub const fn is_FACGT_asisdsame_only(d: u32) -> bool {
    (d & 0xffa0fc00) == 0x7ea0ec00
}
pub const fn is_FMULX_asisdsamefp16_only(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x5e401c00
}
pub const fn is_FCMEQ_asisdsamefp16_only(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x5e402400
}
pub const fn is_FRECPS_asisdsamefp16_only(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x5e403c00
}
pub const fn is_FRSQRTS_asisdsamefp16_only(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x5ec03c00
}
pub const fn is_FCMGE_asisdsamefp16_only(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x7e402400
}
pub const fn is_FACGE_asisdsamefp16_only(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x7e402c00
}
pub const fn is_FABD_asisdsamefp16_only(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x7ec01400
}
pub const fn is_FCMGT_asisdsamefp16_only(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x7ec02400
}
pub const fn is_FACGT_asisdsamefp16_only(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x7ec02c00
}
pub const fn is_SQRDMLAH_asisdsame2_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x7e008400
}
pub const fn is_SQRDMLSH_asisdsame2_only(d: u32) -> bool {
    (d & 0xff20fc00) == 0x7e008c00
}
pub const fn is_SUQADD_asisdmisc_R(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x5e203800
}
pub const fn is_SQABS_asisdmisc_R(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x5e207800
}
pub const fn is_CMGT_asisdmisc_Z(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x5e208800
}
pub const fn is_CMEQ_asisdmisc_Z(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x5e209800
}
pub const fn is_CMLT_asisdmisc_Z(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x5e20a800
}
pub const fn is_ABS_asisdmisc_R(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x5e20b800
}
pub const fn is_SQXTN_asisdmisc_N(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x5e214800
}
pub const fn is_FCVTNS_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x5e21a800
}
pub const fn is_FCVTMS_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x5e21b800
}
pub const fn is_FCVTAS_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x5e21c800
}
pub const fn is_SCVTF_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x5e21d800
}
pub const fn is_FCMGT_asisdmisc_FZ(d: u32) -> bool {
    (d & 0xffbffc00) == 0x5ea0c800
}
pub const fn is_FCMEQ_asisdmisc_FZ(d: u32) -> bool {
    (d & 0xffbffc00) == 0x5ea0d800
}
pub const fn is_FCMLT_asisdmisc_FZ(d: u32) -> bool {
    (d & 0xffbffc00) == 0x5ea0e800
}
pub const fn is_FCVTPS_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x5ea1a800
}
pub const fn is_FCVTZS_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x5ea1b800
}
pub const fn is_FRECPE_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x5ea1d800
}
pub const fn is_FRECPX_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x5ea1f800
}
pub const fn is_USQADD_asisdmisc_R(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x7e203800
}
pub const fn is_SQNEG_asisdmisc_R(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x7e207800
}
pub const fn is_CMGE_asisdmisc_Z(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x7e208800
}
pub const fn is_CMLE_asisdmisc_Z(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x7e209800
}
pub const fn is_NEG_asisdmisc_R(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x7e20b800
}
pub const fn is_SQXTUN_asisdmisc_N(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x7e212800
}
pub const fn is_UQXTN_asisdmisc_N(d: u32) -> bool {
    (d & 0xff3ffc00) == 0x7e214800
}
pub const fn is_FCVTXN_asisdmisc_N(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7e216800
}
pub const fn is_FCVTNU_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7e21a800
}
pub const fn is_FCVTMU_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7e21b800
}
pub const fn is_FCVTAU_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7e21c800
}
pub const fn is_UCVTF_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7e21d800
}
pub const fn is_FCMGE_asisdmisc_FZ(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7ea0c800
}
pub const fn is_FCMLE_asisdmisc_FZ(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7ea0d800
}
pub const fn is_FCVTPU_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7ea1a800
}
pub const fn is_FCVTZU_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7ea1b800
}
pub const fn is_FRSQRTE_asisdmisc_R(d: u32) -> bool {
    (d & 0xffbffc00) == 0x7ea1d800
}
pub const fn is_FCVTNS_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5e79a800
}
pub const fn is_FCVTMS_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5e79b800
}
pub const fn is_FCVTAS_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5e79c800
}
pub const fn is_SCVTF_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5e79d800
}
pub const fn is_FCMGT_asisdmiscfp16_FZ(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5ef8c800
}
pub const fn is_FCMEQ_asisdmiscfp16_FZ(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5ef8d800
}
pub const fn is_FCMLT_asisdmiscfp16_FZ(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5ef8e800
}
pub const fn is_FCVTPS_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5ef9a800
}
pub const fn is_FCVTZS_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5ef9b800
}
pub const fn is_FRECPE_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5ef9d800
}
pub const fn is_FRECPX_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5ef9f800
}
pub const fn is_FCVTNU_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x7e79a800
}
pub const fn is_FCVTMU_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x7e79b800
}
pub const fn is_FCVTAU_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x7e79c800
}
pub const fn is_UCVTF_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x7e79d800
}
pub const fn is_FCMGE_asisdmiscfp16_FZ(d: u32) -> bool {
    (d & 0xfffffc00) == 0x7ef8c800
}
pub const fn is_FCMLE_asisdmiscfp16_FZ(d: u32) -> bool {
    (d & 0xfffffc00) == 0x7ef8d800
}
pub const fn is_FCVTPU_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x7ef9a800
}
pub const fn is_FCVTZU_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x7ef9b800
}
pub const fn is_FRSQRTE_asisdmiscfp16_R(d: u32) -> bool {
    (d & 0xfffffc00) == 0x7ef9d800
}
pub const fn is_SQDMLAL_asisdelem_L(d: u32) -> bool {
    (d & 0xff00f400) == 0x5f003000
}
pub const fn is_SQDMLSL_asisdelem_L(d: u32) -> bool {
    (d & 0xff00f400) == 0x5f007000
}
pub const fn is_SQDMULL_asisdelem_L(d: u32) -> bool {
    (d & 0xff00f400) == 0x5f00b000
}
pub const fn is_SQDMULH_asisdelem_R(d: u32) -> bool {
    (d & 0xff00f400) == 0x5f00c000
}
pub const fn is_SQRDMULH_asisdelem_R(d: u32) -> bool {
    (d & 0xff00f400) == 0x5f00d000
}
pub const fn is_FMLA_asisdelem_RH_H(d: u32) -> bool {
    (d & 0xffc0f400) == 0x5f001000
}
pub const fn is_FMLS_asisdelem_RH_H(d: u32) -> bool {
    (d & 0xffc0f400) == 0x5f005000
}
pub const fn is_FMUL_asisdelem_RH_H(d: u32) -> bool {
    (d & 0xffc0f400) == 0x5f009000
}
pub const fn is_FMLA_asisdelem_R_SD(d: u32) -> bool {
    (d & 0xff80f400) == 0x5f801000
}
pub const fn is_FMLS_asisdelem_R_SD(d: u32) -> bool {
    (d & 0xff80f400) == 0x5f805000
}
pub const fn is_FMUL_asisdelem_R_SD(d: u32) -> bool {
    (d & 0xff80f400) == 0x5f809000
}
pub const fn is_SQRDMLAH_asisdelem_R(d: u32) -> bool {
    (d & 0xff00f400) == 0x7f00d000
}
pub const fn is_SQRDMLSH_asisdelem_R(d: u32) -> bool {
    (d & 0xff00f400) == 0x7f00f000
}
pub const fn is_FMULX_asisdelem_RH_H(d: u32) -> bool {
    (d & 0xffc0f400) == 0x7f009000
}
pub const fn is_FMULX_asisdelem_R_SD(d: u32) -> bool {
    (d & 0xff80f400) == 0x7f809000
}
pub const fn is_SSHR_asimdshf_R(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x0f000400 && (d & 0x780000) != 0x000000
}
pub const fn is_SSRA_asimdshf_R(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x0f001400 && (d & 0x780000) != 0x000000
}
pub const fn is_SRSHR_asimdshf_R(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x0f002400 && (d & 0x780000) != 0x000000
}
pub const fn is_SRSRA_asimdshf_R(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x0f003400 && (d & 0x780000) != 0x000000
}
pub const fn is_SHL_asimdshf_R(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x0f005400 && (d & 0x780000) != 0x000000
}
pub const fn is_SQSHL_asimdshf_R(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x0f007400 && (d & 0x780000) != 0x000000
}
pub const fn is_SHRN_asimdshf_N(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x0f008400 && (d & 0x780000) != 0x000000
}
pub const fn is_RSHRN_asimdshf_N(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x0f008c00 && (d & 0x780000) != 0x000000
}
pub const fn is_SQSHRN_asimdshf_N(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x0f009400 && (d & 0x780000) != 0x000000
}
pub const fn is_SQRSHRN_asimdshf_N(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x0f009c00 && (d & 0x780000) != 0x000000
}
pub const fn is_SSHLL_asimdshf_L(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x0f00a400 && (d & 0x780000) != 0x000000
}
pub const fn is_SCVTF_asimdshf_C(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x0f00e400 && (d & 0x780000) != 0x000000
}
pub const fn is_FCVTZS_asimdshf_C(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x0f00fc00 && (d & 0x780000) != 0x000000
}
pub const fn is_USHR_asimdshf_R(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f000400 && (d & 0x780000) != 0x000000
}
pub const fn is_USRA_asimdshf_R(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f001400 && (d & 0x780000) != 0x000000
}
pub const fn is_URSHR_asimdshf_R(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f002400 && (d & 0x780000) != 0x000000
}
pub const fn is_URSRA_asimdshf_R(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f003400 && (d & 0x780000) != 0x000000
}
pub const fn is_SRI_asimdshf_R(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f004400 && (d & 0x780000) != 0x000000
}
pub const fn is_SLI_asimdshf_R(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f005400 && (d & 0x780000) != 0x000000
}
pub const fn is_SQSHLU_asimdshf_R(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f006400 && (d & 0x780000) != 0x000000
}
pub const fn is_UQSHL_asimdshf_R(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f007400 && (d & 0x780000) != 0x000000
}
pub const fn is_SQSHRUN_asimdshf_N(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f008400 && (d & 0x780000) != 0x000000
}
pub const fn is_SQRSHRUN_asimdshf_N(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f008c00 && (d & 0x780000) != 0x000000
}
pub const fn is_UQSHRN_asimdshf_N(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f009400 && (d & 0x780000) != 0x000000
}
pub const fn is_UQRSHRN_asimdshf_N(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f009c00 && (d & 0x780000) != 0x000000
}
pub const fn is_USHLL_asimdshf_L(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f00a400 && (d & 0x780000) != 0x000000
}
pub const fn is_UCVTF_asimdshf_C(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f00e400 && (d & 0x780000) != 0x000000
}
pub const fn is_FCVTZU_asimdshf_C(d: u32) -> bool {
    (d & 0xbf80fc00) == 0x2f00fc00 && (d & 0x780000) != 0x000000
}
pub const fn is_TBL_asimdtbl_L1_1(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e000000
}
pub const fn is_TBX_asimdtbl_L1_1(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e001000
}
pub const fn is_TBL_asimdtbl_L2_2(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e002000
}
pub const fn is_TBX_asimdtbl_L2_2(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e003000
}
pub const fn is_TBL_asimdtbl_L3_3(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e004000
}
pub const fn is_TBX_asimdtbl_L3_3(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e005000
}
pub const fn is_TBL_asimdtbl_L4_4(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e006000
}
pub const fn is_TBX_asimdtbl_L4_4(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e007000
}
pub const fn is_SADDL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e200000
}
pub const fn is_SADDW_asimddiff_W(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e201000
}
pub const fn is_SSUBL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e202000
}
pub const fn is_SSUBW_asimddiff_W(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e203000
}
pub const fn is_ADDHN_asimddiff_N(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e204000
}
pub const fn is_SABAL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e205000
}
pub const fn is_SUBHN_asimddiff_N(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e206000
}
pub const fn is_SABDL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e207000
}
pub const fn is_SMLAL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e208000
}
pub const fn is_SQDMLAL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e209000
}
pub const fn is_SMLSL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e20a000
}
pub const fn is_SQDMLSL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e20b000
}
pub const fn is_SMULL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e20c000
}
pub const fn is_SQDMULL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e20d000
}
pub const fn is_PMULL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e20e000
}
pub const fn is_UADDL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e200000
}
pub const fn is_UADDW_asimddiff_W(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e201000
}
pub const fn is_USUBL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e202000
}
pub const fn is_USUBW_asimddiff_W(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e203000
}
pub const fn is_RADDHN_asimddiff_N(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e204000
}
pub const fn is_UABAL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e205000
}
pub const fn is_RSUBHN_asimddiff_N(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e206000
}
pub const fn is_UABDL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e207000
}
pub const fn is_UMLAL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e208000
}
pub const fn is_UMLSL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e20a000
}
pub const fn is_UMULL_asimddiff_L(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e20c000
}
pub const fn is_SHADD_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e200400
}
pub const fn is_SQADD_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e200c00
}
pub const fn is_SRHADD_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e201400
}
pub const fn is_SHSUB_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e202400
}
pub const fn is_SQSUB_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e202c00
}
pub const fn is_CMGT_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e203400
}
pub const fn is_CMGE_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e203c00
}
pub const fn is_SSHL_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e204400
}
pub const fn is_SQSHL_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e204c00
}
pub const fn is_SRSHL_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e205400
}
pub const fn is_SQRSHL_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e205c00
}
pub const fn is_SMAX_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e206400
}
pub const fn is_SMIN_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e206c00
}
pub const fn is_SABD_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e207400
}
pub const fn is_SABA_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e207c00
}
pub const fn is_ADD_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e208400
}
pub const fn is_CMTST_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e208c00
}
pub const fn is_MLA_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e209400
}
pub const fn is_MUL_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e209c00
}
pub const fn is_SMAXP_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e20a400
}
pub const fn is_SMINP_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e20ac00
}
pub const fn is_SQDMULH_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e20b400
}
pub const fn is_ADDP_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e20bc00
}
pub const fn is_FMAXNM_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x0e20c400
}
pub const fn is_FMLA_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x0e20cc00
}
pub const fn is_FADD_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x0e20d400
}
pub const fn is_FMULX_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x0e20dc00
}
pub const fn is_FCMEQ_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x0e20e400
}
pub const fn is_FMAX_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x0e20f400
}
pub const fn is_FRECPS_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x0e20fc00
}
pub const fn is_AND_asimdsame_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e201c00
}
pub const fn is_BIC_asimdsame_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e601c00
}
pub const fn is_FMINNM_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x0ea0c400
}
pub const fn is_FMLS_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x0ea0cc00
}
pub const fn is_FSUB_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x0ea0d400
}
pub const fn is_FMIN_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x0ea0f400
}
pub const fn is_FRSQRTS_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x0ea0fc00
}
pub const fn is_ORR_asimdsame_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0ea01c00
}
pub const fn is_ORN_asimdsame_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0ee01c00
}
pub const fn is_UHADD_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e200400
}
pub const fn is_UQADD_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e200c00
}
pub const fn is_URHADD_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e201400
}
pub const fn is_UHSUB_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e202400
}
pub const fn is_UQSUB_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e202c00
}
pub const fn is_CMHI_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e203400
}
pub const fn is_CMHS_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e203c00
}
pub const fn is_USHL_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e204400
}
pub const fn is_UQSHL_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e204c00
}
pub const fn is_URSHL_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e205400
}
pub const fn is_UQRSHL_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e205c00
}
pub const fn is_UMAX_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e206400
}
pub const fn is_UMIN_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e206c00
}
pub const fn is_UABD_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e207400
}
pub const fn is_UABA_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e207c00
}
pub const fn is_SUB_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e208400
}
pub const fn is_CMEQ_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e208c00
}
pub const fn is_MLS_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e209400
}
pub const fn is_PMUL_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e209c00
}
pub const fn is_UMAXP_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e20a400
}
pub const fn is_UMINP_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e20ac00
}
pub const fn is_SQRDMULH_asimdsame_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e20b400
}
pub const fn is_FMAXNMP_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x2e20c400
}
pub const fn is_FADDP_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x2e20d400
}
pub const fn is_FMUL_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x2e20dc00
}
pub const fn is_FCMGE_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x2e20e400
}
pub const fn is_FACGE_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x2e20ec00
}
pub const fn is_FMAXP_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x2e20f400
}
pub const fn is_FDIV_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x2e20fc00
}
pub const fn is_EOR_asimdsame_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2e201c00
}
pub const fn is_BSL_asimdsame_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2e601c00
}
pub const fn is_FMINNMP_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x2ea0c400
}
pub const fn is_FABD_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x2ea0d400
}
pub const fn is_FCMGT_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x2ea0e400
}
pub const fn is_FACGT_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x2ea0ec00
}
pub const fn is_FMINP_asimdsame_only(d: u32) -> bool {
    (d & 0xbfa0fc00) == 0x2ea0f400
}
pub const fn is_BIT_asimdsame_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2ea01c00
}
pub const fn is_BIF_asimdsame_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2ee01c00
}
pub const fn is_FMAXNM_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e400400
}
pub const fn is_FMLA_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e400c00
}
pub const fn is_FADD_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e401400
}
pub const fn is_FMULX_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e401c00
}
pub const fn is_FCMEQ_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e402400
}
pub const fn is_FMAX_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e403400
}
pub const fn is_FRECPS_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0e403c00
}
pub const fn is_FMINNM_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0ec00400
}
pub const fn is_FMLS_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0ec00c00
}
pub const fn is_FSUB_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0ec01400
}
pub const fn is_FMIN_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0ec03400
}
pub const fn is_FRSQRTS_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x0ec03c00
}
pub const fn is_FMAXNMP_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2e400400
}
pub const fn is_FADDP_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2e401400
}
pub const fn is_FMUL_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2e401c00
}
pub const fn is_FCMGE_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2e402400
}
pub const fn is_FACGE_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2e402c00
}
pub const fn is_FMAXP_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2e403400
}
pub const fn is_FDIV_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2e403c00
}
pub const fn is_FMINNMP_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2ec00400
}
pub const fn is_FABD_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2ec01400
}
pub const fn is_FCMGT_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2ec02400
}
pub const fn is_FACGT_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2ec02c00
}
pub const fn is_FMINP_asimdsamefp16_only(d: u32) -> bool {
    (d & 0xbfe0fc00) == 0x2ec03400
}
pub const fn is_SDOT_asimdsame2_D(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x0e009400
}
pub const fn is_SQRDMLAH_asimdsame2_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e008400
}
pub const fn is_SQRDMLSH_asimdsame2_only(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e008c00
}
pub const fn is_UDOT_asimdsame2_D(d: u32) -> bool {
    (d & 0xbf20fc00) == 0x2e009400
}
pub const fn is_FCMLA_asimdsame2_C(d: u32) -> bool {
    (d & 0xbf20e400) == 0x2e00c400
}
pub const fn is_FCADD_asimdsame2_C(d: u32) -> bool {
    (d & 0xbf20ec00) == 0x2e00e400
}
pub const fn is_REV64_asimdmisc_R(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e200800
}
pub const fn is_REV16_asimdmisc_R(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e201800
}
pub const fn is_SADDLP_asimdmisc_P(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e202800
}
pub const fn is_SUQADD_asimdmisc_R(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e203800
}
pub const fn is_CLS_asimdmisc_R(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e204800
}
pub const fn is_CNT_asimdmisc_R(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e205800
}
pub const fn is_SADALP_asimdmisc_P(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e206800
}
pub const fn is_SQABS_asimdmisc_R(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e207800
}
pub const fn is_CMGT_asimdmisc_Z(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e208800
}
pub const fn is_CMEQ_asimdmisc_Z(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e209800
}
pub const fn is_CMLT_asimdmisc_Z(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e20a800
}
pub const fn is_ABS_asimdmisc_R(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e20b800
}
pub const fn is_XTN_asimdmisc_N(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e212800
}
pub const fn is_SQXTN_asimdmisc_N(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x0e214800
}
pub const fn is_FCVTN_asimdmisc_N(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0e216800
}
pub const fn is_FCVTL_asimdmisc_L(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0e217800
}
pub const fn is_FRINTN_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0e218800
}
pub const fn is_FRINTM_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0e219800
}
pub const fn is_FCVTNS_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0e21a800
}
pub const fn is_FCVTMS_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0e21b800
}
pub const fn is_FCVTAS_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0e21c800
}
pub const fn is_SCVTF_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0e21d800
}
pub const fn is_FCMGT_asimdmisc_FZ(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0ea0c800
}
pub const fn is_FCMEQ_asimdmisc_FZ(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0ea0d800
}
pub const fn is_FCMLT_asimdmisc_FZ(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0ea0e800
}
pub const fn is_FABS_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0ea0f800
}
pub const fn is_FRINTP_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0ea18800
}
pub const fn is_FRINTZ_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0ea19800
}
pub const fn is_FCVTPS_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0ea1a800
}
pub const fn is_FCVTZS_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0ea1b800
}
pub const fn is_URECPE_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0ea1c800
}
pub const fn is_FRECPE_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x0ea1d800
}
pub const fn is_REV32_asimdmisc_R(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e200800
}
pub const fn is_UADDLP_asimdmisc_P(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e202800
}
pub const fn is_USQADD_asimdmisc_R(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e203800
}
pub const fn is_CLZ_asimdmisc_R(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e204800
}
pub const fn is_UADALP_asimdmisc_P(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e206800
}
pub const fn is_SQNEG_asimdmisc_R(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e207800
}
pub const fn is_CMGE_asimdmisc_Z(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e208800
}
pub const fn is_CMLE_asimdmisc_Z(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e209800
}
pub const fn is_NEG_asimdmisc_R(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e20b800
}
pub const fn is_SQXTUN_asimdmisc_N(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e212800
}
pub const fn is_SHLL_asimdmisc_S(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e213800
}
pub const fn is_UQXTN_asimdmisc_N(d: u32) -> bool {
    (d & 0xbf3ffc00) == 0x2e214800
}
pub const fn is_FCVTXN_asimdmisc_N(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2e216800
}
pub const fn is_FRINTA_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2e218800
}
pub const fn is_FRINTX_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2e219800
}
pub const fn is_FCVTNU_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2e21a800
}
pub const fn is_FCVTMU_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2e21b800
}
pub const fn is_FCVTAU_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2e21c800
}
pub const fn is_UCVTF_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2e21d800
}
pub const fn is_NOT_asimdmisc_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2e205800
}
pub const fn is_RBIT_asimdmisc_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2e605800
}
pub const fn is_FCMGE_asimdmisc_FZ(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2ea0c800
}
pub const fn is_FCMLE_asimdmisc_FZ(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2ea0d800
}
pub const fn is_FNEG_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2ea0f800
}
pub const fn is_FRINTI_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2ea19800
}
pub const fn is_FCVTPU_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2ea1a800
}
pub const fn is_FCVTZU_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2ea1b800
}
pub const fn is_URSQRTE_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2ea1c800
}
pub const fn is_FRSQRTE_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2ea1d800
}
pub const fn is_FSQRT_asimdmisc_R(d: u32) -> bool {
    (d & 0xbfbffc00) == 0x2ea1f800
}
pub const fn is_FRINTN_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0e798800
}
pub const fn is_FRINTM_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0e799800
}
pub const fn is_FCVTNS_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0e79a800
}
pub const fn is_FCVTMS_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0e79b800
}
pub const fn is_FCVTAS_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0e79c800
}
pub const fn is_SCVTF_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0e79d800
}
pub const fn is_FCMGT_asimdmiscfp16_FZ(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0ef8c800
}
pub const fn is_FCMEQ_asimdmiscfp16_FZ(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0ef8d800
}
pub const fn is_FCMLT_asimdmiscfp16_FZ(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0ef8e800
}
pub const fn is_FABS_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0ef8f800
}
pub const fn is_FRINTP_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0ef98800
}
pub const fn is_FRINTZ_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0ef99800
}
pub const fn is_FCVTPS_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0ef9a800
}
pub const fn is_FCVTZS_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0ef9b800
}
pub const fn is_FRECPE_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x0ef9d800
}
pub const fn is_FRINTA_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2e798800
}
pub const fn is_FRINTX_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2e799800
}
pub const fn is_FCVTNU_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2e79a800
}
pub const fn is_FCVTMU_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2e79b800
}
pub const fn is_FCVTAU_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2e79c800
}
pub const fn is_UCVTF_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2e79d800
}
pub const fn is_FCMGE_asimdmiscfp16_FZ(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2ef8c800
}
pub const fn is_FCMLE_asimdmiscfp16_FZ(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2ef8d800
}
pub const fn is_FNEG_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2ef8f800
}
pub const fn is_FRINTI_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2ef99800
}
pub const fn is_FCVTPU_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2ef9a800
}
pub const fn is_FCVTZU_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2ef9b800
}
pub const fn is_FRSQRTE_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2ef9d800
}
pub const fn is_FSQRT_asimdmiscfp16_R(d: u32) -> bool {
    (d & 0xbffffc00) == 0x2ef9f800
}
pub const fn is_SMLAL_asimdelem_L(d: u32) -> bool {
    (d & 0xbf00f400) == 0x0f002000
}
pub const fn is_SQDMLAL_asimdelem_L(d: u32) -> bool {
    (d & 0xbf00f400) == 0x0f003000
}
pub const fn is_SMLSL_asimdelem_L(d: u32) -> bool {
    (d & 0xbf00f400) == 0x0f006000
}
pub const fn is_SQDMLSL_asimdelem_L(d: u32) -> bool {
    (d & 0xbf00f400) == 0x0f007000
}
pub const fn is_MUL_asimdelem_R(d: u32) -> bool {
    (d & 0xbf00f400) == 0x0f008000
}
pub const fn is_SMULL_asimdelem_L(d: u32) -> bool {
    (d & 0xbf00f400) == 0x0f00a000
}
pub const fn is_SQDMULL_asimdelem_L(d: u32) -> bool {
    (d & 0xbf00f400) == 0x0f00b000
}
pub const fn is_SQDMULH_asimdelem_R(d: u32) -> bool {
    (d & 0xbf00f400) == 0x0f00c000
}
pub const fn is_SQRDMULH_asimdelem_R(d: u32) -> bool {
    (d & 0xbf00f400) == 0x0f00d000
}
pub const fn is_SDOT_asimdelem_D(d: u32) -> bool {
    (d & 0xbf00f400) == 0x0f00e000
}
pub const fn is_FMLA_asimdelem_RH_H(d: u32) -> bool {
    (d & 0xbfc0f400) == 0x0f001000
}
pub const fn is_FMLS_asimdelem_RH_H(d: u32) -> bool {
    (d & 0xbfc0f400) == 0x0f005000
}
pub const fn is_FMUL_asimdelem_RH_H(d: u32) -> bool {
    (d & 0xbfc0f400) == 0x0f009000
}
pub const fn is_FMLA_asimdelem_R_SD(d: u32) -> bool {
    (d & 0xbf80f400) == 0x0f801000
}
pub const fn is_FMLS_asimdelem_R_SD(d: u32) -> bool {
    (d & 0xbf80f400) == 0x0f805000
}
pub const fn is_FMUL_asimdelem_R_SD(d: u32) -> bool {
    (d & 0xbf80f400) == 0x0f809000
}
pub const fn is_MLA_asimdelem_R(d: u32) -> bool {
    (d & 0xbf00f400) == 0x2f000000
}
pub const fn is_UMLAL_asimdelem_L(d: u32) -> bool {
    (d & 0xbf00f400) == 0x2f002000
}
pub const fn is_MLS_asimdelem_R(d: u32) -> bool {
    (d & 0xbf00f400) == 0x2f004000
}
pub const fn is_UMLSL_asimdelem_L(d: u32) -> bool {
    (d & 0xbf00f400) == 0x2f006000
}
pub const fn is_UMULL_asimdelem_L(d: u32) -> bool {
    (d & 0xbf00f400) == 0x2f00a000
}
pub const fn is_SQRDMLAH_asimdelem_R(d: u32) -> bool {
    (d & 0xbf00f400) == 0x2f00d000
}
pub const fn is_UDOT_asimdelem_D(d: u32) -> bool {
    (d & 0xbf00f400) == 0x2f00e000
}
pub const fn is_SQRDMLSH_asimdelem_R(d: u32) -> bool {
    (d & 0xbf00f400) == 0x2f00f000
}
pub const fn is_FMULX_asimdelem_RH_H(d: u32) -> bool {
    (d & 0xbfc0f400) == 0x2f009000
}
pub const fn is_FCMLA_asimdelem_C_H(d: u32) -> bool {
    (d & 0xbfc09400) == 0x2f401000
}
pub const fn is_FMULX_asimdelem_R_SD(d: u32) -> bool {
    (d & 0xbf80f400) == 0x2f809000
}
pub const fn is_FCMLA_asimdelem_C_S(d: u32) -> bool {
    (d & 0xbfc09400) == 0x2f801000
}
pub const fn is_SCVTF_S32_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x1e020000
}
pub const fn is_UCVTF_S32_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x1e030000
}
pub const fn is_FCVTZS_32S_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x1e180000
}
pub const fn is_FCVTZU_32S_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x1e190000
}
pub const fn is_SCVTF_D32_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x1e420000
}
pub const fn is_UCVTF_D32_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x1e430000
}
pub const fn is_FCVTZS_32D_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x1e580000
}
pub const fn is_FCVTZU_32D_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x1e590000
}
pub const fn is_SCVTF_H32_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x1ec20000
}
pub const fn is_UCVTF_H32_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x1ec30000
}
pub const fn is_FCVTZS_32H_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x1ed80000
}
pub const fn is_FCVTZU_32H_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x1ed90000
}
pub const fn is_SCVTF_S64_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x9e020000
}
pub const fn is_UCVTF_S64_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x9e030000
}
pub const fn is_FCVTZS_64S_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x9e180000
}
pub const fn is_FCVTZU_64S_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x9e190000
}
pub const fn is_SCVTF_D64_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x9e420000
}
pub const fn is_UCVTF_D64_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x9e430000
}
pub const fn is_FCVTZS_64D_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x9e580000
}
pub const fn is_FCVTZU_64D_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x9e590000
}
pub const fn is_SCVTF_H64_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x9ec20000
}
pub const fn is_UCVTF_H64_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x9ec30000
}
pub const fn is_FCVTZS_64H_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x9ed80000
}
pub const fn is_FCVTZU_64H_float2fix(d: u32) -> bool {
    (d & 0xffff0000) == 0x9ed90000
}
pub const fn is_FCVTNS_32S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e200000
}
pub const fn is_FCVTNU_32S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e210000
}
pub const fn is_SCVTF_S32_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e220000
}
pub const fn is_UCVTF_S32_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e230000
}
pub const fn is_FCVTAS_32S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e240000
}
pub const fn is_FCVTAU_32S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e250000
}
pub const fn is_FMOV_32S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e260000
}
pub const fn is_FMOV_S32_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e270000
}
pub const fn is_FCVTPS_32S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e280000
}
pub const fn is_FCVTPU_32S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e290000
}
pub const fn is_FCVTMS_32S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e300000
}
pub const fn is_FCVTMU_32S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e310000
}
pub const fn is_FCVTZS_32S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e380000
}
pub const fn is_FCVTZU_32S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e390000
}
pub const fn is_FCVTNS_32D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e600000
}
pub const fn is_FCVTNU_32D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e610000
}
pub const fn is_SCVTF_D32_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e620000
}
pub const fn is_UCVTF_D32_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e630000
}
pub const fn is_FCVTAS_32D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e640000
}
pub const fn is_FCVTAU_32D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e650000
}
pub const fn is_FCVTPS_32D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e680000
}
pub const fn is_FCVTPU_32D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e690000
}
pub const fn is_FCVTMS_32D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e700000
}
pub const fn is_FCVTMU_32D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e710000
}
pub const fn is_FCVTZS_32D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e780000
}
pub const fn is_FCVTZU_32D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e790000
}
pub const fn is_FJCVTZS_32D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e7e0000
}
pub const fn is_FCVTNS_32H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee00000
}
pub const fn is_FCVTNU_32H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee10000
}
pub const fn is_SCVTF_H32_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee20000
}
pub const fn is_UCVTF_H32_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee30000
}
pub const fn is_FCVTAS_32H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee40000
}
pub const fn is_FCVTAU_32H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee50000
}
pub const fn is_FMOV_32H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee60000
}
pub const fn is_FMOV_H32_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee70000
}
pub const fn is_FCVTPS_32H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee80000
}
pub const fn is_FCVTPU_32H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee90000
}
pub const fn is_FCVTMS_32H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ef00000
}
pub const fn is_FCVTMU_32H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ef10000
}
pub const fn is_FCVTZS_32H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ef80000
}
pub const fn is_FCVTZU_32H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ef90000
}
pub const fn is_FCVTNS_64S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e200000
}
pub const fn is_FCVTNU_64S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e210000
}
pub const fn is_SCVTF_S64_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e220000
}
pub const fn is_UCVTF_S64_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e230000
}
pub const fn is_FCVTAS_64S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e240000
}
pub const fn is_FCVTAU_64S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e250000
}
pub const fn is_FCVTPS_64S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e280000
}
pub const fn is_FCVTPU_64S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e290000
}
pub const fn is_FCVTMS_64S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e300000
}
pub const fn is_FCVTMU_64S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e310000
}
pub const fn is_FCVTZS_64S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e380000
}
pub const fn is_FCVTZU_64S_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e390000
}
pub const fn is_FCVTNS_64D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e600000
}
pub const fn is_FCVTNU_64D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e610000
}
pub const fn is_SCVTF_D64_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e620000
}
pub const fn is_UCVTF_D64_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e630000
}
pub const fn is_FCVTAS_64D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e640000
}
pub const fn is_FCVTAU_64D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e650000
}
pub const fn is_FMOV_64D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e660000
}
pub const fn is_FMOV_D64_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e670000
}
pub const fn is_FCVTPS_64D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e680000
}
pub const fn is_FCVTPU_64D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e690000
}
pub const fn is_FCVTMS_64D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e700000
}
pub const fn is_FCVTMU_64D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e710000
}
pub const fn is_FCVTZS_64D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e780000
}
pub const fn is_FCVTZU_64D_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9e790000
}
pub const fn is_FMOV_64VX_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9eae0000
}
pub const fn is_FMOV_V64I_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9eaf0000
}
pub const fn is_FCVTNS_64H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9ee00000
}
pub const fn is_FCVTNU_64H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9ee10000
}
pub const fn is_SCVTF_H64_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9ee20000
}
pub const fn is_UCVTF_H64_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9ee30000
}
pub const fn is_FCVTAS_64H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9ee40000
}
pub const fn is_FCVTAU_64H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9ee50000
}
pub const fn is_FMOV_64H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9ee60000
}
pub const fn is_FMOV_H64_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9ee70000
}
pub const fn is_FCVTPS_64H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9ee80000
}
pub const fn is_FCVTPU_64H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9ee90000
}
pub const fn is_FCVTMS_64H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9ef00000
}
pub const fn is_FCVTMU_64H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9ef10000
}
pub const fn is_FCVTZS_64H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9ef80000
}
pub const fn is_FCVTZU_64H_float2int(d: u32) -> bool {
    (d & 0xfffffc00) == 0x9ef90000
}
pub const fn is_AESE_B_cryptoaes(d: u32) -> bool {
    (d & 0xfffffc00) == 0x4e284800
}
pub const fn is_AESD_B_cryptoaes(d: u32) -> bool {
    (d & 0xfffffc00) == 0x4e285800
}
pub const fn is_AESMC_B_cryptoaes(d: u32) -> bool {
    (d & 0xfffffc00) == 0x4e286800
}
pub const fn is_AESIMC_B_cryptoaes(d: u32) -> bool {
    (d & 0xfffffc00) == 0x4e287800
}
pub const fn is_EOR3_VVV16_crypto4(d: u32) -> bool {
    (d & 0xffe08000) == 0xce000000
}
pub const fn is_BCAX_VVV16_crypto4(d: u32) -> bool {
    (d & 0xffe08000) == 0xce200000
}
pub const fn is_SM3SS1_VVV4_crypto4(d: u32) -> bool {
    (d & 0xffe08000) == 0xce400000
}
pub const fn is_SHA1C_QSV_cryptosha3(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x5e000000
}
pub const fn is_SHA1P_QSV_cryptosha3(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x5e001000
}
pub const fn is_SHA1M_QSV_cryptosha3(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x5e002000
}
pub const fn is_SHA1SU0_VVV_cryptosha3(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x5e003000
}
pub const fn is_SHA256H_QQV_cryptosha3(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x5e004000
}
pub const fn is_SHA256H2_QQV_cryptosha3(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x5e005000
}
pub const fn is_SHA256SU1_VVV_cryptosha3(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x5e006000
}
pub const fn is_SHA512H_QQV_cryptosha512_3(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xce608000
}
pub const fn is_SHA512H2_QQV_cryptosha512_3(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xce608400
}
pub const fn is_SHA512SU1_VVV2_cryptosha512_3(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xce608800
}
pub const fn is_RAX1_VVV2_cryptosha512_3(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xce608c00
}
pub const fn is_SM3PARTW1_VVV4_cryptosha512_3(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xce60c000
}
pub const fn is_SM3PARTW2_VVV4_cryptosha512_3(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xce60c400
}
pub const fn is_SM4EKEY_VVV4_cryptosha512_3(d: u32) -> bool {
    (d & 0xffe0fc00) == 0xce60c800
}
pub const fn is_SM3TT1A_VVV4_crypto3_imm2(d: u32) -> bool {
    (d & 0xffe0cc00) == 0xce408000
}
pub const fn is_SM3TT1B_VVV4_crypto3_imm2(d: u32) -> bool {
    (d & 0xffe0cc00) == 0xce408400
}
pub const fn is_SM3TT2A_VVV4_crypto3_imm2(d: u32) -> bool {
    (d & 0xffe0cc00) == 0xce408800
}
pub const fn is_SM3TT2B_VVV_crypto3_imm2(d: u32) -> bool {
    (d & 0xffe0cc00) == 0xce408c00
}
pub const fn is_XAR_VVV2_crypto3_imm6(d: u32) -> bool {
    (d & 0xffe00000) == 0xce800000
}
pub const fn is_SHA1H_SS_cryptosha2(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5e280800
}
pub const fn is_SHA1SU1_VV_cryptosha2(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5e281800
}
pub const fn is_SHA256SU0_VV_cryptosha2(d: u32) -> bool {
    (d & 0xfffffc00) == 0x5e282800
}
pub const fn is_SHA512SU0_VV2_cryptosha512_2(d: u32) -> bool {
    (d & 0xfffffc00) == 0xcec08000
}
pub const fn is_SM4E_VV4_cryptosha512_2(d: u32) -> bool {
    (d & 0xfffffc00) == 0xcec08400
}
pub const fn is_FCMP_S_floatcmp(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x1e202000
}
pub const fn is_FCMP_SZ_floatcmp(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x1e202008
}
pub const fn is_FCMPE_S_floatcmp(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x1e202010
}
pub const fn is_FCMPE_SZ_floatcmp(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x1e202018
}
pub const fn is_FCMP_D_floatcmp(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x1e602000
}
pub const fn is_FCMP_DZ_floatcmp(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x1e602008
}
pub const fn is_FCMPE_D_floatcmp(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x1e602010
}
pub const fn is_FCMPE_DZ_floatcmp(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x1e602018
}
pub const fn is_FCMP_H_floatcmp(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x1ee02000
}
pub const fn is_FCMP_HZ_floatcmp(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x1ee02008
}
pub const fn is_FCMPE_H_floatcmp(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x1ee02010
}
pub const fn is_FCMPE_HZ_floatcmp(d: u32) -> bool {
    (d & 0xffe0fc1f) == 0x1ee02018
}
pub const fn is_FCCMP_S_floatccmp(d: u32) -> bool {
    (d & 0xffe00c10) == 0x1e200400
}
pub const fn is_FCCMPE_S_floatccmp(d: u32) -> bool {
    (d & 0xffe00c10) == 0x1e200410
}
pub const fn is_FCCMP_D_floatccmp(d: u32) -> bool {
    (d & 0xffe00c10) == 0x1e600400
}
pub const fn is_FCCMPE_D_floatccmp(d: u32) -> bool {
    (d & 0xffe00c10) == 0x1e600410
}
pub const fn is_FCCMP_H_floatccmp(d: u32) -> bool {
    (d & 0xffe00c10) == 0x1ee00400
}
pub const fn is_FCCMPE_H_floatccmp(d: u32) -> bool {
    (d & 0xffe00c10) == 0x1ee00410
}
pub const fn is_FCSEL_S_floatsel(d: u32) -> bool {
    (d & 0xffe00c00) == 0x1e200c00
}
pub const fn is_FCSEL_D_floatsel(d: u32) -> bool {
    (d & 0xffe00c00) == 0x1e600c00
}
pub const fn is_FCSEL_H_floatsel(d: u32) -> bool {
    (d & 0xffe00c00) == 0x1ee00c00
}
pub const fn is_FMOV_S_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e204000
}
pub const fn is_FABS_S_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e20c000
}
pub const fn is_FNEG_S_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e214000
}
pub const fn is_FSQRT_S_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e21c000
}
pub const fn is_FCVT_DS_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e22c000
}
pub const fn is_FCVT_HS_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e23c000
}
pub const fn is_FRINTN_S_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e244000
}
pub const fn is_FRINTP_S_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e24c000
}
pub const fn is_FRINTM_S_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e254000
}
pub const fn is_FRINTZ_S_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e25c000
}
pub const fn is_FRINTA_S_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e264000
}
pub const fn is_FRINTX_S_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e274000
}
pub const fn is_FRINTI_S_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e27c000
}
pub const fn is_FMOV_D_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e604000
}
pub const fn is_FABS_D_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e60c000
}
pub const fn is_FNEG_D_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e614000
}
pub const fn is_FSQRT_D_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e61c000
}
pub const fn is_FCVT_SD_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e624000
}
pub const fn is_FCVT_HD_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e63c000
}
pub const fn is_FRINTN_D_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e644000
}
pub const fn is_FRINTP_D_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e64c000
}
pub const fn is_FRINTM_D_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e654000
}
pub const fn is_FRINTZ_D_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e65c000
}
pub const fn is_FRINTA_D_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e664000
}
pub const fn is_FRINTX_D_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e674000
}
pub const fn is_FRINTI_D_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1e67c000
}
pub const fn is_FMOV_H_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee04000
}
pub const fn is_FABS_H_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee0c000
}
pub const fn is_FNEG_H_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee14000
}
pub const fn is_FSQRT_H_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee1c000
}
pub const fn is_FCVT_SH_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee24000
}
pub const fn is_FCVT_DH_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee2c000
}
pub const fn is_FRINTN_H_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee44000
}
pub const fn is_FRINTP_H_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee4c000
}
pub const fn is_FRINTM_H_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee54000
}
pub const fn is_FRINTZ_H_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee5c000
}
pub const fn is_FRINTA_H_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee64000
}
pub const fn is_FRINTX_H_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee74000
}
pub const fn is_FRINTI_H_floatdp1(d: u32) -> bool {
    (d & 0xfffffc00) == 0x1ee7c000
}
pub const fn is_FMUL_S_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e200800
}
pub const fn is_FDIV_S_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e201800
}
pub const fn is_FADD_S_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e202800
}
pub const fn is_FSUB_S_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e203800
}
pub const fn is_FMAX_S_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e204800
}
pub const fn is_FMIN_S_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e205800
}
pub const fn is_FMAXNM_S_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e206800
}
pub const fn is_FMINNM_S_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e207800
}
pub const fn is_FNMUL_S_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e208800
}
pub const fn is_FMUL_D_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e600800
}
pub const fn is_FDIV_D_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e601800
}
pub const fn is_FADD_D_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e602800
}
pub const fn is_FSUB_D_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e603800
}
pub const fn is_FMAX_D_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e604800
}
pub const fn is_FMIN_D_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e605800
}
pub const fn is_FMAXNM_D_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e606800
}
pub const fn is_FMINNM_D_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e607800
}
pub const fn is_FNMUL_D_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1e608800
}
pub const fn is_FMUL_H_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ee00800
}
pub const fn is_FDIV_H_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ee01800
}
pub const fn is_FADD_H_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ee02800
}
pub const fn is_FSUB_H_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ee03800
}
pub const fn is_FMAX_H_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ee04800
}
pub const fn is_FMIN_H_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ee05800
}
pub const fn is_FMAXNM_H_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ee06800
}
pub const fn is_FMINNM_H_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ee07800
}
pub const fn is_FNMUL_H_floatdp2(d: u32) -> bool {
    (d & 0xffe0fc00) == 0x1ee08800
}
pub const fn is_FMADD_S_floatdp3(d: u32) -> bool {
    (d & 0xffe08000) == 0x1f000000
}
pub const fn is_FMSUB_S_floatdp3(d: u32) -> bool {
    (d & 0xffe08000) == 0x1f008000
}
pub const fn is_FNMADD_S_floatdp3(d: u32) -> bool {
    (d & 0xffe08000) == 0x1f200000
}
pub const fn is_FNMSUB_S_floatdp3(d: u32) -> bool {
    (d & 0xffe08000) == 0x1f208000
}
pub const fn is_FMADD_D_floatdp3(d: u32) -> bool {
    (d & 0xffe08000) == 0x1f400000
}
pub const fn is_FMSUB_D_floatdp3(d: u32) -> bool {
    (d & 0xffe08000) == 0x1f408000
}
pub const fn is_FNMADD_D_floatdp3(d: u32) -> bool {
    (d & 0xffe08000) == 0x1f600000
}
pub const fn is_FNMSUB_D_floatdp3(d: u32) -> bool {
    (d & 0xffe08000) == 0x1f608000
}
pub const fn is_FMADD_H_floatdp3(d: u32) -> bool {
    (d & 0xffe08000) == 0x1fc00000
}
pub const fn is_FMSUB_H_floatdp3(d: u32) -> bool {
    (d & 0xffe08000) == 0x1fc08000
}
pub const fn is_FNMADD_H_floatdp3(d: u32) -> bool {
    (d & 0xffe08000) == 0x1fe00000
}
pub const fn is_FNMSUB_H_floatdp3(d: u32) -> bool {
    (d & 0xffe08000) == 0x1fe08000
}
pub const fn is_FMOV_S_floatimm(d: u32) -> bool {
    (d & 0xffe01fe0) == 0x1e201000
}
pub const fn is_FMOV_D_floatimm(d: u32) -> bool {
    (d & 0xffe01fe0) == 0x1e601000
}
pub const fn is_FMOV_H_floatimm(d: u32) -> bool {
    (d & 0xffe01fe0) == 0x1ee01000
}

pub const fn get_A(d: u32) -> u32 {
    (d >> 23) & 1
}
pub const fn get_CRm(d: u32) -> u32 {
    (d >> 8) & 0xF
}
pub const fn get_CRn(d: u32) -> u32 {
    (d >> 12) & 0xF
}
pub const fn get_H(d: u32) -> u32 {
    (d >> 11) & 1
}
// pub const fn get_L(d: u32) -> u32 {(d >> 21) & 1	// system, asisdelem, asimdelem
// pub const fn get_L(d: u32) -> u32 {(d >> 22) & 1	// asisdlse, asisdlsep, asisdlso, asisdlsop, ldstexcl, ldstnapair_offs, ldstpair_off, ldstpair_post, ldstpair_pre
pub const fn get_LL(d: u32) -> u32 {
    d & 3
}
// pub const fn get_M(d: u32) -> u32 {(d >> 20) & 1	// asisdelem, asimdelem
// pub const fn get_M(d: u32) -> u32 {(d >> 23) & 1	// ldst_pac
// pub const fn get_M(d: u32) -> u32 {(d >> 31) & 1	// floatcmp, floatccmp, floatsel, floatdp1, floatdp2, floatdp3, floatimm
// pub const fn get_N(d: u32) -> u32 {(d >> 21) & 1	// log_shift
// pub const fn get_N(d: u32) -> u32 {(d >> 22) & 1	// bitfield, extract, log_imm
pub const fn get_O(d: u32) -> u32 {
    (d >> 14) & 1
}
pub const fn get_Op0(d: u32) -> u32 {
    (d >> 21) & 3
}
pub const fn get_Q(d: u32) -> u32 {
    (d >> 30) & 1
}
// pub const fn get_R(d: u32) -> u32 {(d >> 21) & 1	// asisdlso, asisdlsop
// pub const fn get_R(d: u32) -> u32 {(d >> 22) & 1	// memop
pub const fn get_Ra(d: u32) -> u32 {
    (d >> 10) & 0x1F
}
pub const fn get_Rd(d: u32) -> u32 {
    d & 0x1F
}
// pub const fn get_Rm(d: u32) -> u32 {(d >> 16) & 0x1F	// asisdlsep, asisdlsop, ldst_regoff, extract, addsub_ext, addsub_shift, addsub_carry, condcmp_reg, condsel, dp_2src, dp_3src, log_shift, asimdext, asimdperm, asisddiff, asisdsame, asisdsamefp16, asisdsame2, asimdtbl, asimddiff, asimdsame, asimdsamefp16, asimdsame2, crypto4, cryptosha3, cryptosha512_3, crypto3_imm2, crypto3_imm6, floatcmp, floatccmp, floatsel, floatdp2, floatdp3
// pub const fn get_Rm(d: u32) -> u32 {(d >> 16) & 0xF	// asisdelem, asimdelem
pub const fn get_Rn(d: u32) -> u32 {
    (d >> 5) & 0x1F
}
pub const fn get_Rs(d: u32) -> u32 {
    (d >> 16) & 0x1F
}
pub const fn get_Rt(d: u32) -> u32 {
    d & 0x1F
}
pub const fn get_Rt2(d: u32) -> u32 {
    (d >> 10) & 0x1F
}
// pub const fn get_S(d: u32) -> u32 {(d >> 12) & 1	// asisdlso, asisdlsop, ldst_regoff
// pub const fn get_S(d: u32) -> u32 {(d >> 22) & 1	// ldst_pac
// pub const fn get_S(d: u32) -> u32 {(d >> 29) & 1	// addsub_imm, addsub_ext, addsub_shift, addsub_carry, condcmp_imm, condcmp_reg, condsel, dp_1src, dp_2src, float2fix, float2int, floatcmp, floatccmp, floatsel, floatdp1, floatdp2, floatdp3, floatimm
pub const fn get_U(d: u32) -> u32 {
    (d >> 29) & 1
}
pub const fn get_V(d: u32) -> u32 {
    (d >> 26) & 1
}
pub const fn get_W(d: u32) -> u32 {
    (d >> 11) & 1
}
// pub const fn get_a(d: u32) -> u32 {(d >> 18) & 1	// asimdimm
// pub const fn get_a(d: u32) -> u32 {(d >> 23) & 1	// asisdsamefp16, asisdmiscfp16, asimdsamefp16, asimdmiscfp16
pub const fn get_b(d: u32) -> u32 {
    (d >> 17) & 1
}
pub const fn get_b40(d: u32) -> u32 {
    (d >> 19) & 0x1F
}
pub const fn get_b5(d: u32) -> u32 {
    (d >> 31) & 1
}
pub const fn get_c(d: u32) -> u32 {
    (d >> 16) & 1
}
pub const fn get_cmode(d: u32) -> u32 {
    (d >> 12) & 0xF
}
// pub const fn get_cond(d: u32) -> u32 {d & 0xF	// condbranch
// pub const fn get_cond(d: u32) -> u32 {(d >> 12) & 0xF	// condcmp_imm, condcmp_reg, condsel, floatccmp, floatsel
pub const fn get_d(d: u32) -> u32 {
    (d >> 9) & 1
}
pub const fn get_e(d: u32) -> u32 {
    (d >> 8) & 1
}
pub const fn get_f(d: u32) -> u32 {
    (d >> 7) & 1
}
pub const fn get_g(d: u32) -> u32 {
    (d >> 6) & 1
}
pub const fn get_h(d: u32) -> u32 {
    (d >> 5) & 1
}
pub const fn get_hw(d: u32) -> u32 {
    (d >> 21) & 3
}
pub const fn get_imm12(d: u32) -> u32 {
    (d >> 10) & 0xFFF
}
pub const fn get_imm14(d: u32) -> u32 {
    (d >> 5) & 0x3FFF
}
pub const fn get_imm16(d: u32) -> u32 {
    (d >> 5) & 0xFFFF
}
pub const fn get_imm19(d: u32) -> u32 {
    (d >> 5) & 0x7FFFF
}
pub const fn get_imm2(d: u32) -> u32 {
    (d >> 12) & 3
}
pub const fn get_imm26(d: u32) -> u32 {
    d & 0x3FFFFFF
}
pub const fn get_imm3(d: u32) -> u32 {
    (d >> 10) & 7
}
pub const fn get_imm4(d: u32) -> u32 {
    (d >> 11) & 0xF
}
// pub const fn get_imm5(d: u32) -> u32 {(d >> 16) & 0x1F	// condcmp_imm, asimdins, asisdone
// pub const fn get_imm5(d: u32) -> u32 {(d >> 5) & 0x1F	// floatimm
pub const fn get_imm6(d: u32) -> u32 {
    (d >> 10) & 0x3F
}
pub const fn get_imm7(d: u32) -> u32 {
    (d >> 15) & 0x7F
}
pub const fn get_imm8(d: u32) -> u32 {
    (d >> 13) & 0xFF
}
pub const fn get_imm9(d: u32) -> u32 {
    (d >> 12) & 0x1FF
}
pub const fn get_immb(d: u32) -> u32 {
    (d >> 16) & 7
}
pub const fn get_immh(d: u32) -> u32 {
    (d >> 19) & 0xF
}
pub const fn get_immhi(d: u32) -> u32 {
    (d >> 5) & 0x7FFFF
}
pub const fn get_immlo(d: u32) -> u32 {
    (d >> 29) & 3
}
pub const fn get_immr(d: u32) -> u32 {
    (d >> 16) & 0x3F
}
pub const fn get_imms(d: u32) -> u32 {
    (d >> 10) & 0x3F
}
pub const fn get_len(d: u32) -> u32 {
    (d >> 13) & 3
}
pub const fn get_nzcv(d: u32) -> u32 {
    d & 0xF
}
pub const fn get_rmode(d: u32) -> u32 {
    (d >> 19) & 3
}
pub const fn get_scale(d: u32) -> u32 {
    (d >> 10) & 0x3F
}
pub const fn get_sf(d: u32) -> u32 {
    (d >> 31) & 1
}
pub const fn get_shift(d: u32) -> u32 {
    (d >> 22) & 3
}
// pub const fn get_size(d: u32) -> u32 {(d >> 10) & 3	// asisdlse, asisdlsep, asisdlso, asisdlsop
// pub const fn get_size(d: u32) -> u32 {(d >> 22) & 3	// asimdall, asimdperm, asisdpair, asisddiff, asisdsame, asisdsame2, asisdmisc, asisdelem, asimddiff, asimdsame, asimdsame2, asimdmisc, asimdelem, cryptoaes, cryptosha3, cryptosha2
// pub const fn get_size(d: u32) -> u32 {(d >> 30) & 3	// memop, ldstexcl, ldst_immpost, ldst_immpre, ldst_pac, ldst_regoff, ldst_unpriv, ldst_unscaled, ldst_pos
pub const fn get_type(d: u32) -> u32 {
    (d >> 22) & 3
}

pub const fn get_A_s(d: u32) -> u32 {
    ((d >> 23) & 1) | (if (d & 0x800000) != 0 { !1 } else { 0 })
}
pub const fn get_CRm_s(d: u32) -> u32 {
    ((d >> 8) & 0xF) | (if (d & 0x800) != 0 { !0xF } else { 0 })
}
pub const fn get_CRn_s(d: u32) -> u32 {
    ((d >> 12) & 0xF) | (if (d & 0x8000) != 0 { !0xF } else { 0 })
}
pub const fn get_H_s(d: u32) -> u32 {
    ((d >> 11) & 1) | (if (d & 0x800) != 0 { !1 } else { 0 })
}
// pub const fn get_L_s(d: u32) -> u32 { ((d >> 21) & 1) | ( if (d & 0x200000) != 0 { !1 } else { 0}) }
// pub const fn get_L_s(d: u32) -> u32 { ((d >> 22) & 1) | ( if (d & 0x400000) != 0 { !1 } else { 0}) }
pub const fn get_LL_s(d: u32) -> u32 {
    (d & 3) | (if (d & 2) != 0 { !3 } else { 0 })
}
// pub const fn get_M_s(d: u32) -> u32 { ((d >> 20) & 1) | ( if (d & 0x100000) != 0 { !1 } else { 0}) }
// pub const fn get_M_s(d: u32) -> u32 { ((d >> 23) & 1) | ( if (d & 0x800000) != 0 { !1 } else { 0}) }
// pub const fn get_M_s(d: u32) -> u32 { ((d >> 31) & 1) | ( if (d & 0x80000000) != 0 { !1 } else { 0}) }
// pub const fn get_N_s(d: u32) -> u32 { ((d >> 21) & 1) | ( if (d & 0x200000) != 0 { !1 } else { 0}) }
// pub const fn get_N_s(d: u32) -> u32 { ((d >> 22) & 1) | ( if (d & 0x400000) != 0 { !1 } else { 0}) }
pub const fn get_O_s(d: u32) -> u32 {
    ((d >> 14) & 1) | (if (d & 0x4000) != 0 { !1 } else { 0 })
}
pub const fn get_Op0_s(d: u32) -> u32 {
    ((d >> 21) & 3) | (if (d & 0x400000) != 0 { !3 } else { 0 })
}
pub const fn get_Q_s(d: u32) -> u32 {
    ((d >> 30) & 1) | (if (d & 0x40000000) != 0 { !1 } else { 0 })
}
// pub const fn get_R_s(d: u32) -> u32 { ((d >> 21) & 1) | ( if (d & 0x200000) != 0 { !1 } else { 0}) }
// pub const fn get_R_s(d: u32) -> u32 { ((d >> 22) & 1) | ( if (d & 0x400000) != 0 { !1 } else { 0}) }
pub const fn get_Ra_s(d: u32) -> u32 {
    ((d >> 10) & 0x1F) | (if (d & 0x4000) != 0 { !0x1F } else { 0 })
}
pub const fn get_Rd_s(d: u32) -> u32 {
    (d & 0x1F) | (if (d & 0x10) != 0 { !0x1F } else { 0 })
}
// pub const fn get_Rm_s(d: u32) -> u32 { ((d >> 16) & 0x1F) | ( if (d & 0x100000) != 0 { !0x1F } else { 0}) }
// pub const fn get_Rm_s(d: u32) -> u32 { ((d >> 16) & 0xF) | ( if (d & 0x80000) != 0 { !0xF } else { 0}) }
pub const fn get_Rn_s(d: u32) -> u32 {
    ((d >> 5) & 0x1F) | (if (d & 0x200) != 0 { !0x1F } else { 0 })
}
pub const fn get_Rs_s(d: u32) -> u32 {
    ((d >> 16) & 0x1F) | (if (d & 0x100000) != 0 { !0x1F } else { 0 })
}
pub const fn get_Rt_s(d: u32) -> u32 {
    (d & 0x1F) | (if (d & 0x10) != 0 { !0x1F } else { 0 })
}
pub const fn get_Rt2_s(d: u32) -> u32 {
    ((d >> 10) & 0x1F) | (if (d & 0x4000) != 0 { !0x1F } else { 0 })
}
// pub const fn get_S_s(d: u32) -> u32 { ((d >> 12) & 1) | ( if (d & 0x1000) != 0 { !1 } else { 0}) }
// pub const fn get_S_s(d: u32) -> u32 { ((d >> 22) & 1) | ( if (d & 0x400000) != 0 { !1 } else { 0}) }
// pub const fn get_S_s(d: u32) -> u32 { ((d >> 29) & 1) | ( if (d & 0x20000000) != 0 { !1 } else { 0}) }
pub const fn get_U_s(d: u32) -> u32 {
    ((d >> 29) & 1) | (if (d & 0x20000000) != 0 { !1 } else { 0 })
}
pub const fn get_V_s(d: u32) -> u32 {
    ((d >> 26) & 1) | (if (d & 0x4000000) != 0 { !1 } else { 0 })
}
pub const fn get_W_s(d: u32) -> u32 {
    ((d >> 11) & 1) | (if (d & 0x800) != 0 { !1 } else { 0 })
}
// pub const fn get_a_s(d: u32) -> u32 { ((d >> 18) & 1) | ( if (d & 0x40000) != 0 { !1 } else { 0}) }
// pub const fn get_a_s(d: u32) -> u32 { ((d >> 23) & 1) | ( if (d & 0x800000) != 0 { !1 } else { 0}) }
pub const fn get_b_s(d: u32) -> u32 {
    ((d >> 17) & 1) | (if (d & 0x20000) != 0 { !1 } else { 0 })
}
pub const fn get_b40_s(d: u32) -> u32 {
    ((d >> 19) & 0x1F) | (if (d & 0x800000) != 0 { !0x1F } else { 0 })
}
pub const fn get_b5_s(d: u32) -> u32 {
    ((d >> 31) & 1) | (if (d & 0x80000000) != 0 { !1 } else { 0 })
}
pub const fn get_c_s(d: u32) -> u32 {
    ((d >> 16) & 1) | (if (d & 0x10000) != 0 { !1 } else { 0 })
}
pub const fn get_cmode_s(d: u32) -> u32 {
    ((d >> 12) & 0xF) | (if (d & 0x8000) != 0 { !0xF } else { 0 })
}
// pub const fn get_cond_s(d: u32) -> u32 { (d & 0xF) | ( if (d & 8 {) != 0 !0xF } else { 0}) }
// pub const fn get_cond_s(d: u32) -> u32 { ((d >> 12) & 0xF) | ( if (d & 0x8000) != 0 { !0xF } else { 0}) }
pub const fn get_d_s(d: u32) -> u32 {
    ((d >> 9) & 1) | (if (d & 0x200) != 0 { !1 } else { 0 })
}
pub const fn get_e_s(d: u32) -> u32 {
    ((d >> 8) & 1) | (if (d & 0x100) != 0 { !1 } else { 0 })
}
pub const fn get_f_s(d: u32) -> u32 {
    ((d >> 7) & 1) | (if (d & 0x80) != 0 { !1 } else { 0 })
}
pub const fn get_g_s(d: u32) -> u32 {
    ((d >> 6) & 1) | (if (d & 0x40) != 0 { !1 } else { 0 })
}
pub const fn get_h_s(d: u32) -> u32 {
    ((d >> 5) & 1) | (if (d & 0x20) != 0 { !1 } else { 0 })
}
pub const fn get_hw_s(d: u32) -> u32 {
    ((d >> 21) & 3) | (if (d & 0x400000) != 0 { !3 } else { 0 })
}
pub const fn get_imm12_s(d: u32) -> u32 {
    ((d >> 10) & 0xFFF) | (if (d & 0x200000) != 0 { !0xFFF } else { 0 })
}
pub const fn get_imm14_s(d: u32) -> u32 {
    ((d >> 5) & 0x3FFF) | (if (d & 0x40000) != 0 { !0x3FFF } else { 0 })
}
pub const fn get_imm16_s(d: u32) -> u32 {
    ((d >> 5) & 0xFFFF) | (if (d & 0x100000) != 0 { !0xFFFF } else { 0 })
}
pub const fn get_imm19_s(d: u32) -> u32 {
    ((d >> 5) & 0x7FFFF) | (if (d & 0x800000) != 0 { !0x7FFFF } else { 0 })
}
pub const fn get_imm2_s(d: u32) -> u32 {
    ((d >> 12) & 3) | (if (d & 0x2000) != 0 { !3 } else { 0 })
}
pub const fn get_imm26_s(d: u32) -> u32 {
    (d & 0x3FFFFFF) | (if (d & 0x2000000) != 0 { !0x3FFFFFF } else { 0 })
}
pub const fn get_imm3_s(d: u32) -> u32 {
    ((d >> 10) & 7) | (if (d & 0x1000) != 0 { !7 } else { 0 })
}
pub const fn get_imm4_s(d: u32) -> u32 {
    ((d >> 11) & 0xF) | (if (d & 0x4000) != 0 { !0xF } else { 0 })
}
// pub const fn get_imm5_s(d: u32) -> u32 { ((d >> 16) & 0x1F) | ( if (d & 0x100000) != 0 { !0x1F } else { 0}) }
// pub const fn get_imm5_s(d: u32) -> u32 { ((d >> 5) & 0x1F) | ( if (d & 0x200) != 0 { !0x1F } else { 0}) }
pub const fn get_imm6_s(d: u32) -> u32 {
    ((d >> 10) & 0x3F) | (if (d & 0x8000) != 0 { !0x3F } else { 0 })
}
pub const fn get_imm7_s(d: u32) -> u32 {
    ((d >> 15) & 0x7F) | (if (d & 0x200000) != 0 { !0x7F } else { 0 })
}
pub const fn get_imm8_s(d: u32) -> u32 {
    ((d >> 13) & 0xFF) | (if (d & 0x100000) != 0 { !0xFF } else { 0 })
}
pub const fn get_imm9_s(d: u32) -> u32 {
    ((d >> 12) & 0x1FF) | (if (d & 0x100000) != 0 { !0x1FF } else { 0 })
}
pub const fn get_immb_s(d: u32) -> u32 {
    ((d >> 16) & 7) | (if (d & 0x40000) != 0 { !7 } else { 0 })
}
pub const fn get_immh_s(d: u32) -> u32 {
    ((d >> 19) & 0xF) | (if (d & 0x400000) != 0 { !0xF } else { 0 })
}
pub const fn get_immhi_s(d: u32) -> u32 {
    ((d >> 5) & 0x7FFFF) | (if (d & 0x800000) != 0 { !0x7FFFF } else { 0 })
}
pub const fn get_immlo_s(d: u32) -> u32 {
    ((d >> 29) & 3) | (if (d & 0x40000000) != 0 { !3 } else { 0 })
}
pub const fn get_immr_s(d: u32) -> u32 {
    ((d >> 16) & 0x3F) | (if (d & 0x200000) != 0 { !0x3F } else { 0 })
}
pub const fn get_imms_s(d: u32) -> u32 {
    ((d >> 10) & 0x3F) | (if (d & 0x8000) != 0 { !0x3F } else { 0 })
}
pub const fn get_len_s(d: u32) -> u32 {
    ((d >> 13) & 3) | (if (d & 0x4000) != 0 { !3 } else { 0 })
}
pub const fn get_nzcv_s(d: u32) -> u32 {
    (d & 0xF) | (if (d & 8) != 0 { !0xF } else { 0 })
}
pub const fn get_rmode_s(d: u32) -> u32 {
    ((d >> 19) & 3) | (if (d & 0x100000) != 0 { !3 } else { 0 })
}
pub const fn get_scale_s(d: u32) -> u32 {
    ((d >> 10) & 0x3F) | (if (d & 0x8000) != 0 { !0x3F } else { 0 })
}
pub const fn get_sf_s(d: u32) -> u32 {
    ((d >> 31) & 1) | (if (d & 0x80000000) != 0 { !1 } else { 0 })
}
pub const fn get_shift_s(d: u32) -> u32 {
    ((d >> 22) & 3) | (if (d & 0x800000) != 0 { !3 } else { 0 })
}
// pub const fn get_size_s(d: u32) -> u32 { ((d >> 10) & 3) | ( if (d & 0x800) != 0 { !3 } else { 0}) }
// pub const fn get_size_s(d: u32) -> u32 { ((d >> 22) & 3) | ( if (d & 0x800000) != 0 { !3 } else { 0}) }
// pub const fn get_size_s(d: u32) -> u32 { ((d >> 30) & 3) | ( if (d & 0x80000000) != 0 { !3 } else { 0}) }
pub const fn get_type_s(d: u32) -> u32 {
    ((d >> 22) & 3) | (if (d & 0x800000) != 0 { !3 } else { 0 })
}
