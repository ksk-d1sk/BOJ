# 터렛

t = int(input())

a = [input().split() for i in range(t)]

for i in range(t):
   for j in range(6):
       a[i][j] = int(a[i][j])

for i in range(t):
    x1 = a[i][0]
    y1 = a[i][1]
    r1 = a[i][2]
    x2 = a[i][3]
    y2 = a[i][4]
    r2 = a[i][5]
    d = (x1 - x2) ** 2 + (y1 - y2) ** 2
    rPlus = (r1 + r2) ** 2
    rMinus = (r1 - r2) ** 2

    if x1 == x2 and y1 == y2 and r1 == r2:
        print(-1)
    elif rPlus < d or d < rMinus:
        print(0)
    elif rPlus == d or rMinus == d:
        print(1)
    elif rPlus > d > rMinus:
        print(2)

