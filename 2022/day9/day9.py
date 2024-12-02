from numpy import sign
def solution(length):
    lines = open("/home/adrian/Desktop/advent_of_code/day9/input.txt", "r").readlines()
    knots = [[0, 0] for _ in range(length)]
    visited = set()
    visited.add(tuple(knots[-1]))
    for line in lines:
        dir, dist = line.split()
        for _ in range((int(dist))):
            match dir:
                case "R":
                    knots[0][0] += 1
                case "L":
                    knots[0][0] -= 1
                case "U":
                    knots[0][1] += 1
                case "D":
                    knots[0][1] -= 1
            for k in range(length-1):
                diff_x = knots[k][0] - knots[k+1][0]
                diff_y = knots[k][1] - knots[k+1][1]
                if abs(diff_x) > 1 or abs(diff_y) > 1:
                    knots[k+1][0] += sign(diff_x)
                    knots[k+1][1] += sign(diff_y)
            if not tuple(knots[-1]) in visited:
                visited.add(tuple(knots[-1]))
    return len(visited)

def main():
    print(solution(2))
    print(solution(10))

if __name__ == "__main__":
    main()
