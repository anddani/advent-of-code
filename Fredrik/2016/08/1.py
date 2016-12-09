import sys

def rotate(l, n):
    if n != 0:
        n = n % len(l)
        return l[-n:] + l[:-n]

X = 50
Y = 6
screen = [['.' for _ in range(0,X)] for _ in range (0,Y)]

for line in sys.stdin:
    splitted_line = line.strip('\n').split(' ')
    if splitted_line[0] == 'rect':
        x, y = splitted_line[1].split('x')
        for row in range(0,int(y)):
            for col in range(0,int(x)):
                screen[row][col] = '#'
    elif splitted_line[0] == 'rotate':
        axis, nbr = splitted_line[2].split('=')
        nbr = int(nbr)
        steps = int(splitted_line[4])

        if axis == 'x': #Rotate column
            col_list = [screen[row][nbr] for row in range(0,Y)]
            col_list = rotate(col_list, steps)
            for row in range(0, Y):
                screen[row][nbr] = col_list[row]
                
        elif axis == 'y': #Rotate row
            screen[nbr] = rotate(screen[nbr], steps)

# Count the lights, represented as '#'.
print('Part 1:', sum(screen[i][j] == '#' for j in range(0,X) for i in range(0,Y))) 
for row in range(0,Y):
    # Easier to read the letters
    for i in range(X,0,-5):
        screen[row] = screen[row][:i] + ['  '] + screen[row][i:] 
    print(''.join(screen[row]))
