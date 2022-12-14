import ast
import sys
import functools

packets = []
for line in sys.stdin:
    if line.strip() == "":
        continue
    packet = ast.literal_eval(line)
    packets.append(packet)
packets.append([[6]])
packets.append([[2]])

def compare(left, right):
    if type(left) is list and type(right) is list:
        order = 0
        for j in range(len(left)):
            if (order == 0 and j >= len(right)):
                order = 1
            if order != 0:
                return order
            order = compare(left[j], right[j])
        if order == 0 and len(left) < len(right):
            return -1
        else:
            return order
    elif type(left) is list:
        return compare(left, [right])
    elif type(right) is list:
        return compare([left], right)
    else:
        if left < right:
            return -1
        elif left == right:
            return 0
        else:
            return 1

sorted_list = sorted(packets, key=functools.cmp_to_key(compare))

res = 1
for i in range(len(sorted_list)):
    if sorted_list[i] == [[2]] or sorted_list[i] == [[6]]:
        res *= (i + 1)

print(res)