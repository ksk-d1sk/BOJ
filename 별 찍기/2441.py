# 별 찍기 - 4

import sys

a = int(sys.stdin.readline())

for i in range(a):
    for j in range(i):
        print(' ', end="")
        
    for j in range(a-i):
        print('*', end="")
    print()
