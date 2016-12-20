import sys

# Get input, interpret it and sort it based on the lower endpoint.
lines = [(int(x[0]),int(x[1])) for x in (l.strip('\r\n').split('-') for l in sys.stdin.readlines())]
lines = sorted(lines, key=lambda x: (x[0]))

lower = 0
allowed = []

# For each interval
for interval in lines:
    # If we find a gap
    if interval[0] > lower+1:
        # Add all ip-addresses in the gap to our allowed list.
        for i in range(lower+1, interval[0]):
            allowed.append(i)
    # If this interval will cover longer than the last, update the lower bound.
    if interval[1] > lower:
        lower = interval[1]
    
print('Part1:', allowed[0])
print('Part2:', len(allowed))
