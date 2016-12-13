import queue

z = 1358

def open_space(t):
    x, y = t[0], t[1]
    if x < 0 or y < 0:
        return False
    c = x*x + 3*x + 2*x*y + y + y*y + z
    return bin(c).count('1') % 2 == 0

visited = [(1,1)]
l_direction = [[0,1],[0,-1],[1,0],[-1,0]]
goal = (31,39)

q = queue.Queue()
q.put(((1,1), 0))

while not q.empty():
    p = q.get()
    if p[0] == goal:
        print(p[1])
        break

    for new_p in [(p[0][0]+x, p[0][1]+y) for x, y in l_direction]:
        if new_p not in visited and open_space(new_p):
            q.put((new_p, p[1]+1))
            visited.append(new_p)

