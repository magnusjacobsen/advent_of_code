import sys

def get_power(inp):
    r = 0
    g = 0
    b = 0
    for marbles in inp.replace(';', ',').split(', '):
        splitted = marbles.split(' ')
        num = int(splitted[0])
        if 'red' == splitted[1]:
            r = max(r, num)
        elif 'green' == splitted[1]:
            g = max(g, num)
        elif 'blue' == splitted[1]:
            b = max(b, num)
    return r * g * b

if __name__ == '__main__':
    input = sys.stdin
    sum = 0
    for line in input.readlines():
        splitted = line.split(': ')
        game_id = int(splitted[0].split(' ')[1])
        sum += get_power(splitted[1].strip())
    print(sum)