# 도비의 난독증 테스트

import sys

chk = True

while True:
    chk = int(sys.stdin.readline())
    if not chk:
        break

    dic = dict()
    words = list()
    lower_words = list()

    for i in range(chk):
        word = sys.stdin.readline().strip()
        words.append(word)
        # 아 lower함수 그런거 누가씀ㅋ
        lower_word = str()
        for j in range(len(word)):
            j = word[j]
            if 65 <= ord(j) <= 90:
                j = chr(ord(j) + 32)
            lower_word += j

        lower_words.append(lower_word)
        dic[lower_word] = i

    lower_words.sort()

    print(words[dic[lower_words[0]]])
