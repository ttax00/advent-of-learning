#!./.env/bin/python3

from os import minor
import os


with open(os.path.dirname(__file__) + '/input.txt') as input:
    inputs = input.readlines()

def widdle_oxy(arr,  index):
    answer = []
    columns = [int(b[index]) for b in arr]
    if(sum(columns) >= len(columns)/2):
        most = "1"
    else:
        most = "0"
    for b in arr:
        if(b[index] == most):
            answer.append(b)
    print(len(answer))
    print("index " + str(index))
    if(len(answer) > 1):
        return widdle_oxy(answer,  index+1)
    else:
        return answer[0]

def widdle_co2(arr,  index):
    answer = []
    columns = [int(b[index]) for b in arr]
    if(sum(columns) >= len(columns)/2):
        least = "0"
    else:
        least = "1"
    for b in arr:
        if(b[index] == least):
            answer.append(b)
    print(len(answer))
    if(len(answer) > 1):
        return widdle_co2(answer,  index+1)
    else:
        return answer[0]

oxygen = widdle_oxy(inputs, 0)
co2 = widdle_co2(inputs, 0)

oxygen = int(oxygen, 2)
co2 = int(co2, 2)
print(oxygen)
print(co2)

print(oxygen*co2)