# 명령 프롬프트

import sys

n = int(sys.stdin.readline())
text = list()
result = ""

for _ in range(n):
    text.append(sys.stdin.readline())

for i in range(len(text[0])):
    char = text[0][i]
    for j in text:
        if char != j[i]:
            char = '?'
            break
    result +=  char

print(result)
