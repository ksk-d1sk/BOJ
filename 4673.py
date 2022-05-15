# 셀프 넘버

n = list(range(10001))

for i in range(10001):
    a = str(i)
    b = 0

    for j in a:
        b += int(j)

    if b + i < 10001:
        n[b+i] = False

for i in range(len(n)):
    if n[i]:
        print(n[i])
