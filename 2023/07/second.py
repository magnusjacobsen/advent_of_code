import sys

NUMS = ['Z', 'Y', 'X', 'U', '9', '8', '7', '6', '5', '4', '3', '2', '1']

FIVES = [''.join([f'{i}']*5) for i in NUMS]
FOURS = [''.join([f'{i}']*4) for i in NUMS]
THREES = [''.join([f'{i}']*3) for i in NUMS]
TWOS = [''.join([f'{i}']*2) for i in NUMS]

def get_card_val(c : str) -> str:
    match c:
        case 'A': return 'Z'
        case 'K': return 'Y'
        case 'Q': return 'X'
        case 'J': return '1'
        case 'T': return 'U'
        case x if x.isdigit(): return x

def get_rank(cards, originals):
    original_order_vals = [get_card_val(c) for c in originals]
    cards = [c for c in cards]
    cards.sort(reverse=True)
    cards = ''.join([c for c in cards])
    for five in FIVES:
        if five in cards:
            return (6, original_order_vals)
    for four in FOURS:
        if four in cards:
            return (5, original_order_vals)
    for three in THREES:
        if three in cards:
            for two in TWOS:
                if two not in three and two in cards:
                    return (4, original_order_vals)
            return (3, original_order_vals)
    for two in TWOS:
        if two in cards:
            for two2 in TWOS:
                if two != two2 and two2 in cards:
                    return (2, original_order_vals)
            return (1, original_order_vals)
    return (0, original_order_vals)

if __name__ == '__main__':
    lines = [line.strip() for line in sys.stdin.readlines()]
    hands = []
    for line in lines:
        splitted = line.split(' ')
        cards = [get_card_val(card) for card in splitted[0]]
        cards.sort(reverse=True)
        card_str = ''.join([f'{c}' for c in cards])
        ranks = []
        original = splitted[0]
        for replacement in NUMS:
            new_card_str = card_str.replace('1', replacement)
            rank = get_rank(new_card_str, original)
            ranks.append(rank)
        ranks.sort(reverse=True)
        bid = int(splitted[1])
        hands.append((card_str, bid, ranks[0], original))

    hands.sort(key=lambda x: x[2])

    sum = 0
    for (i, (cards, bid, rank, original)) in enumerate(hands):
        val =  bid * (i + 1)
        sum += val
    print(sum)