#[cfg(test)]
mod gba;

pub fn get_bits(i: u32, lsb: usize, msb: usize) -> u32 {
    let num_bits = msb - lsb + 1;
    let mask  = (1 << num_bits) - 1;
    let result = i >> lsb;
    result & mask
}

mod tests {
    use super::get_bits;
    #[test]
    fn test_get_bits() {
        let a = 0xDEADBEEF;
        let b = get_bits(a,4,16);
        println!("bits {}-{} {:x}",4,12, b);
    }
}
