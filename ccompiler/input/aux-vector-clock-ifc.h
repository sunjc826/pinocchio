#pragma once

#if !defined(VECTOR_CLOCK_SIZE)
#   define VECTOR_CLOCK_SIZE 4
#endif

#define VECTOR_CLOCK_OP_INCREMENT 0
#define VECTOR_CLOCK_OP_MERGE 1

#define DOC(arg)

#define AuxiliaryInput \
    DOC(one of VECTOR_CLOCK_OP_XXX; otherwise no-op)\
    int operation_type;\
    DOC(for increment)\
    int increment_idx;\
    DOC(for merge)\
    int other_clock[VECTOR_CLOCK_SIZE];

#define PublicIO \
    int clock[VECTOR_CLOCK_SIZE];

struct Input
{
    PublicIO
    AuxiliaryInput
};

struct Output
{
    PublicIO
};

void outsource(struct Input *input, struct Output *output);