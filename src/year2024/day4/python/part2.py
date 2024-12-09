

# grid = open("../test2.txt").read().strip().split("\n")
grid = open("../prod2.txt").read().strip().split("\n")
ans = 0
n_rows = len(grid)
n_cols = len(grid[0])


for i in range(n_rows):
    for j in range(n_cols):
        # M.S
        # .A.
        # M.S
        if i + 2 < n_rows and j + 2 < n_cols and grid[i][j] == "M" and grid[i+1][j+1] == "A" and grid[i+2][j+2] == "S" and  grid[i+2][j] == "M" and grid[i][j+2] == "S":
            ans += 1

        # M.M
        # .A.
        # S.S
        if i + 2 < n_rows and j + 2 < n_cols and grid[i][j] == "M" and grid[i+1][j+1] == "A" and grid[i+2][j+2] == "S" and  grid[i+2][j] == "S" and grid[i][j+2] == "M":
            ans += 1

        # S.S
        # .A.
        # M.M
        if i + 2 < n_rows and j + 2 < n_cols and grid[i][j] == "S" and grid[i+1][j+1] == "A" and grid[i+2][j+2] == "M" and  grid[i+2][j] == "M" and grid[i][j+2] == "S":
            ans += 1

        # S.M
        # .A.
        # S.M
        if i + 2 < n_rows and j + 2 < n_cols and grid[i][j] == "S" and grid[i+1][j+1] == "A" and grid[i+2][j+2] == "M" and  grid[i+2][j] == "S" and grid[i][j+2] == "M":
            ans += 1


print(ans)
