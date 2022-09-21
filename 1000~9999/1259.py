# 팰린드롬수

import sys

while True:
    n = str(int(sys.stdin.readline()))

    if n == '0':
        break

    i = 0
    j = len(n) - 1
    check = True

    while i <= j:
        if n[i] != n[j]:
            check = False
            break
        i += 1
        j -= 1
    
    if check:
        print("yes")
    else:
        print("no")
