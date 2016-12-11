import sys
import pprint
import queue

bots = {} 
pp = pprint.PrettyPrinter(indent=4)

def main():
    l_stack = []
    for line in sys.stdin:
        l = line.strip('\n').split()
        if 'bot' == l[0]:
            bots.setdefault(int(l[1]), {})
            bots[int(l[1])]['lo'] = int(l[6])
            bots[int(l[1])]['lo_out'] = (l[5] == 'output')
            bots[int(l[1])]['hi'] = int(l[11])
            bots[int(l[1])]['hi_out'] = (l[10] == 'output')
            bots[int(l[1])]['val'] = []
        elif 'value' == l[0]:
            curr_bot = l[5]
            l_stack.append(l)

    q = queue.Queue()
    c = 0
    res = [17, 61]
    d = {}

    for l in l_stack:
        cur = int(l[5])
        bots[cur]['val'].append(int(l[1]))
        q.put(cur)
        while not q.empty():
            cur = q.get()
            if len(bots[cur]['val']) > 1:
                bots[cur]['val'].sort()
                if bots[cur]['val'] == res:
                    print('Part1:', cur)
                if not bots[cur]['lo_out']:
                    q.put(bots[cur]['lo'])
                    bots[bots[cur]['lo']]['val'].append(bots[cur]['val'][0])
                if not bots[cur]['hi_out']:
                    q.put(bots[cur]['hi'])
                    bots[bots[cur]['hi']]['val'].append(bots[cur]['val'][1])
                if bots[cur]['lo_out'] and bots[cur]['lo'] in [0,1,2]:
                    d[bots[cur]['lo']] = bots[cur]['val'][0]
                if bots[cur]['hi_out'] and bots[cur]['hi'] in [0,1,2]:
                    d[bots[cur]['hi']] = bots[cur]['val'][0]
                bots[cur]['val'] = []
    return d[0]*d[1]*d[2]


print('Part2:', main())

