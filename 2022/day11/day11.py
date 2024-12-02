import re
from math import floor

with open("input.txt", "r") as f:
    info_monkeys = f.read().split("\n\n")

def solve(part):
    monkeys = {}
    for monkey in info_monkeys:
        l = monkey.split("\n")
        id = int(re.findall("\d+", l[0])[0])
        monkeys[id] = {}
        monkeys[id]["items"] = [int(i) for i in re.findall("\d+", l[1])]
        monkeys[id]["op"] = re.findall("(\+|\*) (\d+|old)", l[2])[0]
        monkeys[id]["test"] = int(re.findall("\d+", l[3])[0])
        monkeys[id]["true"] = int(re.findall("\d+", l[4])[0])
        monkeys[id]["false"] = int(re.findall("\d+", l[5])[0])
        monkeys[id]["insp"] = 0

    rounds = 10_000 if part == 2 else 20
    mod = 1
    for t in [monkey["test"] for monkey in monkeys.values()]:
        mod *= t

    for _ in range(rounds):
        for monkey in monkeys.values():
            while monkey["items"]:
                item_worry = monkey["items"].pop(0)
                monkey["insp"] += 1
                match monkey["op"][0]:
                    case "*":
                        mul = item_worry if monkey["op"][1] == "old" else int(monkey["op"][1])
                        item_worry *= mul
                    case "+":
                        sum = item_worry if monkey["op"][1] == "old" else int(monkey["op"][1])
                        item_worry += sum 

                if part == 1:
                    item_worry = floor(item_worry/3)
                else:
                    item_worry %= mod

                if item_worry % monkey["test"] == 0:
                    monkeys[monkey["true"]]["items"].append(item_worry)
                else:
                    monkeys[monkey["false"]]["items"].append(item_worry)
    
    inspections = sorted([monkey["insp"] for monkey in monkeys.values()])
    return inspections[-1] * inspections[-2]

print(solve(1))
print(solve(2))