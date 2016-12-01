import sys
from operator import add
from functools import reduce

moves = sys.stdin.readline().strip('\n').split(' ')
#print(moves)
direction = 0
curr = [0, 0]
dDir = {0: [1, 0], 1: [0, 1], 2: [-1, 0], 3: [0, -1]}
visited = [[0, 0]]
visited_twice = []
for move in moves:
    move = move.strip(',')
    if move[0] == 'R':
        direction = (direction + 1) % 4
    elif move[0] == 'L':
        direction = (direction - 1) % 4

    #curr = list(map(add, curr, [x * int(move[1:]) for x in dDir[direction]]))
    for x in range(1, int(move[1:])+1):
        curr = list(map(add, curr, dDir[direction]))
     
        if curr not in visited:
            visited.append(curr)
        else:
            visited_twice.append(curr)

    #print(move, curr)

print('Part One:', reduce(lambda x,y: abs(x)+abs(y), curr))
print('Part Two:', reduce(lambda x,y: abs(x)+abs(y), visited_twice[0]))

