
room = '^^.^..^.....^..^..^^...^^.^....^^^.^.^^....^.^^^...^^^^.^^^^.^..^^^^.^^.^.^.^.^.^^...^^..^^^..^.^^^^'
rows = 40

row_len = len(room)
grid = range(0,row_len)
trap_generators = ('^^.', '.^^', '^..', '..^')

for y in range(1,rows):
    # Pad the previous row with safe tiles for the walls.
    row_before = '.' + room[-row_len:] + '.'
    # Create the new row and append it to the rest of the room
    room += ''.join(['^' if row_before[i:i+3] in trap_generators else '.' for i in grid])

print(room.count('.'))
#for row in map(''.join, zip(*[iter(room)]*row_len)):
#    print(row)
