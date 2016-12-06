import sys
from collections import Counter

l_columns = [[] for _ in range(0,9)]

for line in sys.stdin:
    l_columns = [l_columns[i] + [line[i:i+1]] for i in range(0, len(line.strip('\n')))]
print(''.join([Counter(col).most_common()[:1][0][0] for col in l_columns]))

