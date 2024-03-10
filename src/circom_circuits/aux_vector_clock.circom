pragma circom 2.1.8;

include "../../circomlib/circuits/comparators.circom";
include "../../circomlib/circuits/gates.circom";

template Max() {
    signal input in[2];
    signal output out;
    // Compare 32-bit numbers
    component lt = LessThan(32);
    lt.in <== in;
    component not = NOT();
    not.in <== lt.out;
    signal step0 <== not.out * in[0];
    out <== step0 + lt.out * in[1];
}

template OperationIncrement(VECTOR_CLOCK_SIZE) {
    signal input input_clock[VECTOR_CLOCK_SIZE];
    signal input increment_idx;
    signal output output_clock[VECTOR_CLOCK_SIZE];
    component is_index[VECTOR_CLOCK_SIZE];
    for (var i = 0; i < VECTOR_CLOCK_SIZE; i++) {
        is_index[i] = IsEqual();
        is_index[i].in[0] <== i;
        is_index[i].in[1] <== increment_idx;
        output_clock[i] <== input_clock[i] + is_index[i].out;
    }
}

template OperationMerge(VECTOR_CLOCK_SIZE) {
    signal input input_clock[VECTOR_CLOCK_SIZE];
    signal input other_clock[VECTOR_CLOCK_SIZE];
    signal input increment_idx;
    signal output output_clock[VECTOR_CLOCK_SIZE];
    component max[VECTOR_CLOCK_SIZE];
    component is_index[VECTOR_CLOCK_SIZE];
    for (var i = 0; i < VECTOR_CLOCK_SIZE; i++) {
        max[i] = Max();
        max[i].in[0] <== input_clock[i];
        max[i].in[1] <== other_clock[i];
        is_index[i] = IsEqual();
        is_index[i].in[0] <== i;
        is_index[i].in[1] <== increment_idx;
        output_clock[i] <== max[i].out + is_index[i].out;
    }
}

template VectorClock(VECTOR_CLOCK_SIZE, VECTOR_CLOCK_OP_INCREMENT, VECTOR_CLOCK_OP_MERGE) {
    signal input operation_type;
    signal input increment_idx;
    signal input input_clock[VECTOR_CLOCK_SIZE];
    signal input other_clock[VECTOR_CLOCK_SIZE];

    signal output output_clock[VECTOR_CLOCK_SIZE];

    component op_increment = OperationIncrement(VECTOR_CLOCK_SIZE);
    op_increment.input_clock <== input_clock;
    op_increment.increment_idx <== increment_idx;
    
    component is_op_increment = IsEqual();
    is_op_increment.in[0] <== operation_type;
    is_op_increment.in[1] <== VECTOR_CLOCK_OP_INCREMENT;
    signal stage0[VECTOR_CLOCK_SIZE];
    for (var i = 0; i < VECTOR_CLOCK_SIZE; i++) {
        stage0[i] <== is_op_increment.out * op_increment.output_clock[i];
    }

    component op_merge = OperationMerge(VECTOR_CLOCK_SIZE);
    op_merge.input_clock <== input_clock;
    op_merge.increment_idx <== increment_idx;
    op_merge.other_clock <== other_clock;
    
    component is_op_merge = IsEqual();
    is_op_merge.in[0] <== operation_type;
    is_op_merge.in[1] <== VECTOR_CLOCK_OP_MERGE;
    signal stage1[VECTOR_CLOCK_SIZE];
    for (var i = 0; i < VECTOR_CLOCK_SIZE; i++) {
        stage1[i] <== stage0[i] + is_op_merge.out * op_merge.output_clock[i];
    }
    
    // Noop case
    signal stage2[VECTOR_CLOCK_SIZE];
    component or = OR();
    or.a <== is_op_increment.out;
    or.b <== is_op_merge.out;
    component not = NOT();
    not.in <== or.out;
    for (var i = 0; i < VECTOR_CLOCK_SIZE; i++) {
        stage2[i] <== stage1[i] + not.out * input_clock[i];
    }

    output_clock <== stage2;
}

template Main(VECTOR_CLOCK_SIZE, VECTOR_CLOCK_OP_INCREMENT, VECTOR_CLOCK_OP_MERGE) {
    signal input step_in[VECTOR_CLOCK_SIZE];
    signal output step_out[VECTOR_CLOCK_SIZE];
    signal input step_auxiliary[2 + VECTOR_CLOCK_SIZE];

    component vc = VectorClock(VECTOR_CLOCK_SIZE, VECTOR_CLOCK_OP_INCREMENT, VECTOR_CLOCK_OP_MERGE);
    vc.input_clock <== step_in;
    var i = 0;
    vc.operation_type <== step_auxiliary[i];
    i++;
    vc.increment_idx <== step_auxiliary[i];
    i++;
    for (var j = 0; j < VECTOR_CLOCK_SIZE; j++) {
        vc.other_clock[j] <== step_auxiliary[i];
        i++;
    }
    step_out <== vc.output_clock;
}

component main { public [step_in] } = Main(4, 0, 1);
