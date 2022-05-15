# 소수 찾기

import sys

n = int(sys.stdin.readline())
a = list(map(int, sys.stdin.readline().split()))
count = 0

for i in range(n):
    if a[i] == 2:
        count += 1
    elif a[i] > 1:
        chk = True
        for j in range(2, a[i]):
            if a[i] % j == 0:
                chk = False
                break
        if chk:
            count += 1

print(count)
