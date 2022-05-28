import re

with open('2015/day5/input.txt') as txt:
	lines = txt.readlines()
# with 'input' as array of lines

def filterVowel(arr):
	result = []
	for string in arr:
		if len(re.findall("[aeiou]", string)) >= 3:
			result.append(string)
	return result

def filterTwice(arr):
	result = []
	for string in arr:
		if re.match(".*([a-z])\\1{1,}.*", string):
			result.append(string)
	return result

def filterNot(arr):
	result = []
	for string in arr:
		if not re.match(".*(ab|cd|pq|xy).*", string):
			result.append(string)
	return result

filtered = filterVowel(filterTwice(filterNot(lines)))


print(len(filtered))

