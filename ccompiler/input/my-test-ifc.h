#pragma once

struct Input
{
    int i1;
    int i2;
};

struct Helper
{
    int x;
    int y;
};

struct Output
{
    int o1;
    int o2;
    int o3;
    int o4[2];
    struct Helper o5;
    struct Helper o6;
};

// struct Input {
// 	int a;
// 	int b;
// };

// struct Output {
// 	int x;
// };


void outsource(struct Input *input, struct Output *output);