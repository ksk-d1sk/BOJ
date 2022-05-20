# 직각삼각형

import sys

while True:
    a = list(map(int, sys.stdin.readline().split()))
    a.sort()
    if a[0] and a[1] and a[2]:
        if a[0] ** 2 + a[1] ** 2 == a[2] ** 2:
            print('right')
        else:
            print('wrong')
    else:
        break
