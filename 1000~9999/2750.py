# 수 정렬하기

import sys

n = int(sys.stdin.readline())
a = []
b = []

for _ in range(n):
    a.append(int(sys.stdin.readline()))

for i in range(len(a)):
    if i == 0:
        b.append(a[i])
    else:
        for j in range(len(b)):
            if a[i] <= b[j]:
                b.insert(j, a[i])
                break
            elif len(b) == j + 1:
                b.append(a[i])

for i in b:
    print(i)
