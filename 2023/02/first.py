import sys

def is_possible(inp):
    for marbles in inp.replace(';', ',').split(', '):
        splitted = marbles.split(' ')
        num = int(splitted[0])
        if 'red' == splitted[1] and num > 12:
            return False
        elif 'green' == splitted[1] and num > 13:
            return False
        elif 'blue' == splitted[1] and num > 14:
            return False
    return True

if __name__ == '__main__':
    input = sys.stdin
    sum = 0
    for line in input.readlines():
        splitted = line.split(': ')
        game_id = int(splitted[0].split(' ')[1])
        if is_possible(splitted[1].strip()):
            sum += game_id
    print(sum)