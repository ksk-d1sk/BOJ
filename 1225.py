# 이상한 곱셈

a, b = input().split()

total = 0

for i in a:
    for j in b:
        total += int(i) * int(j)

print(total)
