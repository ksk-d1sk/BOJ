# 과자

import sys

k, n, m = map(int, sys.stdin.readline().split())

a = k * n - m

if a <= 0:
    print(0)
else:
    print(a)
