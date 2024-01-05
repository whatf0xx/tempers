use crate::mt19937::MT19937;

#[no_mangle]
pub extern "C" fn seeded_generator_ptr(seed: u32) -> *mut MT19937 {
    let boxed_mt = Box::new(MT19937::from_seed(seed));
    let raw_ptr = Box::into_raw(boxed_mt);
    println!("MT allocated at {:?}", raw_ptr);
    raw_ptr
}

#[no_mangle]
pub extern "C" fn generate_random_u32(generator: &mut MT19937) -> u32 {
    println!("Received a pointer to {:p}", generator);
    generator.next().unwrap()
}
