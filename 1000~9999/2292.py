# 벌집

n = int(input())
n -= 2
move = 1

if n == -1:
    print(move)
else:
    n = n // 6
    while True:
        n -= move
        move += 1
        if n < 0:
            break
    print(move)
