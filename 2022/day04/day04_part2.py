#!/usr/local/bin/python3


def first_pair(l):
    first = l.split(",")[0].split("-")
    first[0] = int(first[0])
    first[1] = int(first[1])
    return first


def second_pair(l):
    second = l.split(",")[1].split("-")
    second[0] = int(second[0])
    second[1] = int(second[1])
    return second


def main():
    infile = "input.txt"
    # infile = 'sample.txt'
    with open(infile) as f:
        lines = [line.rstrip() for line in f]

    count = 0
    for line in lines:
        f = first_pair(line)
        s = second_pair(line)
        first_start_in = f[0] >= s[0] and f[0] <= s[1]
        first_end_in = f[1] <= s[1] and f[1] >= s[0]
        second_start_in = s[0] >= f[0] and s[0] <= f[1]
        second_end_in = s[1] <= f[1] and s[1] >= f[0]
        if first_start_in or first_end_in or second_start_in or second_end_in:
            print(
                " {} {} {} {}".format(
                    first_start_in, first_end_in, second_start_in, second_end_in
                )
            )
            print("First  : " + str(f))
            print("Second : " + str(s))
            count += 1

    print(count)


if "__main__" == __name__:
    main()
