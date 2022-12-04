import sys
print(sum([len(''.join(set(x.replace('\n','')))) for x in sys.stdin.read().split('\n\n')]))