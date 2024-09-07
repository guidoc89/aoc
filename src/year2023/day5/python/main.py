from pprint import pprint

# file_path = "test2.txt"
file_path = "prod2.txt"

file = open(file_path, "r").read().split("\n\n")

seeds, *blocks = file
seeds = [int(x) for x  in seeds.split(":")[1].strip().split(" ")]
seed_pairs =  list(zip([x for x in seeds[::2]],[x for x in seeds[1::2]]))
seed_pairs =  [(s[0], s[0]+s[1]) for s in seed_pairs]

for block in blocks:
    _,*block  = block.splitlines()
    block = [[int(x) for x in l.split(" ")] for l in block]
    new = []
    while seed_pairs:
        seed_start, seed_end = seed_pairs.pop()
        transformed = False
        for line in block:
            to, source, size = line
            inter_start = max(seed_start, source)
            inter_end = min(seed_end, source + size)
            if inter_start < inter_end:
                new.append((to + inter_start - source, to + inter_end - source))
                if seed_start < inter_start:
                    seed_pairs.append((seed_start, inter_start))
                if seed_end > inter_end:
                    seed_pairs.append((inter_end, seed_end))
                transformed = True
                break
        if not transformed:
            new.append((seed_start, seed_end))
    seed_pairs = new
print("Min seed:",min(seed_pairs)[0])

