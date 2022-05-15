# 별 찍기 - 5

n = int(input())

for i in range(n):
    print(' ' * (n - i - 1), end='')
    print('*' * (2 * i + 1), end='')
    print()
