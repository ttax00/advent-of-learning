with open('2015/day1/input.txt') as txt:
	lines = txt.readlines()
# with 'input' as array of lines

UP    = "("	# go up 1 floor
DOWN  = ")"	# go down 1 floor
floor = 0   # start at floor 0

for line in lines:
	for char in line:
		if char == UP:
			floor += 1
		elif char == DOWN:
			floor -= 1

print(f"The floor is: {floor}")

		

