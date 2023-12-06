import sys

if __name__ == '__main__':
    input = sys.stdin

    total_points = 0
    for line in input.readlines():
        line = line.strip()
        splitted = line.split(' | ')
        winning_side = splitted[0].split(': ')
        #card_id = int(winning_side[0].split(' ')[1])
        winning_nums_raw = winning_side[1].split(' ')
        winning_nums = set([int(num_str) for num_str in filter(lambda x: x != '', winning_nums_raw)])
        my_nums_raw = splitted[1].split(' ')
        my_nums = [int(num_str) for num_str in filter(lambda x: x != '', my_nums_raw)]
        num_matches = 0
        for num in my_nums:
            if num in winning_nums:
                num_matches += 1
        if num_matches > 0:
            points = 1
            for _i in range(1, num_matches):
                points *= 2
            total_points += points
    print(total_points)