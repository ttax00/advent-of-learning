from asyncio import run_coroutine_threadsafe
import re

with open('2015/day5/input.txt') as txt:
	lines = txt.readlines()
# with 'input' as array of lines

def filterRepeatBetween(arr):
	res = []
	for string in arr:
		if re.match(".*([a-z]).\\1.*", string):
			res.append(string)
	return res
print(filterRepeatBetween(['xxaa', 'bbbb', 'xyx', 'yyay', 'ieodomkazucvgmuy']))
def filterRepeat(arr):
	res = []
	for string in arr:
		if re.match(".*([a-z][a-z]).*\\1", string):
			res.append(string)
	return res
print(filterRepeat(['aa1234543aa', 'abccc', 'ddeeddffee', 'ieodomkazucvgmuy']))
print(len(filterRepeat(filterRepeatBetween(lines))))