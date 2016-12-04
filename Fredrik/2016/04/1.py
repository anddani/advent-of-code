import operator
import re
import sys
from collections import Counter

def ceasar(encrypted_l_of_s, sector_ID):
    l = []
    for word in encrypted_l_of_s:
        tmp = [chr(((ord(letter)+sector_ID-97) % 26) + 97)  for letter in word]
        l.append(''.join(tmp))
    return l

regex = r'([a-z-]+)(\d+)\[(\w+)\]'

count = 0

for line in sys.stdin:
    line = line.strip('\n')

    m = re.findall(regex, line)[0]
    code = m[0].replace('-', '')
    l_code = Counter(code).most_common()

    most_common = sorted(sorted(l_code, key=operator.itemgetter(0)), key=operator.itemgetter(1), reverse=True)
    s = ''.join([x[0] for x in most_common[:5]])
    if s == m[2]:
        count += int(m[1])
        l_decrypted = ceasar([s for s in m[0].split('-')[:-1]], int(m[1]))
        if l_decrypted[0] == 'northpole':
            print(l_decrypted, m[1])

print(count)
