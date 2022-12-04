with open('input', 'r') as file:
    lines = file.readlines()
    current = ''
    count = 0

    group_answers = {}
    group_count = 0
    for line in lines:
        line = line.strip()
        if line == '':
            for a,c in group_answers.items():
                if c == group_count:
                    count += 1
            group_count = 0
            group_answers = {}
        else:
            group_count += 1
            for a in list(line):
                group_answers[a] = 1 + group_answers.get(a, 0)
        

    for a,c in group_answers.items():
        if c == group_count:
            count += 1
    print(count)