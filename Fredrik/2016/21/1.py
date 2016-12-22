import sys

password = 'abcde'
password = 'abcdefgh'
s = password

for line in sys.stdin:
    l = line.strip('\n').split()
    if 'swap' == l[0]:
        if 'position' == l[1]:
            X, Y = min(int(l[2]), int(l[5])), max(int(l[2]), int(l[5]))
        else:
            a, b = s.index(l[2]), s.index(l[5])
            X, Y = min(a,b), max(a,b)
        s = s[:X] + s[Y] + s[X+1:Y] + s[X] + s[Y+1:]
    elif 'rotate' == l[0]:
        if 'based' == l[1]:
            i = s.index(l[6])
            if i > 3:
                i += 1
            X = 1 + i
        else:
            X = int(l[2])
        X %= len(s)
        if 'left' == l[1]:
            s = s[X:] + s[:X]
        else:
            s = s[-X:] + s[:-X]
    elif 'reverse' == l[0]:
        X, Y = int(l[2]), int(l[4])
        s = s[:X] + s[X:Y+1][::-1] + s[Y+1:]
    elif 'move' == l[0]:
        X, Y = int(l[2]), int(l[5])
        if X > Y:
            s = s[:Y] + s[X] + s[Y:X] + s[X+1:]
        elif Y > X:
            s = s[:X] + s[X+1:Y+1] + s[X] + s[Y+1:]
print(s)

