import sys

if __name__ == '__main__':
    lines = [line.strip() for line in sys.stdin.readlines()]
    times = [int(num) for num in lines[0].split(' ')[1:] if num != '']
    distances = [int(num) for num in lines[1].split(' ')[1:] if num != '']
    multiply = 1
    for (best_time, best_distance) in zip(times, distances):
        num_ways = 0
        for speed in range(1, best_time):
            distance = (best_time - speed) * speed
            if distance > best_distance:
                num_ways += 1
        if num_ways > 0:
            multiply *= num_ways
    print(multiply)