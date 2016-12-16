def run():
	with open('input') as f:
		lines = f.readlines()

	numpad = {1: [-1, 1], 2: [0, 1], 3: [1, 1], 4: [-1, 0], 5: [0, 0], 6: [1, 0], 7: [-1, -1], 8: [0, -1], 9: [1, -1]}
	for line in lines:
		code = line.replace("\r\n", "")

		position = numpad[5][:]
		for key in code:
			if key == "U":
				position[1] = position[1] + 1 if position[1] < 1 else 1
			elif key == "D":
				position[1] = position[1] - 1 if position[1] > -1 else -1
			elif key == "R":
				position[0] = position[0] + 1 if position[0] < 1 else 1
			elif key == "L":
				position[0] = position[0] - 1 if position[0] > -1 else -1

		digit = [key for key, value in numpad.iteritems() if value == position][0]
		print digit

run()