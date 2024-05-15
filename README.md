# ZebLang!
Welcome to a very basic programming language WIP. Some of these features are only found on the development branch!

## Installation. 
first clone this repo 
``` git clone https://github.com/zeb33n/zeblang.git ```
then you can build from source using cargo 
``` cargo build --release ```
the executable will be found inside `target/release` it is called `zeblang` add this to your `PATH` in `.bashrc`
Nice you can now run zeblang

``` zeblang file.zb ``` to get the .asm file 
``` zeblang file.zb -j ``` to get the parse tree back as a .json

## Features!
### Assigning Variables
you can assign variables like so `x = 2`, you can also assign variables as a copy of another variable `y = x`

### Exit!
You can exit your program with the following keyword `exit`, you can also provide an integer exit code 
```
exit 1
```
exits with exit code 1
```
x = 2
exit x
```
exits with exit code 2
### Maths
Add stuff together!
```
x = 1 + 1
exit x
```
exits with 2

Add as much stuff as you like!
```
x = 1 + 2 + 3 + 4
exit x
```
exits with 10

Add variables !
```
x = 1
y = 2
exit x + y
```
exits with 3

subtraction `-`, multiplication `*`, division `/` and modulo `%` are also supported. 

operator precedance exists!
```
exit (1 + 2) * 3 + 1 * 1
```
exits with 10
### If Statements!
You can write if statements with the following syntax! 1 is true 0 is false. 
```
x = 1
if 1
    x = 2
fi
```
`x` will be equal to 2!
### While Loops!
You can also write while loops! 
```
x = 0
i = 10
while i
    x = x + 2
    i = i - 1
elihw
exit x
```
exits with 20!

## Examples!
check out this program that tells you whether the input is prime
```
maybe_prime = 127
i = 3
out = 1
while i != maybe_prime
  if maybe_prime % i == 0
    out = 0
  fi
  i = i + 1
elihw
exit out
```





