#!/usr/bin/env python
# -*- coding: utf-8 -*-

from hashlib import md5
from collections import deque
import sys

passcode = sys.stdin.readline().strip()
open_doors = set(['b', 'c', 'd', 'e', 'f'])
dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)]
dirs_c = ['U', 'D', 'L', 'R']

queue = deque([['', (0, 0)]])
paths = []

while len(queue) > 0:
    curr_path, curr_coords = queue.popleft()
    if curr_coords == (3, 3):  # Goal
        paths.append([curr_path, len(curr_path)])
    else:
        hash = md5((passcode+curr_path).encode('utf-8')).hexdigest()
        for i in range(4):
            next_pos = tuple(map(sum, zip(curr_coords, dirs[i])))
            if next_pos[0] in range(4) and next_pos[1] in range(4):
                if hash[i] in open_doors:
                    queue.append([curr_path+dirs_c[i], next_pos])
print('Part 1: ', paths[0][0])
print('Part 2: ', paths[-1][1])
