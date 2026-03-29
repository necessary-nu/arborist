// Expected metrics:
//   cyclomatic_complexity: TBD
//   cognitive_complexity: TBD
//   lines_of_code: TBD
//   nesting_depth: TBD

class NestedControlFlow {
    int Process(int[] items) {
        int total = 0;
        if (items.Length > 0) {
            for (int i = 0; i < items.Length; i++) {
                if (items[i] > 0) {
                    total += items[i];
                }
            }
        }
        return total;
    }
}
