#!/usr/local/bin/python3


def main():
    # infile = "sample.txt"
    infile = "input.txt"

    with open(infile) as f:
        lines = f.readlines()
    l = lines[0]

    a = l[0]
    b = a
    c = a
    d = a
    for i, x in enumerate(l):
        print("{}  {},{},{},{}".format(i, a, b, c, d))
        a = b
        b = c
        c = d
        d = x
        if a != b and a != c and a != d and b != c and b != d and c != d:
            print("FOUND : {}  {},{},{},{}".format(i + 1, a, b, c, d))


if __name__ == "__main__":
    main()
