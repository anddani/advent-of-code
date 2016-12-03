       IDENTIFICATION DIVISION.
       PROGRAM-ID. DAY-1.

       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
       SELECT SYSIN ASSIGN TO KEYBOARD ORGANIZATION LINE SEQUENTIAL.

       DATA DIVISION.
       FILE SECTION.
       FD SYSIN.
       01 ln.
           02 letters PIC X(1000).
               88 EOF VALUE HIGH-VALUES.
       WORKING-STORAGE SECTION.
       01 FILLER.
           05 w-line    PIC X(1000).
           05 w-pos     REDEFINES w-line PIC X(1) OCCURS 1000 TIMES.
           05 w-pos-num REDEFINES w-line PIC 9(1) OCCURS 1000 TIMES.
           05 w-counter PIC 9(4).
           05 w-facing  PIC 9(1) VALUE 0.
           05 w-amount  PIC 9(3) VALUE 0.
           05 w-x       PIC S9(5) VALUE 0.
           05 w-y       PIC S9(5) VALUE 0.
           05 w-result  PIC 9(5).
       PROCEDURE DIVISION.
           OPEN INPUT SYSIN
           READ SYSIN INTO w-line
               AT END SET EOF TO TRUE
           END-READ
           MOVE 1 TO w-counter
           PERFORM UNTIL EOF OR w-counter > 1000
               EVALUATE w-pos(w-counter)
                   WHEN 'L'
                       COMPUTE w-facing = FUNCTION MOD(w-facing + 3, 4)
                   WHEN 'R'
                       COMPUTE w-facing = FUNCTION MOD(w-facing + 1, 4)
                   WHEN NUMERIC
                       COMPUTE w-amount = w-amount * 10
                               + w-pos-num(w-counter)
                   WHEN ','
                       EVALUATE w-facing
      *                    NORTH
                           WHEN 0
                               COMPUTE w-y = w-y + w-amount
      *                    EAST
                           WHEN 1
                               COMPUTE w-x = w-x + w-amount
      *                    SOUTH
                           WHEN 2
                               COMPUTE w-y = w-y - w-amount
      *                    WEST
                           WHEN 3
                               COMPUTE w-x = w-x - w-amount
                       END-EVALUATE
                       MOVE 0 TO w-amount
               END-EVALUATE
               ADD 1 TO w-counter
           END-PERFORM
           COMPUTE w-result = FUNCTION ABS(w-x) + FUNCTION ABS(w-y)
           DISPLAY 'result: ' w-result
           CLOSE SYSIN
           STOP RUN.
