#!/usr/local/bin/python3
import sys

inventory = []
ops = []
test = []
truthy = []
falsy = []
inspections = []
lcm = []


def operate(item, op):
    # print("operate({},{})".format(item, op))
    local_op = op.replace("old", str(item))
    # print("Eval this: {}".format(local_op))
    return eval(local_op)


def p():
    for idx in range(len(inventory)):
        print("{} Inventory: {}".format(idx, inventory[idx]))
        print("{} Ops: {}".format(idx, ops[idx]))
        print("{} Test: {}".format(idx, test[idx]))
        print("{} Truthy: {}".format(idx, truthy[idx]))
        print("{} Falsy: {}".format(idx, falsy[idx]))
        print("{} Inspections: {}\n".format(idx, inspections[idx]))


def do_round(worry):
    for idx in range(len(inspections)):
        while inventory[idx]:
            inspections[idx] += 1
            item = inventory[idx].pop()
            item = operate(item, ops[idx])
            if worry:
                item = int(item / 3)
            item %= lcm[0]
            if item % test[idx] == 0:
                inventory[truthy[idx]].append(item)
            else:
                inventory[falsy[idx]].append(item)


def part2():
    retval = 0
    for round in range(10_000):
        do_round(False)
    # p()
    print("{}".format(inspections))
    r = sorted(inspections, reverse=True)
    print("{}".format(r))
    return r[0] * r[1]


def part1():
    retval = 0
    for round in range(20):
        do_round(True)
    # p()
    r = sorted(inspections, reverse=True)
    print("{}".format(r))
    return r[0] * r[1]


def main():
    lcm.append(1)
    infile = sys.argv[1] if len(sys.argv) > 1 else "sample.txt"
    data = open(infile).read().strip()
    lines = [x for x in data.split("\n")]

    idx = 0
    for l in lines:
        if l.startswith("M"):
            inventory.append([])
            ops.append("")
            test.append(0)
            truthy.append(0)
            falsy.append(0)
            inspections.append(0)
            idx = int(l.split()[1].split(":")[0])
        if l.startswith("  S"):
            for item in l.split(":")[1].split(","):
                inventory[idx].append(int(item))
        if l.startswith("  O"):
            ops[idx] = l.split("=")[1].strip()
        if l.startswith("  T"):
            test[idx] = int(l.split()[3])
            lcm[0] *= test[idx]
        if l.startswith("    If t"):
            truthy[idx] = int(l.split()[5])
        if l.startswith("    If f"):
            falsy[idx] = int(l.split()[5])

    # p()

    # print("Part 1 : {}".format(part1()))
    print("Part 2 : {}".format(part2()))


if __name__ == "__main__":
    main()
