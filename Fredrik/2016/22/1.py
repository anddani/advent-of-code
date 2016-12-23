import sys

nodes = [[None for _ in range(0,26)] for _ in range(0,38)]
list_nodes = []

def count_pairs(l):
    used = sorted(l, key=lambda x: x[2])
    avail = sorted(l, key=lambda x: x[3])
    tot = len(l)

    pairs = 0
    last = 0
    for j in range(0,tot):
        node = used[j]
        if 0 == node[2]:
            continue
        for i in range(last,tot):
            if node[2] <= avail[i][3]:
                if not (node[0]==avail[i][0] and node[1]==avail[i][1]):
                    pairs += 1
            else:
                last = i
    print(pairs)
        

for line in sys.stdin:
    l = line.strip('\n').split()
    x = int(l[0].split('-')[1][1:])
    y = int(l[0].split('-')[2][1:])
    #print(l)
    node = [int(l[2][:-1]), int(l[3][:-1])]
    #print(x,y,node)
    nodes.append(node)
    list_nodes.append([x]+[y]+node)
    #break
#print(list_nodes)
count_pairs(list_nodes)

