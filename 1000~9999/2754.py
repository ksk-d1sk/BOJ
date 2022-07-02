# 학점계산

grades = input()

dic_a = { 'A' : 4, 'B' : 3, 'C' : 2, 'D' : 1, 'F' : 0.0 }

dic_b = { '+' : 0.3, '0' : 0.0, '-' : -0.3 }

score = dic_a[grades[0]]

if score != 0:
    score += dic_b[grades[1]]

print(score)
