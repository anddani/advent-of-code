import re
import sys
import queue



d_hash = {}
c_hash = 1
l = [[] for i in range(0,5)]
floor = 1
for line in sys.stdin:
    line = line.strip('\n')
    for m in re.findall(r'a ([a-z\-]+) (\w+)', line):
        if 'microchip' == m[1]:
            l[floor].append((m[0].split('-')[0], (m[1])))
        else:
            l[floor].append(m)
            if m[0] not in d_hash:
                d_hash[m[0]] = c_hash
                c_hash += 1
    l[floor].sort()
    floor += 1

def get_hash(state):
    return hash(str(state))

def valid_state():



l_prev_states = set() 

q = queue.Queue()
q.put([l, 1, 0])

while not q.empty():
    state = q.get()

    hashed_state = get_hash(state[:-1])
    if hashed_state in l_prev_states:
        continue
    l_prev_states.add(hashed_state)

    l, f, c = state[0], state[1], state[0]
    for items in l[f]:




print(l)
