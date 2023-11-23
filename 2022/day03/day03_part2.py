#!/usr/local/bin/python3


def in_common(arr):
    for l in arr[0]:
        if l in arr[1] and l in arr[2]:
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
    index = 0
    arr = []
    prio = " "
    for line in lines:
        arr.append(line)
        index += 1
        if index % 3 == 0:
            prio = in_common(arr)
            # print(prio)
            index = 0
            arr.clear()
            total_value += my_ord(prio)

    print(total_value)


if "__main__" == __name__:
    main()
