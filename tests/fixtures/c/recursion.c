// Expected metrics:
// factorial: cognitive=TBD, cyclomatic=TBD, sloc=TBD
long factorial(long n) {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}
