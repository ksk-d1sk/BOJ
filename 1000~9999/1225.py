# 이상한 곱셈

a, b = input().split()

def plus(num):
    result = 0
    for i in num:
        result += int(i)
    return result

total = 0

total += plus(a) * plus(b)

print(total)
