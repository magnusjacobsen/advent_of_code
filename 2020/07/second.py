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
            x = x.strip().split(' ')
            conn = ''.join(x[1:3])
            amount = int(x[0])
            conns[i].append((idx[conn], amount))
    else:
        x = line.split(' ')
        conn = ''.join(x[1:3])
        amount = int(x[0])
        conns[i].append((idx[conn], amount))

success = 0
target = idx['shinygold']

def rec(current):
    bags = 0
    for (conn, amount) in conns[current]:
        bags += amount + amount * rec(conn)
    return bags

print(rec(target))