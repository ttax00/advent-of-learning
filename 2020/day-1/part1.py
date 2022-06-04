import os
import numpy

#Import input as raw string array
inputs = open(os.path.dirname(__file__) + '/input.txt').read().split()

#sum value
sum = 2020
#negative array
negatives = []

#Converts string into int and compute negatives based on string
i = 0
while i < len(inputs):
    inputs[i] = int(inputs[i])
    negatives.append(sum - inputs[i])
    i += 1 

#find the true set by intersecting inputs & negatives
answer = set(inputs).intersection(set(negatives))
answer = list(answer)

#print out answer as multiple
print(answer[0] * answer[1])
