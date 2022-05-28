with open('2015/day2/input.txt') as txt:
	lines = txt.readlines()
# with 'input' as array of lines

def calWraping(l, w, h):
	lw = l*w
	lh = l*h
	wh = w*h

	return 2*sum([lw, lh, wh]) + min(lw, lh, wh);

total = 0

for line in lines:
	line = line.replace('\n', '')
	dim = line.split('x')
	total += calWraping(int(dim[0]), int(dim[1]), int(dim[2]))

print(total)

