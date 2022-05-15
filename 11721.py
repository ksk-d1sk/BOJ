# 열 개씩 끊어 출력하기

import sys

a = sys.stdin.readline()
i = 0

while True:
    try:
        for j in range(10):
            print(a[i+j], end='')
        print()
        i += 10
    except IndexError:
        break
