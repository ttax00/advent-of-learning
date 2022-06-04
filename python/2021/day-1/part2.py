#!./.env/bin/python3
import os
import numpy

with open(os.path.dirname(__file__) + '/input.txt') as input:
    inputs = input.readlines()
    numbers = [int(i) for i in inputs]

sliding_size = 3
slided_values = numpy.convolve(numbers, [1, 1, 1],'valid')
diffs = numpy.diff(slided_values)
increased_count = len([x for x in diffs if x > 0])
print(increased_count)