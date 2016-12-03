program december1
implicit none
character(1000) :: directions
integer :: i, x, y, N, j, dir, OFFSET, k, x1,y1, first
integer :: positions(1000, 1000)
character :: turn
character(3) :: blocks

open(1, file='d1.txt')
read(1,'(a)') directions

OFFSET = 500
x = 0
y = 0
dir = 0
j = 1

first = 0
positions = 0
positions(x+OFFSET, y+OFFSET) = 1

do i=1,1000
	if (directions(i:i) == 'R') then
		turn = 'R'
		blocks = ''
	else if (directions(i:i) == 'L') then
		turn = 'L'
		blocks = ''
	else if (directions(i:i) == ',') then
		read( blocks(1:j-1), '(i10)' ) N
		!print *, turn, ' ', N, ' ', dir, ' ', x, ' ', y
		j = 1

		if (turn == 'R') then
    		dir = modulo(dir + 1, 4)
    	end if

		if (turn == 'L') then
    		dir = modulo(dir + 3, 4)
    	end if

    	do k=1,N

      		if (dir == 0) then
        		y = y + 1
    		else if (dir == 1) then
 				x = x + 1
			else if (dir == 2) then
 				y = y - 1
    		else if (dir == 3) then
 				x = x - 1
    		end if

        	if (positions(x+OFFSET, y+OFFSET) == 1 .and. first == 0 ) then
        		x1 = x
            	y1 = y
            	first = 1
        	end if

        	positions(x+OFFSET, y+OFFSET) = 1 + positions(x+OFFSET, y+OFFSET)
		end do

	else if (directions(i:i) == ' ') then
	else
		blocks(j:j) = directions(i:i)
		j = j + 1
	end if
end do


print *, abs(x) + abs(y)
print *, abs(x1) + abs(y1)

end program december1
