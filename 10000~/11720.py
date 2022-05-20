# 숫자의 합

import sys

n = int(sys.stdin.readline())
b = str(sys.stdin.readline())
sum = 0

for i in range(n):
    sum += int(b[i])

print(sum)
