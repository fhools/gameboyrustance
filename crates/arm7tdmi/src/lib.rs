mod thumb;
mod arm;

// ARM7TDMI is an ARM cpu with 2 modes of instruction, a 32-bit ARM and a 16-bit THUMB.
//
// The CPU switches between the two states with a BX instruction.
// The CPU modes share the same register set
// ARM is 32-bit opcodes and THUMB is 16-bit opcodes
//
// ARM7TDMI is a 3 stage pipelined architecture, with fetch, decode, execute stages.
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
