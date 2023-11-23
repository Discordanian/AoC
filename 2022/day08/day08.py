#!/usr/local/bin/python3
import sys
import numpy as np


def part1(trees):
    retval = 0
    for r in range(len(trees)):
        for c in range(len(trees[r])):
            h = trees[r][c]
            print(f"Examine ({r},{c} with value {h}")
            outside = c == 0 or c == len(trees[r]) - 1 or r == 0 or r == len(trees) - 1
            left = trees[r][1:c] and h > max(trees[r][1:c])
            right = trees[r][c + 1 : len(trees[r])] and h > max(
                trees[r][c + 1 : len(trees[r])]
            )
            down = trees[1:r][c] and h > max(trees[1:r][c])
            up = trees[r + 1 : len(trees[r])][c] and h > max(
                trees[r + 1 : len(trees[r])][c]
            )
            if outside or left or right or down or up:
                retval += 1
        return retval


def main():
    infile = sys.argv[1] if len(sys.argv) > 1 else "sample.txt"
    data = open(infile).read().strip()
    lines = [x for x in data.split("\n")]

    trees = []
    for l in lines:
        r = [int(x) for x in l]
        trees.append(r)
    print(np.matrix(trees))

    # print("Part 1 : {}".format(part1(trees)))


if __name__ == "__main__":
    main()
