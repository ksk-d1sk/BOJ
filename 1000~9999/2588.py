# 곱셈

n1 = int(input())
n2 = int(input())

a = n2 // 100

b = (n2 // 10) - (a * 10)

c = (n2 // 1) - ((a * 100) + (b * 10))

print(n1 * c)
print(n1 * b)
print(n1 * a)
print(n1 * n2)
