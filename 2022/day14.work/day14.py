

occupied = set()

def max_y():
    retval = 0
    for o in occupied:
        if o[1] > retval:
            retval = o[1]
    return retval

def fall(pos):
    x = pos[0]
    y = pos[1]
    for npos in [(x, y+1),(x-1, y+1), (x+1, y+1)]:
        if npos not in occupied:
            return npos
    return pos

def part2():
    count = 1
    starting_pos = (500,0)
    pos = starting_pos
    while True:
        npos = fall(pos)
        if npos == starting_pos:
            return count
        if npos == pos:
            occupied.add(npos)
            count += 1
            pos = starting_pos
            continue
        pos = npos

def part1():
    count = 1
    starting_pos = (500,0)
    pos = starting_pos
    floor = max_y()
    while True:
        npos = fall(pos)
        if npos == pos:
            occupied.add(npos)
            count += 1
            pos = starting_pos
            continue
        if npos[1] > floor:
            return count
        pos = npos

def set_occupied(start,end):
    occupied.add(start)
    occupied.add(end)
    if start[0] == end[0]:
        x = start[0]
        if start[1] < end[1]:
            y = start[1]
            for i in range(start[1], end[1] + 1):
                new_pos = (x,i)
                occupied.ad(new_pos)
        else:
            y = end[1]
            for i in range(end[1], start[1] + 1):
                new_pos = (x,i)
                occupied.ad(new_pos)
    else:

