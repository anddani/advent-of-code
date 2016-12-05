program d4
implicit none

type Letter
  integer ascii
  integer repeated
end type

character(100) :: line, decoded
integer :: status,i,j,ID,M
character :: checksum(5)
integer :: alphabet(26), OFFSET
logical :: correct
integer :: total
type(Letter) :: letters(5), test_letter(1), tmp(1)

total = 0
OFFSET = 96

do
  read(5, *, iostat=status) line
  if (status == 0) then

    M = scan(line, '-', .true.)

    read(line(M+1:M+3), '(I3)') ID

    do i=1,5
      checksum(i:i) = line(M+4+i:M+4+i)
    end do

    decoded = ''
    alphabet = 0
    do i=1,M-1
      if (.not. (line(i:i) == '-')) then
        decoded(i:i) = char(OFFSET+modulo(iachar(line(i:i))-OFFSET-1+ID, 26)+1)
        alphabet(iachar(line(i:i))-OFFSET) = alphabet(iachar(line(i:i))-OFFSET) + 1
      end if
    end do

    do i=1,5
      letters(i:i)%repeated=0
    end do

    do i=26,1,-1
      test_letter(1:1)%ascii = i
      test_letter(1:1)%repeated = alphabet(i:i)
      do j=1,5
        if (any(test_letter(1:1)%repeated .ge. letters(j:j)%repeated, 1)) then
          tmp(1:1) = letters(j:j)
          letters(j:j) = test_letter(1:1)
          test_letter = tmp(1:1)
        end if
      end do
    end do

    correct = .true.
    do i=1,5
      if ( .not.(any(char(letters(i:i)%ascii+OFFSET) == checksum(i:i), 1))) then
        correct = .false.
      end if
    end do

    if (correct) then
      total = total + ID
      if (index(decoded, 'pole') > 0) then
        print *, decoded, ID
      end if
    end if

  else
    exit
  end if
end do

print *, total

end program
