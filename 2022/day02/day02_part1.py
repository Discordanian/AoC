#!/usr/local/bin/python3

# A X ROCK
# B Y PAPER
# C Z SCISORS


def my_point(line):
    letter = line.split()[1]
    if letter == "X":
        return 1
    if letter == "Y":
        return 2
    return 3


def win_loss(line):
    letters = line.split()
    me = letters[1]
    them = letters[0]

    if me == "X":
        if them == "A":
            return 3
        if them == "B":
            return 0
        return 6
    if me == "Y":
        if them == "A":
            return 6
        if them == "B":
            return 3
        return 0
    if them == "A":
        return 0
    if them == "B":
        return 6
    return 3


def main():
    with open("input.txt") as f:
        lines = [line.rstrip() for line in f]

    score = 0
    for line in lines:
        score += win_loss(line) + my_point(line)
    print(score)


if "__main__" == __name__:
    main()
