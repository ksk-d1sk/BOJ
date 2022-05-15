# 달팽이는 올라가고 싶다

import sys

a, b, v = map(int, sys.stdin.readline().split())
a -= b
v -= b

if v % a != 0:
    print(v // a + 1)
else:
    print(v // a)
