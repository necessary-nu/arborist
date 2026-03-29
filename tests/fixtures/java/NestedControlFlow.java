// Expected metrics:
//   cyclomatic_complexity: TBD
//   cognitive_complexity: TBD
//   lines_of_code: TBD
//   nesting_depth: TBD

class NestedControlFlow {
    int process(int[] items) {
        int total = 0;
        if (items.length > 0) {
            for (int i = 0; i < items.length; i++) {
                if (items[i] > 0) {
                    total += items[i];
                }
            }
        }
        return total;
    }
}
