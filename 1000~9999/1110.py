# 더하기 사이클

import sys

n = int(sys.stdin.readline())
count = 0

if n < 10:
    n *= 10

d = n

while True:
    a = int(d) // 10
    b = int(d) % 10
    c = (a + b) % 10
    d = str(b) + str(c)
    count += 1
    if int(d) == n:
        print(count)
        break
