
# lines = open("test1.txt").readlines()
# lines = open("prod1.txt").readlines()
# lines = open("test2.txt").readlines()
lines = open("prod2.txt").readlines()


# Both part 1 and part 2 are solved here
def is_safe(line: list[int]) -> bool:
    # How to know whether they increment or not?
    diff = [x-y for x,y in zip(line[:], line[1:])]

    all_increasing = all([x > 0 for x in diff])
    all_decreasing = all([x < 0 for x in diff])
    min_one = min([abs(x) for x in diff]) >= 1
    max_three = max([abs(x) for x in diff]) <= 3
    return (all_decreasing or all_increasing) and max_three and min_one



safe_count = 0
for line in lines:
    line = [int(x) for x in line.split()]
    safe = is_safe(line)

    if not safe:
        # check all removals to see if maybe deleting one number from the sequence helps
        for i in range(len(line)):
            new_line = line[:i] + line[i+1:]
            if is_safe(new_line):
                safe = True
                break
    if safe:
        safe_count += 1

print(safe_count)
