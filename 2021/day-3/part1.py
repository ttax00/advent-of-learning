#!./.env/bin/python3

import os

gamma = ""
with open(os.path.dirname(__file__) + '/input.txt') as input:
    inputs = input.readlines()
    for i in range(len(inputs[0])-1):
        columns = [int(b[i]) for b in inputs]
        if(sum(columns) > len(columns)/2):
            gamma += "1"
        else:
            gamma += "0"

epsilon = gamma.replace('1','x').replace('0','1').replace('x','0')
gamma = int(gamma,2)
epsilon = int(epsilon,2)
print(gamma)
print(epsilon)
print(gamma*epsilon)