# 3003
# 킹, 퀸, 룩, 비숍, 나이트, 폰

my_piece = list(map(int, input().split()))
chess_piece = [1, 1, 2, 2, 2, 8]

for i in range(6):
    print(chess_piece[i] - my_piece[i], end=" ")
