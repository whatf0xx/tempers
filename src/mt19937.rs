use itertools::enumerate;
use core::u32;
use std::num::Wrapping;

#[allow(dead_code)]
pub struct MT19937 {
    w: u32,  // 32 for 32-bit integers, don't change this because all the params depend on it
    n: u32,
    m: u32,
    r: u32,  // a field, but more of a pain to write in than just do this
    a: u32,
    u: u32,
    d: u32,
    s: u32,
    b: u32,
    t: u32,
    c: u32,
    l: u32,
    _state: [u32;624],
    _i: usize
}

#[allow(dead_code)]
impl MT19937 {
    fn _init() -> MT19937 {
        MT19937{
            w: 32,
            n: 624,
            m: 397,
            r: 31,
            a: 0x9908b0df,
            u: 11,
            d: 0xffffffff,
            s: 7,
            b: 0x9d2c5680,
            t: 15,
            c: 0xefc60000,
            l: 18,
            _state: [0;624],
            _i: 0
        }
    }

    pub fn from_seed(seed: u32) -> MT19937 {
        let f: Wrapping<u32> = Wrapping(1812433253);  // default value for seeing generator
        let mut _self = MT19937::_init();
        _self._state[0] = seed;

        let mut prev = Wrapping(seed);
        for (i, x) in enumerate(_self._state[1..].iter_mut()) {
            let temp_overflow: Wrapping<u32> = f * (prev ^ (prev >> (30))) + Wrapping((i+1) as u32);
            *x = temp_overflow.0;  // this is how we access the wrapped unit, apparently
            prev = Wrapping(*x);
        }
        _self._i = 624;
        _self
    }

    pub fn default() -> MT19937 {
        // Generates an MT19937 using the reference values, unseeded
        let seed: u32 = 5489;  // reference C value
        MT19937::from_seed(seed)
    }

    #[allow(non_snake_case)]
    fn twist(&mut self) {
        for i in 0..self.n as usize {  // redo this as an iterator using refcell?
            let mid_value = self._state[(i + self.m as usize) % self.n as usize];
            let base_upper = self._state[i] & (1 << 31);  // just take the top bit
            let base_lower = self._state[(i+1) % self.n as usize] & (u32::MAX >> 1);  // take all but the top
            let x = base_lower | base_upper;
            let mut xA = x >> 1;
            if x % 2 == 1 {
                xA = xA ^ (self.a);
            }
            self._state[i] = mid_value ^ xA;
        }
        self._i = 0;
    }

    fn temper(&mut self) {
        let mut x = self._state[self._i];
        x = x ^ ((x >> self.u) & self.d);
        x = x ^ ((x << self.s) & self.b);
        x = x ^ ((x << self.t) & self.c);
        x = x ^ (x >> self.l);
        self._state[self._i] = x;
        self._i += 1;
    }

    pub fn next(&mut self) -> u32 {
        if self._i as u32 == self.n {
            self.twist();
        }
        self.temper();
        self._state[self._i - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{BufRead, BufReader, Result},
    };
    use std::iter::zip;
     
    fn read_state_from_txt(filename: &str) -> Result<[u32;624]> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
    
        let mut numbers: [u32;624] = [0;624];
    
        for (entry, line) in zip(numbers.iter_mut(), reader.lines()) {
            let line = line?;
            if let Ok(number) = line.trim().parse::<u32>() {
                *entry = number;
            } else {
                eprintln!("Warning: Ignoring non-integer line: {}", line);
            }
        }
    
        Ok(numbers)
    }

    #[test]
    fn test_values() {
        let mut twister = MT19937::from_seed(5489);
        let ouyang_file = read_state_from_txt("ouyang_mt_100 outputs.txt").unwrap();
        for ouyang_num in ouyang_file.iter() {
            let my_num = twister.next();
            assert_eq!(my_num, *ouyang_num);
        }
    }

    // #[test]
    // fn generate() {
    //     let mut twister = MT19937::default();
    //     assert_eq!(twister.next(), 3382763572);
    // }
}
