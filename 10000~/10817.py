# 세 수

import sys

n = list(map(int, sys.stdin.readline().split()))

print(sum(n) - max(n) - min(n))
