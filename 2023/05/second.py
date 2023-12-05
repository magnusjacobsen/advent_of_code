import sys

MAP_NAMES = [
    "soil",
    "fertilizer",
    "water",
    "light",
    "temperature",
    "humidity",
    "location"
]

def create_mapping(line):
    nums = [int(num) for num in line.split(' ')]
    dest_start = nums[0]
    source_start = nums[1]
    range = nums[2]
    return (dest_start, source_start, source_start + range)

def get_maps(lines):
    maps = {}
    current = ''
    current_mappings = []
    for line in lines[2:]:
        if line == '':
            maps[current] = current_mappings
            current = ''
            current_mappings = []
        elif current != '':
            mapping = create_mapping(line)
            current_mappings.append(mapping)
        else:
            current = line.split(' ')[0].split('-to-')[1]
    maps[current] = current_mappings
    return maps
            
def get_seed_ranges(lines):
    seed_ranges_raw = [int(num) for num in lines[0].split(': ')[1].split(' ')]
    seed_ranges_iter = iter(seed_ranges_raw)
    seed_ranges = []
    for start in seed_ranges_iter:
        amount = next(seed_ranges_iter)
        seed_ranges.append((start, amount))
    return seed_ranges

def get_mapping(seed, mappings):
    for (d_start, s_start, s_end) in mappings:
        if seed >= s_start and seed <= s_end:
            return seed + (d_start - s_start)
    return seed

if __name__ == '__main__':
    inp = sys.stdin
    lines = [line.strip() for line in inp.readlines()]
    seed_ranges = get_seed_ranges(lines)
    maps = get_maps(lines)

    min_val = sys.maxsize
    for (start, amount) in seed_ranges:
        for i in range(amount):
            seed = start + i
            for map_name in MAP_NAMES:
                mappings = maps[map_name]
                seed = get_mapping(seed, mappings)
            min_val = min(min_val, seed)
    print(min_val)