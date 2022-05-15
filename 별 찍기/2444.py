# 별 찍기 - 7

n = int(input())

for i in range(n):
    print(' ' * (n - i - 1), end='')
    print('*' * (2 * i + 1), end='')
    print()

for i in range(1, n):
    print(' ' * i, end='')
    print('*' * (2 * n - 2 * i - 1), end='')
    print()
