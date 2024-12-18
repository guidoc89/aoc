from collections import deque, defaultdict

# lines = open("../test2.txt").read().strip()
lines = open("../prod1.txt").read().strip()

stones = [int(s) for s in lines.split()]
q = deque(stones)
memo =  defaultdict(list)
ans = 0

def process_stone(s: int) -> list[int]:
    if s in memo:
        return memo[s]

    if s == 0:
        memo[s] = [1]
        return [1]
        
    string = str(s)
    if len(string) % 2 == 0:
        half = len(string) // 2
        memo[s] = [int(string[:half]), int(string[half:])]
        return [int(string[:half]), int(string[half:])]
    else:
        memo[s] = [s * 2024]
        return [s * 2024]

final = []
BLINKS = 75
for i in range(BLINKS):
    temporary = []
    # q = deque(stones)
    # while q:
    #     s = q.popleft()
    #     results = process_stone(s)
    #     temporary.extend(results)

    for s in stones:
        temporary.extend(process_stone(s))
    #
    stones = list(reversed(temporary))
    # stones = reversed(temporary)
    # if i == BLINKS - 1:
    #     final = temporary

# print(len(list(final)))
# print(len(list(final)))
print(len(stones))
