# 소트인사이드

import sys

n = list(map(int, sys.stdin.readline().strip()))
a = [n[0]]

for i in range(1, len(n)):
    chk = True
    for j in range(i):
        if a[j] <= n[i]:
            a.insert(j, n[i])
            chk = False
            break
    if chk:
        a.append(n[i])

for i in a:
    print(i, end='')
