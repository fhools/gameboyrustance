extern crate arm7tdmi;
use crate::arm7tdmi::arm::ARMCpu;


pub fn dump_cpu(_a: &ARMCpu) {
    println!("using armv7 cpu");

}
#[cfg(test)]
mod tests {
    use crate::dump_cpu;
    extern crate arm7tdmi;
    use crate::arm7tdmi::arm::ARMCpu;
    #[test]
    fn it_works() {
        dump_cpu(&ARMCpu{});
    }
}
