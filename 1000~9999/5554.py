# 심부름 가는 길

travel_time = list()

for _ in range(4):
    travel_time.append(int(input()))

print(sum(travel_time) // 60)
print(sum(travel_time) % 60)
