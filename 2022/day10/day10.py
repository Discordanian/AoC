#!/usr/local/bin/python3
import sys
import numpy as np


def signal(x, vmap):
    retval = vmap[0][1]
    for r in vmap:
        if r[0] >= x:
            # print("Looking for {} got signal strength {} and returning {}".format(x,retval, retval * x))
            return retval * x
        retval = r[1]
    return "WHAT????"


def signal2(x, vmap):
    retval = vmap[0][1]
    for r in vmap:
        if r[0] >= x:
            return retval
        retval = r[1]
    return retval


# 20,60,100,140,180,220
def part2(commands):
    value_map = []
    cycle = 0
    x = 1
    value_map.append([cycle, x])
    for c in commands:
        if c == "noop":
            cycle += 1
            continue
        cycle += 2
        x += int(c.split()[1])
        value_map.append([cycle, x])
    # print(value_map)
    ans = [[None] * 40 for _ in range(6)]
    for row in range(6):
        for col in range(40):
            counter = row * 40 + col
            if (signal2(counter, value_map) - col) <= 1:
                ans[row][col] = "##"
            else:
                ans[row][col] = ".."
    for row in ans:
        print("".join(row))

    return 0


# 20,60,100,140,180,220
def part1(commands):
    interesting = [20, 60, 100, 140, 180, 220]
    value_map = []
    cycle = 0
    x = 1
    value_map.append([cycle, x])
    for c in commands:
        if c == "noop":
            cycle += 1
            continue
        cycle += 2
        x += int(c.split()[1])
        value_map.append([cycle, x])
    # print(value_map)
    retval = 0
    for i in interesting:
        retval += signal(i, value_map)
    return retval


def main():
    infile = sys.argv[1] if len(sys.argv) > 1 else "sample.txt"
    data = open(infile).read().strip()
    commands = [x for x in data.split("\n")]
    # print("{}".format(commands))

    print("Part 1 : {}".format(part1(commands)))
    print("Part 2 : {}".format(part2(commands)))


if __name__ == "__main__":
    main()
