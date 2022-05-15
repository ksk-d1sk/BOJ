# 알파벳 찾기

import sys

t = sys.stdin.readline().strip()
a = [-1 for i in range(26)]

for i in range(len(t)):
    cheak = True
    if i == 0:
        a[ord(t[i]) - 97] = i
    else:
        for j in range(i):
            if t[i] == t[j]:
                cheak = False
                break
        if cheak:    
            a[ord(t[i]) - 97] = i

for i in range(len(a)):
    print(a[i], end=' ')
