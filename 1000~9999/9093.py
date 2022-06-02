# 단어 뒤집기

import sys

t = int(sys.stdin.readline())

for _ in range(t):
    words = list(sys.stdin.readline().split())
    for i in words:
        j = len(i) - 1
        while j >= 0:
            print(i[j], end="")
            j -= 1
        print(end=" ")
    print()
