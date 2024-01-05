import ctypes

def test_load_mt():
    my_lib = ctypes.CDLL("./target/debug/libtempers.so")
    my_rand = my_lib.seeded_generator_ptr(5489)
    print(f"{ctypes.c_uint(my_rand).value:x}")
    generated_value = ctypes.c_uint32(my_lib.generate_random_u32(my_rand)).value
    assert  generated_value == 3499211612