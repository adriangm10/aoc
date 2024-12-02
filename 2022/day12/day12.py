#!/bin/python
map = open("input.txt", "r").readlines()

def solve(map, part):
    nodes = {}
    width = len(map[0][:-1])
    height = len(map)
    if part == 2:
        start = []

    # cargar grafo poniendo nodos adyacentes solo a aquellos a los que me puedo mover
    for i in range(height):
        for j in range(width):
            neighbours = []
            c = map[i][j]
            if c == "E":
                end = (i, j)
                c = "z"
            else:
                if part == 1:
                    if c == "S":
                        start = (i, j)
                        c = "a"
                else:
                    if c == "a":
                        start.append((i, j))
                    elif c == "S":
                        start.append((i, j))
                        c = "a"
            if i > 0:
                comp = map[i - 1][j]
                if comp == "E":
                    comp = "z"
                if ord(c) - ord(comp) >= -1:
                    neighbours.append((i - 1, j))
            if i < height - 1:
                comp = map[i + 1][j]
                if comp == "E":
                    comp = "z"
                if ord(c) - ord(comp) >= -1:
                    neighbours.append((i + 1, j))
            if j > 0:
                comp = map[i][j - 1]
                if comp == "E":
                    comp = "z"
                if ord(c) - ord(comp) >= -1:
                    neighbours.append((i, j - 1))
            if j < width - 1:
                comp = map[i][j + 1]
                if comp == "E":
                    comp = "z"
                if ord(c) - ord(comp) >= -1:
                    neighbours.append((i, j + 1))
            nodes[(i, j)] = neighbours

    # recorrer grafo en anchura para econtrar el camino mas corto
    if part == 1:
        visited = {}
        queue = []
        visited[start] = 0
        queue.append(start)
        while queue:
            node = queue.pop(0)
            if node == end:
                return visited[node]
            for i in nodes[node]:
                if i not in visited:
                    queue.append(i)
                    visited[i] = visited[node] + 1
    else:
        min = 1000000000
        for i in start:
            visited = {}
            queue = []
            visited[i] = 0
            queue.append(i)
            while queue:
                node = queue.pop(0)
                if node == end:
                    if visited[node] < min:
                        min = visited[node]
                for i in nodes[node]:
                    if i not in visited:
                        queue.append(i)
                        visited[i] = visited[node] + 1
        return min

    return -1

print(solve(map, 1))
print(solve(map, 2))
