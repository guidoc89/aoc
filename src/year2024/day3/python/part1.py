
# lines = open("../test1.txt").readlines()
# lines = open("../prod1.txt").readlines()
# lines = open("../test2.txt").readlines()
lines = open("../prod2.txt").readlines()

p = "mul("

ans = 0
for line in lines:
    for i in range(len(line)):
        j = i

        if line[i:].startswith("mul("):
            while line[j] != ")":
                j += 1
            new_line = line[i:j+1].replace("mul(", "").replace(")","")

            try:
                l,r = new_line.split(",")
                ans += int(l) * int(r)
            except:
                pass

print(ans)
