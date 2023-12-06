import sys

def get_card_id(inp):
    return [int(id) for id in filter(lambda x: x != '', inp.split(' ')[1:])]

def get_winning_set(inp):
    winning_nums_raw = inp.split(' ')
    return set([int(num_str) for num_str in filter(lambda x: x != '', winning_nums_raw)])

def get_my_nums(inp):
    my_nums_raw = inp.split(' ')
    return [int(num_str) for num_str in filter(lambda x: x != '', my_nums_raw)]

def get_num_matches(winning_nums, my_nums):
    num_matches = 0
    for num in my_nums:
        if num in winning_nums:
            num_matches += 1
    return num_matches

if __name__ == '__main__':
    input = sys.stdin

    all_cards = []
    for line in input.readlines():
        line = line.strip()
        splitted = line.split(' | ')
        winning_side = splitted[0].split(': ')
        card_id = get_card_id(winning_side[0])
        winning_nums = get_winning_set(winning_side[1])
        my_nums = get_my_nums(splitted[1])
        num_matches = get_num_matches(winning_nums, my_nums)
        all_cards.append(num_matches)

    num_originals = len(all_cards)
    copies = [1 for _ in range(num_originals)]
    for i in range(num_originals):
        num_matches = all_cards[i]
        num_copies = copies[i]
        if num_matches > 0:
            for j in range(i + 1, min(i + num_matches + 1, num_originals)):
                copies[j] += num_copies
    print(sum(copies))