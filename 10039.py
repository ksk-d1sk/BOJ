# 평균 점수

import sys

sum = 0
for _ in range(5):
    b = int(sys.stdin.readline())
    if b < 40:
        sum += 40
    else:
        sum += b

print(sum // 5)
