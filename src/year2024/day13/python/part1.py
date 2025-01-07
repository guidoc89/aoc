
# games = open("../test1.txt").read().strip().split("\n\n")
games = open("../prod1.txt").read().strip().split("\n\n")


def _process_line(line: str) -> tuple[int, int]:
    if "button" in line.lower().strip():
        split_at = "+"
    else:
        split_at = "="

    button = line.split(":")[1].split(",")
    x = int(button[0].split(split_at)[1])
    y = int(button[1].split(split_at)[1])
    return x, y


def process_game(game: str) -> float | int:
    # Button A: X*94 + Y*22 = 8400
    # Button B: X*34 + Y*67 = 5400

    lines = game.split("\n")
    button_a = _process_line(lines[0])
    button_b = _process_line(lines[1])
    prize = _process_line(lines[2])

    # brute force (maybe add something to exit early if the condition is no longer possible, like b_presses * button_b[0] > prize[0] or something)
    cost = float("inf")
    for a_presses in range(1,101):
        for b_presses in range(1,101):
            # From the equation system
            if (a_presses*button_a[0] + b_presses*button_b[0] == prize[0]) and (a_presses*button_a[1] + b_presses*button_b[1] == prize[1]):
                cost = min(cost, 3*a_presses + 1*b_presses)

    return cost if cost != float("inf") else 0

ans = 0
for idx, game in enumerate(games):
    ans += process_game(game)


print(ans)
