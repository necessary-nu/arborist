// Expected metrics:
// check_all: cognitive=TBD, cyclomatic=TBD, sloc=TBD
int check_all(int a, int b, int c) {
    if (a && b && c) {
        return 1;
    }
    return 0;
}

// check_mixed: cognitive=TBD, cyclomatic=TBD, sloc=TBD
int check_mixed(int a, int b, int c) {
    if (a && b || c) {
        return 1;
    }
    return 0;
}
