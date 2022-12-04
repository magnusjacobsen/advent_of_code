def bin_part(inp, lo, hi):
    if lo == hi:
        return lo
    current = inp[:1]
    if current == 'F' or current == 'L':
        return bin_part(inp[1:], lo, int((hi - lo) / 2) + lo)
    if current == 'B' or current == 'R':
        return bin_part(inp[1:], int((hi - lo + 1) / 2) + lo, hi)

with open('input', 'r') as file:
    lines = file.readlines()
    max_id = 0
    for line in lines:
        if line.strip() == '':
            continue
        row, col = bin_part(line[:7], 0, 127), bin_part(line[7:], 0, 7)
        seat_id = row * 8 + col
        if seat_id > max_id:
            max_id = seat_id
    print(max_id)