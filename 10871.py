# X보다 작은 수

n, x = map(int, input().split())

a = input().split()

for i in range(n):
    if int(a[i]) < x:
        print(a[i])
