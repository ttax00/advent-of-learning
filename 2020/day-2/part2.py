import math
import os

#Import input as raw string array
inputs = open(os.path.dirname(__file__) + '/input.txt').read().split('\n')

#split sub array into password & policy
i = 0
answer = 0
while i < len(inputs):
    inputs[i] = str(inputs[i]).split()
    minmax = str(inputs[i][0]).split('-')
    pos1 = int(minmax[0])-1
    pos2 = int(minmax[1])-1
    char = str(inputs[i][1]).replace(':','')
    input = list(inputs[i][2])
    store = 0
    if input[pos1] == char:
        store += 1
    if input[pos2] == char:
        store += 1
    if store == 1:
        answer += 1
    i +=1

print(answer)