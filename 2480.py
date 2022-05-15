# 주사위 세개

dice_list = list(map(int, input().split()))

dice_eye = None
count = 0

dice_list.sort()

for i in range(len(dice_list)):
    if i == 0:
        continue

    if dice_list[i] == dice_list[i-1]:
        count += 1
        dice_eye = dice_list[i]
    elif count == 0:
        dice_eye = dice_list[i]
        
rul = {
    0: dice_eye * 100,
    1: dice_eye * 100 + 1000,
    2: dice_eye * 1000 + 10000
}

print(rul[count])
    