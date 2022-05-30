import array
from ast import parse
from tkinter.tix import Tree
from typing import List


with open('2015/day6/input.txt') as txt:
	lines = txt.readlines()
# with 'input' as array of lines

ON     = 'turn on'
OFF    = 'turn off'
TOGGLE = 'toggle'

def parseInstruction(string):
	handle = string.replace(ON,"").replace(OFF, "").replace(TOGGLE, "").strip().split(' through ')
	return parseCord(handle)
size = 1000
grid = [[False for _ in range(size)] for _ in range(size)]

def parseCord(arr):
	res = []
	for cord in arr:
		c = cord.split(',')
		res.append([int(c[0]), int(c[1])])
	return res

def getPos(cords: array):
	res = []
	for x in range(cords[0][0], cords[1][0] +1):
		for y in range(cords[0][1], cords[1][1] +1):
			res.append([x, y])
	return res

def applyGrid(grid: array, cords, apply: bool):
	for pos in getPos(cords):
		grid[pos[0]][pos[1]] = apply
	return grid

def toggleGrid(grid, cords):
	for pos in getPos(cords):
		grid[pos[0]][pos[1]] = not grid[pos[0]][pos[1]] 
	return grid

for instruction in lines:
	if instruction.find(ON) == 0:
		applyGrid(grid, parseInstruction(instruction), True)
	elif instruction.find(OFF) == 0:
		applyGrid(grid, parseInstruction(instruction), False)
	elif instruction.find(TOGGLE) == 0:
		toggleGrid(grid, parseInstruction(instruction))
total = 0
for row in grid:
	total += sum(row)

print(total)