pub enum Types { //rust类型映射
    RI8(i8),
    RI16(i16),
    RI32(i32),
    RI64(i64),
    RU8(u8),
    RU16(u16),
    RU32(u32),
    RU64(u64),
    RF32(f32),
    RF64(f64),
    RFBOOL(bool),
    RString(String)
}

pub struct Block { //块
    key: u64,
    heap: Vec<Types>,
    codes: Vec<u8>
}
pub struct LmObject { //基类
    key: u64,
    value: Types,
    parent: Block
}

pub enum ByteInstruction { //字节指令
    VMCALL,SYSCALL,
    MOVRI,MOVRR,MOVRM,
    MOVMI,MOVMR,MOVMM,
    ADDI,ADDR,ADDM,
    SUBI,SUBR,SUBM,
    MULI,MULR,MULM,
    DIVI,DIVR,DIVM,

    R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,
    UNKNOWN,
    END,
    
}
impl ByteInstruction {
    pub(crate) fn get_u8_to_byte(b:&u8) -> ByteInstruction {
        match b {
            0 => ByteInstruction::VMCALL,
            1 => ByteInstruction::SYSCALL,
            2 => ByteInstruction::MOVRI,
            3 => ByteInstruction::MOVRR,
            4 => ByteInstruction::MOVRM,
            5 => ByteInstruction::MOVMI,
            6 => ByteInstruction::MOVMR,
            7 => ByteInstruction::MOVMM,
            8 => ByteInstruction::ADDI,
            9 => ByteInstruction::ADDR,
            10 => ByteInstruction::ADDM,
            11 => ByteInstruction::SUBI,
            12 => ByteInstruction::SUBR,
            13 => ByteInstruction::SUBM,
            14 => ByteInstruction::MULI,
            15 => ByteInstruction::MULR,
            16 => ByteInstruction::MULM,
            17 => ByteInstruction::DIVI,
            18 => ByteInstruction::DIVR,
            19 => ByteInstruction::DIVM,
            20 => ByteInstruction::R0,
            21 => ByteInstruction::R1,
            22 => ByteInstruction::R2,
            23 => ByteInstruction::R3,
            24 => ByteInstruction::R4,
            25 => ByteInstruction::R5,
            26 => ByteInstruction::R6,
            27 => ByteInstruction::R7,
            28 => ByteInstruction::R8,
            29 => ByteInstruction::R9,
            30 => ByteInstruction::UNKNOWN,
            31 => ByteInstruction::END,
            _ => ByteInstruction::UNKNOWN,
        }
    }

}