# 별 찍기 - 8

n = int(input())

for i in range(1, n+1):
    print('*' * i, end='')
    print(' ' * (n - i) * 2, end='')
    print('*' * i, end='')
    print()

for i in range(1, n):
    print('*' * (n - i), end='')
    print(' ' * (i * 2), end='')
    print('*' * (n - i), end='')
    print()
