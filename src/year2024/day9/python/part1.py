from collections import defaultdict

# line = open("../test1.txt").read().strip()
line = open("../prod1.txt").read().strip()

ans = 0
res = []
curr  = 0

for i in range(1, len(line), 2):
    for _ in range(int(line[i-1])):
        res.append(str(curr))

    for _ in range(int(line[i])):
        res.append(".")
    curr += 1

if len(line) % 2 != 0:
    for _ in range(int(line[-1])):
        res.append(str(curr))

l = 0
r = len(res) - 1
while l < r:
    if res[l] == ".":
        res[l], res[r] = res[r], "."
        r -= 1
    else:
        l += 1

for idx, n in enumerate(res):
    if n ==".":
        continue
    ans += (int(n) * idx)

print(ans)
