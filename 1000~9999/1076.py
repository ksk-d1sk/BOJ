# 병렬

import sys

dic = {
    'black': '0',
    'brown': '1',
    'red': '2',
    'orange': '3',
    'yellow': '4',
    'green': '5',
    'blue': '6',
    'violet': '7',
    'grey': '8',
    'white': '9'
}

arr = list()

for _ in range(3):
    color = sys.stdin.readline().strip()
    arr.append(dic[color])

print(int(arr[0] + arr[1]) * (10 ** int(arr[2])))
