from random import Random
import ctypes

if __name__ == "__main__":
    my_lib = ctypes.CDLL("./target/debug/libtempers.so")
    my_rand = my_lib.seeded_generator_ptr(5489)
    # print(f"{ctypes.c_uint(my_rand).value:x}")
    # generated_value = ctypes.c_uint32(my_lib.generate_random_u32(my_rand)).value
    # print(generated_value)

    py_rand = Random()
    # print(py_rand.getstate())
    rust_state = [ctypes.c_uint32(my_lib.dump_generator_state(my_rand, i)).value for i in range(624)]
    py_rand.setstate(
        (3, tuple(rust_state + [624]), None)
    )
    # print(py_rand.getstate())
    print(py_rand.getrandbits(32), ctypes.c_uint32(my_lib.generate_random_u32(my_rand)).value)
    print(py_rand.getrandbits(32), ctypes.c_uint32(my_lib.generate_random_u32(my_rand)).value)
    print(py_rand.getrandbits(32), ctypes.c_uint32(my_lib.generate_random_u32(my_rand)).value)
    # print(rust_state[:5], rust_state[n-5:])