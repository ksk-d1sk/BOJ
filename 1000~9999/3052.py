# 나머지

import sys

a = []
sum = 0
count = [False for i in range(42)]
for i in range(10):
    a.append(int(sys.stdin.readline()) % 42)
    count[a[i]] = True

for i in count:
    if i:
        sum += 1

print(sum)
