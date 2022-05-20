# 숫자의 개수

import sys

a = int(sys.stdin.readline())
b = int(sys.stdin.readline())
c = int(sys.stdin.readline())

n = str(a * b * c)
count = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

for i in range(len(n)):
    for j in range(10):
        if j == int(n[i]):
            count[j] += 1

for i in range(10):
    print(count[i])
