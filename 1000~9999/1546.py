# 평균

import sys

n = int(sys.stdin.readline())
a = list(map(int, sys.stdin.readline().split()))
m = 0
s = 0

for i in range(n):
    if a[i] > m:
        m = a[i]

for i in range(n):
    s += a[i] / m * 100

print(s / n)
