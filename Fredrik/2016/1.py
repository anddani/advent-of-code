import sys

l1 = []
l2 = []

for line in sys.stdin:
    line = line.strip('\n').split()
    l1.append([int(line[3]), int(line[6].strip(',').split('=')[1])+int(line[11].strip('.'))])
    l2.append([int(line[3]), int(line[6].strip(',').split('=')[1])+int(line[11].strip('.'))])
    
l2.append([11, 0])
    
def test(x, l):
    for disc in l:
        x += 1
        if (disc[1]+x) % disc[0] != 0:
            return False
    return True

i = 0
l = [False, False]
while True:
    if not l[0] and test(i, l1):
        l[0] = True
        print('Part1:', i)
        
    if not l[1] and test(i, l2):
        l[1] = True
        print('Part2:', i)
        
    if l[0] and l[1]:
        break

    i += 1
