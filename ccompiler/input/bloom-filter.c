#include "bloom-filter-ifc.h"

#define LOOP_LIMIT 1000

int
runtime_mod(int a, int b, int num_loops /* must be known at compile time */)
{
    int i;
    for (i = 0; i < num_loops; i += 1) {
        if (a >= b)
            a -= b;
    }
    return a;
}

int
polynomial_hash(int power, int element[NUM_CHARS])
{
    int i;
    int hash_value = 0;
    for (i = 0; i < NUM_CHARS; i += 1)
    {
        hash_value = hash_value * power + element[i];
        /*
        hash_value is upper bounded by
        (BLOOM_FILTER_SIZE - 1) * power + MAX_CHAR

        Thus, (hash_value / BLOOM_FILTER_SIZE)
        is bounded above by
        power + (MAX_CHAR + BLOOM_FILTER_SIZE - 1) / BLOOM_FILTER_SIZE
        */
        hash_value = runtime_mod(
            hash_value, 
            BLOOM_FILTER_SIZE, 
            power + (MAX_CHAR + BLOOM_FILTER_SIZE - 1) / BLOOM_FILTER_SIZE
        );
    }
    return hash_value;
}

#define UNROLL_ARRAY_INDEXING_BEGIN(arr, iterator, idx, arr_max) \
    for (iterator = 0; iterator < arr_max; iterator += 1) \
    { \
        if (iterator == idx) \
        { // Use arr[iterator] here:
    
#define UNROLL_ARRAY_INDEXING_END(arr, iterator, idx, arr_max) \
        } \
    } \


// #define HASH_VALUES_DO_SORT

#if defined(HASH_VALUES_DO_SORT)
    void
    bubblesort(int *hash_values)
    {
        int i;
        int j;
        int tmp;
        for (i = 0; i < NUM_HASHES; i += 1)
        {
            for (j = 0; j < i; j += 1)
            {
                if (hash_values[i] < hash_values[j])
                {
                    tmp = hash_values[i];
                    hash_values[i] = hash_values[j];
                    hash_values[j] = tmp;    
                }
            }
        }
    }
#endif

void
outsource(struct Input *input, struct Output *output)
{
    int hashes[NUM_HASHES] = HASH_INITIALIZER;
    int hash_values[NUM_HASHES];
    int hash_values_iterator;
    int bloom_filter_iterator;
    int i;
    int j;
    output->maybe_exists = 1;
    for (hash_values_iterator = 0; hash_values_iterator < NUM_HASHES; hash_values_iterator += 1)
    {
        hash_values[hash_values_iterator] = polynomial_hash(
            hashes[hash_values_iterator], 
            input->element
        );
    }
#   if defined(HASH_VALUES_DO_SORT)
        bubblesort(hash_values);
        hash_values_iterator = 0;
        for (bloom_filter_iterator = 0; bloom_filter_iterator < BLOOM_FILTER_SIZE; bloom_filter_iterator += 1)
        {
            output->next_filter_state[bloom_filter_iterator] = input->current_filter_state[bloom_filter_iterator];
            if (hash_values_iterator < NUM_HASHES)
            {
                UNROLL_ARRAY_INDEXING_BEGIN(hash_values, i, hash_values_iterator, NUM_HASHES)
                    if (bloom_filter_iterator == hash_values[i])
                    {
                        if (output->next_filter_state[bloom_filter_iterator] == BLOOM_FILTER_FALSE)
                        {
                            output->maybe_exists = 0;
                        }

                        if (input->operation_type == BLOOM_FILTER_OP_INSERT)
                        {
                            output->next_filter_state[bloom_filter_iterator] = BLOOM_FILTER_TRUE;
                        }

                        hash_values_iterator += 1;
                    }

                UNROLL_ARRAY_INDEXING_END(hash_values, i, hash_values_iterator, BLOOM_FILTER_SIZE)
            }
        }
#   else
        for (bloom_filter_iterator = 0; bloom_filter_iterator < BLOOM_FILTER_SIZE; bloom_filter_iterator += 1)
        {
            for (hash_values_iterator = 0; hash_values_iterator < NUM_HASHES; hash_values_iterator += 1)
            {
                if (bloom_filter_iterator == hash_values[hash_values_iterator])
                {
                    if (output->next_filter_state[bloom_filter_iterator] == BLOOM_FILTER_FALSE)
                    {
                        output->maybe_exists = 0;
                    }
                    if (input->operation_type == BLOOM_FILTER_OP_INSERT)
                    {
                        output->next_filter_state[bloom_filter_iterator] = BLOOM_FILTER_TRUE;
                    }
                }
            }
        }
#   endif
}