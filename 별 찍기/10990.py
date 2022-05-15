# 별 찍기 - 15

n = int(input())
a = 1

for i in range(1, n+1):
    print(' ' * (n - i), end='')
    if i == 1:
        print('*' * (2 * i - 1), end='')
    else:
        print('*', end='')
        print(' ' * a, end='')
        print('*', end='')
        a += 2
    print()
