#!./.env/bin/python3
up = 'up'
down = 'down'
forward = 'forward'

with open('input.txt') as input:
    inputs = input.readlines()
    ups = [int(i.split(' ')[1]) for i in inputs if(i.split(' ')[0] == up)]
    downs = [int(i.split(' ')[1]) for i in inputs if(i.split(' ')[0] == down)]
    forwards = [int(i.split(' ')[1]) for i in inputs if(i.split(' ')[0] == forward)]

up_sum = sum(ups)
down_sum = sum(downs)
forward_sum = sum(forwards)

depth = down_sum - up_sum

print(depth * forward_sum)