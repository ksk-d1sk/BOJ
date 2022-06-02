# 스택

import sys

n = int(sys.stdin.readline())

com = str()
x = int()
arr = list()

def push():
    arr.append(x)

def pop():
    if len(arr) > 0:
        print(arr[len(arr) - 1])
        del arr[len(arr) - 1]
    else:
        print(-1)

def size():
    print(len(arr))

def empty():
    if len(arr) == 0:
        print(1)
    else:
        print(0)

def top():
    if len(arr) > 0:
        print(arr[len(arr) - 1])
    else:
        print(-1)

cmd = {
    "push": push,
    "pop": pop,
    "size": size,
    "empty": empty,
    "top": top
}

for i in range(n):
    com = sys.stdin.readline().strip()
    
    for i in com:
        if i == ' ':
            com, x = com.split()
            break

    cmd[com]()
