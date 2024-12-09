

lines = open("../test1.txt").read().strip().split("\n")
# lines = open("../prod1.txt").read().strip().split("\n")

grid = open("../test1.txt").read().strip().split("\n")

def _turn_right(grid: list[list[int]], curr: tuple[int, int], obstacle: tuple[int, int]) -> list[list[int]]:
    # TODO: need to find a way to update the grid (walker) with the new pos
    # If there is no obstacule, keep going forward (need the previous dir)
    # If there is an obstacule, turn right 90 degrees (need the previous dir)

    # # Keep going forward case
    # if pos.next == ".":

    #<^^>v
    
    # if pos.next == "#":

    # # Obstacle case
    # if curr[0] == obstacle[0] and obstacle[1] == curr[1] - 1: 
    return grid


ans = 0
n_rows = len(grid)
n_cols = len(grid[0])

for i in range(n_rows):
    for j in range(n_cols):
        if grid[i][j] == ">":
            pass
# print(grid)


# print(ans)
