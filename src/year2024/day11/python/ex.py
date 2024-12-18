from collections import Counter, defaultdict

# Input reading
lines = open("../prod1.txt").read().strip()
initial_stones = [int(s) for s in lines.split()]

# Memoization for process_stone
memo = {}

def process_stone(s: int) -> list[int]:
    if s in memo:
        return memo[s]

    if s == 0:
        result = [1]
    else:
        string = str(s)
        if len(string) % 2 == 0:
            half = len(string) // 2
            result = [int(string[:half]), int(string[half:])]
        else:
            result = [s * 2024]
    
    memo[s] = result
    return result

# BLINKS iterations
BLINKS = 5
stone_counts = Counter(initial_stones)  # Count occurrences of each stone
print(stone_counts)

for _ in range(BLINKS):
    next_stone_counts = Counter()  # Track counts for the next iteration
    
    for stone, count in stone_counts.items():
        for result in process_stone(stone):
            next_stone_counts[result] += count  # Accumulate counts for resulting stones
    
    stone_counts = next_stone_counts  # Update stone counts

# Final result: total number of stones after BLINKS iterations
print(sum(stone_counts.values()))
