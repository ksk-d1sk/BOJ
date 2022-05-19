# 11653
# 소인수분해

n = int(input())

i = 2
while n > 1:
    if n % i == 0:
        print(i)
        n = n // i
        i = 2
    else:
        i += 1 if i % 2 == 0 else 2
