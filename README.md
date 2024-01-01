# tempers
#### a Rust library for cloning PRNGs from their outputs

## Usage
Clone the repository and use Cargo to set everything up.

## Documentation
A struct is defined for the Mersenne Twister PRNG and it implements the `Iterator` trait for easy calls to `next()`.

There are 3 main ways to initialise an `MT19937`:

- `MT19937::default()` creates the iterator based on the seed of 5489 (C standard).
- `MT19937::from_seed(seed: u32)` creates the iterator based on a provided `u32` seed.
- `MT19937::from_iter<T>(stream: &mut T) where T: Iterator<Item = u32>` reassembles the internal state of a `MT19937` based on the stream output, initialising the `MT19937` so that it produces the same output values as the input stream.

## Proof of Concept
See `test_from_unknown_state`.
