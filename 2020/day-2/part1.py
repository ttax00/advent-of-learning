#Import input as raw string array
import os

inputs = open(os.path.dirname(__file__) + '/input.txt').read().split('\n')

#split sub array into password & policy
i = 0
while i < len(inputs):
    inputs[i] = str(inputs[i]).split()
    minmax = str(inputs[i][0]).split('-')
    minmax[0] = int(minmax[0])
    minmax[1] = int(minmax[1])
    holder = []
    holder.append(minmax[0])
    holder.append(minmax[1])
    holder.append(str(inputs[i][1]).replace(':',''))
    holder.append(inputs[i][2])
    inputs[i] = holder
    i += 1
#catagorize and count characters
i=0
while i < len(inputs):
    chars = list(inputs[i][3])
    y = 0
    storeDict = {}
    while y < len(chars):
        char = chars[y]
        if char in storeDict:
            storeDict[char] += 1
        else:
            storeDict[char] = 1
        y += 1
    inputs[i].append(storeDict)
    i +=1  

#apply condition to final counts
i=0
answer = 0
while i < len (inputs):
    input = inputs[i]
    min = input[0]
    max = input[1]
    char = input [2]
    storeDict = input[4]
    if char in storeDict:
        if storeDict[char] >= min and storeDict[char] <= max:
            answer += 1
    i += 1


print(answer)