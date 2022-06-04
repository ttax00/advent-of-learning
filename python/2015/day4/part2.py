import hashlib
import os

with open(os.path.dirname(__file__) + '/input.txt') as txt:
	lines = txt.readlines()
# with 'input' as array of lines
ZEROES = 6

for line in lines:
	i = 0
	hash = hashlib.md5(f"{line}{i}".encode()).digest().hex()

	while str(hash)[0:ZEROES] != '0'*ZEROES:
		i += 1
		string = line + str(i)
		hash = hashlib.md5(string.encode()).digest().hex()

	print(i)
	print(hash)
