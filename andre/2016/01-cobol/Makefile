CC=cobc
SOURCE=day1.cbl day1-2.cbl
FLAGS=-x -O

all: clean 1 2

1:
	$(CC) $(FLAGS) day1.cbl

2:
	$(CC) $(FLAGS) day1-2.cbl

clean:
	rm -f $(SOURCE:%.cbl=%)
