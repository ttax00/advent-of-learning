from telnetlib import DO
from turtle import left


with open('2015/day3/input.txt') as txt:
	lines = txt.readlines()
# with 'input' as array of lines
UP    = '^'
DOWN  = 'v'
LEFT  = '<'
RIGHT = '>'
x = 0
y = 0
dict = {}
for line in lines:
	for instruction in line:
		dict[f"{x}-{y}"] = 1
		if instruction == UP:
			y += 1
		elif instruction == DOWN:
			y -= 1
		elif instruction == LEFT:
			x -= 1
		elif instruction == RIGHT:
			x += 1



print(len(dict))