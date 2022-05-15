# 오븐 시계

import sys

a, b = map(int, sys.stdin.readline().split())
c = int(sys.stdin.readline())
b += c

if b >= 60:
    a += b // 60
    b = b % 60
    if a >= 24:
        a -= 24

print(a, b)
