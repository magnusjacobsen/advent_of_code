import sys

lines = sys.stdin.read().split('\n')

idx = {}
conns = []

for i, line in enumerate(lines):
    if line.strip() == '':
        continue
    name = ''.join(line.split(' ')[:2])
    idx[name] = i
    conns.append([])

for i, line in enumerate(lines):
    if 'no other' in line or line.strip() == '':
        continue
    line = line.split('contain ')[1]
    if ',' in line:
        line = line.split(',')
        for x in line:
            conn = ''.join(x.strip().split(' ')[1:3])
            conns[i].append(idx[conn])
    else:
        conns[i].append(idx[''.join(line.split(' ')[1:3])])

success = 0
target = idx['shinygold']
for i in range(len(conns)):
    if target == i:
        continue
    current = i
    unvisited = [True] * len(conns)
    unvisited[current] = False
    queue = [x for x in conns[i]]
    while len(queue) > 0:
        current = queue.pop()
        if current == target:
            success += 1
            break
        if unvisited[current]:
            unvisited[current] = False
            for c in conns[current]:
                queue.append(c)

print(success)