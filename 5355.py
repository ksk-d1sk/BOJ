# 화성 수학

import sys

t = int(sys.stdin.readline())

for _ in range(t):
    a = list(sys.stdin.readline().split())
    a[0] = float(a[0])
    
    for i in range(1, len(a)):
        if a[i] == '@':
            a[0] *= 3
        elif a[i] == '%':
            a[0] += 5
        elif a[i] == '#':
            a[0] -= 7

    print(f'{a[0]:.2f}')
