# 인공지능 시계

import sys

a, b, c = map(int, sys.stdin.readline().split())
d = int(sys.stdin.readline())
c += d

if c >= 60:
    b += c // 60
    c = c % 60
    if b >= 60:
        a += b // 60
        b = b % 60
        if a >= 24:
            a = a % 24

print(a, b, c)
