import sys

r = {'a': 0,'b': 0,'c': 0,'d': 0}
l_r = ['a','b','c','d']

l = [line.strip('\n').split() for line in sys.stdin]

i = 0
while i < len(l):
    ins = l[i]
    if 'inc' == ins[0]:
        r[ins[1]] += 1
    elif 'dec' == ins[0]:
        r[ins[1]] -= 1
    elif 'cpy' == ins[0]:
        if ins[1] in l_r:
            val = r[ins[1]]
        else:
            val = int(ins[1])
        r[ins[2]] = val
    elif 'jnz' == ins[0]:
        if ins[1] in l_r:
            val = r[ins[1]]
        else:
            val = int(ins[1])
        if val != 0:
            i += int(ins[2])
            continue

    #print(i)
    i += 1

print(r)
