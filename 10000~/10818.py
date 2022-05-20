# 최소, 최대

import sys

n = int(sys.stdin.readline())
k = list(map(int, sys.stdin.readline().split()))

for i in range(n):
    if i == 0:
        a = k[i]
        b = k[i]

    if k[i] > a:
        a = k[i]
    elif k[i] < b:
        b = k[i]

print(b, a)
