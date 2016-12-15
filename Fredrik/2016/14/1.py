from hashlib import md5
import re

i = 18
keys = []
salt = 'abc'
salt = 'yjdafjpo'
poss_keys = set()

while True:
    if len(keys) == 64:
        break

    h = md5((salt+str(i)).encode('utf-8'))
    s = h.hexdigest()
    remove = []
    for poss in poss_keys:
        if len(keys) == 64:
            break
        if poss[1] < i:
            remove.append(poss)
        elif poss[0] in s:
            keys.append(poss[1]-1000)
            remove.append(poss)
            
    for x in remove:
        poss_keys.remove(x)

    for m in re.findall(r'(\w)\1\1', s)[:1]:
        poss_keys.add((m*5, i+1000))
        break

    i += 1

print(sorted(keys)[-1])

