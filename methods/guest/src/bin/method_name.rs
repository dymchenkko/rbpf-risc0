// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]

risc0_zkvm_guest::entry!(main);

use execute_test;

pub fn main(){
    let prog = &[
        0xb4, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // mov32 r0, 2
        0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    ];

/*    let mut reg: [u64;11] = [
       0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ];

*/
//    eprintln!("Initializing reg inside the function:");

    let result = execute_test::test_execute_program(prog);

/*    eprintln!("Reg as input:");
    let result = execute_test::test_execute_program2(prog, reg);
*/
}
