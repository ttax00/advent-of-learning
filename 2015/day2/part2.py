import statistics

with open('2015/day2/input.txt') as txt:
	lines = txt.readlines()
# with 'input' as array of lines

def calRibbon(l, w, h):
	return 2*(min(l,w,h)+statistics.median([l,w,h])) + l*w*h;

total = 0

for line in lines:
	line = line.replace('\n', '')
	dim = line.split('x')
	total += calRibbon(int(dim[0]), int(dim[1]), int(dim[2]))

print(total)
