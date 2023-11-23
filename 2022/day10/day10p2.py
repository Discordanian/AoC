#!/usr/local/bin/python3
import sys


def part2(commands):
    vmap = []
    cycle = 0
    x = 1
    vmap.append([cycle, x])
    for c in commands:
        if c == "noop":
            cycle += 1
            vmap.append([cycle, x])
            continue
        cycle += 1
        vmap.append([cycle, x])
        cycle += 1
        vmap.append([cycle, x])
        x += int(c.split()[1])

    for row in range(6):
        for col in range(40):
            idx = row * 40 + col + 1
            if abs(vmap[idx][1] - col) <= 1:
                print("##", end="")
            else:
                print("  ", end="")
        print("")
    return 0


# 20,60,100,140,180,220
def part1(commands):
    interesting = [20, 60, 100, 140, 180, 220]
    vmap = []
    cycle = 0
    x = 1
    vmap.append([cycle, x])
    for c in commands:
        if c == "noop":
            cycle += 1
            vmap.append([cycle, x])
            continue
        cycle += 1
        vmap.append([cycle, x])
        cycle += 1
        vmap.append([cycle, x])
        x += int(c.split()[1])
    retval = 0
    for i in interesting:
        retval += i * vmap[i][1]
    return retval


def main():
    infile = sys.argv[1] if len(sys.argv) > 1 else "sample.txt"
    data = open(infile).read().strip()
    commands = [x for x in data.split("\n")]
    # print("{}".format(commands))

    print("Part 1 : {}".format(part1(commands)))
    part2(commands)
    # print("Part 2 : {}".format(part2(commands)))


if __name__ == "__main__":
    main()
