# OX퀴즈

import sys

t = int(sys.stdin.readline())
a = [sys.stdin.readline().strip() for i in range(t)]

for i in range(t):
    count = 1
    sum = 0
    for j in a[i]:
        if j == 'O':
            sum += count
            count += 1
        elif j == 'X':
            count = 1
    print(sum)
