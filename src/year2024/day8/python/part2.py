from collections import defaultdict

# grid = open("../test2.txt").read().strip().split("\n")
grid = open("../prod2.txt").read().strip().split("\n")

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
            # So, instead of just one sum, need multiple sums that follow the same line
            x, y = coords[i], coords[j]
            x1, y1 = x
            x2, y2 = y

            delta_x = x2 - x1
            delta_y = y2 - y1

            total_antinodes = []
            total_antinodes.extend([(x1 - delta_x*x, y1 - delta_y * x) for x in range(max(n_rows, n_cols))])
            total_antinodes.extend([(x2 + delta_x*x, y2 + delta_y * x) for x in range(max(n_rows, n_cols))])

            for antinode in total_antinodes:
                a_x, a_y = antinode
                # Dont need to check for "#" since an antinode can fall at the same spot than an antenna
                if 0 <= a_x < n_rows and 0 <= a_y < n_cols :
                    grid[a_x][a_y] = "#"
                    ANTINODES.add(antinode)

import pprint
grid  = ["".join(x) for x in grid]
pprint.pprint(grid)
print(len(ANTINODES))
