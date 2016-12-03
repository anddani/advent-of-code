
program december3
  implicit none
  call u1
  call u2
end program

subroutine u2
  integer :: i,a1,a2,a3,b1,b2,b3,c1,c2,c3, N
  integer :: status
  integer isTriangle
  open(2, file='d3.txt')
  N = 0
  do
    read(2, '(I3,2X,I3,2X,I3/I3,2X,I3,2X,I3/I3,2X,I3,2X,I3)', iostat=status) a1,a2,a3,b1,b2,b3,c1,c2,c3
    if (status == 0) then
      N = N + isTriangle(a1, b1, c1)
      N = N + isTriangle(a2, b2, c2)
      N = N + isTriangle(a3, b3, c3)
    else
      exit
    end if
  end do
  close(2)
  print *, N
  return
end

subroutine u1
  integer :: i,a,b,c,N
  integer :: status
  integer isTriangle

  open(1, file='d3.txt')
  N = 0
  do
    read(1, '(I3,2X,I3,2X,I3)', iostat=status) a,b,c
    if (status == 0) then
      N = N + isTriangle(a,b,c)
    else
      exit
    end if
  end do
  close(1)
  print *, N
  return
end

integer function isTriangle(a,b,c)
  integer :: a,b,c
  if (a + b > c .and. a + c > b .and. c + b > a) then
    isTriangle = 1
  else
    isTriangle = 0
  end if
  return
end
