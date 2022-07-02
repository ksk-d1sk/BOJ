# 문자열

import sys

def first_end(str):
    return str[0] + str[len(str) - 1]

t = int(sys.stdin.readline())

for _ in range(t):
    print(first_end(sys.stdin.readline().strip()))
