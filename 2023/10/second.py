import sys

DIRECTIONS = ['N', 'E', 'S', 'W']

def find_start(grid):
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            if grid[i][j] == 'S':
                return (i, j)
    return (-1, -1)

def get_new_dir(i, j, dir, grid):
    match grid[i][j]:
        case '│':
            if dir == 'S':
                return (i + 1, j, dir)
            elif dir == 'N':
                return (i - 1, j, dir)
        case '─':
            if dir == 'E':
                return (i, j + 1, dir)
            elif dir == 'W':
                return (i, j - 1, dir)
        case '└':
            if dir == 'S':
                return (i, j + 1, 'E')
            if dir == 'W':
                return (i - 1, j, 'N')
        case '┘':
            if dir == 'S':
                return (i, j - 1, 'W')
            if dir == 'E':
                return (i - 1, j, 'N')
        case '┐':
            if dir == 'N':
                return (i, j - 1, 'W')
            if dir == 'E':
                return (i + 1, j, 'S')
        case '┌':
            if dir == 'N':
                return (i, j + 1, 'E')
            if dir == 'W':
                return (i + 1, j, 'S')
    return (-1, -1, '-1')

def get_lookat(i, j, dir, grid):
    lookat = []
    match grid[i][j]:
        case '│':
            if dir == 'S':
                lookat = [(0, 1)]
            elif dir == 'N':
                lookat = [(0, -1)]
        case '─':
            if dir == 'E':
                lookat = [(-1, 0)]
            elif dir == 'W':
                lookat = [(1, 0)]
        case '└':
            if dir == 'S':
                lookat = [(-1, 1)]
            if dir == 'W':
                lookat = [(1, 0), (1, -1), (0, -1)]
        case '┘':
            if dir == 'S':
                lookat = [(0, 1), (1, 1), (1, 0)]
            if dir == 'E':
                lookat = [(-1, -1)]
        case '┐':
            if dir == 'N':
                lookat = [(1, -1)]
            if dir == 'E':
                lookat = [(-1, 0), (-1, 1), (0, 1)]
        case '┌':
            if dir == 'N':
                lookat = [(0, -1), (-1, -1), (-1, 0)]
            if dir == 'W':
                lookat = [(1, 1)]
    return [(i + y, j + x) for (y, x) in lookat]

def mark_depth_first(i, j, max_i, max_j, map, mark):
    if is_inside(i, j, max_i, max_j) and map[i][j] == FREE:
        map[i][j] = mark
        mark_depth_first(i + 1, j, max_i, max_j, map, mark) # down
        mark_depth_first(i + 1, j + 1, max_i, max_j, map, mark) # down-right
        mark_depth_first(i + 1, j - 1, max_i, max_j, map, mark) # down-left
        mark_depth_first(i - 1, j, max_i, max_j, map, mark) # up
        mark_depth_first(i - 1, j + 1, max_i, max_j, map, mark) # up-right
        mark_depth_first(i - 1, j - 1, max_i, max_j, map, mark) # up-left
        mark_depth_first(i, j + 1, max_i, max_j, map, mark) # right
        mark_depth_first(i, j - 1, max_i, max_j, map, mark) # left

def mark_outside(map):
    len_y = len(map)
    len_x = len(map[0])
    for i in range(len(grid)):
        mark_depth_first(0, i, len_y, len_x, map, OUTSIDE)
        mark_depth_first(len_y - 1, i, len_y, len_x, map, OUTSIDE)
        mark_depth_first(i, 0, len_y, len_x, map, OUTSIDE)
        mark_depth_first(i, len_x - 1, len_y, len_x, map, OUTSIDE)

FREE = '░'
INSIDE = '▓'
OUTSIDE = ' '

def is_inside(i, j, max_i, max_j):
    return i >= 0 and j >= 0 and i < max_i and j < max_j

def replace(line):
    line = line.replace('-', '─')
    line = line.replace('|', '│')
    line = line.replace('L', '└')
    line = line.replace('J', '┘')
    line = line.replace('7', '┐')
    line = line.replace('F', '┌')
    return line

def get_mark(s):
    match s:
        case '─':
            return '-'
        case '│':
            return '|'
        case '└':
            return 'L'
        case '┘':
            return 'J'
        case '┐':
            return '7'
        case '┌':
            return 'F'

if __name__ == '__main__':
    sys.setrecursionlimit(200000)
    grid = [list(replace(line.strip())) for line in sys.stdin.readlines()]
    (si, sj) = find_start(grid)
    starts = [(si - 1, sj, 'N'), (si + 1, sj, 'S'), (si, sj - 1, 'W'), (si, sj + 1, 'E')]
    len_y = len(grid)
    len_x = len(grid[0])
    all_steps = []
    # ▓
    for start in starts:
        loop_grid = [[FREE for _x in range(len_x)] for _y in range(len_y)]
        steps = 1
        i, j, dir = start
        loop_grid[i][j] = grid[i][j]
        while True:
            (i, j, dir) = get_new_dir(i, j, dir, grid)
            steps += 1
            loop_grid[i][j] = grid[i][j]
            if i == -1 or j == -1 or i == len_y or j == len_x:
                steps = -1
                all_steps.append((-1, []))
                break
            elif grid[i][j] == 'S':
                all_steps.append((steps, loop_grid))
                break
        if steps > 0:
            mark_outside(loop_grid)
            i, j, dir = start
            while True: # mark to the left
                (i, j, dir) = get_new_dir(i, j, dir, grid)
                for look_i, look_j in get_lookat(i, j, dir, grid):
                    if is_inside(look_i, look_j, len_y, len_x) and loop_grid[look_i][look_j] == FREE:
                         mark_depth_first(look_i, look_j, len_y, len_x, loop_grid, INSIDE)
                if i == -1 or j == -1 or i == len_y or j == len_x:
                    break
                elif grid[i][j] == 'S':
                    break
    for step, _map in all_steps:
        print(step)
    max_steps = max([step for step, _map in all_steps])
    for steps, map in all_steps:
        if steps == max_steps:
            count_inside = 0
            for i in range(len(map)):
                replaced = ''.join(map[i])
                print(f'{i:03d} {replaced}')
                for j in range(len(map[i])):
                    if map[i][j] == INSIDE:
                        count_inside += 1
            print(f'inside: {count_inside}')
