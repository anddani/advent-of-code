def run():
    with open("input") as f:
        lines = f.readlines()

    triangles = []
    [triangles.append(tuple(line.replace("\r\n", "").split())) for line in lines]
    
    valid_triangles = 0
    for i in range(0, len(triangles), 3): 
        for j in range (0, 3):
            triangle = (triangles[i][j], triangles[i+1][j], triangles[i+2][j])
            side1 = int(triangle[0])
            side2 = int(triangle[1])
            side3 = int(triangle[2])
            if (side1 + side2 > side3) and (side1 + side3 > side2) and (side2 + side3 > side1):
                valid_triangles += 1
                
    print valid_triangles

run()