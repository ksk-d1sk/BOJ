# ACM 호텔

import sys

t = int(sys.stdin.readline())

for _ in range(t):
    h, w, n = map(int, sys.stdin.readline().split())

    y = n % h
    x = n // h + 1
    if n % h == 0:
        y = h
        x = n // h
        
    if x < 10:
        print(str(y) + "0" + str(x))
    else:
        print(str(y) + str(x))
