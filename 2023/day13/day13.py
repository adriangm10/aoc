def distance(l: str, r: str) -> int:
    return sum(a != b for a, b in zip(l, r))


def reflection_row(block: list[str], distance_to_match: int) -> int:
    for idx in range(len(block)):
        if idx == 0:
            continue

        if (
            sum(distance(l, r) for l, r in zip(reversed(block[:idx]), block[idx:]))
            == distance_to_match
        ):
            return idx

    return 0


def score_block(block: str, distance_to_match: int) -> int:
    rows = block.split("\n")
    if row := reflection_row(rows, distance_to_match):
        return 100 * row

    if col := reflection_row(list(zip(*rows)), distance_to_match):
        return col

    raise ValueError("no reflection found!")


# input = open("inputs/13_input.txt", "r").read().split("\n\n")
# print(sum(score_block(block, 0) for block in input))
# print(sum(score_block(block, 1) for block in input))

ps = list(map(str.split, open('inputs/13_input.txt').read().split('\n\n')))

def f(p):
    for i in range(len(p)):
        if sum(c != d for l, m in zip(p[i - 1::-1], p[i:])
               for c, d in zip(l, m)) == s:
            return i
    else:
        return 0

for s in 0, 1:
    print(sum(100 * f(p) + f([*zip(*p)]) for p in ps))
