import os
from telnetlib import DO
from turtle import left


with open(os.path.dirname(__file__) + '/input.txt') as txt:
	lines = txt.readlines()
# with 'input' as array of lines
UP    = '^'
DOWN  = 'v'
LEFT  = '<'
RIGHT = '>'
x = 0
y = 0
rx = 0
ry = 0
dict = {}
for line in lines:
	arr = [ch for ch in line]
	for i in range(len(arr)):
		dict[f"{x}-{y}"] = 1
		dict[f"{rx}-{ry}"] = 1
		if arr[i] == UP:
			if i % 2 == 0:
				y += 1
			else:
				ry += 1
		elif arr[i] == DOWN:
			if i % 2 == 0:
				y -= 1
			else:
				ry -= 1
		elif arr[i] == LEFT:
			if i % 2 == 0:
				x += 1
			else:
				rx += 1
		elif arr[i] == RIGHT:
			if i % 2 == 0:
				x -= 1
			else:
				rx -= 1



print(len(dict))