# import target.release.libpylib as libpylib
from target.release.libpylib import StructTest

class PyStructTest(StructTest):
    pass

struct_test = PyStructTest
struct_test.name = "Test"
print( struct_test )
print( struct_test.name )
print( struct_test.say_hello() )