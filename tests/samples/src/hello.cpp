#include <iostream>

/*
Compile arm:
    /usr/bin/clang++ -arch arm64 -Os tests/samples/hello.cpp -o tests/samples/hello_arm64
Compile x86:
    /usr/bin/clang++ -arch x86_64 -Os tests/samples/hello.cpp -o tests/samples/hello_x86_64
Compile x86 without Chained Fixups:
    usr/bin/clang++ -arch x86_64 -Os tests/samples/src/hello.cpp -Wl,-no_fixup_chains -o tests/samples/hello_x86_64_nochain

Combine both into a Fat/Universal binary: lipo -create tests/samples/hello_arm64 tests/samples/hello_x86_64 -output tests/samples/hello_fat

Verify with: lipo -info tests/samples/hello_fat
Expected Output: `Architectures in the fat file: tests/samples/hello_fat are: x86_64 arm64`


*/

int main(void) {
    std::cout << "Hello world!" << std::endl;
    return 0;
}