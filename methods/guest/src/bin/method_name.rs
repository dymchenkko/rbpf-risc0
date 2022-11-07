// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]

risc0_zkvm_guest::entry!(main);

pub fn test_execute_program(prog: &[u8]) -> u64 {
    let mut reg: [u64;2] = [
        0, 1
    ];

    reg[prog[0] as usize] = 2;
    eprintln!("{:?}", reg);
    eprintln!("{:?}", prog);
    assert_eq!(reg[0], 2);

    return reg[0];
}


//use execute_test;

pub fn main(){
    let prog = &[  
          0x0
    ];

    let result = test_execute_program(prog);
}
