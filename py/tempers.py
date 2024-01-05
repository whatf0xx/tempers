from random import Random
import ctypes

if __name__ == "__main__":
    r = Random()
    r.seed(5489)
    print(r.getrandbits(32))

    my_lib = ctypes.CDLL("./target/debug/libtempers.so")
    my_rand = my_lib.seeded_generator_ptr(5489)
    print(ctypes.c_uint32(my_lib.generate_random_u32(my_rand)).value)