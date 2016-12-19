#include <stdio.h>
#include <stdlib.h>

typedef struct ring_t {
    struct ring_t *next;
    struct ring_t *prev;
    int elf;
}Ring;

int main(int argc, const char *argv[])
{
    int i, j;
    int num_elves;
    scanf("%d", &num_elves);

    /* Build ring */
    Ring* head = (Ring*)malloc(num_elves * sizeof(Ring));
    Ring* curr = head;
    for (i = 0; i < num_elves-1; ++i) {
        curr->elf = i+1;
        curr->next = curr+1;
        curr->next->prev = curr;
        curr++;
    }
    curr->elf = i+1;
    curr->next = head;
    curr->next->prev = curr;

    Ring* across = head + num_elves/2;
    for (curr = head; curr->next != curr; curr = curr->next) {
        /* Remove across */
        Ring* ptr = across->prev;
        ptr->next->next->prev = ptr;
        ptr->next = ptr->next->next;

        num_elves--;
        /* Alternate shifting of across (integer division) */
        if (num_elves % 2 == 0) {
            across = across->next->next;
        } else {
            across = across->next;
        }
    }

    printf("%d\n", curr->elf);
    return 0;
}
