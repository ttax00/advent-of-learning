#!./.env/bin/python3
import os
import numpy
with open(os.path.dirname(__file__) + '/input.txt') as input:
    inputs = input.readlines()
    numbers = [int(i) for i in inputs]

diffs = numpy.diff(numbers)
increased_count = len([x for x in diffs if x > 0])
print(increased_count)