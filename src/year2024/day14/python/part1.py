import re
from dataclasses import dataclass

# lines = open("../test1.txt").read().strip().split("\n")
lines = open("../prod1.txt").read().strip().split("\n")

WIDTH = 101
HEIGHT = 103

@dataclass
class Robot:
    px: int
    py: int
    vx: int
    vy: int

    @classmethod
    def from_list(cls, info: list[int]) -> "Robot":
        return cls(info[0], info[1], info[2], info[3])

    def __repr__(self) -> str:
        return f"<Position ({self.px=},{self.py=})> <Velocity ({self.vx=},{self.vy=})>"

    def is_present(self, x: int, y:int) -> bool:
        return (self.px == x) and (self.py == y)

    def update_position(self, time:int = 100) -> None:
        self.px = (self.px + (time * self.vx)) % WIDTH
        self.py = (self.py + (time * self.vy)) % HEIGHT


class Grid:
    def __init__(self, width: int = WIDTH, height: int = HEIGHT) -> None:
        self.width = width
        self.height = height
        self.grid = [["." for _ in range(width)] for _ in range(height)]
        self._is_initialized = False

    @property
    def is_initialized(self) -> bool:
        return self._is_initialized

    def place_robots(self, robots: list[Robot]) -> None:
        for robot in robots:
            r_x, r_y = robot.px, robot.py
            if self.grid[r_y][r_x] ==".":
                self.grid[r_y][r_x] = "1"
            else:
                self.grid[r_y][r_x] = str(int(self.grid[r_y][r_x]) + 1)

        self._is_initialized = True

    def count_quadrants(self) -> list[int]:
        assert self.is_initialized, "Can't count quadrants when no robots have been placed"

        middle_height = self.height // 2
        middle_width = self.width // 2
        counts = [0,0,0,0]

        for i in range(self.height):
            for j in range(self.width):
                if self.grid[i][j] == ".":
                    continue

                count = int(self.grid[i][j])
                if i < middle_height and j < middle_width:
                    counts[0] += count
                elif i < middle_height and j > middle_width:
                    counts[1] += count
                elif i > middle_height and j > middle_width:
                    counts[2] += count
                elif i > middle_height and j < middle_width:
                    counts[3] += count

        return counts

robots: list[Robot] = []


for line in lines:
    nums = re.findall(r"-?\d+", line)
    robot = Robot.from_list(list(map(int, nums)))
    robots.append(robot)
    robot.update_position()

grid = Grid(WIDTH, HEIGHT)
grid.place_robots(robots)
ans = grid.count_quadrants()

final = 1
for n in ans:
    if n != 0:
        final *= n

print(final)
