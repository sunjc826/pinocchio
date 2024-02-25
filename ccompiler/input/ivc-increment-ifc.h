#pragma once

#define Shared \
    int counter;

struct Input
{
    Shared
};

struct Output
{
    Shared
};

#undef Shared

void
outsource(struct Input *input, struct Output *output);
