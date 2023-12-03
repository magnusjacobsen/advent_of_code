import sys

if __name__ == ('__main__'):
    input = sys.stdin
    sum = 0
    for line in input.readlines():
        filtered = list(filter(lambda x: x.isdigit(), line))
        sum += (ord(filtered[0]) - ord('0')) * 10 + ord(filtered[-1]) - ord('0')
    print(sum)