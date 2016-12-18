#include <stdio.h>
#include <stdlib.h>

#define HIGHEST_BIT (1l<<63lu)
#define SET_N_HIGHEST(n) (~(0x0lu) << (64 - n))
#define ROTR(x,s,n,set) (((x) >> (n)) | ((set) & ((x) << ((s) - (n)))))
#define NUM_ROWS 6
#define NUM_COLS 50

typedef unsigned long ul;
ul rows[NUM_ROWS];

int main(int argc, const char *argv[])
{
    int i, j;
    int a, b;
    char line[100];

    while ((fgets(line, sizeof(line), stdin)) != NULL) {
        if (sscanf(line, "rect %dx%d", &a, &b)) {
            ul m = SET_N_HIGHEST(a);
            for (j = 0; j < b; ++j) {
                rows[j] |= m;
            }
        } else if (sscanf(line, "rotate row y=%d by %d", &a, &b)) {
            rows[a] = ROTR((ul)rows[a] >> (64-NUM_COLS), NUM_COLS, b, (1l << NUM_COLS) - 1) << (64-NUM_COLS);
        } else if (sscanf(line, "rotate column x=%d by %d", &a, &b)) {

            /* Shift mask bit to correct column */
            ul mask = (ul)HIGHEST_BIT >> a;
            ul col = 0l;

            /* Mask out column into col */
            for (i = 0; i < NUM_ROWS; ++i, col <<= 1) {
                col |= (rows[i] & mask) != 0;
            }
            col >>= 1;

            /* Shift column b times */
            col = ROTR(col, NUM_ROWS, b % (NUM_ROWS), (1l << NUM_ROWS) - 1);

            /* Put back column */
            for (i = NUM_ROWS-1; i >= 0; --i, col >>= 1) {
                rows[i] = (rows[i] & ~((ul)HIGHEST_BIT >> a)) | ((col & 1l) << (63-a));
            }
        }
    }

    int num_lit = 0;
    for (i = 0; i < NUM_ROWS; i++) {
        ul mask;
        for (mask = 1l; mask > 0; mask <<= 1l) {
            num_lit += (rows[i] & mask) > 0;
        }
    }

    printf("%d\n", num_lit);
    for (i = 0; i < NUM_ROWS; i++) {
        ul mask = HIGHEST_BIT;
        for (j = 0; j < NUM_COLS; j++) {
            printf("%c ", (mask & rows[i]) > 0 ? '#' : '.');
            mask >>= 1;
        }
        printf("\n");

    }
    return 0;
}
