# 크로아티아 알파벳

import sys

a = sys.stdin.readline().strip()
j = 0
length = 0

for i in range(len(a)):
    try:
        if i + j == len(a):
            break
        elif a[i+j] == 'c' and a[i+j+1] == '=':
            j += 1
        elif a[i+j] == 'c' and a[i+j+1] == '-':
            j += 1
        elif a[i+j] == 'd' and a[i+j+1] == 'z' and a[i+j+2] == '=':
            j += 2
        elif a[i+j] == 'd' and a[i+j+1] == '-':
            j += 1
        elif a[i+j] == 'l' and a[i+j+1] == 'j':
            j += 1
        elif a[i+j] == 'n' and a[i+j+1] == 'j':
            j += 1
        elif a[i+j] == 's' and a[i+j+1] == '=':
            j += 1
        elif a[i+j] == 'z' and a[i+j+1] == '=':
            j += 1
    except IndexError:
        length += 1
        continue
    length += 1

print(length)
