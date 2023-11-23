#!/usr/local/bin/python3


def in_common(first, second):
    for l in first:
        if l in second:
            return l
    print("Nothing in common?")
    return "?"


def my_ord(l):
    if l.islower():
        return ord(l) - ord("a") + 1
    return ord(l) - ord("A") + 27


def main():
    # infile = 'sample.txt'
    infile = "input.txt"
    with open(infile) as f:
        lines = [line.rstrip() for line in f]

    total_value = 0
    for line in lines:
        mid = int(len(line) / 2)
        # print(mid)
        first_half = line[:mid]
        second_half = line[-mid:]
        # print(first_half)
        # print(second_half)
        prio = in_common(first_half, second_half)
        # print(prio)
        prio_val = my_ord(prio)
        # print(prio_val)
        total_value += prio_val
    print(total_value)


if "__main__" == __name__:
    main()
