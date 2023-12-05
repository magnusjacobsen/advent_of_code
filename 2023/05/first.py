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
            
def get_seeds(lines):
    return [int(num) for num in lines[0].split(': ')[1].split(' ')]

def get_mapping(seed, mappings):
    for (d_start, s_start, s_end) in mappings:
        if seed >= s_start and seed <= s_end:
            return seed + (d_start - s_start)
    return seed

if __name__ == '__main__':
    inp = sys.stdin
    lines = [line.strip() for line in inp.readlines()]
    seeds = get_seeds(lines)
    maps = get_maps(lines)

    for map_name in MAP_NAMES:
        new_seeds = []
        mappings = maps[map_name]
        for seed in seeds:
            new_seed = get_mapping(seed, mappings)
            new_seeds.append(new_seed)
        seeds = new_seeds
    print(min(seeds))