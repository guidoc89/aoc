
OFFSET = 10000000000000
# games = open("../test2.txt").read().strip().split("\n\n")
games = open("../prod2.txt").read().strip().split("\n\n")

def _process_line(line: str) -> tuple[int, int]:
    if "button" in line.lower().strip():
        split_at = "+"
        OFFSET = 0
    else:
        split_at = "="
        OFFSET = 10000000000000

    button = line.split(":")[1].split(",")
    x = int(button[0].split(split_at)[1]) + OFFSET
    y = int(button[1].split(split_at)[1]) + OFFSET
    return x, y


def process_game(game: str) -> float | int:
    # Button A: X*94 + Y*22 = 8400
    # Button B: X*34 + Y*67 = 5400

    lines = game.split("\n")
    button_a = _process_line(lines[0])
    button_b = _process_line(lines[1])
    prize = _process_line(lines[2])

    # Solve for: (a_presses,  b_presses)
    # a_presses * button_a[0] + b_presses * button_b[0] = prize[0] + CONST
    # a_presses * button_a[1] + b_presses * button_b[1] = prize[1] + CONST

    # Substition: a_presses = (prize[0] - b_presses*button_b[0]) / button_a[0]
    #   -> solving for "a_presses" to equal on both sides
    cost = float("inf")
    b_presses = (prize[0] * button_a[1] - prize[1] * button_a[0]) / (button_b[0] * button_a[1] - button_a[0] * button_b[1])
    a_presses = (prize[1] - b_presses * button_b[1]) / button_a[1]

    if b_presses.is_integer() and a_presses.is_integer():
        cost = min(cost, 3 * a_presses + 1 * b_presses)

    return cost if cost != float("inf") else 0

ans = 0
for idx, game in enumerate(games):
    ans += process_game(game)

print(ans)
