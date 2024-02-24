#include "aux-vector-clock-ifc.h"

#define RANGE_BEGIN(iterator, arr_max) \
    for (iterator = 0; iterator < arr_max; iterator += 1) \
    {
    
#define RANGE_END() \
    } \


int max(int a, int b)
{
    int result;
    if (a > b) result = a;
    else result = b;
    return result;
}

void outsource(struct Input *input, struct Output *output)
{
    int clock_iterator;
    if (input->operation_type == VECTOR_CLOCK_OP_INCREMENT)
    {
        RANGE_BEGIN(clock_iterator, VECTOR_CLOCK_SIZE)
            output->clock[clock_iterator] = input->clock[clock_iterator];
            if (clock_iterator == input->increment_idx)
                output->clock[clock_iterator] += 1;
        RANGE_END()
    }
    else
    if (input->operation_type == VECTOR_CLOCK_OP_MERGE)
    {
        RANGE_BEGIN(clock_iterator, VECTOR_CLOCK_SIZE)
            output->clock[clock_iterator] = max(input->clock[clock_iterator], input->other_clock[clock_iterator]);
            if (clock_iterator == input->increment_idx)
                output->clock[clock_iterator] += 1;
        RANGE_END()
    }
    else // defaults to no-op
    {
        RANGE_BEGIN(clock_iterator, VECTOR_CLOCK_SIZE)
            output->clock[clock_iterator] = input->clock[clock_iterator];
        RANGE_END()
    }
}
