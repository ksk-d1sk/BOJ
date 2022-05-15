# 공백 없는 A+B

inp = input()

arr = list()
for i in range(len(inp)):
    if inp[i] == '0':
        arr[arr.__len__()-1] = 10
    else:
        arr.append(int(inp[i]))

print(sum(arr))
