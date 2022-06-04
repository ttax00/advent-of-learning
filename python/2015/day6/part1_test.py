from part1 import getPos, parseInstruction, applyGrid, toggleGrid
 
def test_parse_instruction():
	"""It should parse instructions into [[x,y],[x',y']]"""
	assert parseInstruction("turn off 660,55 through 986,197") == [[660, 55],[986, 197]]
	assert parseInstruction("toggle 322,558 through 977,958") == [[322, 558],[977, 958]]
	assert parseInstruction("turn on 226,196 through 599,390") == [[226, 196],[599, 390]]

def test_getPos():
	"""It should return array of every position within the cords"""
	assert getPos([[0,0],[3,3]]) == [[0,0], [0,1], [0,2], [0,3],
									[1,0], [1,1], [1,2], [1,3],
									[2,0], [2,1], [2,2], [2,3],
									[3,0], [3,1], [3,2], [3,3]]

def test_applyGrid():
	"""It should apply value to grid in cords range and return grid"""
	assert applyGrid([[False]*10]*10, [[0,0], [9,9]], True) == [[True]*10]*10

def test_toggleGrid():
	"""It should toggle grid value in cords range"""
	assert toggleGrid([[False for _ in range(10)] for _ in range(10)], [[0,0], [9,9]]) == [[True]*10]*10
