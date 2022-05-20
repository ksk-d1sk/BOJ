# 분산처리

import sys

t = int(sys.stdin.readline())

for _ in range(t):
    a, b = sys.stdin.readline().split()

    a = int(a[-1])
    b = int(b)
    c = a * a % 10

    arr = list()
    arr.append(10 if a==0 else a)
    arr.append(10 if c==0 else c)

    i = 1
    while True:
        if arr[0] == arr[i]:
            del arr[i]
            break
        c = c * a % 10
        arr.append(10 if c==0 else c)
        i += 1

    length = arr.__len__()-1 if b%arr.__len__()==0 else b%arr.__len__()-1
    print(arr[length])
