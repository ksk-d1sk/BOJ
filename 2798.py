# 블랙잭

import sys

n, m = map(int, sys.stdin.readline().split())
a = list(map(int, sys.stdin.readline().split()))
b = 0

for i in range(n):
    for j in range(i+1, n):
        for k in range(j+1, n):
            if b < a[i] + a[j] + a[k] <= m:
                b = a[i] + a[j] + a[k]

print(b)
