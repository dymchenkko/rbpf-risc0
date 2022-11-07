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