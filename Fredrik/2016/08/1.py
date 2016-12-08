import sys

def rotate(l, n):
        return l[-n:] + l[:-n]

sX = 50
sY = 6
#sX = 7
#sY = 3
#sX = 7
#sY = 3
k = [[0 for _ in range(0,sX)] for _ in range (0,sY)]
#print(k)
for line in sys.stdin:
    l = line.strip('\n').split(' ')
    if l[0] == 'rect':
        x, y = l[1].split('x')
        for a in range(0,int(y)):
            for b in range(0,int(x)):
                k[a][b] = 1
    elif l[0] == 'rotate':
        ax, nbr = l[2].split('=')
        nbr = int(nbr)

        if ax == 'x':
            n = int(l[4])
            if n > sY:
                n = int(int(l[4])/sY)
            l = [k[i][nbr] for i in range(0,sY)]
            l = rotate(l, n)
            for i in range(0, sY):
                k[i][nbr] = l[i]


        elif ax == 'y':
            #print(l[4])
            n = int(l[4])
            if n > sX:
                n = int(int(l[4])/sX)
            
            k[nbr] = rotate(k[nbr], n)


        '''if l[1] == 'column':
            ax, nbr = l[2].split('=')

            if ax == 'x':
                nbr = int(int(nbr)/sX)

            elif ax == 'y':
                nbr = int(int(nbr)/sY)


        elif l[1] == 'row':
            ax, nbr = l[2].split('=')
            if ax == 'x':
            elif ax == 'y':'''
    #print(k)

print(k)
print(sum(any(k[i][j] for j in range(0,sX)) for i in range(0,sY)))
c = 0
for l in range(0,46, 5):
    print('l:', l)
    for i in range(0,sY):
        s = ''
        for j in range(l,l+5):
            s += str(k[i][j]) 
            #print(k[i][j])
            if k[i][j]:
                c += 1
        print(s)
print(c)


