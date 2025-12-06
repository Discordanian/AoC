class_name D06 extends RefCounted

var W: int = 0
var OPIDX: int = 0

func string_to_grid(d: String) -> Array[Array]:
    var retval : Array[Array] = []
    for line: String in d.split("\n"):
        var row: Array  = line.split(" ", false)
        retval.append(row)
    return retval

func col_of_numbers(grid: Array[Array], col: int) -> int:
    var retval: String = ""
    for idx:int in range(OPIDX):
        if grid[idx][col] != " ":
            retval += grid[idx][col]
    if retval.length() == 0:
        return 0 # there are no zeros in the input so this would be an empty row
    return int(retval)

func part1(data: String) -> String:
    var retval: int = 0
    var grid: Array[Array] = string_to_grid(data)
    W = grid[0].size()
    OPIDX = grid.size() -1
    print(W, " ", OPIDX)
    for col: int in range(W):
        var op: String = grid[OPIDX][col]
        var colval: int = int(grid[0][col])
        for row: int in range(1,OPIDX):
            if op == "+":
                colval += int(grid[row][col])
            else:
                colval *= int(grid[row][col])
        retval += colval
    
    
    return str(retval)
    
func part2(data: String) -> String:
    var retval: int = 0
    var grid: Array[Array] = []
    for line: String in data.split("\n"):
        var row: Array = []
        for idx: int in range(line.length()):
            row.append(line[idx])
        grid.append(row)
    W = grid[0].size()
    OPIDX = grid.size() - 1
    
    var col: int = 0
    var op: String = grid[OPIDX][col]
    var val: int = 0
    while col < W:
        var colval: int = col_of_numbers(grid, col)
        if colval == 0:
            col += 1
            op = grid[OPIDX][col]
            print("Adding val to retval ", val)
            retval += val
            val = 0
            continue
        if val == 0:
            val = colval
        else:
            if op == "*":
                val *= colval
            if op == "+":
                val += colval
        col += 1
    retval += val

    return str(retval)
