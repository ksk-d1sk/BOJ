# 다이얼

import sys

a = sys.stdin.readline().strip()
b = []
sum = 0

for i in a:
    b.append(i)

for i in range(len(b)):
    if b[i] == 'A' or b[i] == 'B' or b[i] == 'C':
        b[i] = 3
    elif b[i] == 'D' or b[i] == 'E' or b[i] == 'F':
        b[i] = 4
    elif b[i] == 'G' or b[i] == 'H' or b[i] == 'I':
        b[i] = 5
    elif b[i] == 'J' or b[i] == 'K' or b[i] == 'L':
        b[i] = 6
    elif b[i] == 'M' or b[i] == 'N' or b[i] == 'O':
        b[i] = 7
    elif b[i] == 'P' or b[i] == 'Q' or b[i] == 'R' or b[i] == 'S':
        b[i] = 8
    elif b[i] == 'T' or b[i] == 'U' or b[i] == 'V':
        b[i] = 9
    else:
        b[i] = 10

for i in range(len(b)):
    sum += b[i]

print(sum)
