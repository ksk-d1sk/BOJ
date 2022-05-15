# 평균은 넘겠지

import sys

c = int(sys.stdin.readline())
for i in range(c):
    n, *s = map(int, sys.stdin.readline().split())
    sum = 0
    count = 0
    
    for j in range(n):
        sum += s[j]
    
    avg = sum / n

    for j in range(n):
        if s[j] > avg:
            count += 1
    
    ratio = (count / n) * 100

    print(f"{ratio:.3f}%")
