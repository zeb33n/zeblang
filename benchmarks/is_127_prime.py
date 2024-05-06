import sys

maybe_prime = 127
i = 2
out = 1
while i != maybe_prime:
    if maybe_prime % i == 0:
        out = 0
    i += 1
sys.exit(out)
