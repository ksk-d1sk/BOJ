# 검증수

import sys

a = list(map(int, sys.stdin.readline().split()))

for i in range(len(a)):
    a[i] = a[i] ** 2

print(sum(a) % 10)
