import re
import sys

def is_abba(l):
    return any(a == d and b == c and a != b for a, b, c, d in zip(l, l[1:], l[2:], l[3:]))

def is_bab(w1, w2):
    return any(a != b and a == c and b != ' ' and b+a+b in w2 for a,b,c in zip(w1, w1[1:], w1[2:]))

lines = []
for line in sys.stdin:
    l = re.split(r'\[([^\]]+)\]', line.strip('\n')) # Splitta på []

    # [0] kommer innehålla det som får vara ett abba, och [1] det som inte får.
    lines.append([' '.join(l[::2]), ' '.join(l[1::2])])

print(sum(is_abba(w1) and not(is_abba(w2)) for w1, w2 in lines))
#print(sum(is_bab(w1,w2) for w1, w2 in lines))
print(sum(any(a != b and a == c and b+a+b in w2 for a,b,c in zip(w1, w1[1:], w1[2:])) for w1, w2 in lines))

