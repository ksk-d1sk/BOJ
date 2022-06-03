# 수 정렬하기 3

import sys

n = int(sys.stdin.readline())
arr = [0] * 10000

for _ in range(n):
    arr[int(sys.stdin.readline()) - 1] += 1

for i in range(len(arr)):
    while arr[i]:
        print(i + 1)
        arr[i] -= 1
