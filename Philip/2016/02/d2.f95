program december2
implicit none

integer :: x, y, i, j, xn, yn
character(1000) :: line
integer :: code(5)
integer, target :: keypad(5,5), keypad2(7,7)
integer, pointer :: kp(:,:)

keypad = -1

keypad(2,2) = 7
keypad(3,2) = 8
keypad(4,2) = 9

keypad(2,3) = 4
keypad(3,3) = 5
keypad(4,3) = 6

keypad(2,4) = 1
keypad(3,4) = 2
keypad(4,4) = 3

keypad2 = -1

keypad2(4,2) = 13

keypad2(3,3) = 10
keypad2(4,3) = 11
keypad2(5,3) = 12

keypad2(2,4) = 5
keypad2(3,4) = 6
keypad2(4,4) = 7
keypad2(5,4) = 8
keypad2(6,4) = 9

keypad2(3,5) = 2
keypad2(4,5) = 3
keypad2(5,5) = 4

keypad2(4,6) = 1

do i=1,7
  !print *, keypad2(1:7,i)
end do

do i=1,5
  !print *, keypad(1:5,i)
end do


kp => keypad2
x = 2
y = 4
!x = 3
!y = 3
xn = x
yn = y

open(1, file='d2.txt')
do i = 1,5
  read(1, '(a)') line
  do j = 1,1000

    if (line(j:j) == '') then
      code(i) = kp(x, y)
      exit
    end if

    if (line(j:j) == 'U') then
      yn = y + 1
    else if (line(j:j) == 'R') then
      xn = x + 1
    else if (line(j:j) == 'D') then
      yn = y - 1
    else if (line(j:j) == 'L') then
      xn = x - 1
    end if

    if (kp(xn, yn) == -1) then
      yn = y
      xn = x
    else
      x = xn
      y = yn
    end if

  end do
end do

print *, code

end program
