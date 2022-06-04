import itertools
import os 

#Import input as raw string array & converts to int
inputs = open(os.path.dirname(__file__) + '/input.txt').read().split()
i = 0
while i < len(inputs):
    inputs[i] = int(inputs[i])
    i += 1 

#values
total = 2020
subLen = 3

#find all the subset with subLen
subsets = list(itertools.combinations(inputs, subLen))

#filter out bad sums
answers = []
for ss in subsets:
    if sum(ss) == total:
        answers.append(ss)
if len(answers) >= 1:
    print("there are more than 1 combination!")

#mutiply answer
result = 1
for ans in answers[0]:
    result *= ans

print(result)