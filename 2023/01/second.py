import sys

TRANSFORMS = [
    ('zero', 0),
    ('one', 1),
    ('two', 2),
    ('three', 3),
    ('four', 4),
    ('five', 5),
    ('six', 6),
    ('seven', 7),
    ('eight', 8),
    ('nine', 9)
]

def get_digit(line, first=True):
    i = 0
    length = len(line)
    while i < length:
        sub = line[i:] if first else line[:length - i - 1]
        for (key, num) in TRANSFORMS:
            if (first and sub.startswith(key)) or (not first and sub.endswith(key)):
                return num
        fst = sub[0] if first else sub[-1]
        if fst >= '0' and fst <= '9':
            return ord(fst) - ord('0')
        i += 1
    return -1

if __name__ == ('__main__'):
    input = sys.stdin
    sum = 0
    for line in input.readlines():
        first = get_digit(line, True)
        last = get_digit(line, False)
        num = first * 10 + last
        print(num)
        sum += num
    print(sum)