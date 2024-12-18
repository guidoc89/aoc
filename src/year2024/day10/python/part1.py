from collections import defaultdict, deque

# grid = open("../test1.txt").read().strip().split("\n")
grid = open("../prod1.txt").read().strip().split("\n")

n_rows = len(grid)
n_cols = len(grid[0])
grid = [[int(grid[i][j]) for j in range(n_cols)] for i in range(n_rows)]

DIRECTIONS = [(1,0), (-1,0), (0,1), (0,-1)]
ans = 0

for i in range(n_rows):
    for j in range(n_cols):
        if grid[i][j] == 0:
            q = deque([(i, j)])
            visited = set()
            while q:
                r, c = q.popleft()
                if (r, c) in visited:
                    continue
                visited.add((r,c))
                if grid[r][c] == 9:
                    ans += 1

                for delta_r, delta_c in DIRECTIONS:
                    new_r = r + delta_r
                    new_c = c + delta_c

                    if 0 <= new_r < n_rows and 0 <= new_c < n_cols and grid[new_r][new_c] == grid[r][c] + 1:
                        q.append((new_r, new_c))

print(ans)
