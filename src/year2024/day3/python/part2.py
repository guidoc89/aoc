import argparse  # not used, but to change the input file?

# lines = open("../test1.txt").readlines()
# lines = open("../prod1.txt").readlines()
# lines = open("../test2.txt").readlines()
lines = open("../prod2.txt").readlines()


p = "mul("

ans = 0
enabled = True
for line in lines:
    for i in range(len(line)):
        j = i

        if line[i:].startswith("don't()"):
            enabled = False
            continue
        if line[i:].startswith("do()"):
            enabled = True
            continue

        if line[i:].startswith("mul("):
            while line[j] != ")":
                j += 1
            new_line = line[i:j+1].replace("mul(", "").replace(")","")

            try:
                if enabled:
                    l,r = new_line.split(",")
                    ans += int(l) * int(r)
            except:
                pass

print(ans)
