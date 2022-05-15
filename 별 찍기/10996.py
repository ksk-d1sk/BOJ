# 별 찍기 - 21

n = int(input())

for i in range(n*2):
    for j in range(n):
        if i % 2 == 1:
            j += 1
        if j % 2 == 0:
            print('*', end='')
        else:
            print(' ', end='')
    print()
