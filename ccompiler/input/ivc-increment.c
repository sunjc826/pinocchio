#include "ivc-increment-ifc.h"

void
outsource(struct Input *input, struct Output *output)
{
    output->counter = input->counter + 1;
}