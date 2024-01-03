use itertools::enumerate;
use core::u32;
use std::num::Wrapping;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
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
    /// Create an MT19937 with its parameters set to the reference values and its internal state uninitialised.
    /// 
    /// This should hence not be used for generating random numbers, but can serve as a precursor to a useful generator
    /// when its internal state is populated via a seed or by copying the state of another generator.
    pub fn blank() -> MT19937 {
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
        let f: Wrapping<u32> = Wrapping(1812433253);  // default value for seeding generator
        let mut _self = MT19937::blank();
        _self._state[0] = seed;

        let mut prev = Wrapping(seed);
        for (i, x) in enumerate(_self._state[1..].iter_mut()) {
            let temp_overflow: Wrapping<u32> = f * (prev ^ (prev >> (30))) + Wrapping((i+1) as u32);
            *x = temp_overflow.0;  // this is how we access the wrapped unit, apparently
            prev = Wrapping(*x);
        }
        _self.twist();
        _self
    }
    
    pub fn default() -> MT19937 {
        // Generates an MT19937 using the reference values, unseeded
        let seed: u32 = 5489;  // reference C value
        MT19937::from_seed(seed)
    }

    #[allow(non_snake_case)]
    pub fn twist(&mut self) {
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

    pub fn state_index(&self) -> usize {
        self._i
    }

    pub fn internal_state(&mut self) -> &mut [u32] {
        self._state.as_mut()
    }
    
    pub fn state_length(&self) -> usize {
        return self.n as usize
    }
    
    pub fn a(&self) -> u32 {
        self.a
    }

    pub fn b(&self) -> u32 {
        self.b
    }

    pub fn c(&self) -> u32 {
        self.c
    }

    pub fn d(&self) -> u32 {
        self.d
    }

    pub fn u(&self) -> u32 {
        self.u
    }

    pub fn s(&self) -> u32 {
        self.s
    }

    pub fn t(&self) -> u32 {
        self.t
    }

    pub fn l(&self) -> u32 {
        self.l
    }

    #[inline]
    fn _subtemper1(&self, x: u32) -> u32 {
        x ^ ((x >> self.u) & self.d)
    }

    #[inline]
    fn _subtemper2(&self, x: u32) -> u32 {
        x ^ ((x << self.s) & self.b)
    }

    #[inline]
    fn _subtemper3(&self, x: u32) -> u32 {
        x ^ ((x << self.t) & self.c)
    }

    #[inline]
    fn _subtemper4(&self, x: u32) -> u32 {
        x ^ (x >> self.l)
    }

    pub fn temper_transform(&self, p: u32) -> u32 {
        let mut x = p;
        x = self._subtemper1(x);
        x = self._subtemper2(x);
        x = self._subtemper3(x);
        self._subtemper4(x)
    }
    
    fn temper(&mut self) -> u32 {
        self._i += 1;
        self.temper_transform(self._state[self._i-1])  // kept in-bounds as twist() is always called in time
    }
}


impl Iterator for MT19937 {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self._i as u32 == self.n {
            self.twist();
        }
        Some(self.temper())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{BufRead, BufReader, Result},
        iter::zip,
    };
     
    fn read_u32_arr_from_txt(filename: &str, n: u32) -> Result<Vec<u32>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
    
        let mut numbers: Vec<u32> = Vec::new();
    
        for (_, line) in zip(0..n, reader.lines()) {
            let line = line?;
            if let Ok(number) = line.trim().parse::<u32>() {
                numbers.push(number)
            } else {
                panic!("Got an uninterpretable line: {}", line);
            }
        }
    
        Ok(numbers)
    }

    #[test]
    fn blank_state() {
        let twister = MT19937::from_seed(5489);
        let ouyang_file = read_u32_arr_from_txt("test-txt/ouyang_mt_init_state.txt", 624).unwrap();
        for (my_state, ouyang_state) in zip(twister._state.iter(), ouyang_file.iter()) {
            assert_eq!(my_state, ouyang_state);
        }
    }
    
    #[test]
    fn output_values() {
        let mut twister = MT19937::from_seed(5489);
        let ouyang_file = read_u32_arr_from_txt("test-txt/ouyang_mt_1000_outputs.txt", 1000).unwrap();
        for (i, ouyang_num) in enumerate(ouyang_file.iter()) {
            let my_num = twister.next().unwrap();
            assert_eq!(my_num, *ouyang_num,
                "\n(failed on the {}th number)", i);
        }
    }
}
