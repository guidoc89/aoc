from collections import deque

# lines = open("../test1.txt").read().strip()
lines = open("../prod1.txt").read().strip()
stones = [int(s) for s in lines.split()]
q = deque(stones)
ans = 0


def process_stone(s: int) -> list[int]:
    if s == 0:
        return [1]
    string = str(s)
    if len(string) % 2 == 0:
        half = len(string) // 2
        return [int(string[:half]), int(string[half:])]
    else:
        return [s * 2024]

final = []
BLINKS = 25
for i in range(BLINKS):

    temporary = []
    q = deque(stones)

    while q:
        s = q.popleft()
        results = process_stone(s)
        temporary.extend(results)

    stones = reversed(temporary)

    if i == BLINKS - 1:
        final = temporary

print(len(list(final)))
