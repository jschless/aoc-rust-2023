s = open("./data/inputs/02.txt").read()

color_map = {"red": 12, "blue": 14, "green": 13}
acc = 0
for i, g in enumerate(s.split("\n")):
    if len(g) == 0:
        break
    colon = g.find(":")
    tokens = g[colon + 2 :].replace(",", "").replace(";", "").split(" ")
    good_game = True
    # print(tokens)
    for j in range(0, len(tokens), 2):
        count, color = int(tokens[j]), tokens[j + 1]
        if count > color_map[color]:
            good_game = False
            break

    if good_game:
        print(i + 1, tokens)
        acc += i + 1

print(acc)
