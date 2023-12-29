from random import Random
from struct import unpack
from MT19937.code.implement_mt19937 import MT19937  # thanks Anne Ouyang
import ctypes

cpp_twister = ctypes.CDLL("./mersenne-twister/mersenne-twister.so") # thanks Christian Stigen Larsen

# def test_internal_state():
#     rng = MT19937(5489)
#     py_rand = Random(5489)

#     for mt_state, rand_state in zip(rng.X, py_rand.getstate()[1]):
#         assert(mt_state == rand_state)

def test_generation():
    anne__rng = MT19937(5489)
    chris_rng = cpp_twister.seed(5489)
    anne_values = (anne__rng.temper() for _ in range(1000))
    chris_values = (ctypes.c_uint32((cpp_twister.rand_u32())).value for _ in range(1000))
    for anne_value, chris_value in zip(anne_values, chris_values):
        assert(anne_value == chris_value)




if __name__ == "__main__":
    rng = MT19937(5489)
    # py_rand = Random(5489)
    cpp_twister.seed(5489)
    print("-" * 60)
    print(f"Anne Ouyang's Python twister:\t\t\t{rng.temper()}")
    # first = py_rand.randbytes(4)
    # print(first)
    # print(unpack("<I", first)[0])
    cpp_value = ctypes.c_uint32((cpp_twister.rand_u32())).value
    print(
        f"Christian Stigen Larsen's C++ twister:\t\t{cpp_value}"
        )  # no thanks, Python, for casting the types