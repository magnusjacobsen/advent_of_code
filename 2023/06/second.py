import sys

def get_num(line):
    parts = [num for num in line.split(' ')[1:] if num != '']
    return int("".join(parts))

if __name__ == '__main__':
    lines = [line.strip() for line in sys.stdin.readlines()]
    best_time = get_num(lines[0])
    best_distance = get_num(lines[1])
    num_ways = 0
    for speed in range(1, best_time):
        distance = (best_time - speed) * speed
        if distance > best_distance:
            num_ways += 1
    print(num_ways)