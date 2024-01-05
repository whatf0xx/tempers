use crate::mt19937::MT19937;

#[no_mangle]
pub extern "C" fn seeded_generator_ptr(seed: u32) -> *mut MT19937 {
    Box::into_raw(Box::new(MT19937::from_seed(seed)))
}

#[no_mangle]
pub extern "C" fn generate_random_u32(generator: &mut MT19937) -> u32 {
    generator.next().unwrap()
}
