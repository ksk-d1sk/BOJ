# 별 찍기 - 6

n = int(input())

for i in range(n):
    print(' ' * i, end='')
    print('*' * (2 * n - 2 * i - 1), end='')
    print()
