# 음계

import sys

n = list(map(int, sys.stdin.readline().split()))
a = []
b = []

for i in range(1, 9):
    a.append(i)
    b.append(9-i)
if n == a:
    print('ascending')
elif n == b:
    print('descending')
else:
    print('mixed')
