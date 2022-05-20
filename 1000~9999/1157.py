# 단어 공부

import sys

t = sys.stdin.readline().strip()
a = [0 for i in range(26)]
b = 0
cheak = False

for i in t:
    if ord(i) < 91:
        a[ord(i) - 65] += 1
    else:
        a[ord(i) - 97] += 1

for i in range(len(a)):
    if i == 0:
        b = i
    elif a[b] < a[i]:
        b = i
        cheak = False
    elif a[b] == a[i]:
        cheak = True

if cheak:
    print('?')
else:
    print(chr(b + 65))
