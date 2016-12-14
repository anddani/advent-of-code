
def run():
    with open("input") as f:
        directions = f.readlines()[0].split(", ")

    position = [0,0] 
    facing = 0 
    visited = []
    #directions = ["R2", "R2", "R2"]
    found_duplicate = False
    destination_assign2 = None

    for item in directions:
        if item[0] == 'R':
            facing = (facing + 1) % 4
        else:
            facing = (facing - 1) % 4

        for step in range(0,int(item[1:])):
            if (not found_duplicate) and (tuple(position) in visited):
                destination_assign2 = tuple(position)
                found_duplicate = True
            else:
                visited.append(tuple(position))

            if facing == 0:
                    position[1] += 1
            elif facing == 1:
                    position[0] += 1
            elif facing == 2:
                    position[1] -= 1
            elif facing == 3:
                    position[0] -= 1

    distance_assign1 = abs(position[0]) + abs(position[1])
    distance_assign2 = abs(destination_assign2[0]) + abs(destination_assign2[1])

    print "\nASSIGNMENT 1"
    print "Destination: " + str(tuple(position))
    print "Distance: " + str(distance_assign1)
    print "\nASSIGNMENT 2"
    print "Destination: " + str(destination_assign2)
    print "Distance: " + str(distance_assign2)

run()