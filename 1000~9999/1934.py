# 최소공배수

import sys

def gcd(a, b):
    if b == 0:
        return a
    return gcd(b, a%b)

t = int(sys.stdin.readline())

for i in range(t):
    a, b = map(int, sys.stdin.readline().split())
    print(a * b // gcd(a, b))
