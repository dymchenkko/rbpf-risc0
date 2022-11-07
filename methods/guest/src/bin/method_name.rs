// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]

risc0_zkvm_guest::entry!(main);
use rbpf::ebpf;
use execute_test;

pub fn main(){
    // TODO: Implement your guest code here

    let prog = &[
        0xb4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov32 r0, 0
        0xb4, 0x01, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // mov32 r1, 2
        0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // add32 r0, 1
        0x0c, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // add32 r0, r1
        0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    ];

    let stack = vec![0u8;ebpf::STACK_SIZE];

        let mut reg: [u64;11] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, stack.as_ptr() as u64 + stack.len() as u64
        ];


    // Instantiate a struct EbpfVmNoData. This is an eBPF VM for programs that
    // takes no packet data in argument.
    // The eBPF program is passed to the constructor.
    let vm = rbpf::EbpfVmNoData::new(Some(prog)).unwrap();
    eprintln!("Initializing reg inside the function:");

    let result = execute_test::test_execute_program(vm, prog);

    eprintln!("Reg as input:");
    let result = execute_test::test_execute_program2(prog, reg);

    // Execute (interpret) the program. No argument required for this VM.
}
