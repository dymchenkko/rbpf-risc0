use rbpf::ebpf;
use std::io::{Error, ErrorKind};

pub fn test_execute_program2(prog: &[u8], mut reg: [u64;11]) -> Result<u64, Error> {

    let mut insn_ptr:usize = 0;

    while insn_ptr * ebpf::INSN_SIZE < prog.len() {
        let insn = ebpf::get_insn(prog, insn_ptr);
        insn_ptr += 1;
        let _dst = insn.dst as usize;
        eprintln!("{:?}", insn);
        eprintln!("{:?}", reg);
        if (insn.opc == ebpf::EXIT) { return Ok(reg[0]) }
        reg[_dst] = insn.imm  as u32                                as u64;
    }
    unreachable!()
}


pub fn test_execute_program(s: rbpf::EbpfVmNoData, mut prog: &[u8]) -> Result<u64, Error> {
        
    let stack = vec![0u8;ebpf::STACK_SIZE];

    let mut reg: [u64;11] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, stack.as_ptr() as u64 + stack.len() as u64
    ];

    let mut insn_ptr:usize = 0;
    while insn_ptr * ebpf::INSN_SIZE < prog.len() {
        let insn = ebpf::get_insn(prog, insn_ptr);
        insn_ptr += 1;
        let _dst = insn.dst as usize;
        eprintln!("{:?}", insn);
        eprintln!("{:?}", reg);
        if (insn.opc == ebpf::EXIT) { return Ok(reg[0]) }
        reg[_dst] = insn.imm  as u32                                as u64;
    }
    unreachable!()
}