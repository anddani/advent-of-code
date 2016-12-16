def run():
    with open('input') as f:
        lines = f.readlines()

    grid = [[None, None, 1, None, None], \
            [None, 2, 3, 4, None], \
            [5, 6, 7, 8, 9], \
            [None, 'A', 'B', 'C', None], \
            [None, None, 'D', None, None]]

    previous_digit = None
    position = [2, 0]
    for line in lines:
        code = line.replace("\r\n", "")

        for key in code:
            try:
                if key == "U":
                    position[0] = position[0] - 1 if position[0] > 0 and grid[position[0]-1][position[1]] is not None else position[0]
                elif key == "D":
                    position[0] = position[0] + 1 if grid[position[0]+1][position[1]] is not None else position[0] 
                elif key == "R":
                    position[1] = position[1] + 1 if grid[position[0]][position[1]+1] is not None else position[1] 
                elif key == "L":
                    position[1] = position[1] - 1 if position[1] > 0 and grid[position[0]][position[1]-1] is not None else position[1]
            except IndexError:
                pass
        digit =  grid[position[0]][position[1]]
        print digit

run()