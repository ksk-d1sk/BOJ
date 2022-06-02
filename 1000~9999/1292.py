# 쉽게 푸는 문제

a, b = map(int, input().split())

def BOJ_1292():
    sum = 0
    count = 0
    
    for i in range(1, b + 1):
        for j in range(i):
            count += 1
            if count >= a:
                sum += i
            if count == b:
                print(sum)
                return

BOJ_1292()
