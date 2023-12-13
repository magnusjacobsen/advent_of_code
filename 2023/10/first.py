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
        case '|':
            if dir == 'S':
                return (i + 1, j, dir)
            elif dir == 'N':
                return (i - 1, j, dir)
        case '-':
            if dir == 'E':
                return (i, j + 1, dir)
            elif dir == 'W':
                return (i, j - 1, dir)
        case 'L':
            if dir == 'S':
                return (i, j + 1, 'E')
            if dir == 'W':
                return (i - 1, j, 'N')
        case 'J':
            if dir == 'S':
                return (i, j - 1, 'W')
            if dir == 'E':
                return (i - 1, j, 'N')
        case '7':
            if dir == 'N':
                return (i, j - 1, 'W')
            if dir == 'E':
                return (i + 1, j, 'S')
        case 'F':
            if dir == 'N':
                return (i, j + 1, 'E')
            if dir == 'W':
                return (i + 1, j, 'S')
    return (-1, -1, '-1')

if __name__ == '__main__':
    grid = [list(line.strip()) for line in sys.stdin.readlines()]
    (si, sj) = find_start(grid)
    starts = [(si - 1, sj, 'N'), (si + 1, sj, 'S'), (si, sj - 1, 'W'), (si, sj + 1, 'E')]
    #starts = [(si + 1, sj, 'S')]
    len_y = len(grid)
    len_x = len(grid[0])
    all_steps = []
    print(f'start i: {si}, start j: {sj}')
    for (i, j, dir) in starts:
        steps = 1
        print(f'step: {steps}, i: {i}, j: {j}, dir: {dir}')
        while True:
            (i, j, dir) = get_new_dir(i, j, dir, grid)
            steps += 1
            print(f'step: {steps}, i: {i}, j: {j}, dir: {dir}')
            if i == -1 or j == -1 or i == len_y or j == len_x:
                all_steps.append(-1)
                break
            elif grid[i][j] == 'S':
                all_steps.append(steps)
                break
    print(all_steps)
    print((max(all_steps) + 1) // 2)
