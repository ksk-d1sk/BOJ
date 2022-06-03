# 분해합

def deSum(x):
    a = x
    b = str(x)
    for i in range(len(b)):
        a += int(b[i])
    return a

n = input()

start_num = 0

if len(n) > 2:
    num = '8'
    for _ in range(len(n) - 2):
        num += '9'
    start_num = int(num)

n = int(n)
a = 0

for i in range(start_num, n):
    b = deSum(i)
    if b == n:
        a = i
        break

print(a)
