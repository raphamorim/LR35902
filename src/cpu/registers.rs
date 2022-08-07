#[derive(Debug)]
pub struct Clock {
    pub m: u32,
    pub t: u32,
}

impl Clock {
    fn set_t(&mut self, t: u32) {
        self.t = t;
    }

    fn set_m(&mut self, m: u32) {
        self.m = m;
    }

    pub fn inc_t(&mut self, t: u16) {
        let at = t as u32;
        self.t += at;
    }

    pub fn inc_m(&mut self, m: u16) {
        let am = m as u32;
        self.t += am;
    }
}

#[derive(Debug)]
pub struct Registers {
    // 8-bit registers
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub ime: u8,

    // The flags register (F)
    // it automatically calculates certain bits, or flags, based on the result of the last operation.
    pub f: u8,

    // Clock for last instruction
    pub m: u16,
    pub t: u16,

    // 16-bit registers
    pub pc: u16,
    pub sp: u16,
    // Internal state
    // pub clock: Clock,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x11,    // 17
            f: 0xB0,    // 176
            b: 0x00,    // 0
            c: 0x13,    // 19
            d: 0x00,    // 0
            e: 0xD8,    // 216
            h: 0x01,    // 1
            l: 0x4D,    // 77
            pc: 0x0100, // Starts with 256
            sp: 0xFFFE, // 65534
            m: 0,
            t: 0,
            ime: 0,
            // clock: Clock { m: 0, t: 0 },
        }
    }
    pub fn flag(&mut self, flags: CpuFlag, set: bool) {
        let mask = flags as u8;
        match set {
            true => self.f |= mask,
            false => self.f &= !mask,
        }
        self.f &= 0xF0;
    }

    pub fn getflag(&self, flags: CpuFlag) -> bool {
        let mask = flags as u8;
        self.f & mask > 0
    }
}

#[derive(Copy, Clone)]
pub enum CpuFlag {
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_set() {
        let mut clock: Clock = Clock { t: 0, m: 0 };
        assert_eq!(clock.t, 0);
        assert_eq!(clock.m, 0);

        clock.set_m(1);
        clock.set_t(2);

        assert_eq!(clock.t, 2);
        assert_eq!(clock.m, 1);
    }
}
