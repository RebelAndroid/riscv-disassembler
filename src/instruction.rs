use crate::cinstruction::CInstruction;
use crate::immediates::{
    BImmediate, CSR, CSRImmediate, JImmediate, SImmediate, Shamt, ShamtW, UImmediate,
};
use crate::register::{CIRegister, FRegister, IRegister};
use crate::{immediates::IImmediate, opcode::Opcode};
use std::fmt::{Display, Formatter};

use proc_macros::{
    amo_assemble, b_assemble, fr_assemble, i_assemble, l_assemble, r_assemble, s_assemble,
    sh_assemble, shw_assemble,
};

#[derive(Debug, PartialEq)]
pub enum RoundingMode {
    /// round to nearest, ties to even
    RNE = 0b000,
    /// round towards zero
    RTZ = 0b001,
    /// round down
    RDN = 0b010,
    /// round up
    RUP = 0b011,
    /// round to nearest, ties to max magnitude
    RMM = 0b100,
    /// use rounding mode in fcsr
    DYN = 0b111,
}

impl Display for RoundingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            RoundingMode::RNE => write!(f, "rne"),
            RoundingMode::RTZ => write!(f, "rtz"),
            RoundingMode::RDN => write!(f, "rdn"),
            RoundingMode::RUP => write!(f, "rup"),
            RoundingMode::RMM => write!(f, "rmm"),
            RoundingMode::DYN => write!(f, "dyn"),
        }
    }
}

impl RoundingMode {
    pub fn from_int(x: u32) -> Result<RoundingMode, String> {
        match x {
            0b000 => Ok(RoundingMode::RNE),
            0b001 => Ok(RoundingMode::RTZ),
            0b010 => Ok(RoundingMode::RDN),
            0b011 => Ok(RoundingMode::RUP),
            0b100 => Ok(RoundingMode::RMM),
            0b111 => Ok(RoundingMode::DYN),
            _ => Err("attempted to create invalid rounding mode".to_owned()),
        }
    }
    pub fn from_str(x: &str) -> Result<RoundingMode, String> {
        match x {
            "rne" => Ok(RoundingMode::RNE),
            "rtz" => Ok(RoundingMode::RTZ),
            "rdn" => Ok(RoundingMode::RDN),
            "rup" => Ok(RoundingMode::RUP),
            "rmm" => Ok(RoundingMode::RMM),
            "dyn" => Ok(RoundingMode::DYN),
            _ => Err("attempted to create invalid rounding mode".to_owned()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    //
    // Instructions from RV32I
    //
    /// Load upper immediate
    LUI(IRegister, UImmediate),
    /// Add upper immediate to PC
    AUIPC(IRegister, UImmediate),
    /// Jump and Link
    JAL(IRegister, JImmediate),
    /// Jump and Link Register
    JALR(IRegister, IRegister, IImmediate),
    BEQ(IRegister, IRegister, BImmediate),
    BNE(IRegister, IRegister, BImmediate),
    BLT(IRegister, IRegister, BImmediate),
    BGE(IRegister, IRegister, BImmediate),
    BLTU(IRegister, IRegister, BImmediate),
    BGEU(IRegister, IRegister, BImmediate),
    /// Load Byte
    LB(IRegister, IRegister, IImmediate),
    /// Load Halfword
    LH(IRegister, IRegister, IImmediate),
    /// Load Word
    LW(IRegister, IRegister, IImmediate),
    /// Load Byte Unsigned
    LBU(IRegister, IRegister, IImmediate),
    /// Load Halfword Unsigned
    LHU(IRegister, IRegister, IImmediate),
    /// Store Byte
    SB(IRegister, IRegister, SImmediate),
    /// Store Halfword
    SH(IRegister, IRegister, SImmediate),
    /// Store Word
    SW(IRegister, IRegister, SImmediate),
    ADDI(IRegister, IRegister, IImmediate),
    SLTI(IRegister, IRegister, IImmediate),
    SLTIU(IRegister, IRegister, IImmediate),
    XORI(IRegister, IRegister, IImmediate),
    ORI(IRegister, IRegister, IImmediate),
    ANDI(IRegister, IRegister, IImmediate),
    /// Left Shift Immediate
    SLLI(IRegister, IRegister, Shamt),
    /// Logical Right Shift Immediate
    SRLI(IRegister, IRegister, Shamt),
    /// Arithmetic Right Shift Immediate
    SRAI(IRegister, IRegister, Shamt),
    ADD(IRegister, IRegister, IRegister),
    SUB(IRegister, IRegister, IRegister),
    /// Left Shift
    SLL(IRegister, IRegister, IRegister),
    /// Branch if Equal
    SLT(IRegister, IRegister, IRegister),
    SLTU(IRegister, IRegister, IRegister),
    XOR(IRegister, IRegister, IRegister),
    /// Logical Right Shift Immediate
    SRL(IRegister, IRegister, IRegister),
    /// Arithmetic Right Shift Immediate
    SRA(IRegister, IRegister, IRegister),
    OR(IRegister, IRegister, IRegister),
    AND(IRegister, IRegister, IRegister),
    FENCE(IRegister, IRegister, u8, u8),
    ECALL,
    EBREAK,
    //
    // Instructions Added In RV64I
    //
    /// Load Word Unsigned
    LWU(IRegister, IRegister, IImmediate),
    /// Load Doubleword
    LD(IRegister, IRegister, IImmediate),
    /// Store Doubleword
    SD(IRegister, IRegister, SImmediate),
    /// Add Immediate (word)
    ADDIW(IRegister, IRegister, IImmediate),
    /// Left Shift Immediate (word)
    SLLIW(IRegister, IRegister, ShamtW),
    /// Logical Right Shift Immediate (word)
    SRLIW(IRegister, IRegister, ShamtW),
    /// Arithmetic Right Shift Immediate (word)
    SRAIW(IRegister, IRegister, ShamtW),
    /// Add (word)
    ADDW(IRegister, IRegister, IRegister),
    /// Subtract (word)
    SUBW(IRegister, IRegister, IRegister),
    /// Left Shift (word)
    SLLW(IRegister, IRegister, IRegister),
    /// Logical Right Shift (word)
    SRLW(IRegister, IRegister, IRegister),
    /// Arithmetic Right Shift (word)
    SRAW(IRegister, IRegister, IRegister),
    //
    // Instructions In M Extension
    //
    /// Multiply
    MUL(IRegister, IRegister, IRegister),
    /// Multiply (High bits)
    MULH(IRegister, IRegister, IRegister),
    /// Multiply Signed-Unsigned (High bits)
    MULHSU(IRegister, IRegister, IRegister),
    /// Multiply Unsigned (High)
    MULHU(IRegister, IRegister, IRegister),
    /// Divide
    DIV(IRegister, IRegister, IRegister),
    /// Divide (Unsigned)
    DIVU(IRegister, IRegister, IRegister),
    /// Remainder
    REM(IRegister, IRegister, IRegister),
    /// Remainder (Unsigned)
    REMU(IRegister, IRegister, IRegister),
    /// Multiply Word
    MULW(IRegister, IRegister, IRegister),
    /// Divide Word
    DIVW(IRegister, IRegister, IRegister),
    /// Divide Unsigned Word
    DIVUW(IRegister, IRegister, IRegister),
    /// Remainder Word
    REMW(IRegister, IRegister, IRegister),
    /// Remainder Unsigned Word
    REMUW(IRegister, IRegister, IRegister),
    //
    // Instructions In A Extension
    //
    /// Load Reserved Word
    // rd, rs1, ac, rl
    LRW(IRegister, IRegister, bool, bool),
    SCW(IRegister, IRegister, IRegister, bool, bool),
    AMOSWAPW(IRegister, IRegister, IRegister, bool, bool),
    AMOADDW(IRegister, IRegister, IRegister, bool, bool),
    AMOXORW(IRegister, IRegister, IRegister, bool, bool),
    AMOANDW(IRegister, IRegister, IRegister, bool, bool),
    AMOORW(IRegister, IRegister, IRegister, bool, bool),
    AMOMINW(IRegister, IRegister, IRegister, bool, bool),
    AMOMAXW(IRegister, IRegister, IRegister, bool, bool),
    AMOMINUW(IRegister, IRegister, IRegister, bool, bool),
    AMOMAXUW(IRegister, IRegister, IRegister, bool, bool),
    //
    LRD(IRegister, IRegister, bool, bool),
    SCD(IRegister, IRegister, IRegister, bool, bool),
    AMOSWAPD(IRegister, IRegister, IRegister, bool, bool),
    AMOADDD(IRegister, IRegister, IRegister, bool, bool),
    AMOXORD(IRegister, IRegister, IRegister, bool, bool),
    AMOANDD(IRegister, IRegister, IRegister, bool, bool),
    AMOORD(IRegister, IRegister, IRegister, bool, bool),
    AMOMIND(IRegister, IRegister, IRegister, bool, bool),
    AMOMAXD(IRegister, IRegister, IRegister, bool, bool),
    AMOMINUD(IRegister, IRegister, IRegister, bool, bool),
    AMOMAXUD(IRegister, IRegister, IRegister, bool, bool),
    //
    // Instructions in F Extension
    //
    FLW(FRegister, IRegister, IImmediate),
    FSW(IRegister, FRegister, SImmediate),
    FMADDS(FRegister, FRegister, FRegister, FRegister, RoundingMode),
    FMSUBS(FRegister, FRegister, FRegister, FRegister, RoundingMode),
    FNMSUBS(FRegister, FRegister, FRegister, FRegister, RoundingMode),
    FNMADDS(FRegister, FRegister, FRegister, FRegister, RoundingMode),
    FADDS(FRegister, FRegister, FRegister, RoundingMode),
    FSUBS(FRegister, FRegister, FRegister, RoundingMode),
    FMULS(FRegister, FRegister, FRegister, RoundingMode),
    FDIVS(FRegister, FRegister, FRegister, RoundingMode),
    FSQRTS(FRegister, FRegister, RoundingMode),
    FSGNJS(FRegister, FRegister, FRegister),
    FSGNJNS(FRegister, FRegister, FRegister),
    FSGNJXS(FRegister, FRegister, FRegister),
    FMINS(FRegister, FRegister, FRegister),
    FMAXS(FRegister, FRegister, FRegister),
    FCVTWS(IRegister, FRegister, RoundingMode),
    FCVTWUS(IRegister, FRegister, RoundingMode),
    FMVXW(IRegister, FRegister),
    FEQS(IRegister, FRegister, FRegister),
    FLTS(IRegister, FRegister, FRegister),
    FLES(IRegister, FRegister, FRegister),
    FCLASSS(IRegister, FRegister),
    FCVTSW(FRegister, IRegister, RoundingMode),
    FCVTSWU(FRegister, IRegister, RoundingMode),
    FMVWX(FRegister, IRegister),
    //
    // Instructions in F Extension (RV64)
    //
    FCVTLS(IRegister, FRegister, RoundingMode),
    FCVTLUS(IRegister, FRegister, RoundingMode),
    FCVTSL(FRegister, IRegister, RoundingMode),
    FCVTSLU(FRegister, IRegister, RoundingMode),
    //
    // Instructions in Zicsr Extension
    //
    CSRRW(IRegister, IRegister, CSR),
    CSRRS(IRegister, IRegister, CSR),
    CSRRC(IRegister, IRegister, CSR),
    CSRRWI(IRegister, CSRImmediate, CSR),
    CSRRSI(IRegister, CSRImmediate, CSR),
    CSRRCI(IRegister, CSRImmediate, CSR),
}

fn aq_rl_suffix(aq: &bool, rl: &bool) -> &'static str {
    match (aq, rl) {
        (true, true) => ".aqrl",
        (true, false) => ".aq",
        (false, true) => ".rl",
        (false, false) => "",
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Instruction::LUI(rd, imm) => write!(f, "lui {},{}", rd, imm),
            Instruction::AUIPC(rd, imm) => write!(f, "auipc {},{}", rd, imm),
            Instruction::JAL(rd, offset) => write!(f, "jal {},{}", rd, offset),
            Instruction::JALR(rd, rs1, imm) => write!(f, "jalr {},{}({})", rd, imm, rs1),
            Instruction::BEQ(rs1, rs2, imm) => write!(f, "beq {},{},{}", rs1, rs2, imm),
            Instruction::BNE(rs1, rs2, imm) => write!(f, "bne {},{},{}", rs1, rs2, imm),
            Instruction::BLT(rs1, rs2, imm) => write!(f, "blt {},{},{}", rs1, rs2, imm),
            Instruction::BGE(rs1, rs2, imm) => write!(f, "bge {rs1},{rs2},{imm}"),
            Instruction::BLTU(rs1, rs2, imm) => write!(f, "bltu {rs1},{rs2},{imm}"),
            Instruction::BGEU(rs1, rs2, imm) => write!(f, "bgeu {rs1},{rs2},{imm}"),
            Instruction::LB(rd, rs1, imm) => write!(f, "lb {rd},{imm}({rs1})"),
            Instruction::LH(rd, rs1, imm) => write!(f, "lh {rd},{imm}({rs1})"),
            Instruction::LW(rd, rs1, imm) => write!(f, "lw {rd},{imm}({rs1})"),
            Instruction::LBU(rd, rs1, imm) => write!(f, "lbu {rd},{imm}({rs1})"),
            Instruction::LHU(rd, rs1, imm) => write!(f, "lhu {rd},{imm}({rs1})"),
            Instruction::SB(rs1, rs2, imm) => write!(f, "sb {rs2},{imm}({rs1})"),
            Instruction::SH(rs1, rs2, imm) => write!(f, "sh {rs2},{imm}({rs1})"),
            Instruction::SW(rs1, rs2, imm) => write!(f, "sw {rs2},{imm}({rs1})"),
            Instruction::ADDI(rd, rs1, imm) => write!(f, "addi {rd},{rs1},{imm}"),
            Instruction::SLTI(rd, rs1, imm) => write!(f, "slti {rd},{rs1},{imm}"),
            Instruction::SLTIU(rd, rs1, imm) => write!(f, "sltiu {rd},{rs1},{imm}"),
            Instruction::XORI(rd, rs1, imm) => write!(f, "xori {rd},{rs1},{imm}"),
            Instruction::ORI(rd, rs1, imm) => write!(f, "ori {rd},{rs1},{imm}"),
            Instruction::ANDI(rd, rs1, imm) => write!(f, "andi {rd},{rs1},{imm}"),
            Instruction::SLLI(rd, rs1, imm) => write!(f, "slli {rd},{rs1},{imm}"),
            Instruction::SRLI(rd, rs1, imm) => write!(f, "srli {rd},{rs1},{imm}"),
            Instruction::SRAI(rd, rs1, imm) => write!(f, "srai {rd},{rs1},{imm}"),
            Instruction::ADD(rd, rs1, rs2) => write!(f, "add {rd},{rs1},{rs2}"),
            Instruction::SUB(rd, rs1, rs2) => write!(f, "sub {rd},{rs1},{rs2}"),
            Instruction::SLL(rd, rs1, rs2) => write!(f, "sll {rd},{rs1},{rs2}"),
            Instruction::SLT(rd, rs1, rs2) => write!(f, "slt {rd},{rs1},{rs2}"),
            Instruction::SLTU(rd, rs1, rs2) => write!(f, "sltu {rd},{rs1},{rs2}"),
            Instruction::XOR(rd, rs1, rs2) => write!(f, "xor {rd},{rs1},{rs2}"),
            Instruction::SRL(rd, rs1, rs2) => write!(f, "srl {rd},{rs1},{rs2}"),
            Instruction::SRA(rd, rs1, rs2) => write!(f, "sra {rd},{rs1},{rs2}"),
            Instruction::OR(rd, rs1, rs2) => write!(f, "or {rd},{rs1},{rs2}"),
            Instruction::AND(rd, rs1, rs2) => write!(f, "and {rd},{rs1},{rs2}"),
            Instruction::FENCE(_, _, _, _) => write!(f, "{}", self.fmt_fence()),
            Instruction::ECALL => write!(f, "ecall"),
            Instruction::EBREAK => write!(f, "ebreak"),
            Instruction::LWU(rd, rs1, imm) => write!(f, "lwu {rd},{imm}({rs1})"),
            Instruction::LD(rd, rs1, imm) => write!(f, "ld {rd},{imm}({rs1})"),
            Instruction::SD(rs1, rs2, imm) => write!(f, "sd {rs2},{imm}({rs1})"),
            Instruction::ADDIW(rd, rs1, imm) => write!(f, "addiw {rd},{rs1},{imm}"),
            Instruction::SLLIW(rd, rs1, imm) => write!(f, "slliw {rd},{rs1},{imm}"),
            Instruction::SRLIW(rd, rs1, imm) => write!(f, "srliw {rd},{rs1},{imm}"),
            Instruction::SRAIW(rd, rs1, imm) => write!(f, "sraiw {rd},{rs1},{imm}"),
            Instruction::ADDW(rd, rs1, rs2) => write!(f, "addw {rd},{rs1},{rs2}"),
            Instruction::SUBW(rd, rs1, rs2) => write!(f, "subw {rd},{rs1},{rs2}"),
            Instruction::SLLW(rd, rs1, rs2) => write!(f, "sllw {rd},{rs1},{rs2}"),
            Instruction::SRLW(rd, rs1, rs2) => write!(f, "srlw {rd},{rs1},{rs2}"),
            Instruction::SRAW(rd, rs1, rs2) => write!(f, "sraw {rd},{rs1},{rs2}"),
            Instruction::MUL(rd, rs1, rs2) => write!(f, "mul {rd},{rs1},{rs2}"),
            Instruction::MULH(rd, rs1, rs2) => write!(f, "mulh {rd},{rs1},{rs2}"),
            Instruction::MULHSU(rd, rs1, rs2) => write!(f, "mulhsu {rd},{rs1},{rs2}"),
            Instruction::MULHU(rd, rs1, rs2) => write!(f, "mulhu {rd},{rs1},{rs2}"),
            Instruction::DIV(rd, rs1, rs2) => write!(f, "div {rd},{rs1},{rs2}"),
            Instruction::DIVU(rd, rs1, rs2) => write!(f, "divu {rd},{rs1},{rs2}"),
            Instruction::REM(rd, rs1, rs2) => write!(f, "rem {rd},{rs1},{rs2}"),
            Instruction::REMU(rd, rs1, rs2) => write!(f, "remu {rd},{rs1},{rs2}"),
            Instruction::MULW(rd, rs1, rs2) => write!(f, "mulw {rd},{rs1},{rs2}"),
            Instruction::DIVW(rd, rs1, rs2) => write!(f, "divw {rd},{rs1},{rs2}"),
            Instruction::DIVUW(rd, rs1, rs2) => write!(f, "divuw {rd},{rs1},{rs2}"),
            Instruction::REMW(rd, rs1, rs2) => write!(f, "remw {rd},{rs1},{rs2}"),
            Instruction::REMUW(rd, rs1, rs2) => write!(f, "remuw {rd},{rs1},{rs2}"),
            Instruction::LRW(rd, rs1, aq, rl) => {
                write!(f, "lr.w{} {rd},{rs1}", aq_rl_suffix(aq, rl))
            }
            Instruction::SCW(rd, rs1, rs2, aq, rl) => {
                write!(f, "sc.w{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOSWAPW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoswap.w{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOADDW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoadd.w{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOXORW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoxor.w{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOANDW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoand.w{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOORW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoor.w{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMINW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amomin.w{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMAXW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amomax.w{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMINUW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amominu.w{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMAXUW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amomaxu.w{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::LRD(rd, rs1, aq, rl) => {
                write!(f, "lr.d{} {rd},{rs1}", aq_rl_suffix(aq, rl))
            }
            Instruction::SCD(rd, rs1, rs2, aq, rl) => {
                write!(f, "sc.d{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOSWAPD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoswap.d{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOADDD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoadd.d{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOXORD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoxor.d{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOANDD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoand.d{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOORD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoor.d{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMIND(rd, rs1, rs2, aq, rl) => {
                write!(f, "amomin.d{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMAXD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amomax.d{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMINUD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amominu.d{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMAXUD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amomaxu.d{} {rd},{rs1},{rs2}", aq_rl_suffix(aq, rl))
            }
            Instruction::FLW(rd, rs1, imm) => write!(f, "flw {rd},{imm}({rs1})"),
            Instruction::FSW(rs1, rs2, imm) => write!(f, "fsw {rs2},{imm}({rs1})"),
            Instruction::FMADDS(rd, rs1, rs2, rs3, rm) => {
                write!(f, "fmadd.s.{rm} {rd},{rs1},{rs2},{rs3}")
            }
            Instruction::FMSUBS(rd, rs1, rs2, rs3, rm) => {
                write!(f, "fmsub.s.{rm} {rd},{rs1},{rs2},{rs3}")
            }
            Instruction::FNMSUBS(rd, rs1, rs2, rs3, rm) => {
                write!(f, "fnmsub.s.{rm} {rd},{rs1},{rs2},{rs3}")
            }
            Instruction::FNMADDS(rd, rs1, rs2, rs3, rm) => {
                write!(f, "fnmadd.s.{rm} {rd},{rs1},{rs2},{rs3}")
            }
            Instruction::FADDS(rd, rs1, rs2, rm) => write!(f, "fadd.s.{rm} {rd},{rs1},{rs2}"),
            Instruction::FSUBS(rd, rs1, rs2, rm) => write!(f, "fsub.s.{rm} {rd},{rs1},{rs2}"),
            Instruction::FMULS(rd, rs1, rs2, rm) => write!(f, "fmul.s.{rm} {rd},{rs1},{rs2}"),
            Instruction::FDIVS(rd, rs1, rs2, rm) => write!(f, "fdiv.s.{rm} {rd},{rs1},{rs2}"),
            Instruction::FSQRTS(rd, rs1, rm) => write!(f, "fsqrt.s.{rm} {rd},{rs1}"),
            Instruction::FSGNJS(rd, rs1, rs2) => write!(f, "fsgnj.s {rd},{rs1},{rs2}"),
            Instruction::FSGNJNS(rd, rs1, rs2) => write!(f, "fsgnjn.s {rd},{rs1},{rs2}"),
            Instruction::FSGNJXS(rd, rs1, rs2) => write!(f, "fsgnjx.s {rd},{rs1},{rs2}"),
            Instruction::FMINS(rd, rs1, rs2) => write!(f, "fmin.s {rd},{rs1},{rs2}"),
            Instruction::FMAXS(rd, rs1, rs2) => write!(f, "fmax.s {rd},{rs1},{rs2}"),
            Instruction::FCVTWS(rd, rs1, rm) => write!(f, "fcvt.w.s.{rm} {rd},{rs1}"),
            Instruction::FCVTWUS(rd, rs1, rm) => write!(f, "fcvt.wu.s.{rm} {rd},{rs1}"),
            Instruction::FMVXW(rd, rs1) => write!(f, "fmv.x.w {rd},{rs1}"),
            Instruction::FEQS(rd, rs1, rs2) => write!(f, "feq.s {rd},{rs1},{rs2}"),
            Instruction::FLTS(rd, rs1, rs2) => write!(f, "flt.s {rd},{rs1},{rs2}"),
            Instruction::FLES(rd, rs1, rs2) => write!(f, "fle.s {rd},{rs1},{rs2}"),
            Instruction::FCLASSS(rd, rs1) => write!(f, "fclass.s {rd},{rs1}"),
            Instruction::FCVTSW(rd, rs1, rm) => write!(f, "fcvt.s.w.{rm} {rd},{rs1}"),
            Instruction::FCVTSWU(rd, rs1, rm) => write!(f, "fcvt.s.wu.{rm} {rd},{rs1}"),
            Instruction::FMVWX(rd, rs1) => write!(f, "fmv.w.x {rd},{rs1}"),
            Instruction::FCVTLS(rd, rs1, rm) => write!(f, "fcvt.l.s.{rm} {rd},{rs1}"),
            Instruction::FCVTLUS(rd, rs1, rm) => write!(f, "fcvt.lu.s.{rm} {rd},{rs1}"),
            Instruction::FCVTSL(rd, rs1, rm) => write!(f, "fcvt.s.l.{rm} {rd},{rs1}"),
            Instruction::FCVTSLU(rd, rs1, rm) => write!(f, "fcvt.s.lu.{rm} {rd},{rs1}"),
            Instruction::CSRRW(rd, rs1, csr) => write!(f, "csrrw {rd},{csr},{rs1}"),
            Instruction::CSRRS(rd, rs1, csr) => write!(f, "csrrs {rd},{csr},{rs1}"),
            Instruction::CSRRC(rd, rs1, csr) => write!(f, "csrrc {rd},{csr},{rs1}"),
            Instruction::CSRRWI(rd, imm, csr) => write!(f, "csrrwi {rd},{csr},{imm}"),
            Instruction::CSRRSI(rd, imm, csr) => write!(f, "csrrsi {rd},{csr},{imm}"),
            Instruction::CSRRCI(rd, imm, csr) => write!(f, "csrrci {rd},{csr},{imm}"),
        }
    }
}

impl Instruction {
    fn fmt_fence(&self) -> String {
        if let Instruction::FENCE(_, _, ops, fm) = *self {
            let sw = if ops & 0b0000_0001 != 0 { "w" } else { "" };
            let sr = if ops & 0b0000_0010 != 0 { "r" } else { "" };
            let so = if ops & 0b0000_0100 != 0 { "o" } else { "" };
            let si = if ops & 0b0000_1000 != 0 { "i" } else { "" };
            let pw = if ops & 0b0001_0000 != 0 { "w" } else { "" };
            let pr = if ops & 0b0010_0000 != 0 { "r" } else { "" };
            let po = if ops & 0b0100_0000 != 0 { "o" } else { "" };
            let pi = if ops & 0b1000_0000 != 0 { "i" } else { "" };
            if fm == 0b1000 {
                format!("fence.tso {pi}{po}{pr}{pw},{si}{so}{sr}{sw}")
            } else {
                format!("fence {pi}{po}{pr}{pw},{si}{so}{sr}{sw}")
            }
        } else {
            unreachable!();
        }
    }
}

pub fn parse_int(str: &str) -> Result<i64, String> {
    match str.parse::<i64>() {
        Ok(e) => Ok(e),
        Err(_) => Err(format!("unable to parse int:{str}").to_owned()),
    }
}

pub fn parse_address_expression(str: &str) -> Result<(IRegister, i64), String> {
    let (offset, register): (&str, &str) = if let Some(x) = str.split_once("(") {
        x
    } else {
        panic!("no (");
    };
    match register.strip_suffix(")") {
        Some(y) => {
            let r = IRegister::from_string(y)?;
            let i = parse_int(offset)?;
            Ok((r, i))
        }
        _ => Err("Address expression should end in a )".to_owned()),
    }
}

pub fn parse_address_expression_compressed(str: &str) -> Result<(CIRegister, i64), String> {
    let (offset, register): (&str, &str) = if let Some(x) = str.split_once("(") {
        x
    } else {
        panic!("no (");
    };
    match register.strip_suffix(")") {
        Some(y) => {
            let r = CIRegister::from_string(y)?;
            let i = parse_int(offset)?;
            Ok((r, i))
        }
        _ => Err("Address expression should end in a )".to_owned()),
    }
}

#[derive(Debug, PartialEq)]
pub enum AssemblyResult {
    I(Instruction),
    C(CInstruction),
}
impl AssemblyResult {
    pub fn c(self) -> CInstruction {
        match self {
            AssemblyResult::I(_) => panic!("c called on regular instruction"),
            AssemblyResult::C(cinstruction) => cinstruction,
        }
    }
    pub fn i(self) -> Instruction {
        match self {
            AssemblyResult::I(instruction) => instruction,
            AssemblyResult::C(_) => panic!("i called on compressed instruction"),
        }
    }
}

/// Constructs an `Instruction` from a line of assembly.
pub fn assemble_line(line: &str) -> Result<AssemblyResult, String> {
    let (mnemonic, operands): (&str, &str) = if let Some(x) = line.split_once(" ") {
        x
    } else {
        (line, "")
    };

    let mnemonics: Vec<&str> = mnemonic.split(".").collect();

    let operands: Vec<&str> = if operands.is_empty() {
        vec![]
    } else {
        operands.split(',').collect()
    };
    let operands: Vec<&str> = operands
        .iter()
        .map(|operand| operand.to_owned().trim())
        .collect();

    println!("operands: {:?}", operands);
    println!("mnemonics: {:?}", mnemonics);

    if mnemonics[0] == "c" {
        if mnemonics.len() == 1 {
            Err("compressed instruction must be specified".to_owned())
        } else {
            CInstruction::assemble_line(&mnemonics[1..], operands).map(AssemblyResult::C)
        }
    } else {
        let x = match mnemonics[0] {
            // register-immediate instructions
            "addi" => i_assemble!(ADDI),
            "addiw" => i_assemble!(ADDIW),
            "andi" => i_assemble!(ANDI),
            "ori" => i_assemble!(ORI),
            "xori" => i_assemble!(XORI),
            "slti" => i_assemble!(SLTI),
            "sltiu" => i_assemble!(SLTIU),
            "slli" => sh_assemble!(SLLI),
            "srai" => sh_assemble!(SRAI),
            "sraiw" => shw_assemble!(SRAIW),
            "srli" => sh_assemble!(SRLI),
            "srliw" => shw_assemble!(SRLIW),
            "slliw" => shw_assemble!(SLLIW),
            // register-register instructions
            "add" => r_assemble!(ADD),
            "addw" => r_assemble!(ADDW),
            "subw" => r_assemble!(SUBW),
            "and" => r_assemble!(AND),
            "sub" => r_assemble!(SUB),
            "or" => r_assemble!(OR),
            "xor" => r_assemble!(XOR),
            "sllw" => r_assemble!(SLLW),
            "srl" => r_assemble!(SRL),
            "sra" => r_assemble!(SRA),
            "srlw" => r_assemble!(SRLW),
            "sraw" => r_assemble!(SRAW),
            "sll" => r_assemble!(SLL),
            "slt" => r_assemble!(SLT),
            "sltu" => r_assemble!(SLTU),
            "mul" => r_assemble!(MUL),
            "mulh" => r_assemble!(MULH),
            "mulhsu" => r_assemble!(MULHSU),
            "mulhu" => r_assemble!(MULHU),
            "div" => r_assemble!(DIV),
            "divu" => r_assemble!(DIVU),
            "rem" => r_assemble!(REM),
            "remu" => r_assemble!(REMU),
            "mulw" => r_assemble!(MULW),
            "divw" => r_assemble!(DIVW),
            "divuw" => r_assemble!(DIVUW),
            "remw" => r_assemble!(REMW),
            "remuw" => r_assemble!(REMUW),
            // load instructions
            "lb" => l_assemble!(LB),
            "lbu" => l_assemble!(LBU),
            "lhu" => l_assemble!(LHU),
            "lw" => l_assemble!(LW),
            "lwu" => l_assemble!(LWU),
            "lh" => l_assemble!(LH),
            // store instructions
            "ld" => l_assemble!(LD),
            "sd" => s_assemble!(SD),
            "sw" => s_assemble!(SW),
            "sh" => s_assemble!(SH),
            "sb" => s_assemble!(SB),
            // branch instructions
            "blt" => b_assemble!(BLT),
            "beq" => b_assemble!(BEQ),
            "bne" => b_assemble!(BNE),
            "bge" => b_assemble!(BGE),
            "bgeu" => b_assemble!(BGEU),
            "bltu" => b_assemble!(BLTU),
            "jalr" => {
                if operands.len() != 2 {
                    Err("jalr instruction requires 2 operands".to_owned())
                } else {
                    let (base, offset) = parse_address_expression(operands[1])?;
                    Ok(Instruction::JALR(
                        IRegister::from_string(operands[0])?,
                        base,
                        IImmediate::from_val(offset),
                    ))
                }
            }
            "jal" => {
                if operands.len() != 2 {
                    Err("jal instruction requires 2 operands".to_owned())
                } else {
                    Ok(Instruction::JAL(
                        IRegister::from_string(operands[0])?,
                        JImmediate::from_val(parse_int(operands[1])?),
                    ))
                }
            }
            "lui" => {
                if operands.len() != 2 {
                    Err("lui instruction requires 2 operands".to_owned())
                } else {
                    let int: i64 = parse_int(operands[1])?;
                    if int > 2i64.pow(19) - 1 || int < -2i64.pow(19) {
                        Err("UImmediate out of range".to_owned())
                    } else {
                        Ok(Instruction::LUI(
                            IRegister::from_string(operands[0])?,
                            UImmediate::from_val(int),
                        ))
                    }
                }
            }
            "auipc" => {
                if operands.len() != 2 {
                    Err("auipc instruction requires 2 operands".to_owned())
                } else {
                    let int: i64 = parse_int(operands[1])?;
                    if int > 2i64.pow(19) - 1 || int < -2i64.pow(19) {
                        Err("UImmediate out of range".to_owned())
                    } else {
                        Ok(Instruction::AUIPC(
                            IRegister::from_string(operands[0])?,
                            UImmediate::from_val(int),
                        ))
                    }
                }
            }
            "fence" => {
                if mnemonics.len() == 1 {
                    if operands.len() != 2 {
                        Err("fence instruction requires 2 operands".to_owned())
                    } else {
                        let ops =
                            parse_fence_set(operands[1]) | (parse_fence_set(operands[0]) << 4);
                        Ok(Instruction::FENCE(
                            // rd and rs1 are currently unused
                            IRegister::Zero,
                            IRegister::Zero,
                            ops,
                            0, //fm field, always zero for a non-tso fence
                        ))
                    }
                } else if mnemonics[1] == ".tso" {
                    if operands.len() != 2 {
                        Err("fence.tso instruction requires 2 operands".to_owned())
                    } else {
                        let ops =
                            parse_fence_set(operands[1]) | (parse_fence_set(operands[0]) << 4);
                        if ops != (parse_fence_set("rw") | (parse_fence_set("rw") << 4)) {
                            Err("fence.tso should be rw,rw".to_owned())
                        } else {
                            Ok(Instruction::FENCE(
                                // rd and rs1 are currently unused
                                IRegister::Zero,
                                IRegister::Zero,
                                ops,
                                0b1000, // tso fence
                            ))
                        }
                    }
                } else {
                    Err("invalid fence".to_owned())
                }
            }
            // LR can't use `amo_assemble!` because it only has two operands
            "lr" => {
                if mnemonics.len() == 1 {
                    Err("lr must have size (w/d)".to_owned())
                } else if mnemonics.len() == 2 {
                    if mnemonics[1] == "w" {
                        Ok(Instruction::LRW(
                            IRegister::from_string(operands[0])?,
                            IRegister::from_string(operands[1])?,
                            false,
                            false,
                        ))
                    } else if mnemonics[1] == "d" {
                        Ok(Instruction::LRD(
                            IRegister::from_string(operands[0])?,
                            IRegister::from_string(operands[1])?,
                            false,
                            false,
                        ))
                    } else {
                        Err("size of lr isntruction must be word (w) or doubleword (d)".to_owned())
                    }
                } else if mnemonics.len() == 3 {
                    let (aq, rl) = match mnemonics[2] {
                        "" => (false, false),
                        "aq" => (true, false),
                        "rl" => (false, true),
                        "aqrl" => (true, true),
                        _ => return Err("ordering should be (aq)(rl)".to_owned()),
                    };
                    if mnemonics[1] == "w" {
                        Ok(Instruction::LRW(
                            IRegister::from_string(operands[0])?,
                            IRegister::from_string(operands[1])?,
                            aq,
                            rl,
                        ))
                    } else if mnemonics[1] == "d" {
                        Ok(Instruction::LRD(
                            IRegister::from_string(operands[0])?,
                            IRegister::from_string(operands[1])?,
                            aq,
                            rl,
                        ))
                    } else {
                        Err("size of lr isntruction must be word (w) or doubleword (d)".to_owned())
                    }
                } else {
                    Err(
                        "lr instruction has too many suffixes, expected lr.size.ordering"
                            .to_owned(),
                    )
                }
            }
            "sc" => amo_assemble!(SC),
            "amoswap" => amo_assemble!(AMOSWAP),
            "amoadd" => amo_assemble!(AMOADD),
            "amoxor" => amo_assemble!(AMOXOR),
            "amoand" => amo_assemble!(AMOAND),
            "amoor" => amo_assemble!(AMOOR),
            "amomin" => amo_assemble!(AMOMIN),
            "amomax" => amo_assemble!(AMOMAX),
            "amominu" => amo_assemble!(AMOMINU),
            "amomaxu" => amo_assemble!(AMOMAXU),
            "flw" => {
                if operands.len() != 2 {
                    println!("{:?}", operands);
                    Err("flw instruction requires 2 operands".to_owned())
                } else {
                    let (base, offset) = parse_address_expression(operands[1])?;
                    Ok(Instruction::FLW(
                        FRegister::from_string(operands[0])?,
                        base,
                        IImmediate::from_val(offset),
                    ))
                }
            }
            "fsw" => {
                if operands.len() != 2 {
                    println!("{:?}", operands);
                    Err("fsw instruction requires 2 operands".to_owned())
                } else {
                    let (base, offset) = parse_address_expression(operands[1])?;
                    Ok(Instruction::FSW(
                        base,
                        FRegister::from_string(operands[0])?,
                        SImmediate::from_val(offset),
                    ))
                }
            }
            "fsqrt" => {
                if operands.len() != 2 {
                    Err("fsqrt instruction requires 2 operands".to_owned())
                } else if mnemonics.len() == 2 {
                    Ok(Instruction::FSQRTS(
                        FRegister::from_string(operands[0])?,
                        FRegister::from_string(operands[1])?,
                        RoundingMode::DYN,
                    ))
                } else if mnemonics.len() == 3 {
                    Ok(Instruction::FSQRTS(
                        FRegister::from_string(operands[0])?,
                        FRegister::from_string(operands[1])?,
                        RoundingMode::from_str(mnemonics[2])?,
                    ))
                } else {
                    Err("fsqrt instruction requires a suffix {s,d}".to_owned())
                }
            }
            "fadd" => fr_assemble!(FADD),
            "fsub" => fr_assemble!(FSUB),
            "fmul" => fr_assemble!(FMUL),
            "fdiv" => fr_assemble!(FDIV),
            "fmin" => {
                if operands.len() != 3 {
                    Err("fmin instruction requires 3 operands".to_owned())
                } else if mnemonics.len() == 2 {
                    Ok(Instruction::FMINS(
                        FRegister::from_string(operands[0])?,
                        FRegister::from_string(operands[1])?,
                        FRegister::from_string(operands[2])?,
                    ))
                } else {
                    Err("fmin instruction requires a suffix {s,d}".to_owned())
                }
            }
            "fmax" => {
                if operands.len() != 3 {
                    Err("fmax instruction requires 3 operands".to_owned())
                } else if mnemonics.len() == 2 {
                    Ok(Instruction::FMAXS(
                        FRegister::from_string(operands[0])?,
                        FRegister::from_string(operands[1])?,
                        FRegister::from_string(operands[2])?,
                    ))
                } else {
                    Err("fmax instruction requires a suffix {s,d}".to_owned())
                }
            }
            "fcvt" => {
                if operands.len() != 2 {
                    Err("fcvt requires 3 operands".to_owned())
                } else if mnemonics.len() == 3 {
                    // default rounding mode
                    match (mnemonics[1], mnemonics[2]) {
                        ("w", "s") => Ok(Instruction::FCVTWS(
                            IRegister::from_string(operands[0])?,
                            FRegister::from_string(operands[1])?,
                            RoundingMode::DYN,
                        )),
                        ("wu", "s") => Ok(Instruction::FCVTWUS(
                            IRegister::from_string(operands[0])?,
                            FRegister::from_string(operands[1])?,
                            RoundingMode::DYN,
                        )),
                        ("s", "w") => Ok(Instruction::FCVTSW(
                            FRegister::from_string(operands[0])?,
                            IRegister::from_string(operands[1])?,
                            RoundingMode::DYN,
                        )),
                        ("s", "wu") => Ok(Instruction::FCVTSWU(
                            FRegister::from_string(operands[0])?,
                            IRegister::from_string(operands[1])?,
                            RoundingMode::DYN,
                        )),
                        ("l", "s") => Ok(Instruction::FCVTLS(
                            IRegister::from_string(operands[0])?,
                            FRegister::from_string(operands[1])?,
                            RoundingMode::DYN,
                        )),
                        ("lu", "s") => Ok(Instruction::FCVTLUS(
                            IRegister::from_string(operands[0])?,
                            FRegister::from_string(operands[1])?,
                            RoundingMode::DYN,
                        )),
                        ("s", "l") => Ok(Instruction::FCVTSL(
                            FRegister::from_string(operands[0])?,
                            IRegister::from_string(operands[1])?,
                            RoundingMode::DYN,
                        )),
                        ("s", "lu") => Ok(Instruction::FCVTSLU(
                            FRegister::from_string(operands[0])?,
                            IRegister::from_string(operands[1])?,
                            RoundingMode::DYN,
                        )),
                        _ => Err("invalid fcvt suffixes".to_owned()),
                    }
                } else if mnemonics.len() == 4 {
                    match (mnemonics[1], mnemonics[2]) {
                        ("w", "s") => Ok(Instruction::FCVTWS(
                            IRegister::from_string(operands[0])?,
                            FRegister::from_string(operands[1])?,
                            RoundingMode::from_str(mnemonics[3])?,
                        )),
                        ("wu", "s") => Ok(Instruction::FCVTWUS(
                            IRegister::from_string(operands[0])?,
                            FRegister::from_string(operands[1])?,
                            RoundingMode::from_str(mnemonics[3])?,
                        )),
                        ("s", "w") => Ok(Instruction::FCVTSW(
                            FRegister::from_string(operands[0])?,
                            IRegister::from_string(operands[1])?,
                            RoundingMode::from_str(mnemonics[3])?,
                        )),
                        ("s", "wu") => Ok(Instruction::FCVTSWU(
                            FRegister::from_string(operands[0])?,
                            IRegister::from_string(operands[1])?,
                            RoundingMode::from_str(mnemonics[3])?,
                        )),
                        ("l", "s") => Ok(Instruction::FCVTLS(
                            IRegister::from_string(operands[0])?,
                            FRegister::from_string(operands[1])?,
                            RoundingMode::from_str(mnemonics[3])?,
                        )),
                        ("lu", "s") => Ok(Instruction::FCVTLUS(
                            IRegister::from_string(operands[0])?,
                            FRegister::from_string(operands[1])?,
                            RoundingMode::from_str(mnemonics[3])?,
                        )),
                        ("s", "l") => Ok(Instruction::FCVTSL(
                            FRegister::from_string(operands[0])?,
                            IRegister::from_string(operands[1])?,
                            RoundingMode::from_str(mnemonics[3])?,
                        )),
                        ("s", "lu") => Ok(Instruction::FCVTSLU(
                            FRegister::from_string(operands[0])?,
                            IRegister::from_string(operands[1])?,
                            RoundingMode::from_str(mnemonics[3])?,
                        )),
                        _ => Err("invalid fcvt suffixes".to_owned()),
                    }
                } else {
                    Err("fcvt should have 2 or 3 suffixes".to_owned())
                }
            }
            "fmv" => {
                if operands.len() != 2 {
                    Err("fmv requires 2 operands".to_owned())
                } else if mnemonics.len() == 3 {
                    match (mnemonics[1], mnemonics[2]) {
                        ("x", "w") => Ok(Instruction::FMVXW(
                            IRegister::from_string(operands[0])?,
                            FRegister::from_string(operands[1])?,
                        )),
                        ("w", "x") => Ok(Instruction::FMVWX(
                            FRegister::from_string(operands[0])?,
                            IRegister::from_string(operands[1])?,
                        )),
                        _ => Err("invalid fmv suffixes".to_owned()),
                    }
                } else {
                    Err("fmv requires 2 suffixes".to_owned())
                }
            }
            "feq" => {
                if operands.len() != 3 {
                    Err("feq requires 3 operands".to_owned())
                } else if mnemonics.len() == 2 {
                    match mnemonics[1] {
                        "s" => Ok(Instruction::FEQS(
                            IRegister::from_string(operands[0])?,
                            FRegister::from_string(operands[1])?,
                            FRegister::from_string(operands[2])?,
                        )),
                        "d" => todo!(),
                        "q" => todo!(),
                        "h" => todo!(),
                        _ => Err("feq requires a suffix {s,d}".to_owned()),
                    }
                } else {
                    Err("feq requires a suffix {s,d}".to_owned())
                }
            }
            "flt" => {
                if operands.len() != 3 {
                    Err("flt requires 3 operands".to_owned())
                } else if mnemonics.len() == 2 {
                    match mnemonics[1] {
                        "s" => Ok(Instruction::FLTS(
                            IRegister::from_string(operands[0])?,
                            FRegister::from_string(operands[1])?,
                            FRegister::from_string(operands[2])?,
                        )),
                        "d" => todo!(),
                        "q" => todo!(),
                        "h" => todo!(),
                        _ => Err("flt requires a suffix {s,d}".to_owned()),
                    }
                } else {
                    Err("flt requires a suffix {s,d}".to_owned())
                }
            }
            "fle" => {
                if operands.len() != 3 {
                    Err("fle requires 3 operands".to_owned())
                } else if mnemonics.len() == 2 {
                    match mnemonics[1] {
                        "s" => Ok(Instruction::FLES(
                            IRegister::from_string(operands[0])?,
                            FRegister::from_string(operands[1])?,
                            FRegister::from_string(operands[2])?,
                        )),
                        "d" => todo!(),
                        "q" => todo!(),
                        "h" => todo!(),
                        _ => Err("fle requires a suffix {s,d}".to_owned()),
                    }
                } else {
                    Err("fle requires a suffix {s,d}".to_owned())
                }
            }
            "fclass" => {
                if operands.len() != 2 {
                    Err("fclass requires 2 operands".to_owned())
                } else if mnemonics.len() == 2 {
                    match mnemonics[1] {
                        "s" => Ok(Instruction::FCLASSS(
                            IRegister::from_string(operands[0])?,
                            FRegister::from_string(operands[1])?,
                        )),
                        "d" => todo!(),
                        "q" => todo!(),
                        "h" => todo!(),
                        _ => Err("fle requires a suffix {s,d}".to_owned()),
                    }
                } else {
                    Err("fle requires a suffix {s,d}".to_owned())
                }
            }
            "csrrw" => {
                if operands.len() != 3 {
                    Err("csrrw requires 3 operands".to_owned())
                }else {
                    Ok(Instruction::CSRRW(IRegister::from_string(operands[0])?, IRegister::from_string(operands[2])?, CSR::from_val(parse_int(operands[1])?)))
                }
            }
            "csrrs" => {
                if operands.len() != 3 {
                    Err("csrrs requires 3 operands".to_owned())
                }else {
                    Ok(Instruction::CSRRS(IRegister::from_string(operands[0])?, IRegister::from_string(operands[2])?, CSR::from_val(parse_int(operands[1])?)))
                }
            }
            "csrrc" => {
                if operands.len() != 3 {
                    Err("csrrc requires 3 operands".to_owned())
                }else {
                    Ok(Instruction::CSRRC(IRegister::from_string(operands[0])?, IRegister::from_string(operands[2])?, CSR::from_val(parse_int(operands[1])?)))
                }
            }
            "csrrwi" => {
                if operands.len() != 3 {
                    Err("csrrwi requires 3 operands".to_owned())
                }else {
                    Ok(Instruction::CSRRWI(IRegister::from_string(operands[0])?, CSRImmediate::from_val(parse_int(operands[2])?), CSR::from_val(parse_int(operands[1])?)))
                }
            }
            "csrrsi" => {
                if operands.len() != 3 {
                    Err("csrrsi requires 3 operands".to_owned())
                }else {
                    Ok(Instruction::CSRRSI(IRegister::from_string(operands[0])?, CSRImmediate::from_val(parse_int(operands[2])?), CSR::from_val(parse_int(operands[1])?)))
                }
            }
            "csrrci" => {
                if operands.len() != 3 {
                    Err("csrrci requires 3 operands".to_owned())
                }else {
                    Ok(Instruction::CSRRCI(IRegister::from_string(operands[0])?, CSRImmediate::from_val(parse_int(operands[2])?), CSR::from_val(parse_int(operands[1])?)))
                }
            }
            _ => Err(format!("unknown mnemonic: {}", mnemonic)),
        };
        x.map(AssemblyResult::I)
    }
}

/// Converts a string representing operations into a fence u8
pub fn parse_fence_set(s: &str) -> u8 {
    let mut x = 0;
    if s.contains("w") {
        x |= 0b1;
    }
    if s.contains("r") {
        x |= 0b10;
    }
    if s.contains("o") {
        x |= 0b100;
    }
    if s.contains("i") {
        x |= 0b1000;
    }
    x
}

/// Disassembles an instruction.
pub fn disassemble_instruction(instruction: &Instruction) -> String {
    format!("{}", instruction)
}

/// Constructs an `Instruction` from it's machine code representation.
pub fn decode_instruction(instruction: u32) -> Result<Instruction, String> {
    let opcode = Opcode::from_int(instruction & 0b111_1111);

    let func3 = (instruction >> 12) & 0b111;
    let func7 = (instruction >> 25) & 0b111_1111;

    let rd = IRegister::from_int((instruction >> 7) & 0b1_1111);
    let rs1 = IRegister::from_int((instruction >> 15) & 0b1_1111);
    let rs2 = IRegister::from_int((instruction >> 20) & 0b1_1111);

    let frd = FRegister::from_int((instruction >> 7) & 0b1_1111);
    let frs1 = FRegister::from_int((instruction >> 15) & 0b1_1111);
    let frs2 = FRegister::from_int((instruction >> 20) & 0b1_1111);
    let frs3 = FRegister::from_int((instruction >> 27) & 0b1_1111);

    let i_immediate: IImmediate = IImmediate::from_u32(instruction);

    let s_immediate: SImmediate = SImmediate::from_u32(instruction);

    let u_immediate = UImmediate::from_u32(instruction);

    let b_immediate = BImmediate::from_u32(instruction);

    let shamt: Shamt = Shamt::from_u32(instruction);

    let shamtw: ShamtW = ShamtW::from_u32(instruction);

    // aq is bit 26, rl is bit 25
    let aq: bool = ((instruction >> 26) & 0b1) == 0b1;
    let rl: bool = ((instruction >> 25) & 0b1) == 0b1;

    match opcode {
        Opcode::Load => match func3 {
            0b000 => Ok(Instruction::LB(rd, rs1, i_immediate)),
            0b001 => Ok(Instruction::LH(rd, rs1, i_immediate)),
            0b010 => Ok(Instruction::LW(rd, rs1, i_immediate)),
            0b011 => Ok(Instruction::LD(rd, rs1, i_immediate)),
            0b100 => Ok(Instruction::LBU(rd, rs1, i_immediate)),
            0b101 => Ok(Instruction::LHU(rd, rs1, i_immediate)),
            0b110 => Ok(Instruction::LWU(rd, rs1, i_immediate)),
            0b111 => Err("Invalid load func3".to_owned()),
            _ => unreachable!(),
        },
        Opcode::Auipc => Ok(Instruction::AUIPC(rd, u_immediate)),
        Opcode::Store => match func3 {
            0b000 => Ok(Instruction::SB(rs1, rs2, s_immediate)),
            0b001 => Ok(Instruction::SH(rs1, rs2, s_immediate)),
            0b010 => Ok(Instruction::SW(rs1, rs2, s_immediate)),
            0b011 => Ok(Instruction::SD(rs1, rs2, s_immediate)),
            x => Err(format!("invalid store func3: {}", x)),
        },
        Opcode::Lui => Ok(Instruction::LUI(rd, u_immediate)),
        Opcode::Op => match (func7, func3) {
            (0b000_0000, 0b000) => Ok(Instruction::ADD(rd, rs1, rs2)),
            (0b000_0000, 0b001) => Ok(Instruction::SLL(rd, rs1, rs2)),
            (0b000_0000, 0b010) => Ok(Instruction::SLT(rd, rs1, rs2)),
            (0b000_0000, 0b011) => Ok(Instruction::SLTU(rd, rs1, rs2)),
            (0b000_0000, 0b100) => Ok(Instruction::XOR(rd, rs1, rs2)),
            (0b000_0000, 0b101) => Ok(Instruction::SRL(rd, rs1, rs2)),
            (0b000_0000, 0b110) => Ok(Instruction::OR(rd, rs1, rs2)),
            (0b000_0000, 0b111) => Ok(Instruction::AND(rd, rs1, rs2)),
            (0b010_0000, 0b000) => Ok(Instruction::SUB(rd, rs1, rs2)),
            (0b010_0000, 0b101) => Ok(Instruction::SRA(rd, rs1, rs2)),
            (0b000_0001, 0b000) => Ok(Instruction::MUL(rd, rs1, rs2)),
            (0b000_0001, 0b001) => Ok(Instruction::MULH(rd, rs1, rs2)),
            (0b000_0001, 0b010) => Ok(Instruction::MULHSU(rd, rs1, rs2)),
            (0b000_0001, 0b011) => Ok(Instruction::MULHU(rd, rs1, rs2)),
            (0b000_0001, 0b100) => Ok(Instruction::DIV(rd, rs1, rs2)),
            (0b000_0001, 0b101) => Ok(Instruction::DIVU(rd, rs1, rs2)),
            (0b000_0001, 0b110) => Ok(Instruction::REM(rd, rs1, rs2)),
            (0b000_0001, 0b111) => Ok(Instruction::REMU(rd, rs1, rs2)),
            _ => Err(format!("unknown Op. func3: {}, func7: {}", func3, func7)),
        },
        Opcode::Op32 => match (func3, func7) {
            (0b000, 0b000_0000) => Ok(Instruction::ADDW(rd, rs1, rs2)),
            (0b000, 0b000_0001) => Ok(Instruction::MULW(rd, rs1, rs2)),
            (0b000, 0b010_0000) => Ok(Instruction::SUBW(rd, rs1, rs2)),
            (0b001, 0b000_0000) => Ok(Instruction::SLLW(rd, rs1, rs2)),
            (0b100, 0b0000_001) => Ok(Instruction::DIVW(rd, rs1, rs2)),
            (0b101, 0b000_0000) => Ok(Instruction::SRLW(rd, rs1, rs2)),
            (0b101, 0b000_0001) => Ok(Instruction::DIVUW(rd, rs1, rs2)),
            (0b101, 0b010_0000) => Ok(Instruction::SRAW(rd, rs1, rs2)),
            (0b110, 0b000_0001) => Ok(Instruction::REMW(rd, rs1, rs2)),
            (0b111, 0b000_0001) => Ok(Instruction::REMUW(rd, rs1, rs2)),
            _ => Err(format!("unknown Op32. func3: {}, func7: {}", func3, func7)),
        },
        Opcode::OpImm => match func3 {
            0b000 => Ok(Instruction::ADDI(rd, rs1, i_immediate)),
            // SLLi requires special handling because shamt uses the bottom bit of func7
            0b001 => match func7 | 0b1 {
                0b000000_1 => Ok(Instruction::SLLI(rd, rs1, shamt)),
                _ => Err(format!("unknown OpImm. func3: {}, func7: {}", func3, func7)),
            },
            0b010 => Ok(Instruction::SLTI(rd, rs1, i_immediate)),
            0b011 => Ok(Instruction::SLTIU(rd, rs1, i_immediate)),
            0b100 => Ok(Instruction::XORI(rd, rs1, i_immediate)),
            // SRLI SRAI require special handling because shamt uses the bottom bit of func7
            0b101 => match func7 | 0b1 {
                0b000000_1 => Ok(Instruction::SRLI(rd, rs1, shamt)),
                0b010000_1 => Ok(Instruction::SRAI(rd, rs1, shamt)),
                _ => Err(format!("unknown OpImm. func3: {}, func7: {}", func3, func7)),
            },
            0b110 => Ok(Instruction::ORI(rd, rs1, i_immediate)),
            0b111 => Ok(Instruction::ANDI(rd, rs1, i_immediate)),
            _ => Err(format!("unknown OpImm. func3: {}, func7: {}", func3, func7)),
        },
        Opcode::OpImm32 => match func3 {
            0b000 => Ok(Instruction::ADDIW(rd, rs1, i_immediate)),
            0b001 => Ok(Instruction::SLLIW(rd, rs1, shamtw)),
            0b101 => match func7 {
                0b000_0000 => Ok(Instruction::SRLIW(rd, rs1, shamtw)),
                0b010_0000 => Ok(Instruction::SRAIW(rd, rs1, shamtw)),
                x => Err(format!("unknown OpImm32(101) func7: {}", x).to_owned()),
            },
            x => Err(format!("unkown OpImm32 func3: {}", x).to_owned()),
        },
        Opcode::Jalr => Ok(Instruction::JALR(rd, rs1, i_immediate)),
        Opcode::Jal => Ok(Instruction::JAL(rd, JImmediate::from_u32(instruction))),
        Opcode::Branch => match func3 {
            0b000 => Ok(Instruction::BEQ(rs1, rs2, b_immediate)),
            0b001 => Ok(Instruction::BNE(rs1, rs2, b_immediate)),
            0b100 => Ok(Instruction::BLT(rs1, rs2, b_immediate)),
            0b101 => Ok(Instruction::BGE(rs1, rs2, b_immediate)),
            0b110 => Ok(Instruction::BLTU(rs1, rs2, b_immediate)),
            0b111 => Ok(Instruction::BGEU(rs1, rs2, b_immediate)),
            x => Err(format!("invalid branch func3: {x}").to_owned()),
        },
        Opcode::MiscMem => match func3 {
            0b000 => {
                if rd != IRegister::Zero || rs1 != IRegister::Zero {
                    // technicially, we are supposed to ignore these fields
                    Err("reserved register fields not set to zero".to_owned())
                } else {
                    let fm = ((instruction >> 28) & 0b1111) as u8;
                    if fm != 0 && fm != 0b1000 {
                        Err(format!("reserved fence FM: {fm}").to_owned())
                    } else if fm == 0b1000 && ((instruction >> 20) & 0xFF) != 0b0011_0011 {
                        Err("fence.tso must be rw,rw".to_owned())
                    } else {
                        Ok(Instruction::FENCE(
                            rd,
                            rs1,
                            ((instruction >> 20) & 0xFF) as u8,
                            ((instruction >> 28) & 0b1111) as u8,
                        ))
                    }
                }
            }
            x => Err(format!("unknown fence func3: {x}")),
        },
        Opcode::AMO => match (func3, func7 >> 2) {
            (0b010, 0b00010) => {
                if rs2 != IRegister::Zero {
                    Err("LR.W expects rs2 to be 0".to_owned())
                } else {
                    Ok(Instruction::LRW(rd, rs1, aq, rl))
                }
            }
            (0b011, 0b00010) => {
                if rs2 != IRegister::Zero {
                    Err("LR.D expects rs2 to be 0".to_owned())
                } else {
                    Ok(Instruction::LRD(rd, rs1, aq, rl))
                }
            }
            (0b010, 0b00011) => Ok(Instruction::SCW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b00011) => Ok(Instruction::SCD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b00001) => Ok(Instruction::AMOSWAPW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b00001) => Ok(Instruction::AMOSWAPD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b00000) => Ok(Instruction::AMOADDW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b00000) => Ok(Instruction::AMOADDD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b00100) => Ok(Instruction::AMOXORW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b00100) => Ok(Instruction::AMOXORD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b01100) => Ok(Instruction::AMOANDW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b01100) => Ok(Instruction::AMOANDD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b01000) => Ok(Instruction::AMOORW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b01000) => Ok(Instruction::AMOORD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b10000) => Ok(Instruction::AMOMINW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b10000) => Ok(Instruction::AMOMIND(rd, rs1, rs2, aq, rl)),
            (0b010, 0b10100) => Ok(Instruction::AMOMAXW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b10100) => Ok(Instruction::AMOMAXD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b11000) => Ok(Instruction::AMOMINUW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b11000) => Ok(Instruction::AMOMINUD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b11100) => Ok(Instruction::AMOMAXUW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b11100) => Ok(Instruction::AMOMAXUD(rd, rs1, rs2, aq, rl)),
            _ => Err(format!("unknown AMO. func3: {func3}, func7: {func7}")),
        },
        Opcode::LoadFp => {
            println!("{i_immediate}, {:b}", instruction);
            if func3 == 0b010 {
                Ok(Instruction::FLW(frd, rs1, i_immediate))
            } else {
                Err(format!("unknown func3: {func3} in opcode LoadFp"))
            }
        }
        Opcode::StoreFp => {
            if func3 == 0b010 {
                Ok(Instruction::FSW(rs1, frs2, s_immediate))
            } else {
                Err(format!("unknown func3: {func3} in opcode LoadFp"))
            }
        }
        Opcode::OpFp => match func7 {
            0b000_0000 => Ok(Instruction::FADDS(
                frd,
                frs1,
                frs2,
                RoundingMode::from_int(func3)?,
            )),
            0b000_0100 => Ok(Instruction::FSUBS(
                frd,
                frs1,
                frs2,
                RoundingMode::from_int(func3)?,
            )),
            0b000_1000 => Ok(Instruction::FMULS(
                frd,
                frs1,
                frs2,
                RoundingMode::from_int(func3)?,
            )),
            0b000_1100 => Ok(Instruction::FDIVS(
                frd,
                frs1,
                frs2,
                RoundingMode::from_int(func3)?,
            )),
            0b010_1100 => Ok(Instruction::FSQRTS(
                frd,
                frs1,
                RoundingMode::from_int(func3)?,
            )),
            0b001_0000 => match func3 {
                0b000 => Ok(Instruction::FSGNJS(frd, frs1, frs2)),
                0b001 => Ok(Instruction::FSGNJNS(frd, frs1, frs2)),
                0b010 => Ok(Instruction::FSGNJXS(frd, frs1, frs2)),
                x => Err(format!("unknown OpFp func7=0b001_0000 func3: {}", x)),
            },
            0b001_0100 => match func3 {
                0b000 => Ok(Instruction::FMINS(frd, frs1, frs2)),
                0b001 => Ok(Instruction::FMAXS(frd, frs1, frs2)),
                x => Err(format!("unknown OpFp func7=0b001_0100 func3: {}", x)),
            },
            0b101_0000 => match func3 {
                0b000 => Ok(Instruction::FLES(rd, frs1, frs2)),
                0b001 => Ok(Instruction::FLTS(rd, frs1, frs2)),
                0b010 => Ok(Instruction::FEQS(rd, frs1, frs2)),
                x => Err(format!("unknown OpFp func7=0b101_0000 func3: {}", x)),
            },
            0b110_0000 => match (instruction >> 20) & 0b1_1111 {
                0b0_0000 => Ok(Instruction::FCVTWS(
                    rd,
                    frs1,
                    RoundingMode::from_int(func3)?,
                )),
                0b0_0001 => Ok(Instruction::FCVTWUS(
                    rd,
                    frs1,
                    RoundingMode::from_int(func3)?,
                )),
                0b0_0010 => Ok(Instruction::FCVTLS(
                    rd,
                    frs1,
                    RoundingMode::from_int(func3)?,
                )),
                0b0_0011 => Ok(Instruction::FCVTLUS(
                    rd,
                    frs1,
                    RoundingMode::from_int(func3)?,
                )),
                x => Err(format!("unknown OpFp func7=0b001_0100 rs2: {}", x)),
            },
            0b110_1000 => match (instruction >> 20) & 0b1_1111 {
                0b0_0000 => Ok(Instruction::FCVTSW(
                    frd,
                    rs1,
                    RoundingMode::from_int(func3)?,
                )),
                0b0_0001 => Ok(Instruction::FCVTSWU(
                    frd,
                    rs1,
                    RoundingMode::from_int(func3)?,
                )),
                0b0_0010 => Ok(Instruction::FCVTSL(
                    frd,
                    rs1,
                    RoundingMode::from_int(func3)?,
                )),
                0b0_0011 => Ok(Instruction::FCVTSLU(
                    frd,
                    rs1,
                    RoundingMode::from_int(func3)?,
                )),
                x => Err(format!("unknown OpFp func7=0b001_0100 rs2: {}", x)),
            },
            0b111_0000 => {
                if (instruction >> 20) & 0b1_1111 == 0 {
                    if func3 == 0 {
                        Ok(Instruction::FMVXW(rd, frs1))
                    } else if func3 == 1 {
                        Ok(Instruction::FCLASSS(rd, frs1))
                    } else {
                        Err(format!(
                            "unknown OpFp func7=0b111_0000 rs2=0 func3: {}",
                            func3
                        ))
                    }
                } else {
                    Err(format!(
                        "unknown OpFp func7=0b111_0000 unknown rs2: {} and func3: {}",
                        (instruction >> 20) & 0b1_1111,
                        func3
                    ))
                }
            }
            0b111_1000 => {
                if (instruction >> 20) & 0b1_1111 == 0 {
                    if func3 == 0 {
                        Ok(Instruction::FMVWX(frd, rs1))
                    } else {
                        Err(format!(
                            "unknown OpFp func7=0b111_1000 rs2=0 func3: {}",
                            func3
                        ))
                    }
                } else {
                    Err(format!(
                        "unknown OpFp func7=0b111_0000 unknown rs2: {} and func3: {}",
                        (instruction >> 20) & 0b1_1111,
                        func3
                    ))
                }
            }
            x => Err(format!("Unknown OpFp func7: {x}")),
        },
        Opcode::Reserved => Err("instruction uses reserved opcode".to_owned()),
        Opcode::Madd => {
            if func7 & 0b11 == 0 {
                Ok(Instruction::FMADDS(
                    frd,
                    frs1,
                    frs2,
                    frs3,
                    RoundingMode::from_int(func3)?,
                ))
            } else {
                Err(format!(
                    "FMADD unknown lower 2 bits of func7: {}",
                    func7 & 0b11
                ))
            }
        }
        Opcode::Msub => {
            if func7 & 0b11 == 0 {
                Ok(Instruction::FMSUBS(
                    frd,
                    frs1,
                    frs2,
                    frs3,
                    RoundingMode::from_int(func3)?,
                ))
            } else {
                Err(format!(
                    "FMSUB unknown lower 2 bits of func7: {}",
                    func7 & 0b11
                ))
            }
        }
        Opcode::Nmsub => {
            if func7 & 0b11 == 0 {
                Ok(Instruction::FNMSUBS(
                    frd,
                    frs1,
                    frs2,
                    frs3,
                    RoundingMode::from_int(func3)?,
                ))
            } else {
                Err(format!(
                    "FMNSUB unknown lower 2 bits of func7: {}",
                    func7 & 0b11
                ))
            }
        }
        Opcode::Nmadd => {
            if func7 & 0b11 == 0 {
                Ok(Instruction::FNMADDS(
                    frd,
                    frs1,
                    frs2,
                    frs3,
                    RoundingMode::from_int(func3)?,
                ))
            } else {
                Err(format!(
                    "FNMADD unknown lower 2 bits of func7: {}",
                    func7 & 0b11
                ))
            }
        }
        Opcode::System => match func3 {
            0b000 => Err("Reserved func3 in Opcode SYSTEM".to_owned()),
            0b001 => Ok(Instruction::CSRRW(rd, rs1, CSR::from_u32(instruction))),
            0b010 => Ok(Instruction::CSRRS(rd, rs1, CSR::from_u32(instruction))),
            0b011 => Ok(Instruction::CSRRC(rd, rs1, CSR::from_u32(instruction))),
            0b100 => Err("Reserved func3 in Opcode SYSTEM".to_owned()),
            0b101 => Ok(Instruction::CSRRWI(rd, CSRImmediate::from_u32(instruction), CSR::from_u32(instruction))),
            0b110 => Ok(Instruction::CSRRSI(rd, CSRImmediate::from_u32(instruction), CSR::from_u32(instruction))),
            0b111 => Ok(Instruction::CSRRCI(rd, CSRImmediate::from_u32(instruction), CSR::from_u32(instruction))),
            _ => unreachable!(),
        },
    }
}
