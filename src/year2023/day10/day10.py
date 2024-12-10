from collections import defaultdict, deque


from collections import defaultdict, deque
def find_longest_distance(grid):
    """Finds the longest distance within a continuous loop of pipes.

    Args:
        grid: A list of strings representing the pipe grid.

    Returns:
        The maximum distance within the loop.
    """

    rows, cols = len(grid), len(grid[0])
    directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]

    def is_valid(x, y):
        return 0 <= x < cols and 0 <= y < rows

    def iterative_dfs(start):
        stack = [(start, 0)]
        visited = set()
        loop_tiles = set()

        while stack:
            (x, y), distance = stack.pop()
            visited.add((x, y))
            loop_tiles.add((x, y))

            for dx, dy in directions:
                nx, ny = x + dx, y + dy
                if is_valid(nx, ny) and (nx, ny) not in visited and grid[ny][nx] != '.':
                    stack.append(((nx, ny), distance + 1))

        return loop_tiles

    def bfs(start):
        queue = deque([(start, 0)])
        visited = set()
        max_distance = 0

        while queue:
            (x, y), distance = queue.popleft()
            visited.add((x, y))
            max_distance = max(max_distance, distance)

            for dx, dy in directions:
                nx, ny = x + dx, y + dy
                if is_valid(nx, ny) and (nx, ny) not in visited and grid[ny][nx] != '.':
                    queue.append(((nx, ny), distance + 1))

        return max_distance

    # Find the starting point (any tile that is not a '.' or a 'S')
    start_x, start_y = None, None
    for y in range(rows):
        for x in range(cols):
            if grid[y][x] != '.' and grid[y][x] != 'S':
                start_x, start_y = x, y
                break

    if start_x is None:
        return 0  # No pipes found

    # Find the loop using iterative DFS
    loop_tiles = iterative_dfs((start_x, start_y))

    # Calculate the longest distance within the loop
    max_distance = 0
    for x, y in loop_tiles:
        distance = bfs((x, y))
        max_distance = max(max_distance, distance)

    return max_distance

file_path = "prod1.txt"
# file_path = "test1.txt"

file_lines = open(file_path, "r").read().splitlines()

result = find_longest_distance(file_lines)
print(result)  # Output: 4
