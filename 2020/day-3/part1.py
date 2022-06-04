#Import input as raw string array
import os

inputs = open(os.path.dirname(__file__) + '/input.txt').read().split('\n')

#Make it a grid
i = 0
while i < len(inputs):
    inputs[i] = list(inputs[i])
    i += 1

#Solve
def Solve(right, down):
    tree = '#'
    height = len(inputs)
    width = len(inputs[0])
    modHeight = height % down
    modWidth = height % right
    x = right
    y = down
    answer = 0
    while y < height:
        if inputs[y][x] == tree:
            answer += 1
        if x+right > width -1:
            x -= width
        y += down
        x += right

    print(answer)
    return answer

print(
    Solve(1,1) * Solve(3,1) * Solve(5,1) * Solve(7,1) * Solve(1,2)
)
