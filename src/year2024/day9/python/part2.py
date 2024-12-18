from collections import defaultdict, deque
import heapq

# line = open("../test2.txt").read().strip()
line = open("../prod2.txt").read().strip()

ans = 0
res = []
curr  = 0
files = []
spaces = []

file_start = 0
for i in range(1, len(line), 2):
    added_file = [str(curr) for _ in range(int(line[i-1]))]
    added_space = ["." for _ in range(int(line[i]))]

    res.extend(added_file)
    res.extend(added_space)

    # (-ID, start, length) (FOR HEAP)
    files.append((-curr, file_start, int(line[i-1])))
    # (start, length)
    spaces.append((file_start+int(line[i-1]), int(line[i])))

    file_start += int(line[i]) + int(line[i-1])
    curr += 1

if len(line) % 2 != 0:
    added_file = [str(curr) for _ in range(int(line[-1]))]
    res.extend(added_file)
    files.append((-curr, file_start, int(line[-1])))

while spaces:
    s_start, s_len = heapq.heappop(spaces)
    temp_files = []  # To store popped spaces that cant be used

    while files:
        f_id, f_start, f_len = heapq.heappop(files)
        f_id = -f_id

        # IMPORTANT: the space needs to be to the left of the original file block
        if s_len >= f_len and s_start < f_start:
            file_range = (f_start, f_start + f_len)
            space_range = (s_start, s_start + f_len)
            res[f_start:f_start + f_len] = ["." for _ in range(f_len)]
            res[s_start:s_start + f_len] = [str(f_id) for _ in range(f_len)]

            # Update remaining spaces
            if s_len > f_len:
                heapq.heappush(spaces, (s_start + f_len, s_len - f_len))
            break

        else:
            # Space is too small, store it temporarily
            temp_files.append((-f_id, f_start, f_len))

    # Push back any unused spaces
    for file in temp_files:
        heapq.heappush(files, file)


ans = 0
for idx, n in enumerate(res):
    if n ==".":
        continue
    ans += (int(n) * idx)
print(ans)
