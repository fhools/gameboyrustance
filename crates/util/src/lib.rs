use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;
#[cfg(test)]
mod gba;

pub fn get_bits(i: u32, lsb: usize, msb: usize) -> u32 {
    let num_bits = msb - lsb + 1;
    let mask = (1 << num_bits) - 1;
    let result = i >> lsb;
    result & mask
}

pub fn to_vec_words<T: AsRef<[u8]>>(v: T) -> Vec<u32> {
    assert!(
        v.as_ref().len() % 4 == 0,
        "vector length must be mutiple of 4 bytes"
    );
    println!("v: {:?}", v.as_ref());
    let u = v
        .as_ref()
        .chunks_exact(4)
        .map::<u32, fn(&[u8]) -> u32>(|w| {
            let mut w32: u32 = 0;
            w32 |= w[0] as u32;
            w32 |= (w[1] as u32) << 8;
            w32 |= (w[2] as u32) << 16;
            w32 |= (w[3] as u32) << 24;
            println!("w: {:x}", w32);
            w32
        })
        .collect::<Vec<u32>>();
    u
}

pub fn read_instructions_file<T: AsRef<Path>>(
    filepath: T,
    offset: usize,
    num_words: usize,
) -> Result<Vec<u32>, std::io::Error> {
    let mut buf: Vec<u8> = vec![0; num_words * 4];
    let mut f = File::open(filepath)?;
    f.seek(SeekFrom::Start(offset as u64))?;
    f.read_exact(&mut buf)?;
    println!("buf: {:?}", buf);
    Ok(to_vec_words(&buf))
}

mod tests {
    use super::get_bits;
    use super::read_instructions_file;
    use super::to_vec_words;
    #[test]
    fn test_get_bits() {
        let a = 0xDEADBEEF;
        let b = get_bits(a, 4, 16);
        println!("bits {}-{} {:x}", 4, 12, b);
    }

    #[test]
    fn test_to_vec_words() {
        let input = vec![0xde, 0xea, 0xbe, 0xef];
        let out = to_vec_words(&input);
        println!("out: {:x}", out[0]);
        assert_eq!(out[0], 0xefbeeade);
    }

    #[test]
    fn test_read_instructions_file() -> Result<(), std::io::Error> {
        let instrs = read_instructions_file("a.gba", 0xe0, 10)?;
        assert_eq!(instrs.len(), 10);
        Ok(())
    }
}
