from itertools import zip_longest

def compare(left, right):
    for l, r in zip_longest(left, right):
        if l == None: return True
        if r == None: return False

        if isinstance(l, int) and isinstance(r, int):
            if l > r: return False
            if r > l: return True
        
        else:
            if isinstance(r, int): r = [r]
            if isinstance(l, int): l = [l]
            res = compare(l, r)
            if res in [True, False]:
                return res

def solve():
    packets = open("input.txt", "r").read().split("\n\n")
    packets = [i.splitlines() for i in packets]
    packets = [eval(j) for i in packets for j in i] + [[[2]], [[6]]]

    done = False
    while not done:
        done = True
        for i in range(len(packets) - 1):
            if not compare(packets[i], packets[i + 1]):
                packets[i], packets[i + 1] = packets[i + 1], packets[i]
                done = False
    
    print((packets.index([[2]]) + 1) * (packets.index([[6]]) + 1))

solve()