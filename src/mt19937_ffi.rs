mod mt19937;

#[no_mangle]
pub extern "C" fn allocate_seeded_generator(seed: u32) -> MT19937 {
    MT19937::from_seed(seed)
}

#[no_mangle]
pub extern "C" fn generate_random_u32(generator: &mut MT19937) -> u32 {
    MT19937::next()
}