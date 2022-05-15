# 최대공약수와 최소공배수

a, b = map(int, input().split())

def gcd(a, b):
    if b == 0:
        return a
    return gcd(b, a%b)

print(gcd(a, b))
print(a * b // gcd(a, b))