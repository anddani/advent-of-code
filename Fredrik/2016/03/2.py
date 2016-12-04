import sys

valid_triangles = 0

triangles = [[], [], []]

row = 0
for line in sys.stdin:
    row += 1
    line = line.strip('\n')

    for i, nbr in enumerate(line.split()):
        triangles[i].append(int(nbr))

    if row % 3 == 0:
        for l in triangles:
            x, y, z = sorted(l)

            if x + y > z:
                valid_triangles += 1
        
        triangles = [[], [], []]

print(valid_triangles)

