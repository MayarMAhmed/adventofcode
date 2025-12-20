import time
import os
from collections import deque

start_time = time.time()

script_dir = os.path.dirname(os.path.abspath(__file__))
file = os.path.join(script_dir, "input_d4_25.txt")
# file = os.path.join(script_dir, "tmp.txt")

with open(file, "r", encoding="utf8") as f:
    grid = [list(line.strip()) for line in f if line.strip()]

H = len(grid)
W = len(grid[0])

directions = [(-1, -1), (0, -1), (1, -1), (-1,  0),(1,  0),(-1,  1), (0,  1), (1,  1)]

def valid_direction(x: int, y: int) -> bool:
    if 0 <= x < W and 0 <= y < H:
        return True
    
def neighbors(x: int, y: int):
    for dx, dy in directions:
        new_x, new_y = x + dx, y + dy
        if valid_direction(new_x, new_y):
            yield new_x, new_y #finally try to yield instead of return

# Build neighbor counts
adj = [[0] * W for _ in range(H)]
for y in range(H):
    for x in range(W):
        if grid[y][x] != "@":
            continue
        c = 0
        for new_x, new_y in neighbors(x, y):
            if grid[new_y][new_x] == "@":
                c += 1
        adj[y][x] = c


q = deque()
queued = set()

for y in range(H):
    for x in range(W):
        if grid[y][x] == "@" and adj[y][x] < 4:
            q.append((x, y))
            queued.add((x, y))

part1 = len(q) 
part2 = 0

while q:
    x, y = q.popleft()
    queued.remove((x, y))

    if grid[y][x] != "@":
        continue
    if adj[y][x] >= 4:
        continue

    grid[y][x] = "."
    part2 += 1

    # continue removing (x,y)'s neighbors if count drops below 4
    for new_x, new_y in neighbors(x, y):
        if grid[new_y][new_x] == "@":
            adj[new_y][new_x] -= 1
            if adj[new_y][new_x] < 4 and (new_x, new_y) not in queued:
                q.append((new_x, new_y))
                queued.add((new_x, new_y))

print("Part 1:", part1)
print("Part 2:", part2)
print("Time taken:", time.time() - start_time)
