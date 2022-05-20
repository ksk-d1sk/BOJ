# 문자열 반복

import sys

t = int(sys.stdin.readline())

for _ in range(t):
    r, s = sys.stdin.readline().split()
    r = int(r)

    for i in s:
        for j in range(r):
            print(i, end="")
    print()
