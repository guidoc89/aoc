# lines, updates = open("../prod1.txt").read().strip().split("\n\n")

from collections import defaultdict

lines, updates = open("../test1.txt").read().strip().split("\n\n")
lines = lines.strip().split("\n")
mappings = defaultdict(list)


for line in lines:
    l, r = line.split("|")
    mappings[int(l)].append(int(r))


ans = 0
updates = [x.split(",") for x in updates.split("\n")]
for update in updates:
    update = [int(x) for x in update]
    i = 0
    j = 1

    correct = True
    for i,x in enumerate(update):
        for j,y in enumerate(update):
            if i < j and y not in mappings[x]:
                correct = False
    if correct:
        ans += update[len(update)//2]


print(ans)
