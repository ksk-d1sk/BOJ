# 설탕 배달

import sys

n = int(sys.stdin.readline())
count = 0

while True:
    if n == 0:
        break
    elif n % 5 == 0:
        n -= 5
        count += 1
    elif n % 3 == 0:
        n -= 3
        count += 1
    elif n > 5:
        n -= 5
        count += 1
    elif n > 3:
        n -= 3
        count += 1
    else:
        count = -1
        break

print(count)
