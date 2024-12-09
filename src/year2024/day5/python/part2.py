
from collections import defaultdict, deque

# lines, updates = open("../test2.txt").read().strip().split("\n\n")
lines, updates = open("../prod2.txt").read().strip().split("\n\n")
lines = lines.strip().split("\n")
mappings = defaultdict(list)


visited = set()
q = deque()
topo_order = []

for line in lines:
    l, r = line.split("|")
    l, r = int(l), int(r)
    mappings[l].append(r)



ans = 0
updates = [x.split(",") for x in updates.split("\n")]
not_correct = []
corrected = []
for update in updates:
    update = [int(x) for x in update]
    i = 0
    j = 1

    correct = True
    for i,x in enumerate(update):
        for j,y in enumerate(update):
            if i < j and y not in mappings[x]:
                correct = False

    # Here, I need to sort the update (topological sort)
    if not correct:
        topo_order = []
        in_degree = {p: 0 for p in update}
        q = deque()

        for page in update:
            for neighbour in mappings[page]:
                if neighbour in update:
                    in_degree[neighbour] += 1
        q = deque([page for page in update if in_degree[page] == 0])

        while q:
            page = q.popleft()
            topo_order.append(page)

            for neighbour in mappings[page]:
                if neighbour in update:
                    in_degree[neighbour] -= 1
                    if in_degree[neighbour] == 0:
                        q.append(neighbour)
        ans += topo_order[len(topo_order)//2]

print(ans)
