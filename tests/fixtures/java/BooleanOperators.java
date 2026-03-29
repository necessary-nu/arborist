// Expected metrics:
//   cyclomatic_complexity: TBD
//   cognitive_complexity: TBD
//   lines_of_code: TBD
//   nesting_depth: TBD

class BooleanOperators {
    boolean checkAll(boolean a, boolean b, boolean c) {
        if (a && b && c) {
            return true;
        }
        return false;
    }

    boolean checkMixed(boolean a, boolean b, boolean c) {
        if (a && b || c) {
            return true;
        }
        return false;
    }
}
