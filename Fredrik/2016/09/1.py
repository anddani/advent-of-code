import re
import sys

s = sys.stdin.readline().strip('\n')

sumS = 0
while True:
    m = re.search(r'(\((\d+)x(\d+)\))(.*)', s)
    if not m:
        break
    length = int(m.group(2))
    times = int(m.group(3))
    sumS += (times*length)
    s = s.replace(m.group(1),'',1)
    s = s.replace(m.group(4)[:length],'',1)

print(sumS+len(s))
