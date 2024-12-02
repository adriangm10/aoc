def part1():
    x = 1
    cycles = 0
    signal_strengths = []
    lines = open("/home/adrian/Desktop/advent_of_code/day10/input.txt", "r").readlines()
    for line in lines:
        instr = line.split()
        match instr[0]:
            case "addx":
                cycles += 1
                if (cycles + 20) % 40 == 0:
                    signal_strengths.append(cycles * x)
                cycles += 1
                if (cycles + 20) % 40 == 0:
                    signal_strengths.append(cycles * x)
                x += int(instr[1])
            case _:
                cycles += 1
                if (cycles + 20) % 40 == 0:
                    signal_strengths.append(cycles * x)

    
    return sum(signal_strengths)

def part2():
    x = 1
    cycle = 0
    crt = [["."] * 40 for _ in range(6)]
    lines = open("/home/adrian/Desktop/advent_of_code/day10/input.txt", "r").readlines()
    for line in lines:
        instr = line.split()
        match instr[0]:
            case "addx":
                if cycle % 40 in [x-1, x, x+1]:
                    crt[cycle // 40][cycle % 40] = "#"
                cycle += 1
                if cycle % 40 in [x-1, x, x+1]:
                    crt[cycle // 40][cycle  % 40] = "#"
                cycle += 1
                x += int(instr[1])
            case _:
                if cycle % 40 in [x-1, x, x+1]:
                    crt[cycle // 40][cycle % 40] = "#"
                cycle += 1
    return crt

def main():
    print(part1())
    crt = part2()
    for i in crt:
        for j in i:
            print(j, end="")
        print()

if __name__ == "__main__":
    main()
