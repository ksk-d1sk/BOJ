# 알파벳 대소문자로 된 단어가 주어지면, 이 단어에서 가장 많이 사용된 알파벳이 무엇인지 알아내는 프로그램을 작성하시오. 단, 대문자와 소문자를 구분하지 않는다.

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
