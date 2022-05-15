# 수들의 합

s = int(input())
i = 1
cnt = 0

while s > 0:
    if s - i >= i + 1 or s - i == 0:
        s -= i
        cnt += 1
    i += 1

print(cnt)
