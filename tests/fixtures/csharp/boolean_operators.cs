// Expected metrics:
//   cyclomatic_complexity: TBD
//   cognitive_complexity: TBD
//   lines_of_code: TBD
//   nesting_depth: TBD

class BooleanOperators {
    bool CheckAll(bool a, bool b, bool c) {
        if (a && b && c) {
            return true;
        }
        return false;
    }

    bool CheckMixed(bool a, bool b, bool c) {
        if (a && b || c) {
            return true;
        }
        return false;
    }
}
