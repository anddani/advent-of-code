
puzzle = 3017957

# Construct the first half of the list
ring = list(range(0,int(puzzle/2)))
no_presents = set()

# Construct the second half of the list, were we exclude the elfs
# that will get their presents stolen.
i = 1+int(puzzle/2)
while i < puzzle:
    ring.append(i)
    i += 3

i %= puzzle
# As long as there still are elfs left in the race for all the presents
while len(ring) > 1:
    # Don't forget to mark any elfs for removal!
    if i != 0:
        for x in range(0, i):
            no_presents.add(ring[x])
    
    # Marking elfs for removal!
    while i < len(ring):
        for x in range(i+1, min(i+3,len(ring))):
            no_presents.add(ring[x])
        i += 3
    
    # Prepare to start over
    i %= len(ring)
    
    # Remove elfs!
    ring = [elf for elf in ring if elf not in no_presents]

print(ring[0]+1)
