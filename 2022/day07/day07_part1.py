#!/usr/local/bin/python3

#!/usr/bin/python3
import sys
from collections import defaultdict

infile = sys.argv[1] if len(sys.argv) > 1 else "7.in"
data = open(infile).read().strip()
lines = [x for x in data.split("\n")]

# directory path -> total size of that directory (including subdirectories)
SZ = defaultdict(int)
path = []
for line in lines:
    words = line.strip().split()
    if words[1] == "cd":
        if words[2] == "..":
            path.pop()
        else:
            path.append(words[2])
    elif words[1] == "ls":
        continue
    elif words[0] == "dir":
        continue
    else:
        sz = int(words[0])
        # Add this file's size to the current directory size *and* the size of all parents
        for i in range(1, len(path) + 1):
            tgt_dir = "/".join(path[:i])
            print("Adding {} to {}".format(sz, tgt_dir))
            SZ[tgt_dir] += sz
        print("{}".format(SZ))

max_used = 70000000 - 30000000
total_used = SZ["/"]
need_to_free = total_used - max_used

p1 = 0
p2 = 1e9
for k, v in SZ.items():
    # print(k,v)
    if v <= 100000:
        print("{} has {} which is less than 100000".format(k, v))
        p1 += v
    if v >= need_to_free:
        p2 = min(p2, v)
print(p1)
print(p2)
