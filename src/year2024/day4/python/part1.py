

# grid = open("../test1.txt").read().strip().split("\n")
grid = open("../prod1.txt").read().strip().split("\n")
ans = 0
# "X"
# "M"
# "A"
# "S"

n_rows = len(grid)
n_cols = len(grid[0])


for i in range(n_rows):
    for j in range(n_cols):
        # W->E
        if j + 3 < n_cols and grid[i][j] == "X" and grid[i][j+1] == "M" and grid[i][j+2] == "A" and grid[i][j+3] == "S":
            ans += 1
        # E->W
        if j + 3 < n_cols and grid[i][j] == "S" and grid[i][j+1] == "A" and grid[i][j+2] == "M" and grid[i][j+3] == "X":
            ans += 1

        # N->S
        if i + 3 < n_rows and grid[i][j] == "X" and grid[i+1][j] == "M" and grid[i+2][j] == "A" and grid[i+3][j] == "S":
            ans += 1
        # S->N
        if i + 3 < n_rows and grid[i][j] == "S" and grid[i+1][j] == "A" and grid[i+2][j] == "M" and grid[i+3][j] == "X":
            ans += 1

        # NW -> SE
        if i + 3 < n_rows and j + 3 < n_cols and grid[i][j] == "X" and grid[i+1][j+1] == "M" and grid[i+2][j+2] == "A" and grid[i+3][j+3] == "S":
            ans += 1
        # SE -> NW
        if i + 3 < n_rows and j + 3 < n_cols and grid[i][j] == "S" and grid[i+1][j+1] == "A" and grid[i+2][j+2] == "M" and grid[i+3][j+3] == "X":
            ans += 1

        # SW -> NE
        if i - 3 >= 0 and j + 3 < n_cols and grid[i][j] == "X" and grid[i-1][j+1] == "M" and grid[i-2][j+2] == "A" and grid[i-3][j+3] == "S":
            ans += 1
        # NE -> SW
        if i + 3 < n_rows and j - 3 >= 0 and grid[i][j] == "X" and grid[i+1][j-1] == "M" and grid[i+2][j-2] == "A" and grid[i+3][j-3] == "S":
            ans += 1


print(ans)
