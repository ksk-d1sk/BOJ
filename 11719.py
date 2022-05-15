# 그대로 출력하기 2

while True:
    try:
        a = input()
        print(a)
    except EOFError:
        break
