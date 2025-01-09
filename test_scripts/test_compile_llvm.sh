#!/bin/bash
echo compiling $1...
target/debug/zeblang $1 --llvm
echo "running binary..."
lli ${1%.zb}.ll
echo "output: $?"

#cleanup
rm ${1%.zb}.ll
