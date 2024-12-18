

# grid = open("../test1.txt").read().strip().split("\n")
grid = open("../prod1.txt").read().strip().split("\n")

ans = 0
n_rows = len(grid)
n_cols = len(grid[0])

guard_pos = (0,0)
for i in range(n_rows):
    for j in range(n_cols):
        if grid[i][j] == "^":
            guard_pos = (i, j)
            break


# IN ORDER: up, right, down, left 
POSITIONS = [(-1,0), (0,1), (1,0), (0,-1)]
VISITED = set()
VISITED.add(guard_pos)
curr_direction_idx = 0

ans = 0
while True:
    delta_i, delta_j = POSITIONS[curr_direction_idx]
    next_i, next_j = guard_pos[0] + delta_i, guard_pos[1] + delta_j

    if 0 <= next_i < n_rows and 0 <= next_j < n_cols and grid[next_i][next_j] != "#":
        guard_pos = (next_i, next_j)
        VISITED.add((next_i,next_j))

    # Encountered an obstacule, need to turn to the right
    else:
        # cycle through the options IN ORDER  (0->1->2->3->0->etc)
        curr_direction_idx = (curr_direction_idx + 1) % 4

    # we exited the grid
    if not (0 <= next_i < n_rows and 0 <= next_j < n_cols):
        break

print(len(VISITED))
