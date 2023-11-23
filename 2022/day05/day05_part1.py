#!/usr/local/bin/python3
def stack_count(l):
    return 9
    # return int((len(l) - 1) / 3)


def com_tuple(l):
    _, a, _, b, _, c = l.split()
    return (int(a), int(b) - 1, int(c) - 1)


def do_command(c, stacks):
    qty, src, tgt = c
    for j in range(qty):
        stacks[tgt].append(stacks[src].pop())
    return stacks


def main():
    # infile = "sample.txt"
    infile = "input.txt"

    with open(infile) as f:
        lines = f.readlines()

    num_stacks = stack_count(lines[0])
    print("Num Stacks : {}".format(num_stacks))
    stack_lines = [x for x in lines if "[" in x]
    commands = [x for x in lines if x[0] == "m"]
    stacks = [[] for s in range(num_stacks)]

    # Populate stacks

    for l in stack_lines:
        for x in range(num_stacks):
            idx = 3 * x + (x + 1)
            if l[idx] != " ":
                stacks[x].append(l[idx])

    for x in range(num_stacks):
        stacks[x].reverse()
    print(stacks)

    print(commands)

    for c in commands:
        stacks = do_command(com_tuple(c), stacks)
    print(stacks)

    for j in stacks:
        print(j.pop())


if __name__ == "__main__":
    main()
