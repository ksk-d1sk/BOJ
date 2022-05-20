# 피보나치 함수

t = int(input())

for _ in range(t):
    n = int(input())

    arr = [0, 1]

    if n == 0:
        arr.reverse()
    else:
        for i in range(2, n+1):
            arr.append(sum(arr))
            del arr[0]

    print(arr[0], arr[1])
