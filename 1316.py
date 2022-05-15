# 그룹 단어 체커

import sys

n = int(sys.stdin.readline())
a = []
count = 0

for _ in range(n):
    a.append(sys.stdin.readline().strip())

for i in range(len(a)):
    cheak = True
    for j in range(len(a[i])):
        if len(a[i]) > 2 and cheak:
            for k in range(j+1, len(a[i])-1):
                if a[i][j] != a[i][k] and a[i][j] == a[i][k+1]:
                    cheak = False
        else:
            break
    if cheak:
        count += 1

print(count)
