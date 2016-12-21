
puzzle = 5
puzzle = 3017957

# Create ring, but exclude all Elf that will have 
# their present stolen the first iteration.
# We start at 3 since the input is odd.
ring = range(3,puzzle+1,2)
no_presents = set()

while len(ring) != 1:
    for i in range(len(ring)):
        # Only let the Elf steal present if it has presents.
        if ring[i] not in no_presents:
            if i+1 < len(ring):
                no_presents.add(ring[i+1])
            else:
                no_presents.add(ring[0])
    # Exclude all elfs that no longer have any presents.
    ring = [elf for elf in ring if elf not in no_presents]
    
print(ring[0])
