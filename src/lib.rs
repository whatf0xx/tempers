// #![warn(missing_docs)]

mod mt19937;
use mt19937::MT19937;
mod mt19937_ffi;
use std::{
    iter::zip,
    collections::VecDeque
};

#[allow(dead_code)]
impl mt19937::MT19937 {
    fn _from_complete_output(output: &[u32]) -> Result<MT19937, TempersError> {
        let mut _self = MT19937::blank();
        let _len = output.len();
        if _len != _self.state_length() as usize { 
            return Err(TempersError::InputLengthError(_len));
        }

        let untempered_output: Vec<u32> = output.iter()
            .map(|&u| _self.untemper(u))
            .collect();

        for (twister_state, &value) in zip(_self.internal_state().iter_mut(), untempered_output.iter()) {
            *twister_state = value;
        }
        
        // N.B. that this reassembles the state, but as there is no twist involved this is ready
        // to repeat the values already seen, not to produce a matching stream of new values
        return Ok(_self)
    }

    pub fn from_iter<T>(stream: &mut T) -> Result<MT19937, TempersError>
    where T: Iterator<Item = u32> {  
        let mut stream_vals: VecDeque<u32> = VecDeque::new();
        for _ in 0..624 {
            stream_vals.push_back(stream.next().ok_or(TempersError::IncompleteIterator)?);
        }

        for _ in 0..624 {
            // get the Deque as a slice to pass into the function
            stream_vals.make_contiguous();
            let trial_vals = stream_vals.as_slices().0;

            let mut attempted_construction = MT19937::_from_complete_output(&trial_vals)?;
            attempted_construction.twist();

            if attempted_construction.test_next_equal_to_iter(stream)? {
                return Ok(attempted_construction);
            }

            // Update the Deque with the next value from the stream and try again
            stream_vals.push_back(stream.next().ok_or(TempersError::IncompleteIterator)?);
            stream_vals.pop_front();
        }
        
        // If you don't find the value after a complete twist cycle, it doesn't match
        Err(TempersError::UnmatcheableIterator)
    }

    pub fn test_next_equal_to_iter<T>(self: &mut MT19937, a: &mut T) -> Result<bool, TempersError>
    where 
        T: Iterator<Item = u32>
    {
        let a_next = a.next().ok_or(TempersError::IncompleteIterator)?;
        let mt_next = self.next().ok_or(TempersError::UnknownError)?;  // different error because my implementation has failed, clearly
        if a_next == mt_next {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    #[inline]
    fn _inv_subtemper1(&self, x: u32) -> u32 {
        let mut x = x;
        for _ in 0..3 { x = x ^ ((x >> self.u()) & self.d()); }  // can we be smarter with this?
        x
    }

    #[inline]
    fn _inv_subtemper2(&self, x: u32) -> u32 {
        let mut x = x;
        for _ in 0..7 { x = x ^ ((x << self.s()) & self.b()); }  // can we be smarter with this?
        x
    }

    #[inline]
    fn _inv_subtemper3(&self, x: u32) -> u32 {
        x ^ ((x << self.t()) & self.c())
    }

    #[inline]
    fn _inv_subtemper4(&self, x: u32) -> u32 {
        x ^ (x >> self.l())
    }

    fn untemper(&self, p: u32) -> u32 {
        let mut x = p;
        x = self._inv_subtemper4(x);
        x = self._inv_subtemper3(x);
        x = self._inv_subtemper2(x);
        self._inv_subtemper1(x)
    }
}

#[derive(PartialEq, Debug)]
pub enum TempersError {
    InputLengthError(usize),
    IncompleteIterator,
    UnmatcheableIterator,
    UnknownError
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::enumerate;

    #[test]
    fn untemper_values() {
        let twister = MT19937::default();
        assert_eq!(123456789, twister.untemper(twister.temper_transform(123456789)));
        assert_eq!(987654321, twister.untemper(twister.temper_transform(987654321)));
        assert_eq!(u32::MAX, twister.untemper(twister.temper_transform(u32::MAX)));
        assert_eq!(0, twister.untemper(twister.temper_transform(0)));
    }
    
    #[test]
    fn untemper_basic_state() {
        let mut twister = MT19937::from_seed(123456789);
        let internal_state = twister.internal_state().to_owned();
        for (i, e) in enumerate(&internal_state) {
            let curr = twister.next().unwrap();
            assert_eq!(*e, twister.untemper(curr), "\nfailed on the {}th value;\ntwister state dump:\n{:?}",
            i, internal_state);
        }
    }

    #[test]
    fn reassemble_from_complete_state() {
        let mut twister = MT19937::default();
        let twister_clone = MT19937::default();

        let first_twist_output: Vec<u32> = (0..624).map(|_| twister.next().unwrap()).collect();
        
        let twister_from_output = MT19937::_from_complete_output(&first_twist_output).unwrap();
        assert_eq!(twister_from_output, twister_clone);
        assert_eq!(twister_from_output.state_index(), twister_clone.state_index());
    }

    #[test]
    fn reassemble_from_unknown_state() {
        let mut target_twister = MT19937::from_seed(987654321);
        for _ in 0..1000 {
            target_twister.next();  // get us to a random point between twists
        }
        
        let mut matched_twister = MT19937::from_iter(&mut target_twister).unwrap();

        for _ in 0..1000 {
            assert_eq!(target_twister.next(), matched_twister.next());
        }
    }
}