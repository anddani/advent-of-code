import sys

l = ((None, None, 'D', None, None),
     (None, 'A', 'B', 'C', None),
     (5, 6, 7, 8, 9),
     (None, 2, 3, 4, None),
     (None, None, 1, None, None))
r = 2
c = 0

code = ''

fR = lambda x, y: (x+y) if (x+y) in range(0, 5) and l[(x+y)][c] else x
fC = lambda x, y: (x+y) if (x+y) in range(0, 5) and l[r][(x+y)] else x
n = 1
for line in sys.stdin:
  for m in line.strip():
    #print(r, c)
    if m == 'L':
      c = fC(c, -1)
    elif m == 'R':
      c = fC(c, 1)
    elif m == 'D':
      r = fR(r, -1)
    elif m == 'U':
      r = fR(r, 1)
    n += 1
  code += str(l[r][c])
print(code)
