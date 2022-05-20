# 최댓값

import sys

a = []

for i in range(9):
    a.append(int(sys.stdin.readline()))

m1 = a[0]
m2 = int

for i in range(9):
    if a[i] >= m1:
        m1 = a[i]
        m2 = i+1

print(m1)
print(m2)
