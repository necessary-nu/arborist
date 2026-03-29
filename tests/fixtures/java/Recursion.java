// Expected metrics:
//   cyclomatic_complexity: TBD
//   cognitive_complexity: TBD
//   lines_of_code: TBD
//   nesting_depth: TBD

class Recursion {
    long factorial(long n) {
        if (n <= 1) {
            return 1;
        } else {
            return n * factorial(n - 1);
        }
    }
}
