import ast
import sys

lines = [line for line in sys.stdin]

i = 0
pairs = []

while i < len(lines):
    left = ast.literal_eval(lines[i])
    right = ast.literal_eval(lines[i + 1])
    pairs.append((left, right))
    i += 3

def compare(left, right):
    if type(left) is list and type(right) is list:
        order = 0
        for j in range(len(left)):
            if (order == 0 and j >= len(right)):
                order = 2
            if order != 0:
                return order
            order = compare(left[j], right[j])
        if order == 0 and len(left) < len(right):
            return 1
        else:
            return order
    elif type(left) is list:
        return compare(left, [right])
    elif type(right) is list:
        return compare([left], right)
    else:
        if left < right:
            return 1
        elif left == right:
            return 0
        else:
            return 2

sum = 0
index = 1
for k in range(len(pairs)):
    left, right = pairs[k]
    if compare(left, right) == 1:
        sum += index

    index += 1
print(sum)