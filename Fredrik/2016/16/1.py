a = '11110010111001001'

length = 272
length = 35651584

while len(a) < length:
    a += '0' + ''.join(['0' if x == '1' else '1' for x in a[::-1]])

print('test')

a = a[:length]

while len(a) % 2 == 0:
    a = ''.join(['1' if x == y else '0' for x,y in zip(a[0::2],a[1::2])])

print(a)
