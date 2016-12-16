from hashlib import md5
import re

i = 0
keys = []
salt = 'yjdafjpo'
poss_keys = set()
end = 99999999

while True:
    if i == end:
        break

    s = (salt+str(i))
    for _ in range(0,2017):
        h = md5(s.encode('utf-8'))
        s = h.hexdigest()

    remove = []
    for poss in poss_keys:
        if len(keys) == 64:
            end = i+1000
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

keys = sorted(keys)
for i in range(1,len(keys)+1):
    print(i, keys[i-1])

