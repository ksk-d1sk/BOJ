# 14645
# 와이버스 부릉부릉

import sys

n, count = map(int, sys.stdin.readline().split())

name = '비와이'

for _ in range(n):
    a, b = map(int, sys.stdin.readline().split())
    count += a - b

print(name)
