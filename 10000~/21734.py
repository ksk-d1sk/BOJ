# SMUPC의 등장

s = input()

for i in s:
    a = 0
    for j in str(ord(i)):
        a += int(j)
    print(i * a)
