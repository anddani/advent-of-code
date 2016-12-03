import sys

l = ((7, 8, 9), (4, 5, 6), (1, 2, 3))
r = 1
c = 1

code = ''

for line in sys.stdin:
  for m in line.strip():
    if m == 'L':
      c = max(0, c-1)
    elif m == 'R':
      c = min(2, c+1)
    elif m == 'D':
      r = max(0, r-1)
    elif m == 'U':
      r = min(2, r+1)
      
  code += str(l[r][c])

print(code)
  
