with open('2015/day7/input.txt') as txt:
	lines = txt.readlines()
# with 'input' as array of lines

def parseInstruction(instruction: str): 
	return instruction.split(' ');


for instruction in lines:
	print(instruction)
	print(parseInstruction(instruction))