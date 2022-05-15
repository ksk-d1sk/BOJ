# 2007년

import sys

wek = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"]

x, y = map(int, sys.stdin.readline().split())
d = 0
i = 0

while x-1 != i:
    i += 1
    if i==2:
        d += 28
    elif i==4 or i==6 or i==9 or i==11:
        d += 30
    else:
        d += 31

d += y
if d > 7:
    d = d % 7

print(wek[d])
