# 별 찍기 - 23

n = int(input())
a = 1

if n-1 != 1:
    for _ in range(n-2):
        a += 2

print('*' * n, end='')
print(' ' * a, end='')
print('*' * n)

for i in range(1, n):
    a -= 2
    print(' ' * i, end='')
    print('*', end='')
    print(' ' * (n - 2), end='')
    print('*', end='')
    if i != n - 1:
        print(' ' * a, end='')
        print('*', end='')
    print(' ' * (n - 2), end='')
    print('*')

for i in range(1, n):
    a += 2
    if i == n-1:
        break
    print(' ' * (n - i - 1), end='')
    print('*', end='')
    print(' ' * (n - 2), end='')
    print('*', end='')
    print(' ' * a, end='')
    print('*', end='')
    print(' ' * (n - 2), end='')
    print('*')

print('*' * n, end='')
print(' ' * a, end='')
print('*' * n)
