#!/bin/bash
echo compiling $1...
target/debug/zeblang $1 --llvm
lli ${1%.zb}.ll
echo "running binary..."
"./${1%.zb}"
echo "output: $?"

#cleanup
rm ${1%.zb}.asm
rm ${1%.zb}.o
rm ${1%.zb}
