#pragma once
#if !defined(NUM_HASHES)
#   define NUM_HASHES 2
#   define HASH_INITIALIZER { 7, 13 }
#elif !defined(HASH_INITIALIZER)
#   error Both NUM_HASHES and HASH_INITIALIZER must be defined!
#endif

#if !defined(BLOOM_FILTER_SIZE)
#   define BLOOM_FILTER_SIZE 16
#endif

// Each element is an ASCII char
#define ALPHABET_SIZE 128
#define MAX_CHAR (ALPHABET_SIZE - 1)

#if !defined(NUM_CHARS)
#   define NUM_CHARS 4
#endif

#define BLOOM_FILTER_OP_EXISTS 0
#define BLOOM_FILTER_OP_INSERT 1

#define BLOOM_FILTER_FALSE 0
#define BLOOM_FILTER_TRUE 1

struct Input
{
    // One of BLOOM_FILTER_OP_XXX
    int operation_type;
    // An element is represented as a word of length NUM_CHARS
    // We assume that the individual entries are relatively small and there is no integer overflow.
    int element[NUM_CHARS];
    int current_filter_state[BLOOM_FILTER_SIZE];
};

struct Output
{
    // whether Input::element may be present according to the bloom filter
    // This is a boolean and has value 0 or 1.
    // This is set for both operations EXISTS and INSERT
    int maybe_exists; 
    int next_filter_state[BLOOM_FILTER_SIZE];
};

void
outsource(struct Input *input, struct Output *output);
