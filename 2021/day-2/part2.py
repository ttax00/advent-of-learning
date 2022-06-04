up = 'up'
down = 'down'
forward = 'forward'
h_pos = 0
aim = 0
depth = 0

with open('input.txt') as input:
    inputs = input.readlines()
    for i in inputs:
        command = i.split(' ')[0]
        val = int(i.split(' ')[1])

        if(command == up):
            aim -= val
            continue
        if(command == down):
            aim += val
            continue
        if(command == forward):
            h_pos += val
            depth += val*aim
            continue

print(depth*h_pos)