use byteorder::{ByteOrder, LittleEndian};
use std::io::{Error, ErrorKind};


pub const INSN_SIZE: usize = 8;
pub const BPF_JMP   : u8 = 0x05;
pub const BPF_EXIT  : u8 = 0x90;
pub const EXIT       : u8 = BPF_JMP   | BPF_EXIT;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Insn {
    /// Operation code.
    pub opc: u8,
    /// Destination register operand.
    pub dst: u8,
    /// Source register operand.
    pub src: u8,
    /// Offset operand.
    pub off: i16,
    /// Immediate value operand.
    pub imm: i32,
}

pub fn get_insn(prog: &[u8], idx: usize) -> Insn {
    // This guard should not be needed in most cases, since the verifier already checks the program
    // size, and indexes should be fine in the interpreter/JIT. But this function is publicly
    // available and user can call it with any `idx`, so we have to check anyway.
    if (idx + 1) * INSN_SIZE > prog.len() {
        panic!("Error: cannot reach instruction at index {:?} in program containing {:?} bytes",
               idx, prog.len());
    }
    Insn {
        opc:  prog[INSN_SIZE * idx],
        dst:  prog[INSN_SIZE * idx + 1] & 0x0f,
        src: (prog[INSN_SIZE * idx + 1] & 0xf0) >> 4,
        off: LittleEndian::read_i16(&prog[(INSN_SIZE * idx + 2) .. ]),
        imm: LittleEndian::read_i32(&prog[(INSN_SIZE * idx + 4) .. ]),
    }
}


pub fn test_execute_program2(prog: &[u8], mut reg: [u64;11]) -> Result<u64, Error> {

    let mut insn_ptr:usize = 0;

    while insn_ptr * INSN_SIZE < prog.len() {
        let insn = get_insn(prog, insn_ptr);
        insn_ptr += 1;
        let _dst = insn.dst as usize;
        eprintln!("{:?}", insn);
        eprintln!("{:?}", reg);
        if (insn.opc == EXIT) { return Ok(reg[0]) }
        reg[_dst] = insn.imm  as u32                                as u64;
    }
    unreachable!()
}


pub fn test_execute_program(mut prog: &[u8]) -> Result<u64, Error> {
        
    let mut reg: [u64;11] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 //stack.as_ptr() as u64 + stack.len() as u64
    ];

    let mut insn_ptr:usize = 0;
    while insn_ptr * INSN_SIZE < prog.len() {
        let insn = get_insn(prog, insn_ptr);
        insn_ptr += 1;
        let _dst = insn.dst as usize;
        eprintln!("{:?}", insn);
        eprintln!("{:?}", reg);
        if (insn.opc == EXIT) { return Ok(reg[0]) }
        reg[_dst] = insn.imm  as u32                                as u64;
    }
    unreachable!()
}