# 하얀 칸

arr = [['.'] * 8] * 8
count = 0

for i in range(8):
  arr[i] = list(input())
  for j in range(8):
    if i % 2:
      if j % 2 and arr[i][j] == 'F':
        count += 1
    else:
      if j % 2 == 0 and arr[i][j] == 'F':
        count +=1

print(count)
