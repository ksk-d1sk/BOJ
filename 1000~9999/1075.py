# 나누기

n = int(input())
f = int(input())

n = n // 100 * 100

print('%02d' % ((f - n % f) % f))
