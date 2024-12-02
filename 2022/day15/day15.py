import re

input = open("input.txt", "r").readlines();

def manhattan(a, b):
    sum = 0
    for i in range(len(a)):
        sum += abs(a[i] - b[i])
    return sum

def solve():
    sens_beac = {}
    for line in input:
        (a, b, c, d) = [int(i) for i in re.findall(r"(\d+)", line)]
        sens_beac[(a, b)] = (c, d)
    line = []
    for sens, beac in sens_beac.items():
        dist = manhattan(sens, beac)
        
    
print(len("124480007823454171604037915039820840260122786817000"))
print(124480007823454171604037915039820840260122786817 * 0.2**58 * 0.8**(200-48))
