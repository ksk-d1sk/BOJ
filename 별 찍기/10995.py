# 별 찍기 - 20

n = int(input())

for i in range(n):
    for j in range(n):
        if i % 2 == 0:
            print('*', end='')
            print(' ', end='')
        else:
            print(' ', end='')
            print('*', end='')
    print()
