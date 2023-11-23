#!/usr/local/bin/python3

# A  ROCK
# B  PAPER
# C  SCISORS

# X lose
# Y draw
# Z win


def my_point(line):
    letter = line.split()[1]
    if letter == "X":
        return 0
    if letter == "Y":
        return 3
    return 6


def win_loss(line):
    letters = line.split()
    me = letters[1]
    them = letters[0]

    if me == "X":  # lose
        if them == "A":
            return 3
        if them == "B":
            return 1
        return 2
    if me == "Y":  # draw
        if them == "A":
            return 1
        if them == "B":
            return 2
        return 3
    # win
    if them == "A":
        return 2
    if them == "B":
        return 3
    return 1


def main():
    with open("input.main") as f:
        lines = [line.rstrip() for line in f]

    score = 0
    for line in lines:
        score += win_loss(line) + my_point(line)
    print(score)


if "__main__" == __name__:
    main()
