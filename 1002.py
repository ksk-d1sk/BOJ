# 조규현과 백승환은 터렛에 근무하는 직원이다. 하지만 워낙 존재감이 없어서 인구수는 차지하지 않는다. 다음은 조규현과 백승환의 사진이다.



# 이석원은 조규현과 백승환에게 상대편 마린(류재명)의 위치를 계산하라는 명령을 내렸다. 조규현과 백승환은 각각 자신의 터렛 위치에서 현재 적까지의 거리를 계산했다.

# 조규현의 좌표 (x1, y1)와 백승환의 좌표 (x2, y2)가 주어지고, 조규현이 계산한 류재명과의 거리 r1과 백승환이 계산한 류재명과의 거리 r2가 주어졌을 때, 류재명이 있을 수 있는 좌표의 수를 출력하는 프로그램을 작성하시오.

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

