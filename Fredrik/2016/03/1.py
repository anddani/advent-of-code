import sys

valid_triangles = 0

for line in sys.stdin:
    line = line.strip('\n')
    x, y, z = sorted([int(nbr) for nbr in line.split()])
    #print(line, x, y, z)
    if x + y > z:
        valid_triangles += 1

print(valid_triangles)


