from collections import defaultdict

grid = open("../test1.txt").read().strip().split("\n")
# grid = open("../prod1.txt").read().strip().split("\n")

grid = [list(x) for x in grid]

symbols = defaultdict(list)
n_rows = len(grid)
n_cols = len(grid[0])

for i in range(n_rows):
    for j in range(n_cols):
        if grid[i][j] != ".":
            symbols[grid[i][j]].append((i,j))

ANTINODES = set()
for k, coords in symbols.items():
    n_coords = len(coords)
    for i in range(n_coords):
        for j in range(i+1, n_coords):
            x, y = coords[i], coords[j]
            x1, y1 = x
            x2, y2 = y

            delta_x = x2 - x1
            delta_y = y2 - y1

            left_antinode = (x1 - delta_x, y1 - delta_y)
            right_antinode = (x2 + delta_x, y2 + delta_y)

            for node in [left_antinode, right_antinode]:
                a_x, a_y = node
                # Dont need to check for "#" since an antinode can fall at the same spot than an antenna
                if 0 <= a_x < n_rows and 0 <= a_y < n_cols :
                    grid[a_x][a_y] = "#"
                    ANTINODES.add(node)

import pprint
# for a_x, a_y in ANTINODES:
#     grid[a_x][a_y] = "#"

grid  = ["".join(x) for x in grid]
pprint.pprint(grid)
print(len(ANTINODES))
