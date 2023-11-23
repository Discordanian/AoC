#!/usr/local/bin/python3


def no_dupes(arr):
    for i in range(len(arr) - 1):
        for j in range(i + 1, len(arr)):
            if arr[i] == arr[j]:
                return False
    return True


def add_char(arr, c):
    arr = arr[1::]
    arr.append(c)
    return arr


def main():
    # infile = "sample2.txt"
    infile = "input.txt"

    with open(infile) as f:
        lines = f.readlines()
    l = lines[0]

    arr = []
    for i in range(14):
        arr.append(l[0])

    for i, x in enumerate(l):
        arr = add_char(arr, x)
        # print("{}".format(arr))
        if no_dupes(arr):
            print("FOUND: {}".format(i + 1))
            print("{}".format(arr))


if __name__ == "__main__":
    main()
