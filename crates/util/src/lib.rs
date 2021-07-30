#[cfg(test)]
mod gba;

pub fn get_bits(i: u32, start: usize, end: usize) -> u32 {
    let num_bits = end - start;
    let mask  = (1 << num_bits) -1;
    let result = i >> start;
    result & mask
}

mod tests {
    use super::get_bits;
    #[test]
    fn test_get_bits() {
        let a = 0xDEADBEEF;
        let b = get_bits(a,4,12);
        println!("{:x}", b);
    }
}
