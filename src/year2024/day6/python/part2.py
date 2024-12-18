

# grid = open("../test2.txt").read().strip().split("\n")
grid = open("../prod2.txt").read().strip().split("\n")

n_rows = len(grid)
n_cols = len(grid[0])
guard_pos = (0,0)
original_guard_pos = (0,0)

for i in range(n_rows):
    for j in range(n_cols):
        if grid[i][j] == "^":
            original_guard_pos = (i, j)
            break

# IN ORDER: up, right, down, left 
POSITIONS = [(-1,0), (0,1), (1,0), (0,-1)]
ans = 0

grid  = [list(x) for x in grid]
for i in range(n_rows):
    for j in range(n_cols):
        if grid[i][j] != "." or (i, j) == original_guard_pos:
            continue

        curr_direction_idx = 0
        guard_pos = original_guard_pos
        visited = set()
        grid[i][j] = "#"

        # Brute-force process
        while True:
            loop_condition = (guard_pos, curr_direction_idx)  # used to check for loops, same position and with the same direction

            if loop_condition in visited:
                ans += 1
                break

            visited.add(loop_condition)
            delta_i, delta_j = POSITIONS[curr_direction_idx] 
            next_i, next_j = guard_pos[0] + delta_i, guard_pos[1] + delta_j

            if 0 <= next_i < n_rows and 0 <= next_j < n_cols and grid[next_i][next_j] != "#":
                guard_pos = (next_i, next_j)

            else:
                curr_direction_idx = (curr_direction_idx + 1 ) % 4

            if not (0 <= next_i < n_rows and 0 <= next_j < n_cols):
                break

        # Restore original value to let other possbile grids to use it
        grid[i][j] = "."

print(ans)
