#!/usr/local/bin/python3
import sys
import numpy as np

directions = {"R": (1, 0), "L": (-1, 0), "U": (0, 1), "D": (0, -1)}


def pulled_to(head, tail):
    pass


def part1(commands):
    hx = hy = 0
    tx = ty = 0
    visited = set()

    for c in commands:
        print("Command {}".format(c))
        d = directions[c[0]]
        for i in range(c[1]):
            print("Direction {}, {}".format(d, i))

    return len(visited)


def main():
    infile = sys.argv[1] if len(sys.argv) > 1 else "sample.txt"
    data = open(infile).read().strip()
    commands = [[x.split()[0], int(x.split()[1])] for x in data.split("\n")]
    print("{}".format(commands))

    print("Part 1 : {}".format(part1(commands)))


if __name__ == "__main__":
    main()
