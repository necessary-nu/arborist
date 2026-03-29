// Expected metrics:
// process: cognitive=TBD, cyclomatic=TBD, sloc=TBD
int process(int* items, int len) {
    int total = 0;
    if (len > 0) {
        for (int i = 0; i < len; i++) {
            if (items[i] > 0) {
                total += items[i];
            }
        }
    }
    return total;
}
