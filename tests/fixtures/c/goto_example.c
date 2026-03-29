// Expected metrics:
// process_with_goto: cognitive=TBD, cyclomatic=TBD, sloc=TBD
int process_with_goto(int x) {
    if (x < 0) {
        goto error;
    }
    return x * 2;
error:
    return -1;
}
