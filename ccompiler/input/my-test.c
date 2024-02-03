#include "my-test-ifc.h"
#define LOOP_LIMIT 100
int mod(int a, int b, int loop_limit)
{
    int i;
    for (i = 0; i < loop_limit; i += 1) {
        if (a >= b)
            a -= b;
    }
    return a;
}

void outsource(struct Input *input, struct Output *output)
{
    // output->x = (input->a + 5) == (input->b * 2);
    int a, b, c;
    struct Helper helper;
    int d = input->i2;
    
    a = input->i1 + mod(d, 2, LOOP_LIMIT);
    b = input->i1 * input->i2;
    if (a > b)
    {
        c = 1;
        helper.x = 1;
    }
    else 
    {
        c = 2;
        helper.y = 2;
    }
    output->o1 = a;
    output->o2 = b;
    output->o3 = c;
    output->o4[0] = output->o1;
    output->o4[1] = output->o2;
    output->o5.x = output->o1 + output->o2;
    output->o5.y = output->o1 * output->o2;
    output->o6 = helper;
}