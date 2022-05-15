# 피보나치 수 4

n = int(input())

arr = [0, 1]

if n == 0:
    del arr[1]
else:
    for i in range(2, n):
        arr.append(sum(arr))
        del arr[0]

print(sum(arr))
