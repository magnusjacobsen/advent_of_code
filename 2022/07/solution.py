root = (None, {}, {})
current = root
lines = open('input.txt').readlines()

for i in range(1, len(lines)):
    line = lines[i].strip().split(" ")
    match (line[0], line[1]):
        case ("$", "cd"):
            match line[2]:
                case "..":
                    current = current[0]
                case "/":
                    current = root
                case name:
                    current = current[1][name]
        case ("$", "ls"):
            ()
        case ("dir", name):
            if name not in current[1]:
                current[1][name] = (current, {}, {})
        case (filesize, filename):
            if filename not in current[2]:
                current[2][filename] = int(filesize)

def rec(folder, sizes):
    acc = 0
    for (_, sub) in folder[1].items():
        acc += rec(sub, sizes)
    for (_, size) in folder[2].items():
        acc += size
    sizes.append(acc)
    return acc

sizes = []
used = rec(root, sizes)

# part 1
print("first: ", sum(filter(lambda x: x <= 100000, sizes)))

# part 2
max_size = 70000000
update_size = 30000000
available = max_size - used
needed = update_size - available
filtered = list(filter(lambda x: x >= needed, sizes))
filtered.sort()
print("second:", filtered[0])