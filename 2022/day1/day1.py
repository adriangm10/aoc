calorias = open("/home/adrian/Desktop/advent_of_code/day1/input.txt", "r").readlines()
elfos = []
suma = 0

for cal in calorias:
    if cal == "\n":
        elfos.append(suma)
        suma = 0
    else:
        suma += int(cal)

elfos = sorted(elfos)
print(f"suma top3 = {sum([elfos[-1], elfos[-2], elfos[-3]])}")

