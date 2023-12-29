use itertools::enumerate;
use core::u32;
use std::{
    num::Wrapping,
    iter::zip
};

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
        let f: Wrapping<u32> = Wrapping(1812433253);  // default value for seeding generator
        let mut _self = MT19937::_init();
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

    fn _from_complete_output(output: Vec<u32>) -> Result<MT19937, TempersError> {
        let mut _self = MT19937::_init();
        let _len = output.len();
        if _len != _self.n as usize { 
            return Err(TempersError::InputLengthError(_len));
        }

        let untempered_output: Vec<u32> = output.iter()
            .map(|&u| _self.untemper(u))
            .collect();

        for (twister_state, &value) in zip(_self._state.iter_mut(), untempered_output.iter()) {
            *twister_state = value;
        }
        return Ok(_self)
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

    #[inline]
    fn _subtemper1(&self, x: u32) -> u32 {
        x ^ ((x >> self.u) & self.d)
    }

    #[inline]
    fn _inv_subtemper1(&self, x: u32) -> u32 {
        let mut x = x;
        for _ in 0..3 { x = x ^ ((x >> self.u) & self.d); }  // can we be smarter with this?
        x
    }

    #[inline]
    fn _subtemper2(&self, x: u32) -> u32 {
        x ^ ((x << self.s) & self.b)
    }

    #[inline]
    fn _inv_subtemper2(&self, x: u32) -> u32 {
        let mut x = x;
        for _ in 0..7 { x = x ^ ((x << self.s) & self.b); }  // can we be smarter with this?
        x
    }

    #[inline]
    fn _subtemper3(&self, x: u32) -> u32 {
        x ^ ((x << self.t) & self.c)
    }

    #[inline]
    fn _inv_subtemper3(&self, x: u32) -> u32 {
        x ^ ((x << self.t) & self.c)
    }

    #[inline]
    fn _subtemper4(&self, x: u32) -> u32 {
        x ^ (x >> self.l)
    }

    #[inline]
    fn _inv_subtemper4(&self, x: u32) -> u32 {
        x ^ (x >> self.l)
    }

    fn _composite_temper(&self, p: u32) -> u32 {
        let mut x = p;
        x = self._subtemper1(x);
        x = self._subtemper2(x);
        x = self._subtemper3(x);
        self._subtemper4(x)
    }

    fn _composite_untemper(&self, p: u32) -> u32 {
        let mut x = p;
        x = self._inv_subtemper4(x);
        x = self._inv_subtemper3(x);
        x = self._inv_subtemper2(x);
        self._inv_subtemper1(x)
    }

    fn _temper_transform(&self, p: u32) -> u32 {
        let mut x = p;
        x = x ^ ((x >> self.u) & self.d);  // u = 11 (!)
        x = x ^ ((x << self.s) & self.b);  // s = 7 (!!)
        x = x ^ ((x << self.t) & self.c);  // t = 15 (!)
        x = x ^ (x >> self.l);  // l = 18
        x
    }
    
    fn temper(&mut self) -> u32 {
        self._i += 1;
        self._temper_transform(self._state[self._i-1])
    }

    fn _next(&mut self) -> u32 {
        if self._i as u32 == self.n {
            self.twist();
        }
        self.temper()
    }

    fn untemper(&self, mt_output: u32) -> u32 {
        self._composite_untemper(mt_output)
    }
}


impl Iterator for MT19937 {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        Some(self._next())
    }
}

#[derive(PartialEq, Debug)]
enum TempersError {
    InputLengthError(usize)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{BufRead, BufReader, Result},
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
    fn test_initial_state() {
        let twister = MT19937::from_seed(5489);
        let ouyang_file = read_u32_arr_from_txt("ouyang_mt_init_state.txt", 624).unwrap();
        for (my_state, ouyang_state) in zip(twister._state.iter(), ouyang_file.iter()) {
            assert_eq!(my_state, ouyang_state);
        }
    }

    #[test]
    fn test_state_temper() {  // actually this test is mostly useless, the state only mutates on twist()
        let mut twister = MT19937::from_seed(5489);
        let ouyang_file = read_u32_arr_from_txt("ouyang_mt_state_check.txt", 624).unwrap();
        twister._next();  // increment the state by one, are they still equal?
        for (my_state, ouyang_state) in zip(twister._state.iter(), ouyang_file.iter()) {
            assert_eq!(my_state, ouyang_state);
        }
    }
    
    #[test]
    fn test_values() {
        let mut twister = MT19937::from_seed(5489);
        let ouyang_file = read_u32_arr_from_txt("ouyang_mt_100_outputs.txt", 1000).unwrap();
        for (i, ouyang_num) in enumerate(ouyang_file.iter()) {
            let my_num = twister._next();
            assert_eq!(my_num, *ouyang_num,
                "\n(failed on the {}th number)", i);
        }
    }

    #[test]
    fn mt_as_iter() {
        let mut twister = MT19937::default();
        let ouyang_file = read_u32_arr_from_txt("ouyang_mt_100_outputs.txt", 1000).unwrap();
        for (mt_iterated, ouyang_num) in zip(twister, ouyang_file.iter()) {
            assert_eq!(mt_iterated, *ouyang_num);
        }
        
    }

    #[test]
    fn untemper_basic_state() {
        let mut twister = MT19937::from_seed(123456789);
        let internal_state = twister._state.clone();
        for (i, e) in enumerate(internal_state) {
            let curr = twister._next();
            assert_eq!(e, twister.untemper(curr), "\nfailed on the {}th value;\ntwister state dump:\n{:?}",
            i, internal_state);
        }
    }

    #[test]
    fn temper_composite() {
        let twister = MT19937::from_seed(123456789);
        assert_eq!(twister._temper_transform(123456789), twister._composite_temper(123456789));
    }

    #[test]
    fn inv_subtemper4_check() {
        let twister = MT19937::default();
        assert_eq!(123456789, twister._inv_subtemper4(twister._subtemper4(123456789)));
    }

    #[test]
    fn inv_subtemper3_check() {
        let twister = MT19937::default();
        assert_eq!(123456789, twister._inv_subtemper3(twister._subtemper3(123456789)));
        assert_eq!(987654321, twister._inv_subtemper3(twister._subtemper3(987654321)));
        assert_eq!(u32::MAX, twister._inv_subtemper3(twister._subtemper3(u32::MAX)));
        assert_eq!(0, twister._inv_subtemper3(twister._subtemper3(0)));
    }

    #[test]
    fn inv_subtemper2_check() {
        let twister = MT19937::default();
        assert_eq!(123456789, twister._inv_subtemper2(twister._subtemper2(123456789)));
        assert_eq!(987654321, twister._inv_subtemper2(twister._subtemper2(987654321)));
        assert_eq!(u32::MAX, twister._inv_subtemper2(twister._subtemper2(u32::MAX)));
        assert_eq!(0, twister._inv_subtemper2(twister._subtemper2(0)));
    }

    #[test]
    fn inv_subtemper1_check() {
        let twister = MT19937::default();
        assert_eq!(123456789, twister._inv_subtemper1(twister._subtemper1(123456789)));
        assert_eq!(987654321, twister._inv_subtemper1(twister._subtemper1(987654321)));
        assert_eq!(u32::MAX, twister._inv_subtemper1(twister._subtemper1(u32::MAX)));
        assert_eq!(0, twister._inv_subtemper1(twister._subtemper1(0)));
    }

    #[test]
    fn composite_untemper_check() {
        let twister = MT19937::default();
        assert_eq!(123456789, twister._composite_untemper(twister._composite_temper(123456789)));
        assert_eq!(987654321, twister._composite_untemper(twister._composite_temper(987654321)));
        assert_eq!(u32::MAX, twister._composite_untemper(twister._composite_temper(u32::MAX)));
        assert_eq!(0, twister._composite_untemper(twister._composite_temper(0)));
    }
    
    #[test]
    fn test_from_complete_state() {
        let mut twister = MT19937::default();
        let twister_clone = MT19937::default();

        let first_twist_output: Vec<u32> = (0..624).map(|_| twister._next()).collect();
        
        let twister_from_output = MT19937::_from_complete_output(first_twist_output);
        assert_eq!(twister_from_output, Ok(twister_clone));
    }

    // #[test]
    // fn dump_subtemper1() {
    //     let twister = MT19937::default();
    //     let mut u: u32 = 123456789;
    //     let mut trail: Vec<u32> = Vec::new();
    //     trail.push(u.clone());
    //     for _ in 0..20 {
    //         trail.push(twister._subtemper1(u.clone()));
    //         u = twister._subtemper1(u);
    //     }
    //     panic!("Trail: {:?}", trail);
    // }
}
