use std::fmt::{Display, Formatter};

/// The immediate values in I-type instructions (like addi)
#[derive(Debug, PartialEq)]
pub struct IImmediate {
    val: i16,
}

impl IImmediate {
    /// Extracts the `IImmediate` from the appropriate position in a 32-bit instruction
    pub fn from_u32(x: u32) -> Self {
        let unsigned: u32 = ((x >> 20) & 0b1111_1111_1111).try_into().unwrap();
        // sign extend 12 bit value
        let y = unsigned.overflowing_shl(20).0 as i32;
        let val = y.overflowing_shr(20).0 as i16;
        IImmediate { val }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2047 || val < -2048 {
            panic!("attempted to construct out of range IImediate")
        }
        IImmediate { val: val as i16 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for IImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate values in S-type instructions (like SW)
#[derive(Debug, PartialEq)]
pub struct SImmediate {
    val: i16,
}

impl SImmediate {
    /// Extracts the `SImmediate` from the appropriate position in a 32-bit instruction
    pub fn from_u32(x: u32) -> Self {
        let unsigned: u32 =
            (((x >> 25) & 0b111_1111) << 5) | ((x >> 7) & 0b1_1111);
        // sign extend 12 bit value
        let y = unsigned.overflowing_shl(20).0 as i32;
        let val = y.overflowing_shr(20).0 as i16;
        SImmediate { val }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2047 || val < -2048 {
            panic!("attempted to construct out of range IImediate")
        }
        SImmediate { val: val as i16 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for SImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate values in shift-by-immediate instructions (SRAI)
#[derive(Debug, PartialEq)]
pub struct Shamt {
    val: u8,
}

impl Shamt {
    /// Extracts the `Shamt` from the appropriate position in a 32-bit instruction
    pub fn from_u32(x: u32) -> Self {
        let val: u8 = ((x >> 20) & 0b11_1111) as u8;
        Shamt { val }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 63 || val < 0 {
            panic!("attempted to construct out of range Shamt")
        }
        Shamt { val: val as u8 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for Shamt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate values in shift-by-immediate word instructions (SRAIW)
#[derive(Debug, PartialEq)]
pub struct ShamtW {
    val: u8,
}

impl ShamtW {
    /// Extracts the `IImmediate` from the appropriate position in a 32-bit instruction
    pub fn from_u32(x: u32) -> Self {
        let val: u8 = ((x >> 20) & 0b1_1111) as u8;
        ShamtW { val }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 31 || val < 0 {
            panic!("attempted to construct out of range Shamtw")
        }
        ShamtW { val: val as u8 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for ShamtW {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate values in LUI and AUIPC
#[derive(Debug, PartialEq)]
pub struct UImmediate {
    val: i32,
}


impl UImmediate {
    /// Extracts the `UImmediate` from the appropriate position in a 32-bit instruction
    pub fn from_u32(x: u32) -> Self {
        let val = (x as i32) >> 12;
        UImmediate { val }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(19) - 1 || val < -1 * 2i64.pow(19) {
            panic!("attempted to construct out of range UImediate")
        }
        UImmediate { val: val as i32 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for UImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in JAL
#[derive(Debug, PartialEq)]
pub struct JImmediate {
    val: i32,
}


impl JImmediate {
    /// Extracts the `JImmediate` from the appropriate position in a 32-bit instruction
    pub fn from_u32(x: u32) -> Self {
        let a = (x >> 12) & 0b1111_1111;
        let b = (x >> 20) & 0b1;
        let c = (x >> 21) & 0b11_1111_1111;
        let d = x >> 31;
        let i: i32 = ((c << 1) | (b << 11) | (a << 12) | (d << 20)) as i32;
        // sign extend 21 bit value
        let i2: i32 = (i << 11) >> 11;
        JImmediate { val: i2 }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(20) - 1 || val < -1 * 2i64.pow(20) {
            panic!("attempted to construct out of range JImediate")
        }
        if val % 2 != 0 {
            panic!("attempted to construct odd JImmediate")
        }
        JImmediate { val: val as i32 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for JImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in JAL
#[derive(Debug, PartialEq)]
pub struct BImmediate {
    val: i32,
}

impl BImmediate {
    /// Extracts the `BImmediate` from the appropriate position in a 32-bit instruction
    pub fn from_u32(x: u32) -> Self {
        let a = (x >> 7) & 0b1;
        let b = (x >> 8) & 0b1111;
        let c = (x >> 25) & 0b11_1111;
        let d = x >> 31;
        
        let i: i32 = ((b << 1) | (c << 5) | (a << 11) | (d << 12)) as i32;
        let i2 = (i << 19) >> 19;
        BImmediate { val: i2 }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(12) - 1 || val < -1 * 2i64.pow(12) {
            panic!("attempted to construct out of range JImediate")
        }
        if val % 2 != 0 {
            panic!("attempted to construct odd BImmediate")
        }
        BImmediate { val: val as i32 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for BImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in wide immediate compressed instructions
#[derive(Debug, PartialEq)]
pub struct CWideImmediate {
    val: i32,
}


impl CWideImmediate {
    /// Extracts the `CWImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = (x >> 5) & 0b1;
        let b = (x >> 6) & 0b1;
        let c = (x >> 7) & 0b1111;
        let d = (x >> 11) & 0b11;
        
        let i: i32 = ((b << 2) | (a << 3) | (d << 4) | (c << 6)) as i32;
        // CWImmediate is zero-extended
        CWideImmediate { val: i }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(10) - 1 || val < 0 {
            panic!("attempted to construct out of range CWideImmediate")
        }
        if val % 4 != 0 {
            panic!("attempted to construct non multiple of 4 CWideImmediate")
        }
        CWideImmediate { val: val as i32 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for CWideImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in compressed, load/store doubleword instructions
#[derive(Debug, PartialEq)]
pub struct CDImmediate {
    val: i32,
}


impl CDImmediate {
    /// Extracts the `CLDImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = (x >> 5) & 0b11;
        let b = (x >> 10) & 0b111;

        let i: i32 = ((b << 3) | (a << 6)) as i32;
        // CLDImmediate is zero-extended
        CDImmediate { val: i }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(8) - 1 || val < 0 {
            panic!("attempted to construct out of range CDImmediate")
        }
        if val % 8 != 0 {
            panic!("attempted to construct non multiple of 8 CDImmediate")
        }
        CDImmediate { val: val as i32 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for CDImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in compressed, load/store word instructions
#[derive(Debug, PartialEq)]
pub struct CWImmediate {
    val: i32,
}


impl CWImmediate {
    /// Extracts the `CLWImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = (x >> 5) & 0b1;
        let b = (x >> 6) & 0b1;
        let c = (x >> 10) & 0b111;

        let i: i32 = ((b << 2) | (c << 3) | (a << 6)) as i32;
        // CLWImmediate is zero-extended
        CWImmediate { val: i }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(7) - 1 || val < 0 {
            panic!("attempted to construct out of range CWImmediate")
        }
        if val % 4 != 0 {
            panic!("attempted to construct non multiple of 4 CWImmediate")
        }
        CWImmediate { val: val as i32 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for CWImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in compressed immediate operation instructions
#[derive(Debug, PartialEq)]
pub struct CIImmediate {
    val: i32,
}

impl CIImmediate {
    /// Extracts the `CIImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = (x >> 2) & 0b1111;
        let b = (x >> 12) & 0b1;
        let i: i32 = (a | (b << 5)) as i32;
        let i2 = (i << 26) >> 26;
        CIImmediate { val: i2 }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(5) - 1 || val < -1 * 2i64.pow(5) {
            panic!("attempted to construct out of range CIImmediate")
        }
        CIImmediate { val: val as i32 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for CIImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in compressed branch instructions
#[derive(Debug, PartialEq)]
pub struct CBImmediate {
    val: i32,
}

impl CBImmediate {
    /// Extracts the `CBImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = (x >> 2) & 0b1;
        let b = (x >> 3) & 0b11;
        let c = (x >> 5) & 0b11;
        let d = (x >> 10) & 0b11;
        let e = (x >> 12) & 0b1;

        let i: i32 = ((b << 1) | (d << 3) | (a << 5) | (c << 6) | (e << 8)) as i32;
        let i2 = (i << 26) >> 26;
        CBImmediate { val: i2 }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(5) - 1 || val < -1 * 2i64.pow(5) {
            panic!("attempted to construct out of range CBImmediate")
        }
        CBImmediate { val: val as i32 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for CBImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in compressed bit shift instructions
#[derive(Debug, PartialEq)]
pub struct CShamt {
    val: u32,
}

impl CShamt {
    /// Extracts the `CShamt` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = (x >> 2) & 0b1111;
        let b = (x >> 12) & 0b1;
        let i: u32 = (a | (b << 5)) as u32;
        CShamt { val: i }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(6) - 1 || val < 0 {
            panic!("attempted to construct out of range CIImmediate")
        }
        CShamt { val: val as u32 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for CShamt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}