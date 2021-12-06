from functools import reduce
vals = list(map(int, open('input').readlines()[0].split(',')))
counts = [0]*9
for i in vals:
    counts[i] += 1
for _ in range(256):
    tmp = counts.pop(0)
    counts.append(tmp)
    counts[6] += tmp
print(sum(counts))



#print(reduce(lambda x, acc: list(map(lambda i: acc[i] + 1 if i == x else acc[i], range(9))), list(map(int, open('input').readlines()[0].split(','))), [0]*9))

