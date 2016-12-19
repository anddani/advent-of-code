from hashlib import md5
from queue import Queue

import re

grid = range(1,5)
directions = range(0,4)
start = (1,1)
goal = (4,4)
passcode = 'pvhmgsws'
moves = ((0,-1),(0,1),(-1,0),(1,0))
moves_l = ('U', 'D', 'L', 'R')
open_letters = 'bcdef'
q = Queue()

# Check that the new room is in the grid
room_in_grid = lambda t: all([True if x in grid else False for x in new_room])

longest_path = 0

# Starting position
q.put((start, ''))

while not q.empty():
    curr_room, curr_path = q.get()
    hash_hex = md5((passcode + curr_path).encode('utf-8')).hexdigest()
    
    # Try to move in each direction
    for i in directions:
        new_room = tuple(x+y for x,y in zip(curr_room, moves[i]))
        
        # Check that the new room is in the grid and that the door is open
        if room_in_grid(new_room) and hash_hex[i:i+1] in open_letters:
            if new_room == goal:
                if longest_path == 0:
                    print('Part1:', curr_path+moves_l[i])
                longest_path = len(curr_path+moves_l[i])
                continue
            q.put((new_room, curr_path + moves_l[i]))
            
print('Part2:', longest_path)
