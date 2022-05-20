# 상수

import sys

a = sys.stdin.readline().split()
b = [None, None]

for i in range(len(a)):
    for j in range(len(a[i])):
        if j == 0:
            b[i] = a[i][2]
        else:
            b[i] += a[i][2-j]

if b[0] > b[1]:
    print(b[0])
else:
    print(b[1])
