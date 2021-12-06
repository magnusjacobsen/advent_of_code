counts = [0]*9
for i in  [int(x) for x in open('input').readline().split(',')]:
    counts[i] += 1

for _ in range(256):
    tmp = counts.pop(0)
    counts.append(tmp)
    counts[6] += tmp

print(sum(counts))