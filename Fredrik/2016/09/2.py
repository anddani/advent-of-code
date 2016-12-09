import re
import sys

s = sys.stdin.readline().strip('\n')

def rec(s):
    sumS = 0
    while True:
        m = re.search(r'(\((\d+)x(\d+)\))(.*)', s)
        if not m:
            break
        length = int(m.group(2))
        times = int(m.group(3))
        s = s.replace(m.group(1),'',1)
        s = s.replace(m.group(4)[:length],'',1)
        sumS += (times*rec(m.group(4)[:length]))

    return (sumS+len(s))

print(rec(s))


