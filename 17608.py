# 17608
# 막대기

import sys

n = int(sys.stdin.readline())

arr = list()

for i in range(n):
    arr.append(int(sys.stdin.readline()))

arr.reverse()

count = 0
best_height = 0
for i in range(len(arr)):
    if arr[i] > best_height:
        count += 1
        best_height = arr[i]

print(count)
