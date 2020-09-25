#include "wrapper.h"
#include <stdlib.h>
void free_buffer(char *b)
{
    if (b)
        free(b);
}